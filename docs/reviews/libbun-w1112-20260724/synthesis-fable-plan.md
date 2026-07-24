# Correction-5 Fable file plan: W1-11/W1-12 full-SCC synthesis

- Prior independent verdicts: all three correction-4 parts are PART BUNDLE REVISE; correction-5 synthesis is blocked pending fresh passes
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/synthesis-prompt.md
- Prompt SHA-256: 44af045cec07263d1a069b71089e1059044d91f6ee16911a739eab0021b50faf
- Ordered file plan: docs/reviews/libbun-w1112-20260724/synthesis-files.txt
- Ordered file count: 35
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | docs/reviews/libbun-w1112-20260724/owner-generative-manifest.json | b72bfac71e2c535a244f85e74b81da6844827be77e9376fad23818780eb7302e | 11400 |
| 2 | docs/reviews/libbun-w1112-20260724/owner-generative-prompt.md | 619b66dfcb331b30e2dfdb899d80671a04b6a4f36f1a9ebe91120dfcc1904319 | 4486 |
| 3 | docs/reviews/libbun-w1112-20260724/owner-generative-oracle-dry-run.txt | dce78ef056705f5320890a4d8b33a7bd7050deec35a07ecf1224b5780889636b | 4006 |
| 4 | docs/reviews/libbun-w1112-20260724/owner-generative-fable-plan.md | f7278b2bfe14bfacd5a388365f5a472377a5fab2e315cdf7b8d49e813bfeb02f | 6583 |
| 5 | docs/reviews/libbun-w1112-20260724/owner-generative-files.txt | 91d7a1423be971d6b8ff075c8c8238fc5b46ccc84ccae1fa3b6599e7e8b41046 | 3005 |
| 6 | docs/reviews/libbun-w1112-20260724/lifecycle-manifest.json | 617c2004a2c6fa5d2a227205fe155f18cdf22858c5ee283c49c5a78b308bcee3 | 12666 |
| 7 | docs/reviews/libbun-w1112-20260724/lifecycle-prompt.md | e4c4a80927e4ac59986b76cdda8ab6deff2ff32c7e4650fb72e85ba00b56b1d0 | 3599 |
| 8 | docs/reviews/libbun-w1112-20260724/lifecycle-oracle-dry-run.txt | ece4dd81f773e295b0f621042e62ad8fed1b34c849a074b2e266a3a1acf5028b | 4745 |
| 9 | docs/reviews/libbun-w1112-20260724/lifecycle-fable-plan.md | b550f9a17ac049b3756b8c322aa5197a15bf8e82dd6cbc62428bc42bc18f0462 | 7636 |
| 10 | docs/reviews/libbun-w1112-20260724/lifecycle-files.txt | 2946ac9282d5b06bb6fd3188c8a1b6474c3c96768e1bfef912b2485485cd15e4 | 3648 |
| 11 | docs/reviews/libbun-w1112-20260724/containment-release-manifest.json | eef255117f3e0b5e722233754e3af59aad77b6334932ca9c063c85d50422548d | 15605 |
| 12 | docs/reviews/libbun-w1112-20260724/containment-release-prompt.md | 974c690f71b27910d70c37a8321b0e97890c7b93b37bbdc102e2360ae533d625 | 3787 |
| 13 | docs/reviews/libbun-w1112-20260724/containment-release-oracle-dry-run.txt | 337f4ec19af154c203606d4a3d51aab829f9fce3c4ecf93ea0ecf34080e4b358 | 5497 |
| 14 | docs/reviews/libbun-w1112-20260724/containment-release-fable-plan.md | c1939a74af68ff16a5c3021aa4a9a9b94a8bd09b612e6ab851b200e778fb4632 | 9546 |
| 15 | docs/reviews/libbun-w1112-20260724/containment-release-files.txt | d8c4226d1c05f83c73ce1fe98c118ec5d151f84ff7708b59c2aee5d298d79040 | 3987 |
| 16 | docs/reviews/libbun-w1112-20260724/correction5-index.md | 08fd92e21a78e05b1e48dd2ee7e754b69ac4731cde9a11e409a5b5560dd0e7dd | 1161 |
| 17 | docs/reviews/libbun-w1112-20260724/owner-generative-correction4-independent-verdict.md | e814343062c217d283313c02f612a6afce0da4a354cbf571238306c384a90807 | 14977 |
| 18 | docs/reviews/libbun-w1112-20260724/lifecycle-correction4-independent-verdict.md | adaafa862469dcb90eb6a4d7181fc6dc400a1294bc3c1fba794ff85b8c720a89 | 7613 |
| 19 | docs/reviews/libbun-w1112-20260724/containment-release-correction4-independent-verdict.md | c751b5d48a8331062e7ad23a02f29234bc63d05dc31c810c2df1705787c2973f | 9027 |
| 20 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 21 | docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md | 7c8f9d64d9fc4d05f5581097f651af28f92f221a81852d81d0a420770420bc08 | 16493 |
| 22 | docs/LIBBUN-WORKER-RELEASE-CONTRACT.md | 030f64ecfbdcc046dec66d4c767333d50ed21471c084c885e2576c770f1d394c | 8765 |
| 23 | docs/reviews/libbun-w1112-20260724/adjacent-generative-source-bundle.md | 0727a7efbd268d05bb6ee04d99a168bc9cc2cb7ac18b45b7ccfe9c1e0ebb121f | 353167 |
| 24 | docs/reviews/libbun-w1112-20260724/lifecycle-process-worker-source-bundle.md | f5647100ea07d9204c2a451ae43e5dd2af7aa56db313d61be5371c1f2339460a | 78506 |
| 25 | docs/reviews/libbun-w1112-20260724/atomic-deletion-tests-source-bundle.md | 30e620ee8efd602632a7a70a64a8536e21200d31fc8d94bc8df5c31edf32788f | 37849 |
| 26 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 27 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 28 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 29 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 30 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-value-model/src/lib.rs | 90f0cb4dd8a4a71afcaf4ef81724088ba5f44348e63cfd304944b94027b0940a | 20764 |
| 31 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss/tests/external_capability_provider.rs | 22aa9c60530d9ab1a7d8e1d4ca9b3587d5dd520e2a3f7b442d7046072fc35191 | 14251 |
| 32 | scripts/package-prepared-export-worker-release.sh | ce1f433f73312f761d0bf4ffd6b30296b6fe88dfb3740f7b5a53450f95986f44 | 2111 |
| 33 | scripts/prepare-native-bun-link.sh | 498bd2954bb44b7d738c56e05a3615a82ca0baf6092765dfa992de24732d37d6 | 3233 |
| 34 | scripts/verify-vendored-bun-reproducible.sh | 96cc8eb86f506f288a0c46efd89b284ec0a89ddf120e70e8481593a91db81304 | 1913 |
| 35 | scripts/verify-vendored-bun.sh | d95267b26304c545bb4cffb2234daab6b5735bd818d22ef2a27312bed951c5a7 | 1515 |

No Fable session, request, response, or output exists. Fresh literal independent PART BUNDLE PASS verdicts for correction 5 remain required before launch. Synthesis remains blocked until all three part verdicts pass.
