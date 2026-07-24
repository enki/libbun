# Correction-3 Fable file plan: Containment/persistent output/locks/packaging/release

- Prior independent verdict: PART BUNDLE REVISE at 84feaf68aa99c5bc0e393cbfc1b6a92716cefdf1
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/containment-release-prompt.md
- Prompt SHA-256: 5bd10e1239095553de044151d47a59b8fc15e78429e62b7eaf8764510b75af7c
- Ordered file plan: docs/reviews/libbun-w1112-20260724/containment-release-files.txt
- Ordered file count: 46
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | docs/reviews/libbun-w1112-20260724/containment-release-correction2-independent-verdict.md | 9f23368f53133134da8472be30c6608d4d4fffdc942e7797a452c3d13ff6a6ea | 10001 |
| 2 | docs/reviews/libbun-w1112-20260724/lifecycle-vendored-jsc-source-bundle.md | 67c94590d9b2203205a74dd3c49d2bb1a79f83ba0123d6d012d70f2244dd2849 | 15244 |
| 3 | docs/reviews/libbun-w1112-20260724/process-drop-caller-and-fixture-report.md | a27f74cc0ae6a76719000312b10c7180c28403ab24890afa1b875c12bf3d8d34 | 79220 |
| 4 | docs/reviews/libbun-w1112-20260724/lock-privacy-compliance-index.md | 3ea128354cf3b85710c4aa89abd8549a57e340155aaac27a451d3bd65fde3dd1 | 35205 |
| 5 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 6 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 7 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 8 | runtime/build.rs | c3badf2aaf44535f8d5bedbfa45c36e21c69626e69b4aad44a79eb4f29c91be0 | 4902 |
| 9 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 10 | Cargo.lock | cb9819c613ccd991508d245de06261b03b2284d2800dca1ab26a04191952a392 | 2779 |
| 11 | native/Cargo.toml | 5d624b8bcee44bab2a2ae8f87bc0238cc15c16b5975b0f071af7e80581b0b6fd | 639 |
| 12 | native/Cargo.lock | 6dd537cd346d2ee311708e4a88db8e250530d1c428a5728caab6e3bda2d03496 | 70206 |
| 13 | runtime/Cargo.toml | c9c8a17548f5350805af9c95835cf7a8c27055a54b26992e0ffffd1fd352a7d2 | 439 |
| 14 | runtime/Cargo.lock | 7066c2adfbebf44e5b156ff1b3864b2e74976b5edce1634a11b2a99f56294617 | 70107 |
| 15 | wire/Cargo.toml | a0517ebfe3c61df2f6e2516aa39f8ca56851eb18d44e21f55a47ae2e8813dc8f | 125 |
| 16 | tests/fixtures/public_api_boundary/Cargo.lock | 685de9512211743a6430574bba5832a3175bd49ce184ab8553013a64fe38c6f2 | 1017 |
| 17 | tests/fixtures/public_api_boundary/Cargo.toml | c211189cf1e2c05017baec08952dce72fe6d8c372b7b887049dd52a78bbb8c96 | 409 |
| 18 | tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs | b09059159bc4035e7799e881805ba30216a651b7f2a9c816d28f40f249fd8dcc | 413 |
| 19 | tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs | 5f6bc0ec079770540e3e9f49654180da9ef74a03f6a54975e2e9a467c08df281 | 98 |
| 20 | tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs | b290426f65fcc9da855969004223d5383ae64f37bcaf3e6b0035cc3a2e0992bb | 128 |
| 21 | tests/public_api_boundary.rs | e53307e8fdcd0e12ed63056a32c8e1836b24e866acd00bdf4bd5da25afc9b370 | 1784 |
| 22 | .github/workflows/ci.yml | 45b6b0838b4db84c57d37cc30fde61e6ebbeecdaa51477c1ff3ac896893c04f9 | 782 |
| 23 | patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch | c1b2b67eec0451354d0576ac6661ea6522f4ccefdfcecbb410c2a461a63378ee | 5960 |
| 24 | patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch | 4a64634a2eeea14b72986a17c0b8186da10d031febd31b24c53ca00f7ebc0f06 | 804 |
| 25 | BUN_SOURCE_COMMIT | e458fa88014449eb548d3e80e65dbe188b296989545dfaa235b274dc2267acca | 41 |
| 26 | LICENSE | c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4 | 11357 |
| 27 | vendor/README.md | a8339961bd5659a2cdd534cd905cecca5d000fcfbdec93933f676512d2a27f0e | 825 |
| 28 | vendor/bun.LIBBUN_VENDOR.json | b5f3748b5c985b86de748777948d850045572e348d0f8e9649e74259b73a0d02 | 453 |
| 29 | vendor/bun/LICENSE.md | 2c6160ec8fb853f7e8f97d9b249e756c9b0ac44860a68b6bf4f1b0bcbc5c3741 | 5376 |
| 30 | vendor/bun/Cargo.lock | b9137da3f975f37c6f225d81f96b01dda58119f5212349a9a88c746fc27147fb | 75040 |
| 31 | vendor/bun/Cargo.toml | 2101849a8242a31c43d5952b663771455f21266489839fbd774d6a98283daadb | 12011 |
| 32 | vendor/bun/src/clap/LICENSE | 88d9b4eb60579c191ec391ca04c16130572d7eedc4a86daa58bf28c6e14c9bcd | 1210 |
| 33 | vendor/bun/src/unicode/uucode_lib/LICENSE.md | 75b52b07e8f6ed6b1700ca6e4bcff93a59258624d4fd1ab7eae4c071c860b69b | 1298 |
| 34 | vendor/bun/vendor/lolhtml/LICENSE | e4ddaa9d7391bb9536fcb8c59b570a8b85a0bf6da54df5b3b26f098f6f99c9cc | 1487 |
| 35 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs | 847c4f7b13917810adaa373299ae0df20c373cd4671a97eac265c6c093cc3399 | 78342 |
| 36 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarmvm-image/Cargo.toml | c300820c208d5610a2d5a9c16c3de57ebd2fcc855d622f6443d1430bddec4db7 | 1342 |
| 37 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-rust-sdk-static-provider-host/src/lib_parts/request_and_output.rs | c4dcc653964e40c424bfff20278dd62e6e1d04e7134d09320bc21208c08b6e2d | 108022 |
| 38 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-rust-sdk-static-provider-host/Cargo.toml | cbe99d18d1eb62551907fa249b6632c0ca0a691d246559b0459039c25d9339aa | 1074 |
| 39 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-value-model/Cargo.toml | 8f6a90758239f6dc323ff83fcf71cc6915cbd4291998e1cc8718d171a364ca64 | 709 |
| 40 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-external-capability-provider-owner/src/lib.rs | e3818b7ce41be8a6a5c5f424418881b4f433883b54a81ddeb96650b2ce9a4439 | 14848 |
| 41 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-external-capability-provider-owner/Cargo.toml | 6f9a0d10b0aa4f13049b63013f1469726de88a4adbd2ac48bc862c29b5bb9ee0 | 690 |
| 42 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/src/external_transport.rs | b855d0cc8662591ee1f53473831a47034e7c59ecde415883c33dadb6908f4623 | 4907 |
| 43 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/Cargo.toml | ce3c3333e5e73bd678f11f4b18cd447e82aee0891c1781d89c6f51aaab3083d1 | 829 |
| 44 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss | 3ae07a264289b52bc77c4bf8da0e88c659a3ded32db5d8ea0d3799953ca361c0 | 588 |
| 45 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss | 9a26ebc4737439e4ae0e8342a3b19a357d2a8008d2d09330cc92fcae924dae43 | 771 |
| 46 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/negative/ss/provider/external_provider_json_nfc_duplicate_keys.test.ss | 5d238da6cfd9e1f46a46fc243409d62c3940964cb084343adae08b9079d6c99d | 493 |

No Fable session, request, response, or output exists. Fresh literal independent PART BUNDLE PASS verdicts for correction 3 remain required before launch.
