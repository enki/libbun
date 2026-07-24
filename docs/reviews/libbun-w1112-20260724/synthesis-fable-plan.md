# Fable file plan: full-SCC synthesis

- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/synthesis-prompt.md
- Prompt SHA-256: fd7598d9ee53c988420f93bc5fcd94897bfccad3b8d3bbffba4e5a0194b2a18b
- Ordered file plan: docs/reviews/libbun-w1112-20260724/synthesis-files.txt
- Ordered file count: 55
- Joins part manifests: owner-generative, lifecycle, containment-release
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | docs/reviews/libbun-w1112-20260724/owner-generative-manifest.json | 97c7e1186047df9af55a2f85888186ee3ccd5ebe41739288cf6a74440e162938 | 5059 |
| 2 | docs/reviews/libbun-w1112-20260724/owner-generative-prompt.md | fd7a13d5c0f3c6fa8a91fcc4523cc9a8437fb0b6ff3177fa6355e8534a1184af | 2690 |
| 3 | docs/reviews/libbun-w1112-20260724/owner-generative-oracle-dry-run.txt | d281002512fb31cf352640a6c37927fce09c269ede9e5966fe602cf367a66942 | 1149 |
| 4 | docs/reviews/libbun-w1112-20260724/owner-generative-fable-plan.md | a3a0f08db8ff76ac91cb930484a3e4d4c2ef1424c38c33d0b01e43087130b2f9 | 2443 |
| 5 | docs/reviews/libbun-w1112-20260724/owner-generative-files.txt | 38e1de382a603a886ca59701e2ee9b77925308f3a5455775a0266e6358f524da | 558 |
| 6 | docs/reviews/libbun-w1112-20260724/lifecycle-manifest.json | 9b64bc90a14677cc9039130a03b48cd80db38323317c61058bdf911fc1a889b5 | 5319 |
| 7 | docs/reviews/libbun-w1112-20260724/lifecycle-prompt.md | 091a540cea95beffd6cb65df62ae90ad4bd073e498663dbd6130441993c410a1 | 2770 |
| 8 | docs/reviews/libbun-w1112-20260724/lifecycle-oracle-dry-run.txt | c51bf5fa25feceb37c43a2020e5af7d3dca8edac57b1716f4c2aec5706464262 | 1118 |
| 9 | docs/reviews/libbun-w1112-20260724/lifecycle-fable-plan.md | 0d7bcbfc00bee66863cf7914996bef7ea87f7e432b4a4b3778034be58d5a9c44 | 2585 |
| 10 | docs/reviews/libbun-w1112-20260724/lifecycle-files.txt | f0b4fbd017c987e04b5d29ec8d0ba9eaf7c42995b369c6a483db38ed971b0208 | 459 |
| 11 | docs/reviews/libbun-w1112-20260724/containment-release-manifest.json | a6d19bc45336c727a1c6662e425760178291c84481bc91df0b8d05483c03d831 | 8490 |
| 12 | docs/reviews/libbun-w1112-20260724/containment-release-prompt.md | cb898e74365e4932208772f60450f59b14b68680e4fd919f50ec40a721788f53 | 3043 |
| 13 | docs/reviews/libbun-w1112-20260724/containment-release-oracle-dry-run.txt | 541d07aff856c19ac3ad099c0fa004edb174483f7e000383abb7010a536d8322 | 2191 |
| 14 | docs/reviews/libbun-w1112-20260724/containment-release-fable-plan.md | 784922d9f2303e3285ab4806a686f72e897e281a746033bfb006b7817ee6c723 | 4788 |
| 15 | docs/reviews/libbun-w1112-20260724/containment-release-files.txt | cf723736eb055d33a6543893f9be3c7025bb0b6fbaaa3369a8ccbf6ad8f73ff6 | 1153 |
| 16 | README.md | 7b8a88d47e3ccf5fe518f0dfa44bdc09d26440dc7e461cb33160cf1e2e618d70 | 3369 |
| 17 | docs/README.md | 9b615f84198dae1ffac3a99af67afca6bae39fc28622329fc8db51575a797740 | 12566 |
| 18 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 19 | docs/LIBBUN-W1112-POISON-INSTALLER-EVIDENCE-20260724.md | 72ece5d9eeef7a220ee4d309d5e28f7c852376c7fecd6288cddc4e643dc61fe8 | 2382 |
| 20 | docs/reviews/libbun-w1112-20260724/verdict-snapshot.md | 3138238c945c94f8212855889b0d23600fd97009a006bc57ea862a5aa7d6a6d6 | 16588 |
| 21 | docs/reviews/libbun-w1112-20260724/exact-source-search-report.md | 8460cf4417ad0ef6742cbf2e82bd573dff2a9a8d2b0afff939e5e8657769ef93 | 36610 |
| 22 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 23 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 24 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 25 | tests/public_api_boundary.rs | e53307e8fdcd0e12ed63056a32c8e1836b24e866acd00bdf4bd5da25afc9b370 | 1784 |
| 26 | tests/fixtures/public_api_boundary/Cargo.toml | c211189cf1e2c05017baec08952dce72fe6d8c372b7b887049dd52a78bbb8c96 | 409 |
| 27 | tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs | b09059159bc4035e7799e881805ba30216a651b7f2a9c816d28f40f249fd8dcc | 413 |
| 28 | tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs | b290426f65fcc9da855969004223d5383ae64f37bcaf3e6b0035cc3a2e0992bb | 128 |
| 29 | tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs | 5f6bc0ec079770540e3e9f49654180da9ef74a03f6a54975e2e9a467c08df281 | 98 |
| 30 | docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md | 7c8f9d64d9fc4d05f5581097f651af28f92f221a81852d81d0a420770420bc08 | 16493 |
| 31 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 32 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 33 | runtime/src/main.rs | 3cfa3711281938d2752ce45d4f5b4a3395466ce1df5550e2647a4572e6dfccee | 1117 |
| 34 | native/Cargo.toml | 5d624b8bcee44bab2a2ae8f87bc0238cc15c16b5975b0f071af7e80581b0b6fd | 639 |
| 35 | runtime/Cargo.toml | c9c8a17548f5350805af9c95835cf7a8c27055a54b26992e0ffffd1fd352a7d2 | 439 |
| 36 | wire/Cargo.toml | a0517ebfe3c61df2f6e2516aa39f8ca56851eb18d44e21f55a47ae2e8813dc8f | 125 |
| 37 | docs/LIBBUN-WORKER-RELEASE-CONTRACT.md | 030f64ecfbdcc046dec66d4c767333d50ed21471c084c885e2576c770f1d394c | 8765 |
| 38 | docs/reviews/libbun-w1112-20260724/vendored-bun-boundary-report.md | 4b3b06f539b945a52e44012e1d065cc7202b98f0bc861f4f0119763edef863f7 | 51273 |
| 39 | native/build.rs | 4122c83383a012da82336b223bc10f625f7fc49da33375b76a775ac212a0c72d | 5195 |
| 40 | runtime/build.rs | c3badf2aaf44535f8d5bedbfa45c36e21c69626e69b4aad44a79eb4f29c91be0 | 4902 |
| 41 | scripts/apply-vendored-bun-patches.sh | c879592a32f12d27f39072c523ba33140bea9bdfefd98ca2e01bba28e33dc784 | 1511 |
| 42 | scripts/check-vendored-bun-rust.sh | a38d93ac03764751a1bdaf1089c0652bc67ffc8c30453c6e5a208eabcffa9839 | 528 |
| 43 | scripts/configure-vendored-bun.sh | 9fc73ed4e5d04d71a176bad14a31c292a213c513f861573f1fd5251477d6115e | 1481 |
| 44 | scripts/package-prepared-export-worker-release.sh | ce1f433f73312f761d0bf4ffd6b30296b6fe88dfb3740f7b5a53450f95986f44 | 2111 |
| 45 | scripts/prepare-native-bun-link.sh | 498bd2954bb44b7d738c56e05a3615a82ca0baf6092765dfa992de24732d37d6 | 3233 |
| 46 | scripts/stage-vendored-bun-source.sh | b5d3740d54f61b9a15c9ca9188abbe8758f6119f9e09f0c91ba0e76e319c0f22 | 1227 |
| 47 | scripts/update-vendored-bun.sh | f07d0fb9553147bae79f51634d07c52659bf0ae8805ed3fcbd89c61b9dcb063d | 1540 |
| 48 | scripts/vendor-bun-deps.sh | 62b2f6a24dfa68946a643c58af6a3923c90fc07e4ab65dadc2701dc6009cc94a | 1321 |
| 49 | scripts/verify-vendored-bun-reproducible.sh | 96cc8eb86f506f288a0c46efd89b284ec0a89ddf120e70e8481593a91db81304 | 1913 |
| 50 | scripts/verify-vendored-bun.sh | d95267b26304c545bb4cffb2234daab6b5735bd818d22ef2a27312bed951c5a7 | 1515 |
| 51 | .github/workflows/ci.yml | 45b6b0838b4db84c57d37cc30fde61e6ebbeecdaa51477c1ff3ac896893c04f9 | 782 |
| 52 | patches/vendored-bun/0001-add-libbun-native-plugin-pic-build-mode.patch | c1b2b67eec0451354d0576ac6661ea6522f4ccefdfcecbb410c2a461a63378ee | 5960 |
| 53 | patches/vendored-bun/0002-export-call-frame-describe-frame-in-release.patch | 4a64634a2eeea14b72986a17c0b8186da10d031febd31b24c53ca00f7ebc0f06 | 804 |
| 54 | patches/vendored-bun/README.md | 2c3c5fc7aceb4dfe53d3ef1091573b3a230fb729aa2d4b002f1ef1624fd6469f | 1082 |
| 55 | BUN_SOURCE_COMMIT | e458fa88014449eb548d3e80e65dbe188b296989545dfaa235b274dc2267acca | 41 |

No part review, synthesis session, request, response, or output exists. Launch remains gated on three independent part outputs and literal source-aware BUNDLE PASS at the exact prelaunch commit.
