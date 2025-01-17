pragma solidity ^0.8.17;

// Forked from https://github.com/keep-network/tbtc-v2

import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";

import "./LightRelay.sol";

/// @title Test Light Relay
/// @notice TestLightRelay is a stub version of LightRelay intended to be
///         used on for testing network. It allows to set the relay's
///         difficulty based on arbitrary Bitcoin headers thus effectively
///         bypass the validation of difficulties of Bitcoin testnet blocks.
///         Since difficulty in Bitcoin testnet often falls to `1` it would not
///         be possible to validate blocks with the real LightRelay.
/// @dev Notice that TestLightRelay is derived from LightRelay so that the two
///      contracts have the same API and correct bindings can be generated.
contract TestLightRelay is LightRelay {
    using BTCUtils for bytes;
    using BTCUtils for uint256;

    /// @notice Sets the current and previous difficulty based on the difficulty
    ///         inferred from the provided Bitcoin headers.
    function setDifficultyFromHeaders(bytes memory bitcoinHeaders) external {
        uint256 firstHeaderDiff = bitcoinHeaders.extractTarget().calculateDifficulty();

        currentEpochDifficulty = firstHeaderDiff;
        prevEpochDifficulty = firstHeaderDiff;
    }
}
