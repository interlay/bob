// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.17;

import {Test, console2} from "forge-std/Test.sol";

import {LightRelay} from "../src/relay/LightRelay.sol";
import {ILightRelay} from "../src/relay/LightRelay.sol";
import {BitcoinTx} from "../src/bridge/BitcoinTx.sol";
import {BridgeState} from "../src/bridge/BridgeState.sol";

// Light relay test cases imported from: https://github.com/keep-network/tbtc-v2/blob/cadead9ecd6005325ace4d64288c20733b058352/solidity/test/relay/LightRelay.test.ts

contract LightRelayTest is Test {
    using BitcoinTx for BridgeState.Storage;

    LightRelay public relay;
    BridgeState.Storage internal state;

    struct Header {
        bytes data;
        uint256 height;
    }

    Header nextStartHeader;
    Header genesisHeader;
    Header retargetWrongHeader;
    Header[1] proofHeaders;
    Header[15] retargetHeaders;
    Header orphan437478;
    Header[8] postRetargetChain;

    uint256 genesisDifficulty;
    uint256 nextDifficulty;

    event Genesis(uint256 blockHeight);
    event ProofLengthChanged(uint256 newLength);
    event SubmitterDeauthorized(address submitter);
    event AuthorizationRequirementChanged(bool newStatus);
    event Retarget(uint256 oldDifficulty, uint256 newDifficulty);
    event SubmitterAuthorized(address submitter);

    constructor() public {
        genesisDifficulty = 5646403851534;
        nextDifficulty = 5106422924659;

        orphan437478 = Header({
            data: hex"00000020e156ef206dc738dbe7b7bd449f90b65771292f146213fb000000000000000000c52c39a4e31158ff7c34417c9750038b27c9a94dc08210e53ba624667c9310973d161e5874510418a9f7e0d0",
            height: 437478
        });

        postRetargetChain[0] = Header({
            data: hex"00000020a590cf5dc89b4d2ec22144097231d109c0a2b88431a264030000000000000000a37e2d39aaf31d8bd7c00a07fa9784fee88ab361082f03601119dac4a0d87c8d020d1e58745104189b665b65",
            height: 437472
        });
        postRetargetChain[1] = Header({
            data: hex"00000020b76325d903198d9be86addba452d83c6764644f1235cc70300000000000000001d9e6cc2b0871a8f88db857db870dfa7000f80f3c89c7a0eaaa925974fc392459e111e5874510418f4dcbaed",
            height: 437473
        });
        postRetargetChain[2] = Header({
            data: hex"00000020612bf328b7d0e9395eab852ee7cd9f34818bfbf6d984600000000000000000001698022f9aeaccae63250f24ad66093504b088686d2e307bf7653c700b7e23d1e2111e5874510418914866a7",
            height: 437474
        });
        postRetargetChain[3] = Header({
            data: hex"000000204db13b3109b5b8b62948dfdd179bc3b6dacb8dd84157a10300000000000000005c93b3ce08ce22ba64fb26b8e75d695ac8c2bea0badd34200e698ccc73ca421b97121e58745104183ded0054",
            height: 437475
        });
        postRetargetChain[4] = Header({
            data: hex"00000020a8cd6f757013d17d47bafec89cf0c3a89d0dc501f624a00000000000000000004d7fd5d8c9984f2be03a80a75a4b6fe02dfda894181fcab0b6beb60698c7d6f0db121e587451041816e7fbbd",
            height: 437476
        });
        postRetargetChain[5] = Header({
            data: hex"00000020db3a0776bb7fb1cbe2a5c9263556210069114008784946030000000000000000c8cbccf35d668296e8da72aed974983a8679e5077445b79d1e3e6c4d1bbff2c730161e58745104188674ac6f",
            height: 437477
        });
        postRetargetChain[6] = Header({
            data: hex"00000020e156ef206dc738dbe7b7bd449f90b65771292f146213fb000000000000000000f7e22ae2e5442bac43e18fc032dafd058fbdf886e33a3330e7597ce6eb4c073347161e58745104186cff9846",
            height: 437478
        });
        postRetargetChain[7] = Header({
            data: hex"00000020d339c3b98a46b386a793076518f94b153cf09712231f3f01000000000000000018f078c9c73734d892fef90854381524f30017370d12dfa8319313c5729dbf85b8191e58745104187d43c005",
            height: 437479
        });

        nextStartHeader = Header({
            data: hex"00004020d3b2d7d61ad2d95ffbd556d9e00f07877423600a8da015000000000000000000d192743a2c190a7421f92fefe92505579d7b8eda568cacee13b25751ac704c669d83195cf41e371721bae3e7",
            height: 554400
        });

        genesisHeader = Header({
            data: hex"00000020e2acb3e71e4e443af48e81d381dea7d35e2e8d5e69fe150000000000000000007f2ada224dc4afba6ca37010b099c02322cb5df24fcedb0ff5b87fb3ca64eeaea01a055c7cd9311771f2861e",
            height: 552384
        });

        retargetHeaders[0] = Header({
            data: hex"000040201d375a8534ecd6be1ee481de9391188c9bf0c4fe39050b00000000000000000027852cd901347135f77edb0efdae704aeba4414ff6a25ac3ac1a8f2991e13f632d71195c7cd931174c4002c1",
            height: 554390
        });

        retargetHeaders[1] = Header({
            data: hex"000040209e312154da6d21142bb26b962849db1e4e6294b227c11b00000000000000000033d1d6c65cbfbc8f040e42b0a78047e6553633a0d9fe6cf794f8c93326361b7d7471195c7cd9311715368c2d",
            height: 554391
        });

        retargetHeaders[2] = Header({
            data: hex"000000205e61189112f4ee7ecbb6dc9fecc6c0010b4b817c2a720100000000000000000002b04e0b564fc59be33622435b2049bae3dd0c932cf2798bd5d3450324307616d371195c7cd9311796441d48",
            height: 554392
        });

        retargetHeaders[3] = Header({
            data: hex"00000020e0affbfe12f2eef906436c82d565adb5f1003efd5deb0f00000000000000000051adeb94fc008a292e0c8774161f6b5a43fe4c68d440829118cbd74010d019449774195c7cd93117a27cc4a6",
            height: 554393
        });

        retargetHeaders[4] = Header({
            data: hex"0000002041c5c326d22ca678c79e348628d740847ad1a54efdbc2a000000000000000000ee64ff84da2777ec09aaf34d6f9fad9b445853aeb7421ece6b200a102c7f1726ca76195c7cd93117301e0ab3",
            height: 554394
        });

        retargetHeaders[5] = Header({
            data: hex"0000002088900d4f10be130308e0e3499bf24efb294a6278d8ee070000000000000000007e9746daba41c5604e8708db20a07493aaa5a26c583dd4025d68f56e14536773ca7e195c7cd93117ae46eb69",
            height: 554395
        });

        retargetHeaders[6] = Header({
            data: hex"000000208e3d25087f0c1dc73fb5698fb3a21fc27ab5c09a51a4030000000000000000002fe4c18dc554ac199203ac26358814752bc0db76ed72d2673d3d4f6b20d66760737f195c7cd931171e60b26b",
            height: 554396
        });

        retargetHeaders[7] = Header({
            data: hex"00000020b84058dc5f40275d4c880f9cb63ec5e572524518ce180e0000000000000000008a9ec4242c4003dde9ae9ac167dd36614ee00c56b8ffdf63abf1d0dbdcdcb24fc27f195c7cd931173f2077a9",
            height: 554397
        });

        retargetHeaders[8] = Header({
            data: hex"000000203633c1376556de44ddd528d0a6c27244ee5798bdd4e7110000000000000000008a5a4c9ebf9b6b77e3f63e1f4ddbbe1aeec4b7592554cb3c003068f849647b147180195c7cd93117b8c8e83f",
            height: 554398
        });

        retargetHeaders[9] = Header({
            data: hex"000000202da0c39c117f882d54d03df822915a8a6373be4cfa1a01000000000000000000dd6f24bd263432435f0954c59c022cfd8e5190f4615fc2c249815244a3fe09b34683195c7cd931170f68c64a",
            height: 554399
        });

        retargetHeaders[10] = Header({
            data: hex"00004020d3b2d7d61ad2d95ffbd556d9e00f07877423600a8da015000000000000000000d192743a2c190a7421f92fefe92505579d7b8eda568cacee13b25751ac704c669d83195cf41e371721bae3e7",
            height: 554400
        });

        retargetHeaders[11] = Header({
            data: hex"00000020ae67207125404fa8786acaeb7cff69156fe0a01b0b3c04000000000000000000b668f999166662460c4a9717d3c8e72e3b0c24863fccdd90295dfb0b047aa1f3f484195cf41e37176b176d83",
            height: 554401
        });

        retargetHeaders[12] = Header({
            data: hex"00008020f1b333748571d00f25c262706b03313443f1a4424be70f0000000000000000004d7b233c0f561f7e5d57fb1d9bae5c72576787da5c13ec792d1d87eb1a62795eba85195cf41e3717d904e9c3",
            height: 554402
        });

        retargetHeaders[13] = Header({
            data: hex"00000020b12386369f49f8800f47ab7813e3420b511fd4e2de8907000000000000000000026069ddf2c58926646215a8434823226646051ebcb77b6644d9791732763da8c585195cf41e3717689570ae",
            height: 554403
        });

        retargetHeaders[14] = Header({
            data: hex"00000020e9d169fb84334272f1c3e52748980e54fdbc5b21b16b1d0000000000000000006a504c582c322e88b8478b199536df2ad4b052ffe065975bb8e74321c09e117ef486195cf41e37176b029121",
            height: 554404
        });

        retargetWrongHeader = Header({
            data: hex"1000003007fc81082e2d4d3b44e6ee00ad53a9d926213aee7394960600000000000000003a810e13e6253dd483a95f55cfc3adbc60e112f0d485832ea1d482661487e9dc411cfe56f496061837c4a22f",
            height: 405217
        });

        proofHeaders[0] = Header({
            data: hex"000000208e3d25087f0c1dc73fb5698fb3a21fc27ab5c09a51a4030000000000000000002fe4c18dc554ac199203ac26358814752bc0db76ed72d2673d3d4f6b20d66760737f195c7cd931171e60b26b",
            height: 554396
        });
    }

    function setUp() public {
        relay = new LightRelay();
        relay.genesis(genesisHeader.data, genesisHeader.height, 4);
        vm.warp(14594924750);
        relay.retarget(
            abi.encodePacked(
                retargetHeaders[6].data,
                retargetHeaders[7].data,
                retargetHeaders[8].data,
                retargetHeaders[9].data,
                retargetHeaders[10].data,
                retargetHeaders[11].data,
                retargetHeaders[12].data,
                retargetHeaders[13].data
            )
        );
    }

    function setUpGenesis() public {
        relay = new LightRelay();
        relay.genesis(genesisHeader.data, genesisHeader.height, 4);
    }

    //
    // start genesis
    //
    function test_recordRelayReadyForuse() public {
        assertTrue(relay.ready());
    }

    function test_shouldEmitGenesisEvent() public {
        relay = new LightRelay();
        vm.expectEmit();
        emit Genesis(genesisHeader.height);
        relay.genesis(genesisHeader.data, genesisHeader.height, 4);
        vm.stopPrank();
    }

    function test_recordGenesisEpochDifficulty() public {
        relay = new LightRelay();
        emit Genesis(genesisHeader.height);
        relay.genesis(genesisHeader.data, genesisHeader.height, 4);
        assertEq(relay.getBlockDifficulty(genesisHeader.height), genesisDifficulty, "assertion failed");
    }

    function test_invalidBlockHeight() public {
        relay = new LightRelay();
        vm.expectRevert("Invalid height of relay genesis block");
        relay.genesis(genesisHeader.data, genesisHeader.height + 1, 2);
    }

    function test_invalidHeaderData() public {
        relay = new LightRelay();
        vm.expectRevert("Invalid genesis header length");
        relay.genesis("0xdeadbeef", genesisHeader.height, 2);
    }

    function test_excessiveProofLength() public {
        relay = new LightRelay();
        vm.expectRevert("Invalid genesis header length");
        relay.genesis(
            "04000000473ed7b7ef2fce828c318fd5e5868344a5356c9e93b6040400000000000000004409cae5b7b2f8f18ea55f558c9bfa7c5f4778a1a53172a48fc57e172d0ed3d264c5eb56c3a40618af9bc1c71",
            genesisHeader.height,
            2016
        );
    }

    function test_zeroProofLength() public {
        relay = new LightRelay();
        vm.expectRevert("Proof length may not be zero");
        relay.genesis(genesisHeader.data, genesisHeader.height, 0);
    }

    function test_callByNonOwner() public {
        address nonOwner = address(1234);
        relay = new LightRelay();
        vm.startPrank(nonOwner);
        vm.expectRevert("Ownable: caller is not the owner");
        relay.genesis(genesisHeader.data, genesisHeader.height, 2);
        vm.stopPrank();
    }

    function test_callGenesisTwice() public {
        vm.expectRevert("Genesis already performed");
        relay.genesis(genesisHeader.data, genesisHeader.height, 4);
    }

    //
    // end genesis
    //

    //
    // start setProofLength
    //
    function test_proofLengthBeforeGenesis() public {
        relay = new LightRelay();
        vm.expectRevert("Relay is not ready for use");
        relay.setProofLength(10);
    }

    function test_proofLengthCalledCorrectly() public {
        vm.expectEmit();
        emit ProofLengthChanged(5);
        relay.setProofLength(5);
        vm.stopPrank();
        assertEq(relay.proofLength(), 5);
    }

    function test_excessiveSetProofLength() public {
        vm.expectRevert("Proof length excessive");
        relay.setProofLength(2016);
    }

    function test_zeroSetProofLength() public {
        vm.expectRevert("Proof length may not be zero");
        relay.setProofLength(0);
    }

    function test_unchangedSetProofLength() public {
        vm.expectRevert("Proof length unchanged");
        relay.setProofLength(4);
    }

    function test_setProofLengthCallByNonOwner() public {
        address nonOwner = address(1234);
        vm.startPrank(nonOwner);
        vm.expectRevert("Ownable: caller is not the owner");
        relay.setProofLength(2);
        vm.stopPrank();
    }

    //
    // end setProofLength
    //

    //
    // start authorization
    //
    function test_authorizationRequired() public {
        vm.expectEmit();
        emit AuthorizationRequirementChanged(true);
        relay.setAuthorizationStatus(true);
        assertTrue(relay.authorizationRequired());
        vm.stopPrank();
    }

    function test_authorizationRequiredCallByNonOwner() public {
        address nonOwner = address(1234);
        vm.startPrank(nonOwner);
        vm.expectRevert("Ownable: caller is not the owner");
        relay.setAuthorizationStatus(false);
        vm.stopPrank();
    }

    function test_submitterAuthorization() public {
        address newAuthorizer = address(1234);

        // should start at false
        assertFalse(relay.isAuthorized(newAuthorizer));

        vm.expectEmit();
        emit SubmitterAuthorized(newAuthorizer);
        relay.authorize(newAuthorizer);

        assertTrue(relay.isAuthorized(newAuthorizer));
        vm.stopPrank();
    }

    function test_submitterDeAuthorization() public {
        address author = address(1234);
        relay.authorize(author);

        // should start at false
        assertTrue(relay.isAuthorized(author));

        vm.expectEmit();
        emit SubmitterDeauthorized(author);
        relay.deauthorize(author);

        assertFalse(relay.isAuthorized(author));
        vm.stopPrank();
    }

    //
    // end authorization
    //

    //
    // retarget
    //
    function test_retargetBeforeGenesis() public {
        relay = new LightRelay();
        vm.expectRevert("Relay is not ready for use");
        relay.retarget(
            abi.encodePacked(
                retargetHeaders[0].data, retargetHeaders[1].data, retargetHeaders[2].data, retargetHeaders[3].data
            )
        );
    }

    function test_retarget() public {
        setUpGenesis();

        // we need at least one retarget
        vm.warp(14594924750);

        vm.expectEmit();
        emit Retarget(genesisDifficulty, nextDifficulty);

        relay.retarget(
            abi.encodePacked(
                retargetHeaders[6].data,
                retargetHeaders[7].data,
                retargetHeaders[8].data,
                retargetHeaders[9].data,
                retargetHeaders[10].data,
                retargetHeaders[11].data,
                retargetHeaders[12].data,
                retargetHeaders[13].data
            )
        );

        vm.stopPrank();
    }

    function test_retargetWithIncorrectNumberOfHeader() public {
        setUpGenesis();

        // we need at least one retarget
        vm.warp(14594924750);

        vm.expectRevert("Invalid header length");

        relay.retarget(abi.encodePacked(retargetHeaders[0].data));
    }

    function test_retargetWithFewHeadersBefore() public {
        setUpGenesis();

        // we need at least one retarget
        vm.warp(14594924750);

        vm.expectRevert("Invalid target in pre-retarget headers");
        relay.retarget(
            abi.encodePacked(
                retargetHeaders[7].data,
                retargetHeaders[8].data,
                retargetHeaders[9].data,
                retargetHeaders[10].data,
                retargetHeaders[11].data,
                retargetHeaders[12].data,
                retargetHeaders[13].data,
                retargetHeaders[14].data
            )
        );
        vm.stopPrank();
    }

    function test_retargetWithFewHeadersAfter() public {
        setUpGenesis();

        // we need at least one retarget
        vm.warp(14594924750);

        vm.expectRevert("Invalid target in new epoch");
        relay.retarget(
            abi.encodePacked(
                retargetHeaders[5].data,
                retargetHeaders[6].data,
                retargetHeaders[7].data,
                retargetHeaders[8].data,
                retargetHeaders[9].data,
                retargetHeaders[10].data,
                retargetHeaders[11].data,
                retargetHeaders[12].data
            )
        );
        vm.stopPrank();
    }
    //
    // end retarget
    //

    //
    // start validateChain
    //
    function test_validateChainBeforeGenesis() public {
        relay = new LightRelay();
        vm.expectRevert("Cannot validate chains before relay genesis");
        relay.validateChain(
            abi.encodePacked(
                retargetHeaders[1].data, retargetHeaders[2].data, retargetHeaders[3].data, retargetHeaders[4].data
            )
        );
    }

    function test_validateHeaderChainsEpoch274() public {
        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                retargetHeaders[1].data, retargetHeaders[2].data, retargetHeaders[3].data, retargetHeaders[4].data
            )
        );
        assertEq(headerCount, 4);
    }

    function test_validateShortHeaderChainsEpoch274() public {
        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(retargetHeaders[1].data, retargetHeaders[2].data, retargetHeaders[3].data)
        );
        assertEq(headerCount, 3);
    }

    function test_validateLongHeaderChainsEpoch274() public {
        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                retargetHeaders[1].data,
                retargetHeaders[2].data,
                retargetHeaders[3].data,
                retargetHeaders[4].data,
                retargetHeaders[5].data,
                retargetHeaders[6].data,
                retargetHeaders[7].data,
                retargetHeaders[8].data,
                retargetHeaders[9].data
            )
        );
        assertEq(headerCount, 9);
    }

    function test_validateChainShouldRejectSingleHeaderAfterEpoch274() public {
        vm.expectRevert("Invalid number of headers");
        (uint256 startingHeaderTimestamp, uint256 headerCount) =
            relay.validateChain(abi.encodePacked(retargetHeaders[0].data));
    }

    function test_validateChainShouldRejectUnknownRetargetAfterEpoch274() public {
        vm.expectRevert("Invalid number of headers");
        (uint256 startingHeaderTimestamp, uint256 headerCount) =
            relay.validateChain(abi.encodePacked(retargetHeaders[0].data));
    }

    function test_validateChainShouldRejectChainInFutureEpochAfterEpoch274() public {
        setUpGenesis();
        vm.expectRevert("Invalid target in header chain");

        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                retargetHeaders[10].data, retargetHeaders[11].data, retargetHeaders[12].data, retargetHeaders[13].data
            )
        );
    }

    function test_validateHeaderChainsAfterEpoch275() public {
        relay = new LightRelay();
        relay.genesis(nextStartHeader.data, nextStartHeader.height, 4);
        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                retargetHeaders[10].data, retargetHeaders[11].data, retargetHeaders[12].data, retargetHeaders[13].data
            )
        );
        assertEq(headerCount, 4);
    }

    function test_validateHeaderChainsRejectInPartiallyPastEpochAfterEpoch275() public {
        relay = new LightRelay();
        relay.genesis(nextStartHeader.data, nextStartHeader.height, 4);
        vm.expectRevert("Cannot validate chains before relay genesis");

        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                retargetHeaders[9].data, retargetHeaders[10].data, retargetHeaders[11].data, retargetHeaders[12].data
            )
        );
    }

    function test_validateHeaderChainsRejectInFullyPastEpochAfterEpoch275() public {
        relay = new LightRelay();
        relay.genesis(nextStartHeader.data, nextStartHeader.height, 4);
        vm.expectRevert("Cannot validate chains before relay genesis");

        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                retargetHeaders[6].data, retargetHeaders[7].data, retargetHeaders[8].data, retargetHeaders[9].data
            )
        );
    }

    function test_validateHeaderChainsAfterGenesisEpoch() public {
        setUpGenesis();
        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                retargetHeaders[1].data, retargetHeaders[2].data, retargetHeaders[3].data, retargetHeaders[4].data
            )
        );
        assertEq(headerCount, 4);
    }

    function test_validateHeaderChains3Before1AfterOverRetarget() public {
        setUpGenesis();
        vm.warp(14594924750);
        relay.retarget(
            abi.encodePacked(
                retargetHeaders[6].data,
                retargetHeaders[7].data,
                retargetHeaders[8].data,
                retargetHeaders[9].data,
                retargetHeaders[10].data,
                retargetHeaders[11].data,
                retargetHeaders[12].data,
                retargetHeaders[13].data
            )
        );

        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                retargetHeaders[7].data, retargetHeaders[8].data, retargetHeaders[9].data, retargetHeaders[10].data
            )
        );
        assertEq(headerCount, 4);
    }

    function test_validateHeaderChains2Before2AfterOverRetarget() public {
        setUpGenesis();
        vm.warp(14594924750);
        relay.retarget(
            abi.encodePacked(
                retargetHeaders[6].data,
                retargetHeaders[7].data,
                retargetHeaders[8].data,
                retargetHeaders[9].data,
                retargetHeaders[10].data,
                retargetHeaders[11].data,
                retargetHeaders[12].data,
                retargetHeaders[13].data
            )
        );

        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                retargetHeaders[8].data, retargetHeaders[9].data, retargetHeaders[10].data, retargetHeaders[11].data
            )
        );
        assertEq(headerCount, 4);
    }

    function test_validateHeaderChains1Before3AfterOverRetarget() public {
        setUpGenesis();
        vm.warp(14594924750);
        relay.retarget(
            abi.encodePacked(
                retargetHeaders[6].data,
                retargetHeaders[7].data,
                retargetHeaders[8].data,
                retargetHeaders[9].data,
                retargetHeaders[10].data,
                retargetHeaders[11].data,
                retargetHeaders[12].data,
                retargetHeaders[13].data
            )
        );

        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                retargetHeaders[9].data, retargetHeaders[10].data, retargetHeaders[11].data, retargetHeaders[12].data
            )
        );
        assertEq(headerCount, 4);
    }

    function test_validateChainWithChainReorg() public {
        relay = new LightRelay();
        relay.genesis(postRetargetChain[0].data, postRetargetChain[0].height, 8);

        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                postRetargetChain[0].data,
                postRetargetChain[1].data,
                postRetargetChain[2].data,
                postRetargetChain[3].data,
                postRetargetChain[4].data,
                postRetargetChain[5].data,
                postRetargetChain[6].data,
                postRetargetChain[7].data
            )
        );
        assertEq(headerCount, 8);
    }

    function test_validateChainRejectWithChainReorg() public {
        relay = new LightRelay();
        relay.genesis(postRetargetChain[0].data, postRetargetChain[0].height, 8);
        vm.expectRevert("Invalid chain");

        (uint256 startingHeaderTimestamp, uint256 headerCount) = relay.validateChain(
            abi.encodePacked(
                postRetargetChain[1].data,
                postRetargetChain[2].data,
                postRetargetChain[3].data,
                postRetargetChain[4].data,
                postRetargetChain[5].data,
                postRetargetChain[6].data,
                orphan437478.data,
                postRetargetChain[7].data
            )
        );
    }

    //
    // end validateChain
    //

    //
    // start getBlockDifficulty
    //
    function test_getBlockDifficultyBeforeGenesis() public {
        relay = new LightRelay();
        vm.expectRevert("Epoch is not proven to the relay yet");
        relay.getBlockDifficulty(genesisHeader.height);
    }

    function test_getBlockDifficultyAfterGenesis() public {
        setUpGenesis();
        // should return the difficulty for the first block of the epoch
        assertEq(relay.getBlockDifficulty(genesisHeader.height), genesisDifficulty);

        // should return the difficulty for the last block of the epoch
        assertEq(relay.getBlockDifficulty(554399), genesisDifficulty);

        // should revert for blocks before genesis
        vm.expectRevert("Epoch is before relay genesis");
        relay.getBlockDifficulty(552383);

        // should revert for blocks after the latest epoch
        vm.expectRevert("Epoch is not proven to the relay yet");
        relay.getBlockDifficulty(554400);
    }

    function test_getBlockDifficultyAfterRetarget() public {
        setUpGenesis();

        vm.warp(14594924750);
        relay.retarget(
            abi.encodePacked(
                retargetHeaders[6].data,
                retargetHeaders[7].data,
                retargetHeaders[8].data,
                retargetHeaders[9].data,
                retargetHeaders[10].data,
                retargetHeaders[11].data,
                retargetHeaders[12].data,
                retargetHeaders[13].data
            )
        );

        // should return the difficulty for the first block of the genesis epoch
        assertEq(relay.getBlockDifficulty(genesisHeader.height), genesisDifficulty);

        // should return the difficulty for the last block of the genesis epoch
        assertEq(relay.getBlockDifficulty(554399), genesisDifficulty);

        // should return the difficulty for the first block of the next epoch
        assertEq(relay.getBlockDifficulty(554400), nextDifficulty);

        // should return the difficulty for the first block of the next epoch
        assertEq(relay.getBlockDifficulty(556415), nextDifficulty);

        // should revert for blocks before genesis
        vm.expectRevert("Epoch is before relay genesis");
        relay.getBlockDifficulty(552383);

        // should revert for blocks after the latest epoch
        vm.expectRevert("Epoch is not proven to the relay yet");
        relay.getBlockDifficulty(556416);
    }
    //
    // end getBlockDifficulty
    //

    //
    // start getEpochDifficulty
    //
    function test_getEpochDifficultyBeforeGenesis() public {
        relay = new LightRelay();

        vm.expectRevert("Epoch is not proven to the relay yet");
        relay.getEpochDifficulty(genesisHeader.height / 2016);
    }

    function test_getEpochDifficultyAfterGenesis() public {
        setUpGenesis();

        // should return the difficulty for the genesis epoch
        assertEq(relay.getEpochDifficulty(genesisHeader.height / 2016), genesisDifficulty);

        // should revert for epochs before genesis
        vm.expectRevert("Epoch is before relay genesis");
        relay.getEpochDifficulty((genesisHeader.height / 2016) - 1);

        // should revert for unproven epochs
        vm.expectRevert("Epoch is not proven to the relay yet");
        relay.getEpochDifficulty((genesisHeader.height / 2016) + 1);
    }

    function test_getEpochDifficultyAfterRetarget() public {
        // should return the difficulty for the genesis epoch
        assertEq(relay.getEpochDifficulty(274), genesisDifficulty);

        // should return the difficulty for the next epoch
        assertEq(relay.getEpochDifficulty(275), nextDifficulty);

        // should revert for epochs before genesis
        vm.expectRevert("Epoch is before relay genesis");
        relay.getEpochDifficulty(273);

        // should revert for unproven epochs
        vm.expectRevert("Epoch is not proven to the relay yet");
        relay.getEpochDifficulty(276);
    }

    //
    // end getEpochDifficulty
    //

    //
    // start getRelayRange
    //
    function test_getRelayRangeBeforeGenesis() public {
        relay = new LightRelay();
        (uint256 relayGenesis, uint256 currentEpochEnd) = relay.getRelayRange();
        assertEq(relayGenesis, 0);
        assertEq(currentEpochEnd, 2015);
    }

    function test_getRelayRangeAfterGenesis() public {
        setUpGenesis();
        (uint256 relayGenesis, uint256 currentEpochEnd) = relay.getRelayRange();
        // should return two epochs
        assertEq(relayGenesis, genesisHeader.height);
        assertEq(currentEpochEnd, 554399);
    }

    function test_getRelayRangeAfterRetarget() public {
        (uint256 relayGenesis, uint256 currentEpochEnd) = relay.getRelayRange();
        // should return two epochs
        assertEq(relayGenesis, genesisHeader.height);
        assertEq(currentEpochEnd, 556415);
    }
    //
    // end getRelayRange
    //

    //
    // start getCurrentEpochDifficulty
    //
    function test_getCurrentEpochDifficultyBeforeGenesis() public {
        relay = new LightRelay();
        assertEq(relay.getCurrentEpochDifficulty(), 0);
    }

    function test_getCurrentEpochDifficultyAfterGenesis() public {
        setUpGenesis();
        assertEq(relay.getCurrentEpochDifficulty(), genesisDifficulty);
    }

    function test_getCurrentEpochDifficultyAfterRetarget() public {
        assertEq(relay.getCurrentEpochDifficulty(), nextDifficulty);
    }
    //
    // end getCurrentEpochDifficulty
    //

    //
    // start getPrevEpochDifficulty
    //
    function test_getPrevEpochDifficultyBeforeGenesis() public {
        relay = new LightRelay();
        assertEq(relay.getPrevEpochDifficulty(), 0);
    }

    function test_getPrevEpochDifficultyAfterGenesis() public {
        setUpGenesis();
        assertEq(relay.getPrevEpochDifficulty(), 0);
    }

    function test_getPrevEpochDifficultyAfterRetarget() public {
        assertEq(relay.getPrevEpochDifficulty(), genesisDifficulty);
    }

    //
    // end getPrevEpochDifficulty
    //

    //
    // start getCurrentAndPrevEpochDifficulty
    //
    function test_getCurrentAndPrevEpochDifficultyBeforeGenesis() public {
        relay = new LightRelay();
        (uint256 current, uint256 previous) = relay.getCurrentAndPrevEpochDifficulty();
        assertEq(current, 0);
        assertEq(previous, 0);
    }

    function test_getCurrentAndPrevEpochDifficultyAfterGenesis() public {
        setUpGenesis();
        (uint256 current, uint256 previous) = relay.getCurrentAndPrevEpochDifficulty();
        assertEq(current, genesisDifficulty);
        assertEq(previous, 0);
    }

    function test_getCurrentAndPrevEpochDifficultyAfterRetarget() public {
        (uint256 current, uint256 previous) = relay.getCurrentAndPrevEpochDifficulty();
        assertEq(current, nextDifficulty);
        assertEq(previous, genesisDifficulty);
    }

    //
    // start validateProof
    //
    function test_validateProof() public {
        state.relay = relay;
        state.txProofDifficultyFactor = 1;

        // txId = 15afe550f468cf0134557533e7f0bd6f210c1a2791d75a8ea57f17c4209448f9
        (bytes32 txHash) = state.validateProof(
            BitcoinTx.Info({
                version: hex"02000000",
                inputVector: hex"01123c43f161517343e93191e838b2f04356665ff526bf95cfe6c9986de7a10a3e010000001716001402c8f68bb02b257de42f5ca11b525bd3b47a0369feffffff",
                outputVector: hex"02c20d02070000000017a9140ad70885943285fae498b08e00f90ac9e403e1bb8760d360020000000017a914f9b4725bd496b113d44be16f92d14df62096387387",
                locktime: hex"76750800"
            }),
            BitcoinTx.Proof({
                merkleProof: hex"180011aff0b64a62d2587d6d1b47d4049b77940222084640d71e3af330f1ed7a463219df439dadf415f9e9e0c3887bf25658ccdd75df8226affdb9e85708c6220f043131ee89dc6b9a2791827a62132348204b681e5801c85ffc5bf8857f1ae481210664933aeb429aeaef433148b9c8deb20ff31189ffcf10e940788897fa2ff4ebf1173c5b7eff6b3cf10e0b55bd59e8375ac1717eb2dc5c0eda687f4d4e3db16ec84a99bbda99dd161e44f8830944e052832d8adc56b73bad0ba280b07d491bf705b5562190e01e49f7dcbe40b35d1f0780f10bd69d5c7b6d5739f46ae6302e37fbe381a82a5a11c5436e6432554b46b93fee53da85109f062fa6e07e070bc2bce25a294949d50baccb0fb46b822d4d203e6784065b233e81f3c52e4ca9a538180159cc5864497e9b2ef9a7412e49d6fb724e13d709b50c1fd97306e255296e77523b15bfd67423e801fb428d40597bbeffda818d15759c9dd89b4bac4e87438fff12f1390e675092e1c8a446347c06026ee2b35f5b140b40b8acbb88ee7d",
                txIndexInBlock: 1,
                bitcoinHeaders: abi.encodePacked(proofHeaders[0].data)
            })
        );
    }

    function test_validateProofWithInvalidData() public {
        state.relay = relay;
        state.txProofDifficultyFactor = 1;
        // txId = 15afe550f468cf0134557533e7f0bd6f210c1a2791d75a8ea57f17c4209448f9
        vm.expectRevert("Invalid input vector provided");
        state.validateProof(
            BitcoinTx.Info({
                version: hex"02000000",
                // invalid input
                inputVector: hex"01123c43f161517343e93191e838b2f04356665ff526bf95cfe6c9986de7a10a3e010000001716001402c8f68bb02b257de42f5ca11b525bd3b25bd3b47a0369feffffff",
                outputVector: hex"02c20d02070000000017a9140ad70885943285fae498b08e00f90ac9e403e1bb8760d360020000000017a914f9b4725bd496b113d44be16f92d14df62096387387",
                locktime: hex"76750800"
            }),
            BitcoinTx.Proof({
                merkleProof: hex"180011aff0b64a62d2587d6d1b47d4049b77940222084640d71e3af330f1ed7a463219df439dadf415f9e9e0c3887bf25658ccdd75df8226affdb9e85708c6220f043131ee89dc6b9a2791827a62132348204b681e5801c85ffc5bf8857f1ae481210664933aeb429aeaef433148b9c8deb20ff31189ffcf10e940788897fa2ff4ebf1173c5b7eff6b3cf10e0b55bd59e8375ac1717eb2dc5c0eda687f4d4e3db16ec84a99bbda99dd161e44f8830944e052832d8adc56b73bad0ba280b07d491bf705b5562190e01e49f7dcbe40b35d1f0780f10bd69d5c7b6d5739f46ae6302e37fbe381a82a5a11c5436e6432554b46b93fee53da85109f062fa6e07e070bc2bce25a294949d50baccb0fb46b822d4d203e6784065b233e81f3c52e4ca9a538180159cc5864497e9b2ef9a7412e49d6fb724e13d709b50c1fd97306e255296e77523b15bfd67423e801fb428d40597bbeffda818d15759c9dd89b4bac4e87438fff12f1390e675092e1c8a446347c06026ee2b35f5b140b40b8acbb88ee7d",
                txIndexInBlock: 1,
                bitcoinHeaders: abi.encodePacked(proofHeaders[0].data)
            })
        );

        // Invalid output vector provided ()
        vm.expectRevert("Invalid output vector provided");
        state.validateProof(
            BitcoinTx.Info({
                version: hex"02000000",
                inputVector: hex"01123c43f161517343e93191e838b2f04356665ff526bf95cfe6c9986de7a10a3e010000001716001402c8f68bb02b257de42f5ca11b525bd3b47a0369feffffff",
                // invalid output
                outputVector: hex"0285943285fae498b08e00f90ac9e403e1bb8760d360020000000017a914f9b4725bd496b113d44be16f92d14df62096387387",
                locktime: hex"76750800"
            }),
            BitcoinTx.Proof({
                merkleProof: hex"180011aff0b64a62d2587d6d1b47d4049b77940222084640d71e3af330f1ed7a463219df439dadf415f9e9e0c3887bf25658ccdd75df8226affdb9e85708c6220f043131ee89dc6b9a2791827a62132348204b681e5801c85ffc5bf8857f1ae481210664933aeb429aeaef433148b9c8deb20ff31189ffcf10e940788897fa2ff4ebf1173c5b7eff6b3cf10e0b55bd59e8375ac1717eb2dc5c0eda687f4d4e3db16ec84a99bbda99dd161e44f8830944e052832d8adc56b73bad0ba280b07d491bf705b5562190e01e49f7dcbe40b35d1f0780f10bd69d5c7b6d5739f46ae6302e37fbe381a82a5a11c5436e6432554b46b93fee53da85109f062fa6e07e070bc2bce25a294949d50baccb0fb46b822d4d203e6784065b233e81f3c52e4ca9a538180159cc5864497e9b2ef9a7412e49d6fb724e13d709b50c1fd97306e255296e77523b15bfd67423e801fb428d40597bbeffda818d15759c9dd89b4bac4e87438fff12f1390e675092e1c8a446347c06026ee2b35f5b140b40b8acbb88ee7d",
                txIndexInBlock: 1,
                bitcoinHeaders: abi.encodePacked(proofHeaders[0].data)
            })
        );

        vm.expectRevert("Insufficient accumulated difficulty in header chain");
        state.evaluateProofDifficulty(
            // header difficult is 1 but expected difficult is 2
            abi.encodePacked(proofHeaders[0].data)
        );
    }

    //
    // end validateProof
    //

    //
    // start evaluateProofDifficulty
    //
    function test_evaluateProofDifficulty() public {
        state.relay = relay;
        state.txProofDifficultyFactor = 1;

        state.evaluateProofDifficulty(abi.encodePacked(proofHeaders[0].data));
    }

    //
    // end evaluateProofDifficulty
    //
}
