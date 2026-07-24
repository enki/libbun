# Correction-6 Fable file plan: W1-11/W1-12 full-SCC synthesis

- Prior independent verdicts at 5e74c14a0125c1670be7e37cc31675ebedcd538d: owner/correspondence PART BUNDLE REVISE; lifecycle and containment/release PART BUNDLE PASS; correction-6 synthesis is blocked pending only the corrected owner pass
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/synthesis-prompt.md
- Prompt SHA-256: 28cdad3881c50871199961d0dc83adeae825a64489855249d4f5ce56097f9dc0
- Ordered file plan: docs/reviews/libbun-w1112-20260724/synthesis-files.txt
- Ordered file count: 32
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | docs/reviews/libbun-w1112-20260724/owner-generative-manifest.json | b319a4b176b56734f0cbe0e18a14328b0aa08effd4dc2608b06f05c70bb8864a | 11629 |
| 2 | docs/reviews/libbun-w1112-20260724/owner-generative-prompt.md | aaf1e6c6ff84565329bd29603d64a9467082c91eedd6135ee008e76597c0c0a8 | 5616 |
| 3 | docs/reviews/libbun-w1112-20260724/owner-generative-oracle-dry-run.txt | 3f2c3f29be3ba5fca27b8e3fb2ca70c0649e1b1150d34dd2ae1b2330d165cfdd | 4074 |
| 4 | docs/reviews/libbun-w1112-20260724/owner-generative-fable-plan.md | 488f45301f69ac1588a10c8f07aa221fac6f4b213db7ec4c841fe39a51a2c3c0 | 6853 |
| 5 | docs/reviews/libbun-w1112-20260724/owner-generative-files.txt | da265a6e01be9ac48c0680434d037c8d718cafdbf9ae2852c160123166375787 | 3067 |
| 6 | docs/reviews/libbun-w1112-20260724/lifecycle-manifest.json | a27933d98126db50a87af583956979e423a6936efb89695b1e2abb6206ceb590 | 12702 |
| 7 | docs/reviews/libbun-w1112-20260724/lifecycle-prompt.md | 2074d2fce8159fe91037a62f1f3137b550b579735115971260789058bebd9a10 | 3778 |
| 8 | docs/reviews/libbun-w1112-20260724/lifecycle-oracle-dry-run.txt | bb57b46f980145502bc42c22584206902118a2b10888ca173db4bb2ed5d5b512 | 4734 |
| 9 | docs/reviews/libbun-w1112-20260724/lifecycle-fable-plan.md | ac098d1c31d3a10347f2c7d1739e41bfcc67d043969718fd40e58ee362258c66 | 7762 |
| 10 | docs/reviews/libbun-w1112-20260724/lifecycle-files.txt | 921077f86b224d6c1001eec96f8fc6420db59529807889292cc007341e5851b3 | 3650 |
| 11 | docs/reviews/libbun-w1112-20260724/containment-release-manifest.json | a8173108ee5b2554ad894d6ac75cf9f4839ffe1112d7b71c9b44e54ff8895b45 | 15621 |
| 12 | docs/reviews/libbun-w1112-20260724/containment-release-prompt.md | edbe3c77c64eb344dfd3cab79d837a36032f53aaf5791ab5bf0ebe7fca700e4d | 3969 |
| 13 | docs/reviews/libbun-w1112-20260724/containment-release-oracle-dry-run.txt | 50365e8a5bfdf28d7fc7b8b1841033034e9ee209d0286bf2a7a6163bf5bf19f7 | 5498 |
| 14 | docs/reviews/libbun-w1112-20260724/containment-release-fable-plan.md | ef08844d6d85266105b4424fcbbb46664ff95c06dea24cd00edd8a45fcaeef51 | 9662 |
| 15 | docs/reviews/libbun-w1112-20260724/containment-release-files.txt | ddd93a0d4bada396af7943e5520f5b066f1930bfd5a718dcdc6513c2f019c36b | 3979 |
| 16 | docs/reviews/libbun-w1112-20260724/correction6-index.md | cf6eaa0e692748b347b43dd5542520327aab3c9653af475af3821005884b1151 | 1549 |
| 17 | docs/reviews/libbun-w1112-20260724/correction5-independent-full-family-verdict.md | ee9ca3d178532849f5a670a0b4208ab24f11f95f78d7db9da7d30ef6105d0321 | 11863 |
| 18 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 19 | docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md | 7c8f9d64d9fc4d05f5581097f651af28f92f221a81852d81d0a420770420bc08 | 16493 |
| 20 | docs/LIBBUN-WORKER-RELEASE-CONTRACT.md | 030f64ecfbdcc046dec66d4c767333d50ed21471c084c885e2576c770f1d394c | 8765 |
| 21 | docs/reviews/libbun-w1112-20260724/adjacent-generative-source-bundle.md | 1b5bd872ee9d402f899fc499876bd33e75f294c7635e2bf91dfc7a9fc5e364eb | 470192 |
| 22 | docs/reviews/libbun-w1112-20260724/lifecycle-process-worker-source-bundle.md | dc4445d955b81d7c62af76357c6035efe3c6fe97f27a1ee6743ff5a9b11d659c | 78506 |
| 23 | docs/reviews/libbun-w1112-20260724/atomic-deletion-tests-source-bundle.md | e3cacc0117670744ae690bda45be5b6e4d59ca974652a322c38b5cca51c35568 | 37849 |
| 24 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 25 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 26 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 27 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 28 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-value-model/src/lib.rs | 90f0cb4dd8a4a71afcaf4ef81724088ba5f44348e63cfd304944b94027b0940a | 20764 |
| 29 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss/tests/external_capability_provider.rs | 22aa9c60530d9ab1a7d8e1d4ca9b3587d5dd520e2a3f7b442d7046072fc35191 | 14251 |
| 30 | scripts/package-prepared-export-worker-release.sh | ce1f433f73312f761d0bf4ffd6b30296b6fe88dfb3740f7b5a53450f95986f44 | 2111 |
| 31 | scripts/prepare-native-bun-link.sh | 498bd2954bb44b7d738c56e05a3615a82ca0baf6092765dfa992de24732d37d6 | 3233 |
| 32 | scripts/verify-vendored-bun-reproducible.sh | 96cc8eb86f506f288a0c46efd89b284ec0a89ddf120e70e8481593a91db81304 | 1913 |

No Fable session, request, response, or output exists. Fresh literal independent The correction-5 lifecycle and containment/release PART BUNDLE PASS verdicts remain controlling. A fresh literal owner/correspondence PART BUNDLE PASS for correction 6 remains required before launch, and synthesis stays blocked until that owner verdict passes.
