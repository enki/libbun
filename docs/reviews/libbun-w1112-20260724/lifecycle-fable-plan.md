# Correction-3 Fable file plan: Lifecycle/JSC interruption/retained-host/quarantine/reaper/shutdown

- Prior independent verdict: PART BUNDLE REVISE at 0a3844d11bb42d67550da0bb1e069ecf17fbe69d
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/lifecycle-prompt.md
- Prompt SHA-256: fad68fc98857a4cdcdba72141bd9d7716ab5f0eee7b2961cbcae4d35ac1d3dd7
- Ordered file plan: docs/reviews/libbun-w1112-20260724/lifecycle-files.txt
- Ordered file count: 39
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 2 | docs/LIBBUN-W1112-POISON-INSTALLER-EVIDENCE-20260724.md | 72ece5d9eeef7a220ee4d309d5e28f7c852376c7fecd6288cddc4e643dc61fe8 | 2382 |
| 3 | docs/reviews/libbun-w1112-20260724/correction3-index.md | 39084f069d98bc4c0898d648d63e994e38c0a1011881f3b5e78d3732783da4e1 | 2011 |
| 4 | docs/reviews/libbun-w1112-20260724/verdict-snapshot.md | 3138238c945c94f8212855889b0d23600fd97009a006bc57ea862a5aa7d6a6d6 | 16588 |
| 5 | docs/reviews/libbun-w1112-20260724/lifecycle-correction2-independent-verdict.md | c51b2da064580f411759b1a436fb414abe89d39de3956f8834d1f8d5b1dc6a5a | 9282 |
| 6 | docs/reviews/libbun-w1112-20260724/lifecycle-vendored-jsc-source-bundle.md | 67c94590d9b2203205a74dd3c49d2bb1a79f83ba0123d6d012d70f2244dd2849 | 15244 |
| 7 | docs/reviews/libbun-w1112-20260724/process-drop-caller-and-fixture-report.md | a27f74cc0ae6a76719000312b10c7180c28403ab24890afa1b875c12bf3d8d34 | 79220 |
| 8 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 9 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 10 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 11 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 12 | runtime/src/main.rs | 3cfa3711281938d2752ce45d4f5b4a3395466ce1df5550e2647a4572e6dfccee | 1117 |
| 13 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 14 | native/Cargo.toml | 5d624b8bcee44bab2a2ae8f87bc0238cc15c16b5975b0f071af7e80581b0b6fd | 639 |
| 15 | runtime/Cargo.toml | c9c8a17548f5350805af9c95835cf7a8c27055a54b26992e0ffffd1fd352a7d2 | 439 |
| 16 | wire/Cargo.toml | a0517ebfe3c61df2f6e2516aa39f8ca56851eb18d44e21f55a47ae2e8813dc8f | 125 |
| 17 | vendor/bun/src/jsc/VirtualMachine.rs | aeb953a4b048201059b7230b0f2d64c8a145ca196f011f3a93993240607a9c3a | 296637 |
| 18 | vendor/bun/src/jsc/JSGlobalObject.rs | d6dc512fb3021f0c57bee74fe66230eaebf5f7b94f3bf75602af0ea3a045786e | 69350 |
| 19 | vendor/bun/src/jsc/VM.rs | 47aa4f3b8642dfefe21dd56c2fa446889cd701cc3e9fdbf7d962bf4f6aee7119 | 8822 |
| 20 | vendor/bun/src/jsc/virtual_machine_exports.rs | 95946faae0cd89ac0ccf1d037312e307a88742beed7e5ac2770f91910582b23b | 13386 |
| 21 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-value-model/Cargo.toml | 8f6a90758239f6dc323ff83fcf71cc6915cbd4291998e1cc8718d171a364ca64 | 709 |
| 22 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs | c4dcc653964e40c424bfff20278dd62e6e1d04e7134d09320bc21208c08b6e2d | 108022 |
| 23 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-rust-sdk-static-provider-host/Cargo.toml | cbe99d18d1eb62551907fa249b6632c0ca0a691d246559b0459039c25d9339aa | 1074 |
| 24 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-external-capability-provider-owner/src/lib.rs | e3818b7ce41be8a6a5c5f424418881b4f433883b54a81ddeb96650b2ce9a4439 | 14848 |
| 25 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-external-capability-provider-owner/Cargo.toml | 6f9a0d10b0aa4f13049b63013f1469726de88a4adbd2ac48bc862c29b5bb9ee0 | 690 |
| 26 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/src/provider_host_set.rs | 14a1994c40bb5c0dfb6e0610b523b5deaf67404e4a2785971cd3c56501a77699 | 45089 |
| 27 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/src/external_transport.rs | b855d0cc8662591ee1f53473831a47034e7c59ecde415883c33dadb6908f4623 | 4907 |
| 28 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/Cargo.toml | ce3c3333e5e73bd678f11f4b18cd447e82aee0891c1781d89c6f51aaab3083d1 | 829 |
| 29 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/Cargo.toml | fe357e88528b8f95f48a5b5afc7ddf73bf62294fb0de686b60099bdc7614ee3e | 2847 |
| 30 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/lib.rs | 3575c876d907efdd0f338111e0ff9f567f69b5689a7a671c91b320ceaed9566e | 5914 |
| 31 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/body_authority_registry.rs | 96fe11bf1b1dd64e28cdbf6bc419cccd6c975f063e73a025da30057130573521 | 4718 |
| 32 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs | 3eed047f11c5ac8bb8aee51dc4c5f7e96520af3b8ed41874d6736f643018c9de | 2269 |
| 33 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs | 48b30f77c809f06b7dfb130b11e190cff8e742809140840d14a3a4b5fb9a4323 | 3030 |
| 34 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs | 5519fbbf653aa19cc883dc881eaaf4ff2587dc13bb2b7ab4d3ac2d9575a3b8d5 | 85748 |
| 35 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-provider-host-set-owner/Cargo.toml | 00c9fd4051072e2b66e1342ad118b9e44f7637bcea68325c4c140f2ef692e93b | 400 |
| 36 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-provider-host-set-owner/src/lib.rs | bc6cf5673711c59e8352599643e7c36b54f2e2f2e2b9d005902566a5a3cc265b | 5724 |
| 37 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss | 3ae07a264289b52bc77c4bf8da0e88c659a3ded32db5d8ea0d3799953ca361c0 | 588 |
| 38 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss | 9a26ebc4737439e4ae0e8342a3b19a357d2a8008d2d09330cc92fcae924dae43 | 771 |
| 39 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/negative/ss/provider/external_provider_json_nfc_duplicate_keys.test.ss | 5d238da6cfd9e1f46a46fc243409d62c3940964cb084343adae08b9079d6c99d | 493 |

No Fable session, request, response, or output exists. Fresh literal independent PART BUNDLE PASS verdicts for correction 3 remain required before launch.
