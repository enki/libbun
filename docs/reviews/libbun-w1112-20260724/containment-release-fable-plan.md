# Fable file plan: Private native+wire/containment/output/retained worker/release

- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/containment-release-prompt.md
- Prompt SHA-256: cb898e74365e4932208772f60450f59b14b68680e4fd919f50ec40a721788f53
- Ordered file plan: docs/reviews/libbun-w1112-20260724/containment-release-files.txt
- Ordered file count: 35
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
| 7 | docs/reviews/libbun-w1112-20260724/verdict-snapshot.md | 3138238c945c94f8212855889b0d23600fd97009a006bc57ea862a5aa7d6a6d6 | 16588 |
| 8 | docs/reviews/libbun-w1112-20260724/exact-source-search-report.md | 8460cf4417ad0ef6742cbf2e82bd573dff2a9a8d2b0afff939e5e8657769ef93 | 36610 |
| 9 | docs/reviews/libbun-w1112-20260724/vendored-bun-boundary-report.md | 4b3b06f539b945a52e44012e1d065cc7202b98f0bc861f4f0119763edef863f7 | 51273 |
| 10 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 11 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 12 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 13 | native/build.rs | 4122c83383a012da82336b223bc10f625f7fc49da33375b76a775ac212a0c72d | 5195 |
| 14 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 15 | runtime/src/main.rs | 3cfa3711281938d2752ce45d4f5b4a3395466ce1df5550e2647a4572e6dfccee | 1117 |
| 16 | runtime/build.rs | c3badf2aaf44535f8d5bedbfa45c36e21c69626e69b4aad44a79eb4f29c91be0 | 4902 |
| 17 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 18 | native/Cargo.toml | 5d624b8bcee44bab2a2ae8f87bc0238cc15c16b5975b0f071af7e80581b0b6fd | 639 |
| 19 | runtime/Cargo.toml | c9c8a17548f5350805af9c95835cf7a8c27055a54b26992e0ffffd1fd352a7d2 | 439 |
| 20 | wire/Cargo.toml | a0517ebfe3c61df2f6e2516aa39f8ca56851eb18d44e21f55a47ae2e8813dc8f | 125 |
| 21 | scripts/apply-vendored-bun-patches.sh | c879592a32f12d27f39072c523ba33140bea9bdfefd98ca2e01bba28e33dc784 | 1511 |
| 22 | scripts/check-vendored-bun-rust.sh | a38d93ac03764751a1bdaf1089c0652bc67ffc8c30453c6e5a208eabcffa9839 | 528 |
| 23 | scripts/configure-vendored-bun.sh | 9fc73ed4e5d04d71a176bad14a31c292a213c513f861573f1fd5251477d6115e | 1481 |
| 24 | scripts/package-prepared-export-worker-release.sh | ce1f433f73312f761d0bf4ffd6b30296b6fe88dfb3740f7b5a53450f95986f44 | 2111 |
| 25 | scripts/prepare-native-bun-link.sh | 498bd2954bb44b7d738c56e05a3615a82ca0baf6092765dfa992de24732d37d6 | 3233 |
| 26 | scripts/stage-vendored-bun-source.sh | b5d3740d54f61b9a15c9ca9188abbe8758f6119f9e09f0c91ba0e76e319c0f22 | 1227 |
| 27 | scripts/update-vendored-bun.sh | f07d0fb9553147bae79f51634d07c52659bf0ae8805ed3fcbd89c61b9dcb063d | 1540 |
| 28 | scripts/vendor-bun-deps.sh | 62b2f6a24dfa68946a643c58af6a3923c90fc07e4ab65dadc2701dc6009cc94a | 1321 |
| 29 | scripts/verify-vendored-bun-reproducible.sh | 96cc8eb86f506f288a0c46efd89b284ec0a89ddf120e70e8481593a91db81304 | 1913 |
| 30 | scripts/verify-vendored-bun.sh | d95267b26304c545bb4cffb2234daab6b5735bd818d22ef2a27312bed951c5a7 | 1515 |
| 31 | .github/workflows/ci.yml | 45b6b0838b4db84c57d37cc30fde61e6ebbeecdaa51477c1ff3ac896893c04f9 | 782 |
| 32 | patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch | c1b2b67eec0451354d0576ac6661ea6522f4ccefdfcecbb410c2a461a63378ee | 5960 |
| 33 | patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch | 4a64634a2eeea14b72986a17c0b8186da10d031febd31b24c53ca00f7ebc0f06 | 804 |
| 34 | patches/vendored-bun/README.md | 2c3c5fc7aceb4dfe53d3ef1091573b3a230fb729aa2d4b002f1ef1624fd6469f | 1082 |
| 35 | BUN_SOURCE_COMMIT | e458fa88014449eb548d3e80e65dbe188b296989545dfaa235b274dc2267acca | 41 |

No Fable session, request, response, or output exists. Independent source-aware BUNDLE PASS remains required before any launch.
