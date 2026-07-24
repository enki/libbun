# Correction-2 Fable file plan: Owner/generative correspondence/admission/reservation/release

- Prior independent verdict: BUNDLE REVISE at bd86b8863ed21c19fa46bfdf1a006d8a83ff0330
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/owner-generative-prompt.md
- Prompt SHA-256: 37d5e6ccc1e6349fbd6fa2bfad30892cf88b1e716151a8d889f15fdbd2e6049c
- Ordered file plan: docs/reviews/libbun-w1112-20260724/owner-generative-files.txt
- Ordered file count: 25
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | README.md | 7b8a88d47e3ccf5fe518f0dfa44bdc09d26440dc7e461cb33160cf1e2e618d70 | 3369 |
| 2 | docs/README.md | 9b615f84198dae1ffac3a99af67afca6bae39fc28622329fc8db51575a797740 | 12566 |
| 3 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 4 | docs/LIBBUN-W1112-POISON-INSTALLER-EVIDENCE-20260724.md | 72ece5d9eeef7a220ee4d309d5e28f7c852376c7fecd6288cddc4e643dc61fe8 | 2382 |
| 5 | docs/reviews/libbun-w1112-20260724/correction2-index.md | 42b63e2b37a2f2139a951f5f411305cfae68fd6710f9f07cb2778cf237c814f3 | 1536 |
| 6 | docs/reviews/libbun-w1112-20260724/verdict-snapshot.md | 3138238c945c94f8212855889b0d23600fd97009a006bc57ea862a5aa7d6a6d6 | 16588 |
| 7 | docs/reviews/libbun-w1112-20260724/exact-source-search-report.md | 2f2a8d95b5568fbf01c9ea2f4c5d38903b9cdb362d682b629ac8e2abb99dc9bd | 128039 |
| 8 | docs/reviews/libbun-w1112-20260724/owner-generative-independent-verdict.md | 5c9e172ad549ce88e16e05bc37d023ec4c4856cab5edc94da9f289a091e0b4ba | 7660 |
| 9 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-source-index.md | 3d1f009d0c2d16346a73ae505b92ce3d8033ecc88758468b4eae027359c2ad4b | 5610 |
| 10 | docs/reviews/libbun-w1112-20260724/process-drop-caller-and-fixture-report.md | e3f7f4a06c699faac73511dc27129a66dc22f4d994124632b1449fafc18c056c | 45104 |
| 11 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 12 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 13 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 14 | tests/fixtures/public_api_boundary/Cargo.lock | 685de9512211743a6430574bba5832a3175bd49ce184ab8553013a64fe38c6f2 | 1017 |
| 15 | tests/fixtures/public_api_boundary/Cargo.toml | c211189cf1e2c05017baec08952dce72fe6d8c372b7b887049dd52a78bbb8c96 | 409 |
| 16 | tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs | b09059159bc4035e7799e881805ba30216a651b7f2a9c816d28f40f249fd8dcc | 413 |
| 17 | tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs | 5f6bc0ec079770540e3e9f49654180da9ef74a03f6a54975e2e9a467c08df281 | 98 |
| 18 | tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs | b290426f65fcc9da855969004223d5383ae64f37bcaf3e6b0035cc3a2e0992bb | 128 |
| 19 | tests/public_api_boundary.rs | e53307e8fdcd0e12ed63056a32c8e1836b24e866acd00bdf4bd5da25afc9b370 | 1784 |
| 20 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs | 847c4f7b13917810adaa373299ae0df20c373cd4671a97eac265c6c093cc3399 | 78342 |
| 21 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs | c4dcc653964e40c424bfff20278dd62e6e1d04e7134d09320bc21208c08b6e2d | 108022 |
| 22 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-external-capability-provider-owner/src/lib.rs | e3818b7ce41be8a6a5c5f424418881b4f433883b54a81ddeb96650b2ce9a4439 | 14848 |
| 23 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/src/external_transport.rs | b855d0cc8662591ee1f53473831a47034e7c59ecde415883c33dadb6908f4623 | 4907 |
| 24 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-external-capability-provider-owner/Cargo.toml | 6f9a0d10b0aa4f13049b63013f1469726de88a4adbd2ac48bc862c29b5bb9ee0 | 690 |
| 25 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/Cargo.toml | dd05cd50a38d1ed15cd2f5276c29c6039688eed4f66052b9591946d2a4f74d9b | 24507 |

No Fable session, request, response, or output exists. A fresh independent source-aware BUNDLE PASS for correction 2 remains required before launch.
