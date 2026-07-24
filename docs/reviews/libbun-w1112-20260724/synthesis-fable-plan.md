# Correction-2 Fable file plan: full-SCC synthesis

- Prior part verdicts: three BUNDLE REVISE verdicts incorporated
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/synthesis-prompt.md
- Prompt SHA-256: 5a6daa932ed0fac060d33b26d23804a54236998f58176fa316ba1ce7a99f2e2e
- Ordered file plan: docs/reviews/libbun-w1112-20260724/synthesis-files.txt
- Ordered file count: 48
- Joins part manifests: owner-generative, lifecycle, containment-release
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | docs/reviews/libbun-w1112-20260724/owner-generative-manifest.json | c2385ae1ad9d255b98d98021e69f544570ba9b59e7c27973d2527d9cc5a31145 | 8784 |
| 2 | docs/reviews/libbun-w1112-20260724/owner-generative-prompt.md | 37d5e6ccc1e6349fbd6fa2bfad30892cf88b1e716151a8d889f15fdbd2e6049c | 3964 |
| 3 | docs/reviews/libbun-w1112-20260724/owner-generative-oracle-dry-run.txt | 21761b20c925bd9421f84ac1f058475a566b66f3830f8401e73201bbc56103e5 | 2588 |
| 4 | docs/reviews/libbun-w1112-20260724/owner-generative-fable-plan.md | 3bf47cc87d5503df1e05e06c96abca5d43b0eb7c786b1df295f6e5b5fb17275e | 4684 |
| 5 | docs/reviews/libbun-w1112-20260724/owner-generative-files.txt | 9bb722c3b43cf06bcf333ea939d9c16097199c29d38c6166dbdcf1859f8a2487 | 1759 |
| 6 | docs/reviews/libbun-w1112-20260724/lifecycle-manifest.json | dc66093ef4968aab585d56bdec50d460cb48dcb712b927ac7da664a6e5c94adc | 11713 |
| 7 | docs/reviews/libbun-w1112-20260724/lifecycle-prompt.md | 6f2ffb4a5fce4bd7871aba553aab5ade1389be8b97994cd558afa0958c176855 | 3699 |
| 8 | docs/reviews/libbun-w1112-20260724/lifecycle-oracle-dry-run.txt | d5ed88eb68d70d0058f44f93e70352e239e5f4e519345a4711c8672ecae37386 | 3927 |
| 9 | docs/reviews/libbun-w1112-20260724/lifecycle-fable-plan.md | cd87ae3569e8b55f2507287a1a01e5feaf10e72c41803d2915db81266682cf5d | 6801 |
| 10 | docs/reviews/libbun-w1112-20260724/lifecycle-files.txt | 35a1d3e4a35abb0f572df02880189808cfa7fa8b8dbef25ff5cc644567d224ea | 2800 |
| 11 | docs/reviews/libbun-w1112-20260724/containment-release-manifest.json | 789e3f8c86b8cb37964528d160e07d350e8205b919cd041bda3b35841bc37569 | 12697 |
| 12 | docs/reviews/libbun-w1112-20260724/containment-release-prompt.md | 69a92d85652d18eb1ac7a9076b177ebfd5cea0dab79a1912a96726ffcb80d581 | 3677 |
| 13 | docs/reviews/libbun-w1112-20260724/containment-release-oracle-dry-run.txt | 7618d704256daa22f65ecc498d1bfa8e02f7637b82c78373bec358bb4bf7c938 | 3328 |
| 14 | docs/reviews/libbun-w1112-20260724/containment-release-fable-plan.md | 4e2b0df489acce24c98dbccb96d24e5f9f8755333b9d0669c8bf63affd9c8c91 | 7151 |
| 15 | docs/reviews/libbun-w1112-20260724/containment-release-files.txt | 4b785d64671eceedc25983c9688b977aa1060787208d311747f73f48d2ff0417 | 1884 |
| 16 | README.md | 7b8a88d47e3ccf5fe518f0dfa44bdc09d26440dc7e461cb33160cf1e2e618d70 | 3369 |
| 17 | docs/README.md | 9b615f84198dae1ffac3a99af67afca6bae39fc28622329fc8db51575a797740 | 12566 |
| 18 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 19 | docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md | 7c8f9d64d9fc4d05f5581097f651af28f92f221a81852d81d0a420770420bc08 | 16493 |
| 20 | docs/LIBBUN-WORKER-RELEASE-CONTRACT.md | 030f64ecfbdcc046dec66d4c767333d50ed21471c084c885e2576c770f1d394c | 8765 |
| 21 | docs/LIBBUN-W1112-POISON-INSTALLER-EVIDENCE-20260724.md | 72ece5d9eeef7a220ee4d309d5e28f7c852376c7fecd6288cddc4e643dc61fe8 | 2382 |
| 22 | docs/reviews/libbun-w1112-20260724/correction2-index.md | 42b63e2b37a2f2139a951f5f411305cfae68fd6710f9f07cb2778cf237c814f3 | 1536 |
| 23 | docs/reviews/libbun-w1112-20260724/verdict-snapshot.md | 3138238c945c94f8212855889b0d23600fd97009a006bc57ea862a5aa7d6a6d6 | 16588 |
| 24 | docs/reviews/libbun-w1112-20260724/owner-generative-independent-verdict.md | 5c9e172ad549ce88e16e05bc37d023ec4c4856cab5edc94da9f289a091e0b4ba | 7660 |
| 25 | docs/reviews/libbun-w1112-20260724/lifecycle-independent-verdict.commit | 377706f6e59259f1e5f4b21c2d7fc99854e0ef57d1ac206c2a8c632148838310 | 256 |
| 26 | docs/reviews/libbun-w1112-20260724/lifecycle-correction-ruling.md | 6a50421f4605fc0273aa4d04dbe611e90dcc63943ace9976e446a98d9b868de6 | 2143 |
| 27 | docs/reviews/libbun-w1112-20260724/containment-release-independent-verdict.md | ba792c4113a6e8af5097c636f45a5858b15073409d508f17d2eb384167628837 | 12944 |
| 28 | docs/reviews/libbun-w1112-20260724/exact-source-search-report.md | 2f2a8d95b5568fbf01c9ea2f4c5d38903b9cdb362d682b629ac8e2abb99dc9bd | 128039 |
| 29 | docs/reviews/libbun-w1112-20260724/vendored-bun-boundary-report.md | b87a48b90a5081d974ecec8706e023ef8d36eef5e4ac3c40c5f156db43fd78b9 | 30673 |
| 30 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-source-index.md | 3d1f009d0c2d16346a73ae505b92ce3d8033ecc88758468b4eae027359c2ad4b | 5610 |
| 31 | docs/reviews/libbun-w1112-20260724/lifecycle-vendored-jsc-source-bundle.md | 4d74d89e4417583edcde93f4e78c598ef4bc6acd02f03dd5944e7e2c7c28dcf4 | 87805 |
| 32 | docs/reviews/libbun-w1112-20260724/process-drop-caller-and-fixture-report.md | e3f7f4a06c699faac73511dc27129a66dc22f4d994124632b1449fafc18c056c | 45104 |
| 33 | docs/reviews/libbun-w1112-20260724/lock-privacy-compliance-index.md | 96c9d42afb4444b5da5129a92f1b299762fed254c22629bac9d9b607567a76d8 | 2810 |
| 34 | scripts/generate-libbun-w1112-review-evidence-20260724.py | 0069166626e368d5904b7b7810a21bf0b64f99e996933f07452b36180fadf6af | 24056 |
| 35 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 36 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 37 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 38 | native/build.rs | 4122c83383a012da82336b223bc10f625f7fc49da33375b76a775ac212a0c72d | 5195 |
| 39 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 40 | runtime/src/main.rs | 3cfa3711281938d2752ce45d4f5b4a3395466ce1df5550e2647a4572e6dfccee | 1117 |
| 41 | runtime/build.rs | c3badf2aaf44535f8d5bedbfa45c36e21c69626e69b4aad44a79eb4f29c91be0 | 4902 |
| 42 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 43 | native/Cargo.toml | 5d624b8bcee44bab2a2ae8f87bc0238cc15c16b5975b0f071af7e80581b0b6fd | 639 |
| 44 | runtime/Cargo.toml | c9c8a17548f5350805af9c95835cf7a8c27055a54b26992e0ffffd1fd352a7d2 | 439 |
| 45 | wire/Cargo.toml | a0517ebfe3c61df2f6e2516aa39f8ca56851eb18d44e21f55a47ae2e8813dc8f | 125 |
| 46 | tests/public_api_boundary.rs | e53307e8fdcd0e12ed63056a32c8e1836b24e866acd00bdf4bd5da25afc9b370 | 1784 |
| 47 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-external-capability-provider-owner/src/lib.rs | e3818b7ce41be8a6a5c5f424418881b4f433883b54a81ddeb96650b2ce9a4439 | 14848 |
| 48 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/src/external_transport.rs | b855d0cc8662591ee1f53473831a47034e7c59ecde415883c33dadb6908f4623 | 4907 |

No part review response, synthesis session, request, response, or output exists. Fresh independent source-aware BUNDLE PASS verdicts for all corrected parts remain required before any launch.
