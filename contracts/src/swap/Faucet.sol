// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Ownable, Context} from "@openzeppelin/contracts/access/Ownable.sol";
import {ERC2771Recipient} from "@opengsn/packages/contracts/src/ERC2771Recipient.sol";

interface Erc20Mintable {
    function decimals() external returns (uint256);

    function mint(uint256 amount) external;

    function transfer(address to, uint256 value) external returns (bool);
}

contract Faucet is Ownable, ERC2771Recipient {
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
            token.transfer(_msgSender(), amount);
        }
    }

    function _msgSender() internal view override(Context, ERC2771Recipient) returns (address sender) {
        sender = ERC2771Recipient._msgSender();
    }

    function _msgData() internal view override(Context, ERC2771Recipient) returns (bytes calldata) {
        return ERC2771Recipient._msgData();
    }
}
