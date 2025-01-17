// SPDX-License-Identifier:MIT
pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import "gsn/packages/contracts/src/forwarder/IForwarder.sol";
import "gsn/packages/contracts/src/BasePaymaster.sol";

import {IOracle} from "./Oracle.sol";

// Based on https://github.com/opengsn/gsn/blob/v3.0.0-beta.10/packages/paymasters/contracts/TokenPaymaster.sol
// But modified to use an oracle rather than uniswap.

/// A very basic paymaster that makes the payer pay in ERC20 tokens.
/// - The token prices need to be provided by an IOracle.
/// - No swaps are done - the paymaster simply receives ERC20 tokens. This means
///   that over time, the paymaster's eth balance will decrease. It is up to the
///   owner of the contract to replenish the eth balance.
/// - The owner of the contract can withdraw their received erc20 balances.
/// - Users specify an erc20 address and a maximum amount they are willing to pay
///   for the tx. This reduces the trust put in the oracle.
contract OracleTokenPaymaster is BasePaymaster {
    IOracle nativeTokenOracle;
    mapping(IERC20 => TokenDetails) public tokenOracles;
    uint256 public gasUsedByPost;

    struct TokenDetails {
        uint256 div; // the scaling factor of the token (NOT oracle). Usually 1e18
        IOracle oracle;
    }

    event PreRelayPayment(uint256 ethAmount, IERC20 token, uint256 tokenAmount, address indexed payer);
    event PostRelay(uint256 actualEthAmount, IERC20 token, uint256 actualTokenAmount, address payer);

    constructor(IOracle _nativeTokenOracle) {
        require(_nativeTokenOracle.decimals() == 8, "OTP: native token oracle decimals must be 8");

        nativeTokenOracle = _nativeTokenOracle;
    }

    function versionPaymaster() external view virtual override returns (string memory) {
        return "3.0.0-beta.10+opengsn.oracle.token.ipaymaster";
    }

    function addOracle(IERC20 _token, uint256 _decimals, IOracle _oracle) external onlyOwner {
        require(_oracle.decimals() == 8, "OTP: token oracle decimals must be 8");
        tokenOracles[_token] = TokenDetails({div: 10 ** _decimals, oracle: _oracle});
    }

    function fetchPrice(IOracle _oracle) internal view returns (uint192 price) {
        (uint80 roundId, int256 answer,, uint256 updatedAt, uint80 answeredInRound) = _oracle.latestRoundData();
        require(answer > 0, "OTP : Chainlink price <= 0");
        // 2 days old price is considered stale since the price is updated every 24 hours
        require(updatedAt >= block.timestamp - 60 * 60 * 24 * 2, "OTP : Incomplete round");
        require(answeredInRound >= roundId, "OTP : Stale price");
        price = uint192(int192(answer));
    }

    function _ethToTokens(IERC20 token, uint256 ethAmount) internal view returns (uint256) {
        TokenDetails memory _tokenOracle = tokenOracles[token];
        require(address(_tokenOracle.oracle) != address(0x00), "OTP: Oracle does not exist");

        uint192 tokenPrice = fetchPrice(_tokenOracle.oracle); // #decimals: 8
        uint192 nativeAssetPrice = fetchPrice(nativeTokenOracle); // #decimals: 8
        uint192 relativePrice = (nativeAssetPrice * uint192(_tokenOracle.div)) / tokenPrice; // #decimals: _tokenOracle.div

        // #decimals: 18 + _tokenOracle.div - 18 = _tokenOracle.div
        uint256 tokenAmount = (ethAmount * relativePrice) / 1e18;

        return tokenAmount;
    }

    function setPostGasUsage(uint256 _gasUsedByPost) external onlyOwner {
        gasUsedByPost = _gasUsedByPost;
    }

    function withdrawAll(IERC20 token) external onlyOwner {
        uint256 balance = token.balanceOf(address(this));
        token.transfer(msg.sender, balance);
    }

    function getPayer(GsnTypes.RelayRequest calldata relayRequest) public view virtual returns (address) {
        (this);
        return relayRequest.request.from;
    }

    function _getPaymasterData(bytes memory paymasterData) private pure returns (IERC20 token, uint256 maxTokens) {
        (address tokenAddress, uint256 _maxTokens) = abi.decode(paymasterData, (address, uint256));

        maxTokens = _maxTokens;
        token = IERC20(tokenAddress);
    }

    function _calculatePreCharge(IERC20 token, GsnTypes.RelayRequest calldata relayRequest, uint256 maxPossibleGas)
        internal
        view
        returns (address payer, uint256 ethPrecharge, uint256 tokenPreCharge)
    {
        payer = this.getPayer(relayRequest);
        uint256 ethMaxCharge = relayHub.calculateCharge(maxPossibleGas, relayRequest.relayData);
        ethMaxCharge += relayRequest.request.value;
        tokenPreCharge = _ethToTokens(token, ethMaxCharge);
        ethPrecharge = ethMaxCharge;
    }

    function _verifyPaymasterData(GsnTypes.RelayRequest calldata relayRequest) internal view virtual override {
        require(relayRequest.relayData.paymasterData.length == 64, "paymasterData: invalid length");
    }

    function __preRelayedCall(
        GsnTypes.RelayRequest calldata relayRequest,
        bytes calldata signature,
        bytes calldata approvalData,
        uint256 maxPossibleGas
    ) public {
        _preRelayedCall(relayRequest, signature, approvalData, maxPossibleGas);
    }

    function _preRelayedCall(
        GsnTypes.RelayRequest calldata relayRequest,
        bytes calldata, /* signature */
        bytes calldata, /* approvalData */
        uint256 maxPossibleGas
    ) internal virtual override returns (bytes memory context, bool revertOnRecipientRevert) {
        (IERC20 token, uint256 maxTokens) = _getPaymasterData(relayRequest.relayData.paymasterData);
        (address payer, uint256 ethPrecharge, uint256 tokenPrecharge) =
            _calculatePreCharge(token, relayRequest, maxPossibleGas);

        require(tokenPrecharge <= maxTokens, "Tx cost more than the user-supplied limit");

        token.transferFrom(payer, address(this), tokenPrecharge);

        emit PreRelayPayment(ethPrecharge, token, tokenPrecharge, payer);

        return (abi.encode(payer, tokenPrecharge, token), false);
    }

    function _postRelayedCall(
        bytes calldata context,
        bool,
        uint256 gasUseWithoutPost,
        GsnTypes.RelayData calldata relayData
    ) internal virtual override {
        (address payer, uint256 tokenPrecharge, IERC20 token) = abi.decode(context, (address, uint256, IERC20));
        _postRelayedCallInternal(payer, tokenPrecharge, 0, gasUseWithoutPost, relayData, token);
    }

    function _postRelayedCallInternal(
        address payer,
        uint256 tokenPrecharge,
        uint256 valueRequested,
        uint256 gasUseWithoutPost,
        GsnTypes.RelayData calldata relayData,
        IERC20 token
    ) internal {
        uint256 ethActualCharge = relayHub.calculateCharge(gasUseWithoutPost + gasUsedByPost, relayData);
        uint256 tokenActualCharge = _ethToTokens(token, valueRequested + ethActualCharge);
        uint256 tokenRefund = tokenPrecharge - tokenActualCharge;

        emit PostRelay(ethActualCharge, token, tokenActualCharge, payer);

        require(token.transfer(payer, tokenRefund), "failed refund");
    }
}
