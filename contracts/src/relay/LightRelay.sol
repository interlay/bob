// SPDX-License-Identifier: GPL-3.0-only

// Forked from https://github.com/keep-network/tbtc-v2

pragma solidity ^0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";

import {BytesLib} from "@bob-collective/bitcoin-spv/BytesLib.sol";
import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {ValidateSPV} from "@bob-collective/bitcoin-spv/ValidateSPV.sol";

import "./IRelay.sol";

struct Epoch {
    uint32 timestamp;
    // By definition, bitcoin targets have at least 32 leading zero bits.
    // Thus we can only store the bits that aren't guaranteed to be 0.
    uint224 target;
}

interface ILightRelay is IRelay {
    event Genesis(uint256 blockHeight);
    event Retarget(uint256 oldDifficulty, uint256 newDifficulty);
    event ProofLengthChanged(uint256 newLength);
    event AuthorizationRequirementChanged(bool newStatus);
    event SubmitterAuthorized(address submitter);
    event SubmitterDeauthorized(address submitter);

    function retarget(bytes memory headers) external;

    function validateChain(bytes memory headers)
        external
        view
        returns (uint256 startingHeaderTimestamp, uint256 headerCount);

    function getBlockDifficulty(uint256 blockNumber) external view returns (uint256);

    function getEpochDifficulty(uint256 epochNumber) external view returns (uint256);

    function getRelayRange() external view returns (uint256 relayGenesis, uint256 currentEpochEnd);
}

library RelayUtils {
    using BytesLib for bytes;

    /// @notice Extract the timestamp of the header at the given position.
    /// @param headers Byte array containing the header of interest.
    /// @param at The start of the header in the array.
    /// @return The timestamp of the header.
    /// @dev Assumes that the specified position contains a valid header.
    /// Performs no validation whatsoever.
    function extractTimestampAt(bytes memory headers, uint256 at) internal pure returns (uint32) {
        return BTCUtils.reverseUint32(uint32(headers.slice4(68 + at)));
    }
}

/// @title LightRelay Contract
/// @dev The LightRelay contract manages a relay for Bitcoin header information,
/// allowing retargeting and validation of header chains.
/// THE RELAY MUST NOT BE USED BEFORE GENESIS AND AT LEAST ONE RETARGET.
contract LightRelay is Ownable, ILightRelay {
    using BytesLib for bytes;
    using BTCUtils for bytes;
    using ValidateSPV for bytes;
    using RelayUtils for bytes;

    /// @notice Flag indicating whether the relay is ready for use.
    bool public ready;
    // Whether the relay requires the address submitting a retarget to be
    // authorised in advance by governance.
    /// @notice Flag indicating whether authorization is required for retarget submitters.
    bool public authorizationRequired;
    /// @notice Number of blocks required for each side of a retarget proof.
    // A retarget must provide `proofLength` blocks before the retarget
    // and `proofLength` blocks after it.
    // Governable
    // Should be set to a fairly high number (e.g. 20-50) in production.
    uint64 public proofLength;
    /// @notice The number of the first epoch recorded by the relay.
    // This should equal the height of the block starting the genesis epoch,
    // divided by 2016, but this is not enforced as the relay has no
    // information about block numbers.
    uint64 public genesisEpoch;
    /// @notice The number of the latest epoch whose difficulty is proven to the relay.
    // If the genesis epoch's number is set correctly, and retargets along the
    // way have been legitimate, this equals the height of the block starting
    // the most recent epoch, divided by 2016.
    uint64 public currentEpoch;

    /// @notice Difficulty of the current epoch.
    uint256 internal currentEpochDifficulty;
    /// @notice Difficulty of the previous epoch.
    uint256 internal prevEpochDifficulty;

    /// @notice Mapping of each epoch from genesis to the current one, keyed by their numbers.
    mapping(uint256 => Epoch) internal epochs;

    /// @notice Mapping of authorized submitters.
    mapping(address => bool) public isAuthorized;

    /// @notice Modifier to check if the relay is active.
    modifier relayActive() {
        require(ready, "Relay is not ready for use");
        _;
    }

    /// @notice Establish a starting point for the relay by providing the
    /// target, timestamp and blockheight of the first block of the relay
    /// genesis epoch.
    /// @param genesisHeader The first block header of the genesis epoch.
    /// @param genesisHeight The block number of the first block of the epoch.
    /// @param genesisProofLength The number of blocks required to accept a
    /// proof.
    /// @dev If the relay is used by querying the current and previous epoch
    /// difficulty, at least one retarget needs to be provided after genesis;
    /// otherwise the prevEpochDifficulty will be uninitialised and zero.
    function genesis(bytes calldata genesisHeader, uint256 genesisHeight, uint64 genesisProofLength)
        external
        onlyOwner
    {
        require(!ready, "Genesis already performed");

        require(genesisHeader.length == 80, "Invalid genesis header length");

        require(genesisHeight % 2016 == 0, "Invalid height of relay genesis block");

        require(genesisProofLength < 2016, "Proof length excessive");
        require(genesisProofLength > 0, "Proof length may not be zero");

        genesisEpoch = uint64(genesisHeight / 2016);
        currentEpoch = genesisEpoch;
        uint256 genesisTarget = genesisHeader.extractTarget();
        uint256 genesisTimestamp = genesisHeader.extractTimestamp();
        epochs[genesisEpoch] = Epoch(uint32(genesisTimestamp), uint224(genesisTarget));
        proofLength = genesisProofLength;
        currentEpochDifficulty = BTCUtils.calculateDifficulty(genesisTarget);
        ready = true;

        emit Genesis(genesisHeight);
    }

    /// @notice Set the number of blocks required to accept a header chain.
    /// @param newLength The required number of blocks. Must be less than 2016.
    /// @dev For production, a high number (e.g. 20-50) is recommended.
    /// Small numbers are accepted but should only be used for testing.
    function setProofLength(uint64 newLength) external relayActive onlyOwner {
        require(newLength < 2016, "Proof length excessive");
        require(newLength > 0, "Proof length may not be zero");
        require(newLength != proofLength, "Proof length unchanged");
        proofLength = newLength;
        emit ProofLengthChanged(newLength);
    }

    /// @notice Set whether the relay requires retarget submitters to be
    /// pre-authorised by governance.
    /// @param status True if authorisation is to be required, false if not.
    function setAuthorizationStatus(bool status) external onlyOwner {
        authorizationRequired = status;
        emit AuthorizationRequirementChanged(status);
    }

    /// @notice Authorise the given address to submit retarget proofs.
    /// @param submitter The address to be authorised.
    function authorize(address submitter) external onlyOwner {
        isAuthorized[submitter] = true;
        emit SubmitterAuthorized(submitter);
    }

    /// @notice Rescind the authorisation of the submitter to retarget.
    /// @param submitter The address to be deauthorised.
    function deauthorize(address submitter) external onlyOwner {
        isAuthorized[submitter] = false;
        emit SubmitterDeauthorized(submitter);
    }

    /// @notice Add a new epoch to the relay by providing a proof
    /// of the difficulty before and after the retarget.
    /// @param headers A chain of headers including the last X blocks before
    /// the retarget, followed by the first X blocks after the retarget,
    /// where X equals the current proof length.
    /// @dev Checks that the first X blocks are valid in the most recent epoch,
    /// that the difficulty of the new epoch is calculated correctly according
    /// to the block timestamps, and that the next X blocks would be valid in
    /// the new epoch.
    /// We have no information of block heights, so we cannot enforce that
    /// retargets only happen every 2016 blocks; instead, we assume that this
    /// is the case if a valid proof of work is provided.
    /// It is possible to cheat the relay by providing X blocks from earlier in
    /// the most recent epoch, and then mining X new blocks after them.
    /// However, each of these malicious blocks would have to be mined to a
    /// higher difficulty than the legitimate ones.
    /// Alternatively, if the retarget has not been performed yet, one could
    /// first mine X blocks in the old difficulty with timestamps set far in
    /// the future, and then another X blocks at a greatly reduced difficulty.
    /// In either case, cheating the relay requires more work than mining X
    /// legitimate blocks.
    /// Only the most recent epoch is vulnerable to these attacks; once a
    /// retarget has been proven to the relay, the epoch is immutable even if a
    /// contradictory proof were to be presented later.
    function retarget(bytes memory headers) external relayActive {
        if (authorizationRequired) {
            require(isAuthorized[msg.sender], "Submitter unauthorized");
        }

        require(
            // Require proofLength headers on both sides of the retarget
            headers.length == (proofLength * 2 * 80),
            "Invalid header length"
        );

        Epoch storage latest = epochs[currentEpoch];

        uint256 oldTarget = latest.target;

        bytes32 previousHeaderDigest = bytes32(0);

        // Validate old chain
        for (uint256 i = 0; i < proofLength; i++) {
            (bytes32 currentDigest, uint256 currentHeaderTarget) = validateHeader(headers, i * 80, previousHeaderDigest);

            require(currentHeaderTarget == oldTarget, "Invalid target in pre-retarget headers");

            previousHeaderDigest = currentDigest;
        }

        // get timestamp of retarget block
        uint256 epochEndTimestamp = headers.extractTimestampAt((proofLength - 1) * 80);

        // An attacker could produce blocks with timestamps in the future,
        // in an attempt to reduce the difficulty after the retarget
        // to make mining the second part of the retarget proof easier.
        // In particular, the attacker could reuse all but one block
        // from the legitimate chain, and only mine the last block.
        // To hinder this, require that the epoch end timestamp does not
        // exceed the ethereum timestamp.
        // NOTE: both are unix seconds, so this comparison should be valid.
        require(
            /* solhint-disable-next-line not-rely-on-time */
            epochEndTimestamp < block.timestamp,
            "Epoch cannot end in the future"
        );

        // Expected target is the full-length target
        uint256 expectedTarget = BTCUtils.retargetAlgorithm(oldTarget, latest.timestamp, epochEndTimestamp);

        // Mined target is the header-encoded target
        uint256 minedTarget = 0;

        uint256 epochStartTimestamp = headers.extractTimestampAt(proofLength * 80);

        // validate new chain
        for (uint256 j = proofLength; j < proofLength * 2; j++) {
            (bytes32 _currentDigest, uint256 _currentHeaderTarget) =
                validateHeader(headers, j * 80, previousHeaderDigest);

            if (minedTarget == 0) {
                // The new target has not been set, so check its correctness
                minedTarget = _currentHeaderTarget;
                require(
                    // Although the target is a 256-bit number, there are only 32 bits of
                    // space in the Bitcoin header. Because of that, the version stored in
                    // the header is a less-precise representation of the actual target
                    // using base-256 version of scientific notation.
                    //
                    // The 256-bit unsigned integer returned from BTCUtils.retargetAlgorithm
                    // is the precise target value.
                    // The 256-bit unsigned integer returned from validateHeader is the less
                    // precise target value because it was read from 32 bits of space of
                    // Bitcoin block header.
                    //
                    // We can't compare the precise and less precise representations together
                    // so we first mask them to obtain the less precise version:
                    //   (full & truncated) == truncated
                    _currentHeaderTarget == (expectedTarget & _currentHeaderTarget),
                    "Invalid target in new epoch"
                );
            } else {
                // The new target has been set, so remaining targets should match.
                require(_currentHeaderTarget == minedTarget, "Unexpected target change after retarget");
            }

            previousHeaderDigest = _currentDigest;
        }

        currentEpoch = currentEpoch + 1;

        epochs[currentEpoch] = Epoch(uint32(epochStartTimestamp), uint224(minedTarget));

        uint256 oldDifficulty = currentEpochDifficulty;
        uint256 newDifficulty = BTCUtils.calculateDifficulty(minedTarget);

        prevEpochDifficulty = oldDifficulty;
        currentEpochDifficulty = newDifficulty;

        emit Retarget(oldDifficulty, newDifficulty);
    }

    /// @notice Check whether a given chain of headers should be accepted as
    /// valid within the rules of the relay.
    /// If the validation fails, this function throws an exception.
    /// @param headers A chain of 2 to 2015 bitcoin headers.
    /// @return startingHeaderTimestamp The timestamp of the first header.
    /// @return headerCount The number of headers.
    /// @dev A chain of headers is accepted as valid if:
    /// - Its length is between 2 and 2015 headers.
    /// - Headers in the chain are sequential and refer to previous digests.
    /// - Each header is mined with the correct amount of work.
    /// - The difficulty in each header matches an epoch of the relay,
    ///   as determined by the headers' timestamps. The headers must be between
    ///   the genesis epoch and the latest proven epoch (inclusive).
    /// If the chain contains a retarget, it is accepted if the retarget has
    /// already been proven to the relay.
    /// If the chain contains blocks of an epoch that has not been proven to
    /// the relay (after a retarget within the header chain, or when the entire
    /// chain falls within an epoch that has not been proven yet), it will be
    /// rejected.
    /// One exception to this is when two subsequent epochs have exactly the
    /// same difficulty; headers from the latter epoch will be accepted if the
    /// previous epoch has been proven to the relay.
    /// This is because it is not possible to distinguish such headers from
    /// headers of the previous epoch.
    ///
    /// If the difficulty increases significantly between relay genesis and the
    /// present, creating fraudulent proofs for earlier epochs becomes easier.
    /// Users of the relay should check the timestamps of valid headers and
    /// only accept appropriately recent ones.
    function validateChain(bytes memory headers)
        external
        view
        returns (uint256 startingHeaderTimestamp, uint256 headerCount)
    {
        require(headers.length % 80 == 0, "Invalid header length");

        headerCount = headers.length / 80;

        require(headerCount > 1 && headerCount < 2016, "Invalid number of headers");

        startingHeaderTimestamp = headers.extractTimestamp();

        // Short-circuit the first header's validation.
        // We validate the header here to get the target which is needed to
        // precisely identify the epoch.
        (bytes32 previousHeaderDigest, uint256 currentHeaderTarget) = validateHeader(headers, 0, bytes32(0));

        Epoch memory nullEpoch = Epoch(0, 0);

        uint256 startingEpochNumber = currentEpoch;
        Epoch memory startingEpoch = epochs[startingEpochNumber];
        Epoch memory nextEpoch = nullEpoch;

        // Find the correct epoch for the given chain
        // Fastest with recent epochs, but able to handle anything after genesis
        //
        // The rules for bitcoin timestamps are:
        // - must be greater than the median of the last 11 blocks' timestamps
        // - must be less than the network-adjusted time +2 hours
        //
        // Because of this, the timestamp of a header may be smaller than the
        // starting time, or greater than the ending time of its epoch.
        // However, a valid timestamp is guaranteed to fall within the window
        // formed by the epochs immediately before and after its timestamp.
        // We can identify cases like these by comparing the targets.
        while (startingHeaderTimestamp < startingEpoch.timestamp) {
            startingEpochNumber -= 1;
            nextEpoch = startingEpoch;
            startingEpoch = epochs[startingEpochNumber];
        }

        // We have identified the centre of the window,
        // by reaching the most recent epoch whose starting timestamp
        // or reached before the genesis where epoch slots are empty.
        // Therefore check that the timestamp is nonzero.
        require(startingEpoch.timestamp > 0, "Cannot validate chains before relay genesis");

        // The targets don't match. This could be because the block is invalid,
        // or it could be because of timestamp inaccuracy.
        // To cover the latter case, check adjacent epochs.
        if (currentHeaderTarget != startingEpoch.target) {
            // The target matches the next epoch.
            // This means we are right at the beginning of the next epoch,
            // and retargets during the chain should not be possible.
            if (currentHeaderTarget == nextEpoch.target) {
                startingEpoch = nextEpoch;
                nextEpoch = nullEpoch;
            }
            // The target doesn't match the next epoch.
            // Therefore the only valid epoch is the previous one.
            // Because the timestamp can't be more than 2 hours into the future
            // we must be right near the end of the epoch,
            // so a retarget is possible.
            else {
                startingEpochNumber -= 1;
                nextEpoch = startingEpoch;
                startingEpoch = epochs[startingEpochNumber];

                // We have failed to find a match,
                // therefore the target has to be invalid.
                require(currentHeaderTarget == startingEpoch.target, "Invalid target in header chain");
            }
        }

        // We've found the correct epoch for the first header.
        // Validate the rest.
        for (uint256 i = 1; i < headerCount; i++) {
            bytes32 currentDigest;
            (currentDigest, currentHeaderTarget) = validateHeader(headers, i * 80, previousHeaderDigest);

            // If the header's target does not match the expected target,
            // check if a retarget is possible.
            //
            // If next epoch timestamp exists, a valid retarget is possible
            // (if next epoch timestamp doesn't exist, either a retarget has
            // already happened in this chain, the relay needs a retarget
            // before this chain can be validated, or a retarget is not allowed
            // because we know the headers are within a timestamp irregularity
            // of the previous retarget).
            //
            // In this case the target must match the next epoch's target,
            // and the header's timestamp must match the epoch's start.
            if (currentHeaderTarget != startingEpoch.target) {
                uint256 currentHeaderTimestamp = headers.extractTimestampAt(i * 80);

                require(
                    nextEpoch.timestamp != 0 && currentHeaderTarget == nextEpoch.target
                        && currentHeaderTimestamp == nextEpoch.timestamp,
                    "Invalid target in header chain"
                );

                startingEpoch = nextEpoch;
                nextEpoch = nullEpoch;
            }

            previousHeaderDigest = currentDigest;
        }

        return (startingHeaderTimestamp, headerCount);
    }

    /// @notice Get the difficulty of the specified block.
    /// @param blockNumber The number of the block. Must fall within the relay
    /// range (at or after the relay genesis, and at or before the end of the
    /// most recent epoch proven to the relay).
    /// @return The difficulty of the epoch.
    function getBlockDifficulty(uint256 blockNumber) external view returns (uint256) {
        return getEpochDifficulty(blockNumber / 2016);
    }

    /// @notice Get the range of blocks the relay can accept proofs for.
    /// @dev Assumes that the genesis has been set correctly.
    /// Additionally, if the next epoch after the current one has the exact
    /// same difficulty, headers for it can be validated as well.
    /// This function should be used for informative purposes,
    /// e.g. to determine whether a retarget must be provided before submitting
    /// a header chain for validation.
    /// @return relayGenesis The height of the earliest block that can be
    /// included in header chains for the relay to validate.
    /// @return currentEpochEnd The height of the last block that can be
    /// included in header chains for the relay to validate.
    function getRelayRange() external view returns (uint256 relayGenesis, uint256 currentEpochEnd) {
        relayGenesis = genesisEpoch * 2016;
        currentEpochEnd = (currentEpoch * 2016) + 2015;
    }

    /// @notice Returns the difficulty of the current epoch.
    /// @dev returns 0 if the relay is not ready.
    /// @return The difficulty of the current epoch.
    function getCurrentEpochDifficulty() external view virtual returns (uint256) {
        return currentEpochDifficulty;
    }

    /// @notice Returns the difficulty of the previous epoch.
    /// @dev Returns 0 if the relay is not ready or has not had a retarget.
    /// @return The difficulty of the previous epoch.
    function getPrevEpochDifficulty() external view virtual returns (uint256) {
        return prevEpochDifficulty;
    }

    function getCurrentAndPrevEpochDifficulty() external view returns (uint256 current, uint256 previous) {
        return (currentEpochDifficulty, prevEpochDifficulty);
    }

    /// @notice Get the difficulty of the specified epoch.
    /// @param epochNumber The number of the epoch (the height of the first
    /// block of the epoch, divided by 2016). Must fall within the relay range.
    /// @return The difficulty of the epoch.
    function getEpochDifficulty(uint256 epochNumber) public view returns (uint256) {
        require(epochNumber >= genesisEpoch, "Epoch is before relay genesis");
        require(epochNumber <= currentEpoch, "Epoch is not proven to the relay yet");
        return BTCUtils.calculateDifficulty(epochs[epochNumber].target);
    }

    /// @notice Check that the specified header forms a correct chain with the
    /// digest of the previous header (if provided), and has sufficient work.
    /// @param headers The byte array containing the header of interest.
    /// @param start The start of the header in the array.
    /// @param prevDigest The digest of the previous header
    /// (optional; providing zeros for the digest skips the check).
    /// @return digest The digest of the current header.
    /// @return target The PoW target of the header.
    /// @dev Throws an exception if the header's chain or PoW are invalid.
    /// Performs no other validation.
    function validateHeader(bytes memory headers, uint256 start, bytes32 prevDigest)
        internal
        view
        returns (bytes32 digest, uint256 target)
    {
        // If previous block digest has been provided, require that it matches
        if (prevDigest != bytes32(0)) {
            require(headers.validateHeaderPrevHash(start, prevDigest), "Invalid chain");
        }

        // Require that the header has sufficient work for its stated target
        target = headers.extractTargetAt(start);
        digest = headers.hash256Slice(start, 80);
        require(ValidateSPV.validateHeaderWork(digest, target), "Invalid work");

        return (digest, target);
    }
}
