// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {FullRelay} from "./FullRelay.sol";

import {SafeMath} from "@bob-collective/bitcoin-spv/SafeMath.sol";
import {BytesLib} from "@bob-collective/bitcoin-spv/BytesLib.sol";
import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {ValidateSPV} from "@bob-collective/bitcoin-spv/ValidateSPV.sol";

contract FullRelayWithVerify is FullRelay {
    using SafeMath for uint256;
    using BTCUtils for bytes;
    using BTCUtils for uint256;

    /// @notice                   Gives a starting point for the relay
    /// @dev                      We don't check this AT ALL really. Don't use relays with bad genesis
    /// @param  _genesisHeader    The starting header
    /// @param  _height           The starting height
    /// @param  _periodStart      The hash of the first header in the genesis epoch
    constructor(bytes memory _genesisHeader, uint256 _height, bytes32 _periodStart)
        FullRelay(_genesisHeader, _height, _periodStart)
    {}

    /// @notice                 Provide a proof of a tx that satisfies some request
    /// @dev                    The caller must specify which inputs, which outputs, and which request
    /// @param  _header         The header containing the merkleroot committing to the tx
    /// @param  _proof          The merkle proof intermediate nodes
    /// @param  _txId           The transaction id to verify
    /// @param  _index          The index of the tx in the merkle tree's leaves
    /// @param  _numConfs       Number of confirmations required
    function verifyProof(bytes calldata _header, bytes calldata _proof, bytes32 _txId, uint256 _index, uint8 _numConfs)
        external
        view
    {
        require(isHeaderValidLength(_header), "Bad header block");
        require(isMerkleArrayValidLength(_proof), "Bad merkle array proof");

        require(ValidateSPV.prove(_txId, _header.extractMerkleRootLE(), _proof, _index), "Bad inclusion proof");

        bytes32 _headerHash = _header.hash256();
        bytes32 _GCD = getLastReorgCommonAncestor();

        require(_isAncestor(_headerHash, _GCD, 240), "GCD does not confirm header");
        require(_getConfs(_headerHash) >= _numConfs, "Insufficient confirmations");
    }

    /// @notice             Finds the number of headers on top of the argument
    /// @dev                Bounded to 6400 gas (8 looksups) max
    /// @param _headerHash  The LE double-sha2 header hash
    /// @return             The number of headers on top
    function _getConfs(bytes32 _headerHash) internal view virtual returns (uint8) {
        return uint8(_findHeight(bestKnownDigest) - _findHeight(_headerHash));
    }
}

// For unittests
contract TestRelay is FullRelayWithVerify {
    /// @notice                   Gives a starting point for the relay
    /// @dev                      We don't check this AT ALL really. Don't use relays with bad genesis
    /// @param  _genesisHeader    The starting header
    /// @param  _height           The starting height
    /// @param  _periodStart      The hash of the first header in the genesis epoch
    constructor(bytes memory _genesisHeader, uint256 _height, bytes32 _periodStart)
        FullRelayWithVerify(_genesisHeader, _height, _periodStart)
    {}

    bool isAncestorOverride;
    bool isAncestorOverrideRes;

    function heaviestFromAncestor(bytes32 _ancestor, bytes calldata _left, bytes calldata _right)
        external
        view
        returns (bytes32)
    {
        return _heaviestFromAncestor(_ancestor, _left, _right);
    }

    function isMostRecentAncestor(bytes32 _ancestor, bytes32 _left, bytes32 _right, uint256 _limit)
        external
        view
        returns (bool)
    {
        return _isMostRecentAncestor(_ancestor, _left, _right, _limit);
    }

    function setAncestorOverride(bool _isAncestorOverride, bool _isAncestorOverrideRes) public {
        isAncestorOverride = _isAncestorOverride;
        isAncestorOverrideRes = _isAncestorOverrideRes;
    }

    function _isAncestor(bytes32 _ancestor, bytes32 _descendant, uint256 _limit)
        internal
        view
        virtual
        override
        returns (bool)
    {
        if (isAncestorOverride) {
            return isAncestorOverrideRes;
        } else {
            return super._isAncestor(_ancestor, _descendant, _limit);
        }
    }

    function _getConfs(bytes32) internal view virtual override returns (uint8) {
        return 8;
    }
}
