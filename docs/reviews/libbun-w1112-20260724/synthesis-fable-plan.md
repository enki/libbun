# Correction-3 Fable file plan: W1-11/W1-12 full-SCC synthesis

- Prior independent verdicts: all three correction-2 parts are BUNDLE REVISE
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/synthesis-prompt.md
- Prompt SHA-256: 0834029606f2fbd3f8a9a38c56cc86cc91bc04385ee388d91bece6bf557861c3
- Ordered file plan: docs/reviews/libbun-w1112-20260724/synthesis-files.txt
- Ordered file count: 46
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | docs/reviews/libbun-w1112-20260724/owner-generative-manifest.json | 22facdff8cbf7b8092369961a7cfb33537d85aef7208acb1b206c98435f98af1 | 16202 |
| 2 | docs/reviews/libbun-w1112-20260724/owner-generative-prompt.md | 8dd6d0f33673d54d5ddb88a2a2c42abbaf3ccb0efc5317f0e94cb82548516b5f | 4693 |
| 3 | docs/reviews/libbun-w1112-20260724/owner-generative-oracle-dry-run.txt | d344e67e2bbe2fd918332482db1766e69acdee20ab5729f856306b1230dd1827 | 6875 |
| 4 | docs/reviews/libbun-w1112-20260724/owner-generative-fable-plan.md | 7ccdf6edf87ad46e195af2d365bad0c2bdce5d773c3ce53be1d5cc2436a3471a | 10456 |
| 5 | docs/reviews/libbun-w1112-20260724/owner-generative-files.txt | 1e5e2d22bf1d8b664e9bdfcc241062d2771cdd3bf60b482628b9b69865a60ef5 | 5518 |
| 6 | docs/reviews/libbun-w1112-20260724/lifecycle-manifest.json | 4a337e421686c3066be529bcab7ad4eaa1f202546ac93b60fb5dc9b99fec836e | 13118 |
| 7 | docs/reviews/libbun-w1112-20260724/lifecycle-prompt.md | fad68fc98857a4cdcdba72141bd9d7716ab5f0eee7b2961cbcae4d35ac1d3dd7 | 3724 |
| 8 | docs/reviews/libbun-w1112-20260724/lifecycle-oracle-dry-run.txt | 7f154b257c053a08d7f75cb99c99d9922f69a65727caa52cd868404ad70ef9ee | 4974 |
| 9 | docs/reviews/libbun-w1112-20260724/lifecycle-fable-plan.md | 39f1a8c4777310a3029068b8b81c2a4fb61a05cf27ffcbbab7d115cc10810491 | 7924 |
| 10 | docs/reviews/libbun-w1112-20260724/lifecycle-files.txt | 290820a3d841842f42fe64ba710a7d0ed96e8ffd708fe274497665f5877940b9 | 3835 |
| 11 | docs/reviews/libbun-w1112-20260724/containment-release-manifest.json | c5646166de5f2763613a7cd37efc5ed987cd7c9148241e5e403468aade55671c | 13331 |
| 12 | docs/reviews/libbun-w1112-20260724/containment-release-prompt.md | 5bd10e1239095553de044151d47a59b8fc15e78429e62b7eaf8764510b75af7c | 3875 |
| 13 | docs/reviews/libbun-w1112-20260724/containment-release-oracle-dry-run.txt | f5728c3b80bd01f91dae041d9a2deb5aa7e770b9338d03a5870a7e7197845c49 | 4339 |
| 14 | docs/reviews/libbun-w1112-20260724/containment-release-fable-plan.md | c85ca578e74cd95fd74ef19265e9bcab7e84506db88798ad237782a9af5a53d3 | 7720 |
| 15 | docs/reviews/libbun-w1112-20260724/containment-release-files.txt | 9d02c5946e9a46b231179cc6baf5950d2853f96c08b5b068be4741bdd512330e | 3053 |
| 16 | README.md | 7b8a88d47e3ccf5fe518f0dfa44bdc09d26440dc7e461cb33160cf1e2e618d70 | 3369 |
| 17 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 18 | docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md | 7c8f9d64d9fc4d05f5581097f651af28f92f221a81852d81d0a420770420bc08 | 16493 |
| 19 | docs/LIBBUN-WORKER-RELEASE-CONTRACT.md | 030f64ecfbdcc046dec66d4c767333d50ed21471c084c885e2576c770f1d394c | 8765 |
| 20 | docs/LIBBUN-W1112-POISON-INSTALLER-EVIDENCE-20260724.md | 72ece5d9eeef7a220ee4d309d5e28f7c852376c7fecd6288cddc4e643dc61fe8 | 2382 |
| 21 | docs/reviews/libbun-w1112-20260724/correction3-index.md | 39084f069d98bc4c0898d648d63e994e38c0a1011881f3b5e78d3732783da4e1 | 2011 |
| 22 | docs/reviews/libbun-w1112-20260724/verdict-snapshot.md | 3138238c945c94f8212855889b0d23600fd97009a006bc57ea862a5aa7d6a6d6 | 16588 |
| 23 | docs/reviews/libbun-w1112-20260724/owner-generative-correction2-independent-verdict.md | 27ad245e609a0f8d8e4ee63e24caae78889b988463d761db486fbd74627092f4 | 13736 |
| 24 | docs/reviews/libbun-w1112-20260724/lifecycle-correction2-independent-verdict.md | c51b2da064580f411759b1a436fb414abe89d39de3956f8834d1f8d5b1dc6a5a | 9282 |
| 25 | docs/reviews/libbun-w1112-20260724/containment-release-correction2-independent-verdict.md | 9f23368f53133134da8472be30c6608d4d4fffdc942e7797a452c3d13ff6a6ea | 10001 |
| 26 | docs/reviews/libbun-w1112-20260724/lifecycle-vendored-jsc-source-bundle.md | 67c94590d9b2203205a74dd3c49d2bb1a79f83ba0123d6d012d70f2244dd2849 | 15244 |
| 27 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 28 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 29 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 30 | native/build.rs | 4122c83383a012da82336b223bc10f625f7fc49da33375b76a775ac212a0c72d | 5195 |
| 31 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 32 | runtime/src/main.rs | 3cfa3711281938d2752ce45d4f5b4a3395466ce1df5550e2647a4572e6dfccee | 1117 |
| 33 | runtime/build.rs | c3badf2aaf44535f8d5bedbfa45c36e21c69626e69b4aad44a79eb4f29c91be0 | 4902 |
| 34 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 35 | native/Cargo.toml | 5d624b8bcee44bab2a2ae8f87bc0238cc15c16b5975b0f071af7e80581b0b6fd | 639 |
| 36 | runtime/Cargo.toml | c9c8a17548f5350805af9c95835cf7a8c27055a54b26992e0ffffd1fd352a7d2 | 439 |
| 37 | wire/Cargo.toml | a0517ebfe3c61df2f6e2516aa39f8ca56851eb18d44e21f55a47ae2e8813dc8f | 125 |
| 38 | tests/public_api_boundary.rs | e53307e8fdcd0e12ed63056a32c8e1836b24e866acd00bdf4bd5da25afc9b370 | 1784 |
| 39 | vendor/bun/src/jsc/VirtualMachine.rs | aeb953a4b048201059b7230b0f2d64c8a145ca196f011f3a93993240607a9c3a | 296637 |
| 40 | vendor/bun/src/jsc/JSGlobalObject.rs | d6dc512fb3021f0c57bee74fe66230eaebf5f7b94f3bf75602af0ea3a045786e | 69350 |
| 41 | vendor/bun/src/jsc/VM.rs | 47aa4f3b8642dfefe21dd56c2fa446889cd701cc3e9fdbf7d962bf4f6aee7119 | 8822 |
| 42 | vendor/bun/src/jsc/virtual_machine_exports.rs | 95946faae0cd89ac0ccf1d037312e307a88742beed7e5ac2770f91910582b23b | 13386 |
| 43 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs | c4dcc653964e40c424bfff20278dd62e6e1d04e7134d09320bc21208c08b6e2d | 108022 |
| 44 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-value-model/src/lib.rs | 90f0cb4dd8a4a71afcaf4ef81724088ba5f44348e63cfd304944b94027b0940a | 20764 |
| 45 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-external-capability-provider-owner/src/lib.rs | e3818b7ce41be8a6a5c5f424418881b4f433883b54a81ddeb96650b2ce9a4439 | 14848 |
| 46 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/src/external_transport.rs | b855d0cc8662591ee1f53473831a47034e7c59ecde415883c33dadb6908f4623 | 4907 |

No Fable session, request, response, or output exists. Fresh literal independent PART BUNDLE PASS verdicts for correction 3 remain required before launch.
