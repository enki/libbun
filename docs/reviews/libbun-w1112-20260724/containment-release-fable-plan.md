# Correction-4 Fable file plan: Containment/persistent output/locks/packaging/release

- Prior independent verdict: PART BUNDLE REVISE at 16ae0060d9c8648048b89c8451cc51cfe1ec72db
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/containment-release-prompt.md
- Prompt SHA-256: 9ca91668ca5f266e4432fe6e0b7ace604cd5096b1c28e7d94c8177e6fee0e7be
- Ordered file plan: docs/reviews/libbun-w1112-20260724/containment-release-files.txt
- Ordered file count: 55
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | docs/reviews/libbun-w1112-20260724/correction4-index.md | f2c4b1452288fe220cfe1a0ae167a2c543cc8d2c6ff87843991e99fc397bfbbc | 2074 |
| 2 | docs/reviews/libbun-w1112-20260724/containment-release-correction3-independent-verdict.md | a7076b20968629d5df4b08fab77f38d792c03585d0f26c49c099a62c3ac4367a | 11841 |
| 3 | docs/reviews/libbun-w1112-20260724/lock-privacy-compliance-index.md | 3ea128354cf3b85710c4aa89abd8549a57e340155aaac27a451d3bd65fde3dd1 | 35205 |
| 4 | docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md | 7c8f9d64d9fc4d05f5581097f651af28f92f221a81852d81d0a420770420bc08 | 16493 |
| 5 | docs/LIBBUN-WORKER-RELEASE-CONTRACT.md | 030f64ecfbdcc046dec66d4c767333d50ed21471c084c885e2576c770f1d394c | 8765 |
| 6 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 7 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 8 | native/build.rs | 4122c83383a012da82336b223bc10f625f7fc49da33375b76a775ac212a0c72d | 5195 |
| 9 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 10 | runtime/src/main.rs | 3cfa3711281938d2752ce45d4f5b4a3395466ce1df5550e2647a4572e6dfccee | 1117 |
| 11 | runtime/build.rs | c3badf2aaf44535f8d5bedbfa45c36e21c69626e69b4aad44a79eb4f29c91be0 | 4902 |
| 12 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 13 | Cargo.lock | cb9819c613ccd991508d245de06261b03b2284d2800dca1ab26a04191952a392 | 2779 |
| 14 | native/Cargo.toml | 5d624b8bcee44bab2a2ae8f87bc0238cc15c16b5975b0f071af7e80581b0b6fd | 639 |
| 15 | native/Cargo.lock | 6dd537cd346d2ee311708e4a88db8e250530d1c428a5728caab6e3bda2d03496 | 70206 |
| 16 | runtime/Cargo.toml | c9c8a17548f5350805af9c95835cf7a8c27055a54b26992e0ffffd1fd352a7d2 | 439 |
| 17 | runtime/Cargo.lock | 7066c2adfbebf44e5b156ff1b3864b2e74976b5edce1634a11b2a99f56294617 | 70107 |
| 18 | wire/Cargo.toml | a0517ebfe3c61df2f6e2516aa39f8ca56851eb18d44e21f55a47ae2e8813dc8f | 125 |
| 19 | tests/fixtures/public_api_boundary/Cargo.lock | 685de9512211743a6430574bba5832a3175bd49ce184ab8553013a64fe38c6f2 | 1017 |
| 20 | tests/fixtures/public_api_boundary/Cargo.toml | c211189cf1e2c05017baec08952dce72fe6d8c372b7b887049dd52a78bbb8c96 | 409 |
| 21 | tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs | b09059159bc4035e7799e881805ba30216a651b7f2a9c816d28f40f249fd8dcc | 413 |
| 22 | tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs | 5f6bc0ec079770540e3e9f49654180da9ef74a03f6a54975e2e9a467c08df281 | 98 |
| 23 | tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs | b290426f65fcc9da855969004223d5383ae64f37bcaf3e6b0035cc3a2e0992bb | 128 |
| 24 | tests/public_api_boundary.rs | e53307e8fdcd0e12ed63056a32c8e1836b24e866acd00bdf4bd5da25afc9b370 | 1784 |
| 25 | .github/workflows/ci.yml | 45b6b0838b4db84c57d37cc30fde61e6ebbeecdaa51477c1ff3ac896893c04f9 | 782 |
| 26 | scripts/package-prepared-export-worker-release.sh | ce1f433f73312f761d0bf4ffd6b30296b6fe88dfb3740f7b5a53450f95986f44 | 2111 |
| 27 | scripts/prepare-native-bun-link.sh | 498bd2954bb44b7d738c56e05a3615a82ca0baf6092765dfa992de24732d37d6 | 3233 |
| 28 | scripts/verify-vendored-bun-reproducible.sh | 96cc8eb86f506f288a0c46efd89b284ec0a89ddf120e70e8481593a91db81304 | 1913 |
| 29 | scripts/verify-vendored-bun.sh | d95267b26304c545bb4cffb2234daab6b5735bd818d22ef2a27312bed951c5a7 | 1515 |
| 30 | patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch | c1b2b67eec0451354d0576ac6661ea6522f4ccefdfcecbb410c2a461a63378ee | 5960 |
| 31 | patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch | 4a64634a2eeea14b72986a17c0b8186da10d031febd31b24c53ca00f7ebc0f06 | 804 |
| 32 | BUN_SOURCE_COMMIT | e458fa88014449eb548d3e80e65dbe188b296989545dfaa235b274dc2267acca | 41 |
| 33 | LICENSE | c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4 | 11357 |
| 34 | vendor/README.md | a8339961bd5659a2cdd534cd905cecca5d000fcfbdec93933f676512d2a27f0e | 825 |
| 35 | vendor/bun.LIBBUN_VENDOR.json | b5f3748b5c985b86de748777948d850045572e348d0f8e9649e74259b73a0d02 | 453 |
| 36 | vendor/bun/LICENSE.md | 2c6160ec8fb853f7e8f97d9b249e756c9b0ac44860a68b6bf4f1b0bcbc5c3741 | 5376 |
| 37 | vendor/bun/Cargo.lock | b9137da3f975f37c6f225d81f96b01dda58119f5212349a9a88c746fc27147fb | 75040 |
| 38 | vendor/bun/Cargo.toml | 2101849a8242a31c43d5952b663771455f21266489839fbd774d6a98283daadb | 12011 |
| 39 | vendor/bun/src/clap/LICENSE | 88d9b4eb60579c191ec391ca04c16130572d7eedc4a86daa58bf28c6e14c9bcd | 1210 |
| 40 | vendor/bun/src/unicode/uucode_lib/LICENSE.md | 75b52b07e8f6ed6b1700ca6e4bcff93a59258624d4fd1ab7eae4c071c860b69b | 1298 |
| 41 | vendor/bun/vendor/lolhtml/LICENSE | e4ddaa9d7391bb9536fcb8c59b570a8b85a0bf6da54df5b3b26f098f6f99c9cc | 1487 |
| 42 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss/tests/external_capability_provider.rs | 22aa9c60530d9ab1a7d8e1d4ca9b3587d5dd520e2a3f7b442d7046072fc35191 | 14251 |
| 43 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/Cargo.toml | fe357e88528b8f95f48a5b5afc7ddf73bf62294fb0de686b60099bdc7614ee3e | 2847 |
| 44 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/lib.rs | 3575c876d907efdd0f338111e0ff9f567f69b5689a7a671c91b320ceaed9566e | 5914 |
| 45 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/body_authority_registry.rs | 96fe11bf1b1dd64e28cdbf6bc419cccd6c975f063e73a025da30057130573521 | 4718 |
| 46 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs | 3eed047f11c5ac8bb8aee51dc4c5f7e96520af3b8ed41874d6736f643018c9de | 2269 |
| 47 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs | 48b30f77c809f06b7dfb130b11e190cff8e742809140840d14a3a4b5fb9a4323 | 3030 |
| 48 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs | 5519fbbf653aa19cc883dc881eaaf4ff2587dc13bb2b7ab4d3ac2d9575a3b8d5 | 85748 |
| 49 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs | 3be90814fe8071aed3c48778c307a63266d3fc10aa689581e73f7a4e6bf83012 | 6025 |
| 50 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs | 54c9aaee5abca13fb791eeffff19e8abd3b02134db567bec5135d439bc691760 | 29621 |
| 51 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-provider-host-set-owner/Cargo.toml | 00c9fd4051072e2b66e1342ad118b9e44f7637bcea68325c4c140f2ef692e93b | 400 |
| 52 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-provider-host-set-owner/src/lib.rs | bc6cf5673711c59e8352599643e7c36b54f2e2f2e2b9d005902566a5a3cc265b | 5724 |
| 53 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss | 3ae07a264289b52bc77c4bf8da0e88c659a3ded32db5d8ea0d3799953ca361c0 | 588 |
| 54 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss | 9a26ebc4737439e4ae0e8342a3b19a357d2a8008d2d09330cc92fcae924dae43 | 771 |
| 55 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/negative/ss/provider/external_provider_json_nfc_duplicate_keys.test.ss | 5d238da6cfd9e1f46a46fc243409d62c3940964cb084343adae08b9079d6c99d | 493 |

No Fable session, request, response, or output exists. Fresh literal independent PART BUNDLE PASS verdicts for correction 4 remain required before launch.
