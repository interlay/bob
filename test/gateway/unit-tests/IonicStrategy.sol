// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {IIonicToken, IPool, IonicStrategy} from "../../../src/gateway/strategy/IonicStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/CommonStructs.sol";
import {ArbitaryErc20} from "./Utils.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract DummyIonicToken is IIonicToken, ERC20, Ownable {
    bool private doMint;
    bool private suppressMintError;

    constructor(string memory name_, string memory symbol_, bool _doMint, bool _suppressMintError)
        ERC20(name_, symbol_)
    {
        doMint = _doMint;
        suppressMintError = _suppressMintError;
    }

    function sudoMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    function mint(uint256 mintAmount) external returns (uint256) {
        if (suppressMintError) {
            return 0;
        }
        if (doMint) {
            _mint(_msgSender(), mintAmount);
            return 0;
        }
        return 1;
    }

    function redeem(uint256 /*redeemTokens*/ ) external pure returns (uint256) {
        return 0;
    }
}

contract DummyIonicPool is IPool {
    bool private doEnterMarkets;

    constructor(bool _doEnterMarkets) {
        doEnterMarkets = _doEnterMarkets;
    }

    function enterMarkets(address[] memory cTokens) external view override returns (uint256[] memory) {
        if (doEnterMarkets) {
            // Return an empty array to simulate entered in the market
            return new uint256[](cTokens.length);
        }

        uint256[] memory result = new uint256[](1);
        result[0] = 1;
        return result; // Return the array
    }

    function exitMarket(address /* cTokenAddress */ ) external pure override returns (uint256) {
        // Return 0 to simulate doing nothing
        return 0;
    }
}

contract IonicStrategyTest is Test {
    IIonicToken ionicToken;
    ArbitaryErc20 wrappedBTC;

    event TokenOutput(address tokenReceived, uint256 amountOut);

    function setUp() public {
        ionicToken = new DummyIonicToken("", "", true, false);
        wrappedBTC = new ArbitaryErc20("", "");
        wrappedBTC.sudoMint(address(this), 1 ether); // Mint 100 tokens to this contract
    }

    function testIonicStrategy() public {
        IPool dummyIonicPool = new DummyIonicPool(true);
        IonicStrategy ionicStrategy = new IonicStrategy(ionicToken, dummyIonicPool);

        // Approve strategy to spend tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(ionicStrategy), 1 ether);

        vm.expectEmit();
        emit TokenOutput(address(ionicToken), 1 ether);
        ionicStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }

    function testIonicStrategyWhenMarketNotPresent() public {
        IPool dummyIonicPool = new DummyIonicPool(false);
        IonicStrategy ionicStrategy = new IonicStrategy(ionicToken, dummyIonicPool);

        // Approve strategy to spend 100 tBTC tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(ionicStrategy), 1 ether);

        vm.expectRevert("Couldn't enter in Market");
        ionicStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }

    function testIonicStrategyWhenCouldNotMint() public {
        ionicToken = new DummyIonicToken("Ionic Token", "ion", false, false);
        IPool dummyIonicPool = new DummyIonicPool(true);
        IonicStrategy ionicStrategy = new IonicStrategy(ionicToken, dummyIonicPool);

        // Approve strategy to spend 100 tBTC tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(ionicStrategy), 1 ether);

        vm.expectRevert("Could not mint token in Ionic market");
        ionicStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }

    function testIonicStrategyForInsufficientSupply() public {
        ionicToken = new DummyIonicToken("Ionic Token", "ion", false, true);
        IPool dummyIonicPool = new DummyIonicPool(true);
        IonicStrategy ionicStrategy = new IonicStrategy(ionicToken, dummyIonicPool);

        // Approve strategy to spend 100 tBTC tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(ionicStrategy), 1 ether);

        vm.expectRevert("Insufficient supply provided");
        ionicStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
        );
    }

    function testIonicStrategyForInsufficientOutput() public {
        IPool dummyIonicPool = new DummyIonicPool(true);
        IonicStrategy ionicStrategy = new IonicStrategy(ionicToken, dummyIonicPool);

        // Approve strategy to spend 100 tBTC tokens on behalf of this contract
        wrappedBTC.increaseAllowance(address(ionicStrategy), 1 ether);

        vm.expectRevert("Insufficient output amount");
        ionicStrategy.handleGatewayMessageWithSlippageArgs(
            wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(2 ether)
        );
    }
}
