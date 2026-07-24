# Correction-2 Fable file plan: Private native+wire/containment/output/retained worker/locks/compliance/release

- Prior independent verdict: BUNDLE REVISE at 54241a683e1c68366715456b4517bcf2966bbdf7
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/containment-release-prompt.md
- Prompt SHA-256: 69a92d85652d18eb1ac7a9076b177ebfd5cea0dab79a1912a96726ffcb80d581
- Ordered file plan: docs/reviews/libbun-w1112-20260724/containment-release-files.txt
- Ordered file count: 53
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | README.md | 7b8a88d47e3ccf5fe518f0dfa44bdc09d26440dc7e461cb33160cf1e2e618d70 | 3369 |
| 2 | docs/README.md | 9b615f84198dae1ffac3a99af67afca6bae39fc28622329fc8db51575a797740 | 12566 |
| 3 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 4 | docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md | 7c8f9d64d9fc4d05f5581097f651af28f92f221a81852d81d0a420770420bc08 | 16493 |
| 5 | docs/LIBBUN-WORKER-RELEASE-CONTRACT.md | 030f64ecfbdcc046dec66d4c767333d50ed21471c084c885e2576c770f1d394c | 8765 |
| 6 | docs/LIBBUN-W1112-POISON-INSTALLER-EVIDENCE-20260724.md | 72ece5d9eeef7a220ee4d309d5e28f7c852376c7fecd6288cddc4e643dc61fe8 | 2382 |
| 7 | docs/reviews/libbun-w1112-20260724/correction2-index.md | 42b63e2b37a2f2139a951f5f411305cfae68fd6710f9f07cb2778cf237c814f3 | 1536 |
| 8 | docs/reviews/libbun-w1112-20260724/verdict-snapshot.md | 3138238c945c94f8212855889b0d23600fd97009a006bc57ea862a5aa7d6a6d6 | 16588 |
| 9 | docs/reviews/libbun-w1112-20260724/containment-release-independent-verdict.md | ba792c4113a6e8af5097c636f45a5858b15073409d508f17d2eb384167628837 | 12944 |
| 10 | docs/reviews/libbun-w1112-20260724/exact-source-search-report.md | 2f2a8d95b5568fbf01c9ea2f4c5d38903b9cdb362d682b629ac8e2abb99dc9bd | 128039 |
| 11 | docs/reviews/libbun-w1112-20260724/vendored-bun-boundary-report.md | b87a48b90a5081d974ecec8706e023ef8d36eef5e4ac3c40c5f156db43fd78b9 | 30673 |
| 12 | docs/reviews/libbun-w1112-20260724/lock-privacy-compliance-index.md | 96c9d42afb4444b5da5129a92f1b299762fed254c22629bac9d9b607567a76d8 | 2810 |
| 13 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 14 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 15 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 16 | native/build.rs | 4122c83383a012da82336b223bc10f625f7fc49da33375b76a775ac212a0c72d | 5195 |
| 17 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 18 | runtime/src/main.rs | 3cfa3711281938d2752ce45d4f5b4a3395466ce1df5550e2647a4572e6dfccee | 1117 |
| 19 | runtime/build.rs | c3badf2aaf44535f8d5bedbfa45c36e21c69626e69b4aad44a79eb4f29c91be0 | 4902 |
| 20 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 21 | Cargo.lock | cb9819c613ccd991508d245de06261b03b2284d2800dca1ab26a04191952a392 | 2779 |
| 22 | native/Cargo.toml | 5d624b8bcee44bab2a2ae8f87bc0238cc15c16b5975b0f071af7e80581b0b6fd | 639 |
| 23 | native/Cargo.lock | 6dd537cd346d2ee311708e4a88db8e250530d1c428a5728caab6e3bda2d03496 | 70206 |
| 24 | runtime/Cargo.toml | c9c8a17548f5350805af9c95835cf7a8c27055a54b26992e0ffffd1fd352a7d2 | 439 |
| 25 | runtime/Cargo.lock | 7066c2adfbebf44e5b156ff1b3864b2e74976b5edce1634a11b2a99f56294617 | 70107 |
| 26 | wire/Cargo.toml | a0517ebfe3c61df2f6e2516aa39f8ca56851eb18d44e21f55a47ae2e8813dc8f | 125 |
| 27 | tests/fixtures/public_api_boundary/Cargo.lock | 685de9512211743a6430574bba5832a3175bd49ce184ab8553013a64fe38c6f2 | 1017 |
| 28 | tests/fixtures/public_api_boundary/Cargo.toml | c211189cf1e2c05017baec08952dce72fe6d8c372b7b887049dd52a78bbb8c96 | 409 |
| 29 | tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs | b09059159bc4035e7799e881805ba30216a651b7f2a9c816d28f40f249fd8dcc | 413 |
| 30 | tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs | 5f6bc0ec079770540e3e9f49654180da9ef74a03f6a54975e2e9a467c08df281 | 98 |
| 31 | tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs | b290426f65fcc9da855969004223d5383ae64f37bcaf3e6b0035cc3a2e0992bb | 128 |
| 32 | tests/public_api_boundary.rs | e53307e8fdcd0e12ed63056a32c8e1836b24e866acd00bdf4bd5da25afc9b370 | 1784 |
| 33 | scripts/apply-vendored-bun-patches.sh | c879592a32f12d27f39072c523ba33140bea9bdfefd98ca2e01bba28e33dc784 | 1511 |
| 34 | scripts/check-vendored-bun-rust.sh | a38d93ac03764751a1bdaf1089c0652bc67ffc8c30453c6e5a208eabcffa9839 | 528 |
| 35 | scripts/configure-vendored-bun.sh | 9fc73ed4e5d04d71a176bad14a31c292a213c513f861573f1fd5251477d6115e | 1481 |
| 36 | scripts/package-prepared-export-worker-release.sh | ce1f433f73312f761d0bf4ffd6b30296b6fe88dfb3740f7b5a53450f95986f44 | 2111 |
| 37 | scripts/prepare-native-bun-link.sh | 498bd2954bb44b7d738c56e05a3615a82ca0baf6092765dfa992de24732d37d6 | 3233 |
| 38 | scripts/stage-vendored-bun-source.sh | b5d3740d54f61b9a15c9ca9188abbe8758f6119f9e09f0c91ba0e76e319c0f22 | 1227 |
| 39 | scripts/update-vendored-bun.sh | f07d0fb9553147bae79f51634d07c52659bf0ae8805ed3fcbd89c61b9dcb063d | 1540 |
| 40 | scripts/vendor-bun-deps.sh | 62b2f6a24dfa68946a643c58af6a3923c90fc07e4ab65dadc2701dc6009cc94a | 1321 |
| 41 | scripts/verify-vendored-bun-reproducible.sh | 96cc8eb86f506f288a0c46efd89b284ec0a89ddf120e70e8481593a91db81304 | 1913 |
| 42 | scripts/verify-vendored-bun.sh | d95267b26304c545bb4cffb2234daab6b5735bd818d22ef2a27312bed951c5a7 | 1515 |
| 43 | scripts/generate-libbun-w1112-review-evidence-20260724.py | 0069166626e368d5904b7b7810a21bf0b64f99e996933f07452b36180fadf6af | 24056 |
| 44 | .github/workflows/ci.yml | 45b6b0838b4db84c57d37cc30fde61e6ebbeecdaa51477c1ff3ac896893c04f9 | 782 |
| 45 | patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch | c1b2b67eec0451354d0576ac6661ea6522f4ccefdfcecbb410c2a461a63378ee | 5960 |
| 46 | patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch | 4a64634a2eeea14b72986a17c0b8186da10d031febd31b24c53ca00f7ebc0f06 | 804 |
| 47 | patches/vendored-bun/README.md | 2c3c5fc7aceb4dfe53d3ef1091573b3a230fb729aa2d4b002f1ef1624fd6469f | 1082 |
| 48 | BUN_SOURCE_COMMIT | e458fa88014449eb548d3e80e65dbe188b296989545dfaa235b274dc2267acca | 41 |
| 49 | LICENSE | c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4 | 11357 |
| 50 | vendor/README.md | a8339961bd5659a2cdd534cd905cecca5d000fcfbdec93933f676512d2a27f0e | 825 |
| 51 | vendor/bun.LIBBUN_VENDOR.json | b5f3748b5c985b86de748777948d850045572e348d0f8e9649e74259b73a0d02 | 453 |
| 52 | vendor/bun/LICENSE.md | 2c6160ec8fb853f7e8f97d9b249e756c9b0ac44860a68b6bf4f1b0bcbc5c3741 | 5376 |
| 53 | vendor/bun/Cargo.lock | b9137da3f975f37c6f225d81f96b01dda58119f5212349a9a88c746fc27147fb | 75040 |

No Fable session, request, response, or output exists. A fresh independent source-aware BUNDLE PASS for correction 2 remains required before launch.
