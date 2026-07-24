# Correction-2 Fable file plan: Lifecycle/JSC/quarantine/durable reaper/completion/restart/shutdown

- Prior independent verdict: BUNDLE REVISE at 7a5bfc1cf71299681a9edfb8d4f5a8a7501494e1
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/lifecycle-prompt.md
- Prompt SHA-256: 6f2ffb4a5fce4bd7871aba553aab5ade1389be8b97994cd558afa0958c176855
- Ordered file plan: docs/reviews/libbun-w1112-20260724/lifecycle-files.txt
- Ordered file count: 38
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | README.md | 7b8a88d47e3ccf5fe518f0dfa44bdc09d26440dc7e461cb33160cf1e2e618d70 | 3369 |
| 2 | docs/README.md | 9b615f84198dae1ffac3a99af67afca6bae39fc28622329fc8db51575a797740 | 12566 |
| 3 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 4 | docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md | 7c8f9d64d9fc4d05f5581097f651af28f92f221a81852d81d0a420770420bc08 | 16493 |
| 5 | docs/LIBBUN-W1112-POISON-INSTALLER-EVIDENCE-20260724.md | 72ece5d9eeef7a220ee4d309d5e28f7c852376c7fecd6288cddc4e643dc61fe8 | 2382 |
| 6 | docs/reviews/libbun-w1112-20260724/correction2-index.md | 42b63e2b37a2f2139a951f5f411305cfae68fd6710f9f07cb2778cf237c814f3 | 1536 |
| 7 | docs/reviews/libbun-w1112-20260724/verdict-snapshot.md | 3138238c945c94f8212855889b0d23600fd97009a006bc57ea862a5aa7d6a6d6 | 16588 |
| 8 | docs/reviews/libbun-w1112-20260724/lifecycle-independent-verdict.commit | 377706f6e59259f1e5f4b21c2d7fc99854e0ef57d1ac206c2a8c632148838310 | 256 |
| 9 | docs/reviews/libbun-w1112-20260724/lifecycle-correction-ruling.md | 6a50421f4605fc0273aa4d04dbe611e90dcc63943ace9976e446a98d9b868de6 | 2143 |
| 10 | docs/reviews/libbun-w1112-20260724/exact-source-search-report.md | 2f2a8d95b5568fbf01c9ea2f4c5d38903b9cdb362d682b629ac8e2abb99dc9bd | 128039 |
| 11 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-source-index.md | 3d1f009d0c2d16346a73ae505b92ce3d8033ecc88758468b4eae027359c2ad4b | 5610 |
| 12 | docs/reviews/libbun-w1112-20260724/lifecycle-vendored-jsc-source-bundle.md | 4d74d89e4417583edcde93f4e78c598ef4bc6acd02f03dd5944e7e2c7c28dcf4 | 87805 |
| 13 | docs/reviews/libbun-w1112-20260724/process-drop-caller-and-fixture-report.md | e3f7f4a06c699faac73511dc27129a66dc22f4d994124632b1449fafc18c056c | 45104 |
| 14 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 15 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 16 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 17 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 18 | runtime/src/main.rs | 3cfa3711281938d2752ce45d4f5b4a3395466ce1df5550e2647a4572e6dfccee | 1117 |
| 19 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 20 | native/Cargo.toml | 5d624b8bcee44bab2a2ae8f87bc0238cc15c16b5975b0f071af7e80581b0b6fd | 639 |
| 21 | runtime/Cargo.toml | c9c8a17548f5350805af9c95835cf7a8c27055a54b26992e0ffffd1fd352a7d2 | 439 |
| 22 | wire/Cargo.toml | a0517ebfe3c61df2f6e2516aa39f8ca56851eb18d44e21f55a47ae2e8813dc8f | 125 |
| 23 | tests/public_api_boundary.rs | e53307e8fdcd0e12ed63056a32c8e1836b24e866acd00bdf4bd5da25afc9b370 | 1784 |
| 24 | vendor/bun/src/jsc/VM.rs | 47aa4f3b8642dfefe21dd56c2fa446889cd701cc3e9fdbf7d962bf4f6aee7119 | 8822 |
| 25 | vendor/bun/src/jsc/virtual_machine_exports.rs | 95946faae0cd89ac0ccf1d037312e307a88742beed7e5ac2770f91910582b23b | 13386 |
| 26 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs | 847c4f7b13917810adaa373299ae0df20c373cd4671a97eac265c6c093cc3399 | 78342 |
| 27 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs | c4dcc653964e40c424bfff20278dd62e6e1d04e7134d09320bc21208c08b6e2d | 108022 |
| 28 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-external-capability-provider-owner/src/lib.rs | e3818b7ce41be8a6a5c5f424418881b4f433883b54a81ddeb96650b2ce9a4439 | 14848 |
| 29 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/src/external_transport.rs | b855d0cc8662591ee1f53473831a47034e7c59ecde415883c33dadb6908f4623 | 4907 |
| 30 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-external-capability-provider-owner/Cargo.toml | 6f9a0d10b0aa4f13049b63013f1469726de88a4adbd2ac48bc862c29b5bb9ee0 | 690 |
| 31 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/Cargo.toml | dd05cd50a38d1ed15cd2f5276c29c6039688eed4f66052b9591946d2a4f74d9b | 24507 |
| 32 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/src/provider_host_set.rs | 14a1994c40bb5c0dfb6e0610b523b5deaf67404e4a2785971cd3c56501a77699 | 45089 |
| 33 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss/src/product.rs | 916e4474f9d3d0f2785f7613200b477bc0a0df8ae21245ba4d644650f93d5525 | 37483 |
| 34 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss/tests/external_capability_provider.rs | 22aa9c60530d9ab1a7d8e1d4ca9b3587d5dd520e2a3f7b442d7046072fc35191 | 14251 |
| 35 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss/Cargo.toml | 1a004d65bf705642c090df2249478d83a1ac20740f572b82a296e31375de9a69 | 1240 |
| 36 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/Cargo.toml | ce3c3333e5e73bd678f11f4b18cd447e82aee0891c1781d89c6f51aaab3083d1 | 829 |
| 37 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss | 3ae07a264289b52bc77c4bf8da0e88c659a3ded32db5d8ea0d3799953ca361c0 | 588 |
| 38 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss | 9a26ebc4737439e4ae0e8342a3b19a357d2a8008d2d09330cc92fcae924dae43 | 771 |

No Fable session, request, response, or output exists. A fresh independent source-aware BUNDLE PASS for correction 2 remains required before launch.
