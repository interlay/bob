// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

interface IFullRelay {
    event Extension(bytes32 indexed _first, bytes32 indexed _last);
    event NewTip(bytes32 indexed _from, bytes32 indexed _to, bytes32 indexed _gcd);

    function getCurrentEpochDifficulty() external view returns (uint256);
    function getPrevEpochDifficulty() external view returns (uint256);
    function getRelayGenesis() external view returns (bytes32);
    function getBestKnownDigest() external view returns (bytes32);
    function getLastReorgCommonAncestor() external view returns (bytes32);

    function findHeight(bytes32 _digest) external view returns (uint256);

    function findAncestor(bytes32 _digest, uint256 _offset) external view returns (bytes32);

    function isAncestor(bytes32 _ancestor, bytes32 _descendant, uint256 _limit) external view returns (bool);

    function addHeaders(bytes calldata _anchor, bytes calldata _headers) external returns (bool);

    function addHeadersWithRetarget(
        bytes calldata _oldPeriodStartHeader,
        bytes calldata _oldPeriodEndHeader,
        bytes calldata _headers
    ) external returns (bool);

    function markNewHeaviest(bytes32 _ancestor, bytes calldata _currentBest, bytes calldata _newBest, uint256 _limit)
        external
        returns (bool);
}
