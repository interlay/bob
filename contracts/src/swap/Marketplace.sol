// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ERC2771Recipient} from "@opengsn/packages/contracts/src/ERC2771Recipient.sol";

using SafeERC20 for IERC20;

contract MarketPlace is ERC2771Recipient {
    mapping(uint256 => Order) public ercErcOrders; // cant have struct as key, nor tupple

    event placeOrder(
        uint256 indexed orderId,
        address indexed requesterAddress,
        uint256 offeringAmount,
        address offeringToken,
        uint256 askingAmount,
        address askingToken
    );
    event acceptOrder(uint256 indexed orderId, address indexed who, uint256 buyAmount, uint256 saleAmount);
    event withdrawOrder(uint256 indexed orderId);

    uint256 public nextOrderId;

    struct Order {
        uint256 offeringAmount;
        address offeringToken;
        uint256 askingAmount;
        address askingToken;
        address requesterAddress;
    }

    constructor(address erc2771Forwarder) {
        _setTrustedForwarder(erc2771Forwarder);
    }

    function placeErcErcOrder(address sellingToken, uint256 saleAmount, address buyingToken, uint256 buyAmount)
        public
    {
        require(sellingToken != address(0x0));
        require(buyingToken != address(0x0));

        // "lock" selling token by transferring to contract
        IERC20(sellingToken).safeTransferFrom(_msgSender(), address(this), saleAmount);

        uint256 id = nextOrderId++;
        Order memory order = Order({
            offeringAmount: saleAmount,
            offeringToken: sellingToken,
            askingAmount: buyAmount,
            askingToken: buyingToken,
            requesterAddress: _msgSender()
        });

        ercErcOrders[id] = order;

        emit placeOrder(
            id, order.requesterAddress, order.offeringAmount, order.offeringToken, order.askingAmount, order.askingToken
        );
    }

    function acceptErcErcOrder(uint256 id, uint256 saleAmount) public {
        Order memory order = ercErcOrders[id];

        // make sure the storage item exists
        require(order.requesterAddress != address(0x0));

        require(saleAmount <= order.askingAmount);
        // todo: make this safe
        uint256 buyAmount = (saleAmount * order.offeringAmount) / order.askingAmount;

        assert(saleAmount > 0);
        assert(buyAmount > 0);
        assert(buyAmount <= order.offeringAmount);

        ercErcOrders[id].offeringAmount -= buyAmount;
        ercErcOrders[id].askingAmount -= saleAmount;

        IERC20(order.askingToken).safeTransferFrom(_msgSender(), order.requesterAddress, saleAmount);
        IERC20(order.offeringToken).safeTransfer(_msgSender(), buyAmount);

        emit acceptOrder(id, _msgSender(), buyAmount, saleAmount);
    }

    function withdrawErcErcOrder(uint256 id) public {
        Order memory order = ercErcOrders[id];
        require(order.requesterAddress == _msgSender());

        IERC20(order.offeringToken).safeTransfer(_msgSender(), order.offeringAmount);

        delete ercErcOrders[id];

        emit withdrawOrder(id);
    }

    function getOpenOrders() external view returns (Order[] memory, uint256[] memory) {
        uint256 numOpenOrders = 0;
        for (uint256 i = 0; i < nextOrderId; i++) {
            if (ercErcOrders[i].offeringAmount > 0) {
                numOpenOrders++;
            }
        }
        Order[] memory ret = new Order[](numOpenOrders);
        uint256[] memory identifiers = new uint256[](numOpenOrders);
        uint256 numPushed = 0;
        for (uint256 i = 0; i < nextOrderId; i++) {
            if (ercErcOrders[i].offeringAmount > 0) {
                ret[numPushed] = ercErcOrders[i];
                identifiers[numPushed] = i;
                numPushed++;
            }
        }
        return (ret, identifiers);
    }
}
