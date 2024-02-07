// SPDX-License-Identifier:MIT
pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import "gsn/packages/contracts/src/forwarder/IForwarder.sol";
import "gsn/packages/contracts/src/BasePaymaster.sol";

contract OnboardingPaymaster is BasePaymaster {
    address public whitelistedContract;
    uint32 public whitelistedSelector;
    uint256 public gasUsedByPost;

    constructor(address _whitelistedContract, uint32 _whitelistedSelector) {
        whitelistedContract = _whitelistedContract;
        whitelistedSelector = _whitelistedSelector;
    }

    event PreRelay(address indexed sender);
    event PostRelay(address indexed sender);

    function versionPaymaster() external view virtual override returns (string memory) {
        return "3.0.0-beta.10+opengsn.oracle.token.ipaymaster";
    }

    function setPostGasUsage(uint256 _gasUsedByPost) external onlyOwner {
        gasUsedByPost = _gasUsedByPost;
    }

    function _getPaymasterData(bytes memory paymasterData) private pure returns (IERC20 token, uint256 maxTokens) {
        (address tokenAddress, uint256 _maxTokens) = abi.decode(paymasterData, (address, uint256));

        maxTokens = _maxTokens;
        token = IERC20(tokenAddress);
    }

    function getSelector(bytes calldata call) public pure returns (uint32) {
        uint32 ret = uint32(bytes4(call[0:4]));
        return ret;
    }

    function _preRelayedCall(
        GsnTypes.RelayRequest calldata relayRequest,
        bytes calldata, /* signature*/
        bytes calldata, /* approvalData*/
        uint256 /* maxPossibleGas */
    ) internal virtual override returns (bytes memory context, bool revertOnRecipientRevert) {
        require(relayRequest.request.to == whitelistedContract, "Recipient is not whitelisted");

        uint32 selector = getSelector(relayRequest.request.data);

        require(selector == whitelistedSelector, "Selector is not whitelisted");

        emit PreRelay(relayRequest.request.from);

        return (abi.encode(relayRequest.request.from), false);
    }

    function _postRelayedCall(
        bytes calldata context,
        bool,
        uint256, /* gasUseWithoutPost */
        GsnTypes.RelayData calldata /* relayData */
    ) internal virtual override {
        address from = abi.decode(context, (address));
        emit PostRelay(from);
    }
}
