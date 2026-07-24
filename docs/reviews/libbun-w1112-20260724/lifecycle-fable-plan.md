# Correction-4 Fable file plan: Lifecycle/JSC interruption/retained-host/quarantine/reaper/shutdown

- Prior independent verdict: PART BUNDLE REVISE at a5ab10f422fb955b899e6ce1089b8c74a4600860
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/lifecycle-prompt.md
- Prompt SHA-256: db33f0c83d35142167661c69d3eb55118a0c333753ba4d9f76073519051e230b
- Ordered file plan: docs/reviews/libbun-w1112-20260724/lifecycle-files.txt
- Ordered file count: 37
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 2 | docs/LIBBUN-W1112-POISON-INSTALLER-EVIDENCE-20260724.md | 72ece5d9eeef7a220ee4d309d5e28f7c852376c7fecd6288cddc4e643dc61fe8 | 2382 |
| 3 | docs/reviews/libbun-w1112-20260724/correction4-index.md | f2c4b1452288fe220cfe1a0ae167a2c543cc8d2c6ff87843991e99fc397bfbbc | 2074 |
| 4 | docs/reviews/libbun-w1112-20260724/lifecycle-correction3-independent-verdict.md | 82a9678c444921696178bc051d27484f7ccfa50ea154a52dc369b6a58733a938 | 12423 |
| 5 | docs/reviews/libbun-w1112-20260724/lifecycle-vendored-jsc-source-bundle.md | 0b0c735ef749de1929678c89405ba96537c46e3dd2e9ceb15a89e4ac6d8b2e2f | 15244 |
| 6 | docs/reviews/libbun-w1112-20260724/lifecycle-process-worker-source-bundle.md | 847aa002148fa3c3612c160c56623e425d4e32bfe092df2b152e7a88a346d62d | 70338 |
| 7 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 8 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 9 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 10 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 11 | runtime/src/main.rs | 3cfa3711281938d2752ce45d4f5b4a3395466ce1df5550e2647a4572e6dfccee | 1117 |
| 12 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 13 | native/Cargo.toml | 5d624b8bcee44bab2a2ae8f87bc0238cc15c16b5975b0f071af7e80581b0b6fd | 639 |
| 14 | runtime/Cargo.toml | c9c8a17548f5350805af9c95835cf7a8c27055a54b26992e0ffffd1fd352a7d2 | 439 |
| 15 | wire/Cargo.toml | a0517ebfe3c61df2f6e2516aa39f8ca56851eb18d44e21f55a47ae2e8813dc8f | 125 |
| 16 | vendor/bun/src/jsc/VirtualMachine.rs | aeb953a4b048201059b7230b0f2d64c8a145ca196f011f3a93993240607a9c3a | 296637 |
| 17 | vendor/bun/src/jsc/JSGlobalObject.rs | d6dc512fb3021f0c57bee74fe66230eaebf5f7b94f3bf75602af0ea3a045786e | 69350 |
| 18 | vendor/bun/src/jsc/VM.rs | 47aa4f3b8642dfefe21dd56c2fa446889cd701cc3e9fdbf7d962bf4f6aee7119 | 8822 |
| 19 | vendor/bun/src/jsc/virtual_machine_exports.rs | 95946faae0cd89ac0ccf1d037312e307a88742beed7e5ac2770f91910582b23b | 13386 |
| 20 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/Cargo.toml | dd05cd50a38d1ed15cd2f5276c29c6039688eed4f66052b9591946d2a4f74d9b | 24507 |
| 21 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss/Cargo.toml | 1a004d65bf705642c090df2249478d83a1ac20740f572b82a296e31375de9a69 | 1240 |
| 22 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss/src/product.rs | 916e4474f9d3d0f2785f7613200b477bc0a0df8ae21245ba4d644650f93d5525 | 37483 |
| 23 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss/tests/external_capability_provider.rs | 22aa9c60530d9ab1a7d8e1d4ca9b3587d5dd520e2a3f7b442d7046072fc35191 | 14251 |
| 24 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs | 847c4f7b13917810adaa373299ae0df20c373cd4671a97eac265c6c093cc3399 | 78342 |
| 25 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/Cargo.toml | fe357e88528b8f95f48a5b5afc7ddf73bf62294fb0de686b60099bdc7614ee3e | 2847 |
| 26 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/lib.rs | 3575c876d907efdd0f338111e0ff9f567f69b5689a7a671c91b320ceaed9566e | 5914 |
| 27 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/body_authority_registry.rs | 96fe11bf1b1dd64e28cdbf6bc419cccd6c975f063e73a025da30057130573521 | 4718 |
| 28 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs | 3eed047f11c5ac8bb8aee51dc4c5f7e96520af3b8ed41874d6736f643018c9de | 2269 |
| 29 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs | 48b30f77c809f06b7dfb130b11e190cff8e742809140840d14a3a4b5fb9a4323 | 3030 |
| 30 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs | 5519fbbf653aa19cc883dc881eaaf4ff2587dc13bb2b7ab4d3ac2d9575a3b8d5 | 85748 |
| 31 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs | 3be90814fe8071aed3c48778c307a63266d3fc10aa689581e73f7a4e6bf83012 | 6025 |
| 32 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs | 54c9aaee5abca13fb791eeffff19e8abd3b02134db567bec5135d439bc691760 | 29621 |
| 33 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-provider-host-set-owner/Cargo.toml | 00c9fd4051072e2b66e1342ad118b9e44f7637bcea68325c4c140f2ef692e93b | 400 |
| 34 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-provider-host-set-owner/src/lib.rs | bc6cf5673711c59e8352599643e7c36b54f2e2f2e2b9d005902566a5a3cc265b | 5724 |
| 35 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss | 3ae07a264289b52bc77c4bf8da0e88c659a3ded32db5d8ea0d3799953ca361c0 | 588 |
| 36 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss | 9a26ebc4737439e4ae0e8342a3b19a357d2a8008d2d09330cc92fcae924dae43 | 771 |
| 37 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/negative/ss/provider/external_provider_json_nfc_duplicate_keys.test.ss | 5d238da6cfd9e1f46a46fc243409d62c3940964cb084343adae08b9079d6c99d | 493 |

No Fable session, request, response, or output exists. Fresh literal independent PART BUNDLE PASS verdicts for correction 4 remain required before launch.
