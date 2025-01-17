// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import {ERC2771Recipient} from "@opengsn/packages/contracts/src/ERC2771Recipient.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

// created using https://wizard.openzeppelin.com/ and then modified.
// todo: naming
contract TestingErc20 is ERC20, ERC20Burnable, Ownable, ERC2771Recipient {
    uint8 _numDecimals;

    constructor(string memory _name, string memory _symbol, uint8 _decimals) ERC20(_name, _symbol) {
        _numDecimals = _decimals;
    }

    function setTrustedForwarder(address _forwarder) public onlyOwner {
        _setTrustedForwarder(_forwarder);
    }

    function mint(uint256 amount) external {
        _mint(_msgSender(), amount);
    }

    function _msgSender() internal view override(Context, ERC2771Recipient) returns (address sender) {
        sender = ERC2771Recipient._msgSender();
    }

    function _msgData() internal view override(Context, ERC2771Recipient) returns (bytes calldata) {
        return ERC2771Recipient._msgData();
    }

    // override decimals
    function decimals() public view virtual override returns (uint8) {
        return _numDecimals;
    }
}
