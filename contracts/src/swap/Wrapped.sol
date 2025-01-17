// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

// created using https://wizard.openzeppelin.com/ and then modified.
// todo: naming
contract BobWrappedBtc is ERC20, ERC20Burnable, Ownable {
    constructor() ERC20("Bob Wrapped BTC", "zBTC") {}

    function sudoMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    function sudoBurnFrom(address account, uint256 amount) public onlyOwner {
        _burn(account, amount);
    }

    function sudoTransferFrom(address from, address to, uint256 amount) public onlyOwner {
        _transfer(from, to, amount);
    }
}
