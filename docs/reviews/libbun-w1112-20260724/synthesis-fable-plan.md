# Correction-4 Fable file plan: W1-11/W1-12 full-SCC synthesis

- Prior independent verdicts: all three correction-3 parts are PART BUNDLE REVISE
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/synthesis-prompt.md
- Prompt SHA-256: bf8b243674d869fd114a3b1af67e795dab900df54a4910c52dd5471503dd57e6
- Ordered file plan: docs/reviews/libbun-w1112-20260724/synthesis-files.txt
- Ordered file count: 39
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | docs/reviews/libbun-w1112-20260724/owner-generative-manifest.json | b92f4f5e4bf967f65c0ef680c63fcb4058a6aa29d92c8ab085c3da0c5b6571f9 | 11400 |
| 2 | docs/reviews/libbun-w1112-20260724/owner-generative-prompt.md | 680ca13fb173f88820e52b003069336706f70294ea2f5e606cd55513ecd6eff0 | 4061 |
| 3 | docs/reviews/libbun-w1112-20260724/owner-generative-oracle-dry-run.txt | 951e50a0883e643272993f859b4f90481ef36b32aa5ff47676526f18647f61fd | 4000 |
| 4 | docs/reviews/libbun-w1112-20260724/owner-generative-fable-plan.md | a66b5cacbe0fd61f09cdeec72b1be732c2fa07d714c3f4116dab813dfd29e0c2 | 6521 |
| 5 | docs/reviews/libbun-w1112-20260724/owner-generative-files.txt | 53d6833b62c3347c3ba052f77fe27aa72184b4883020969b38ad95a7019d9aca | 3005 |
| 6 | docs/reviews/libbun-w1112-20260724/lifecycle-manifest.json | a11b05b7a4f90bc9cd81fe44fd31dbf84685fe0de7c4dec8a9da16d6200df515 | 12667 |
| 7 | docs/reviews/libbun-w1112-20260724/lifecycle-prompt.md | db33f0c83d35142167661c69d3eb55118a0c333753ba4d9f76073519051e230b | 3256 |
| 8 | docs/reviews/libbun-w1112-20260724/lifecycle-oracle-dry-run.txt | c6cb5f387ea1b4b66971f0474c862793a5fd7cd07e20c793d68104d1b3388f4e | 4736 |
| 9 | docs/reviews/libbun-w1112-20260724/lifecycle-fable-plan.md | c5ad1aaac0cb5ab71fcbee8e8bcf2fd2fe1d13b01e3f8ab63933b4a68818e4aa | 7575 |
| 10 | docs/reviews/libbun-w1112-20260724/lifecycle-files.txt | 00689c94b1b2e86a91523146bb63c618a3cfd8750df8bb139b44610f98bb49c8 | 3648 |
| 11 | docs/reviews/libbun-w1112-20260724/containment-release-manifest.json | c433a06d9d59a9c5f241da21033f6cbfb17aa47480942778bf0c243d9c58217e | 15397 |
| 12 | docs/reviews/libbun-w1112-20260724/containment-release-prompt.md | 9ca91668ca5f266e4432fe6e0b7ace604cd5096b1c28e7d94c8177e6fee0e7be | 3194 |
| 13 | docs/reviews/libbun-w1112-20260724/containment-release-oracle-dry-run.txt | 6ad5b915c626be855c67596ed78a085265a5804725a83b518df4f1e7988a9e63 | 5411 |
| 14 | docs/reviews/libbun-w1112-20260724/containment-release-fable-plan.md | e60500b368044bbbcf4ab69b0f8eb626c93eb0a1110e989ab3243724c53f12dc | 9327 |
| 15 | docs/reviews/libbun-w1112-20260724/containment-release-files.txt | cd03bc8454b144cf93820b6d13973e2fe9e6ec68b5ca862096d8ebb6f7c30a96 | 3913 |
| 16 | docs/reviews/libbun-w1112-20260724/correction4-index.md | f2c4b1452288fe220cfe1a0ae167a2c543cc8d2c6ff87843991e99fc397bfbbc | 2074 |
| 17 | docs/reviews/libbun-w1112-20260724/owner-generative-correction3-independent-verdict.md | cfbaf63f12927553a36c5377145ec9309b9d7a601cc497861433d1b637f8a30e | 15022 |
| 18 | docs/reviews/libbun-w1112-20260724/lifecycle-correction3-independent-verdict.md | 82a9678c444921696178bc051d27484f7ccfa50ea154a52dc369b6a58733a938 | 12423 |
| 19 | docs/reviews/libbun-w1112-20260724/containment-release-correction3-independent-verdict.md | a7076b20968629d5df4b08fab77f38d792c03585d0f26c49c099a62c3ac4367a | 11841 |
| 20 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 21 | docs/LIBBUN-WORKER-CONTAINMENT-CONTRACT.md | 7c8f9d64d9fc4d05f5581097f651af28f92f221a81852d81d0a420770420bc08 | 16493 |
| 22 | docs/LIBBUN-WORKER-RELEASE-CONTRACT.md | 030f64ecfbdcc046dec66d4c767333d50ed21471c084c885e2576c770f1d394c | 8765 |
| 23 | docs/reviews/libbun-w1112-20260724/adjacent-generative-source-bundle.md | 06273bfe021a85fbab49622a1efe8701fba72a9ee80af2ec88dde225564851fc | 279138 |
| 24 | docs/reviews/libbun-w1112-20260724/lifecycle-process-worker-source-bundle.md | 847aa002148fa3c3612c160c56623e425d4e32bfe092df2b152e7a88a346d62d | 70338 |
| 25 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 26 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 27 | native/src/lib.rs | 7eb9cc6ef601cbbe5d7aa3a40e33dd9870883051a59edc884b150f78468efb8a | 32133 |
| 28 | wire/src/lib.rs | bf30daf3ebd2702fad606a4abe7ac5d1854aa2bf53e19cf5015352d2bd1f87fa | 5885 |
| 29 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-value-model/src/lib.rs | 90f0cb4dd8a4a71afcaf4ef81724088ba5f44348e63cfd304944b94027b0940a | 20764 |
| 30 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss/tests/external_capability_provider.rs | 22aa9c60530d9ab1a7d8e1d4ca9b3587d5dd520e2a3f7b442d7046072fc35191 | 14251 |
| 31 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs | 3eed047f11c5ac8bb8aee51dc4c5f7e96520af3b8ed41874d6736f643018c9de | 2269 |
| 32 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs | 48b30f77c809f06b7dfb130b11e190cff8e742809140840d14a3a4b5fb9a4323 | 3030 |
| 33 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs | 5519fbbf653aa19cc883dc881eaaf4ff2587dc13bb2b7ab4d3ac2d9575a3b8d5 | 85748 |
| 34 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs | 3be90814fe8071aed3c48778c307a63266d3fc10aa689581e73f7a4e6bf83012 | 6025 |
| 35 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs | 54c9aaee5abca13fb791eeffff19e8abd3b02134db567bec5135d439bc691760 | 29621 |
| 36 | scripts/package-prepared-export-worker-release.sh | ce1f433f73312f761d0bf4ffd6b30296b6fe88dfb3740f7b5a53450f95986f44 | 2111 |
| 37 | scripts/prepare-native-bun-link.sh | 498bd2954bb44b7d738c56e05a3615a82ca0baf6092765dfa992de24732d37d6 | 3233 |
| 38 | scripts/verify-vendored-bun-reproducible.sh | 96cc8eb86f506f288a0c46efd89b284ec0a89ddf120e70e8481593a91db81304 | 1913 |
| 39 | scripts/verify-vendored-bun.sh | d95267b26304c545bb4cffb2234daab6b5735bd818d22ef2a27312bed951c5a7 | 1515 |

No Fable session, request, response, or output exists. Fresh literal independent PART BUNDLE PASS verdicts for correction 4 remain required before launch.
