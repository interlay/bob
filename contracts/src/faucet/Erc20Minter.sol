// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/access/Ownable.sol";

interface Erc20Mintable {
    function decimals() external returns (uint256);
    function mint(uint256 amount) external;
    function transfer(address to, uint256 amount) external returns (bool);
    function approve(address spender, uint256 amount) external;
    function balanceOf(address account) external view returns (uint256);
}

contract Erc20Minter is Ownable {
    uint256 nextTokenId;
    mapping(uint256 => address) supportedErc20Addresses;

    function addErc20(address newErc20) public onlyOwner {
        supportedErc20Addresses[nextTokenId++] = newErc20;
    }

    // Mints 30000 of each erc20
    function mint() public {
        for (uint256 id = 0; id < nextTokenId; id++) {
            Erc20Mintable token = Erc20Mintable(supportedErc20Addresses[id]);
            uint256 amount = 30000 * (10 ** token.decimals());
            token.mint(amount);
            token.transfer(msg.sender, amount);
        }
    }
}
