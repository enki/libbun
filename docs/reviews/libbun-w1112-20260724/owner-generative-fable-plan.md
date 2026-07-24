# Correction-4 Fable file plan: Owner/W1-10/generative correspondence/admission/reservation/release

- Prior independent verdict: PART BUNDLE REVISE at 29136ad08f0103cd4338db51552a2a566625d81d
- State: NOT LAUNCHED
- Engine: local Fable wrapper
- Model: claude-fable-5
- Effort: max
- Deliverable: CONCRETE IMPLEMENTATION
- Prompt: docs/reviews/libbun-w1112-20260724/owner-generative-prompt.md
- Prompt SHA-256: 680ca13fb173f88820e52b003069336706f70294ea2f5e606cd55513ecd6eff0
- Ordered file plan: docs/reviews/libbun-w1112-20260724/owner-generative-files.txt
- Ordered file count: 32
- Identical to Oracle ordered attachments: yes

## Ordered attachments

| # | Path | SHA-256 | Bytes |
| ---: | --- | --- | ---: |
| 1 | README.md | 7b8a88d47e3ccf5fe518f0dfa44bdc09d26440dc7e461cb33160cf1e2e618d70 | 3369 |
| 2 | docs/LIBBUN-LIFECYCLE-CONTRACT.md | c3775f713913713ebc36b37f442cf87927d3e445ba82f80ae1c7fef041526881 | 17107 |
| 3 | docs/LIBBUN-W1112-POISON-INSTALLER-EVIDENCE-20260724.md | 72ece5d9eeef7a220ee4d309d5e28f7c852376c7fecd6288cddc4e643dc61fe8 | 2382 |
| 4 | docs/reviews/libbun-w1112-20260724/correction4-index.md | f2c4b1452288fe220cfe1a0ae167a2c543cc8d2c6ff87843991e99fc397bfbbc | 2074 |
| 5 | docs/reviews/libbun-w1112-20260724/owner-generative-correction3-independent-verdict.md | cfbaf63f12927553a36c5377145ec9309b9d7a601cc497861433d1b637f8a30e | 15022 |
| 6 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-source-index.md | 1243798c2522857ed03e2e14c3ee51c22079ae4291ceeac68237c6af712e2e10 | 25577 |
| 7 | docs/reviews/libbun-w1112-20260724/adjacent-generative-source-bundle.md | 06273bfe021a85fbab49622a1efe8701fba72a9ee80af2ec88dde225564851fc | 279138 |
| 8 | src/lib.rs | 25e7a172b0c099e65d289e38c05866a5c4808475194038dd59bf2b29af53f96b | 551 |
| 9 | src/prepared_export.rs | 88bb176940654c17528329ee50cc0f2894a8abf64a069689b46d4903a477d9e8 | 42745 |
| 10 | Cargo.toml | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 |
| 11 | tests/fixtures/public_api_boundary/Cargo.lock | 685de9512211743a6430574bba5832a3175bd49ce184ab8553013a64fe38c6f2 | 1017 |
| 12 | tests/fixtures/public_api_boundary/Cargo.toml | c211189cf1e2c05017baec08952dce72fe6d8c372b7b887049dd52a78bbb8c96 | 409 |
| 13 | tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs | b09059159bc4035e7799e881805ba30216a651b7f2a9c816d28f40f249fd8dcc | 413 |
| 14 | tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs | 5f6bc0ec079770540e3e9f49654180da9ef74a03f6a54975e2e9a467c08df281 | 98 |
| 15 | tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs | b290426f65fcc9da855969004223d5383ae64f37bcaf3e6b0035cc3a2e0992bb | 128 |
| 16 | tests/public_api_boundary.rs | e53307e8fdcd0e12ed63056a32c8e1836b24e866acd00bdf4bd5da25afc9b370 | 1784 |
| 17 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/Cargo.toml | dd05cd50a38d1ed15cd2f5276c29c6039688eed4f66052b9591946d2a4f74d9b | 24507 |
| 18 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarmvm-image/src/prepared_runtime_artifact_owner.rs | 847c4f7b13917810adaa373299ae0df20c373cd4671a97eac265c6c093cc3399 | 78342 |
| 19 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarmvm-image/Cargo.toml | c300820c208d5610a2d5a9c16c3de57ebd2fcc855d622f6443d1430bddec4db7 | 1342 |
| 20 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-value-model/src/lib.rs | 90f0cb4dd8a4a71afcaf4ef81724088ba5f44348e63cfd304944b94027b0940a | 20764 |
| 21 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-value-model/Cargo.toml | 8f6a90758239f6dc323ff83fcf71cc6915cbd4291998e1cc8718d171a364ca64 | 709 |
| 22 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-capability-model/Cargo.toml | f436221858f1280a49c6a807f944c0d56dfc23e5048efc06ab284b88c8541bea | 410 |
| 23 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-source-compiler-owner/Cargo.toml | 207f1a8f4db914475df0872a8a1a69a9ea6b6b98e644de31b859bcced8fa237e | 6657 |
| 24 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-provider-host-set/Cargo.toml | ce3c3333e5e73bd678f11f4b18cd447e82aee0891c1781d89c6f51aaab3083d1 | 829 |
| 25 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-rust-sdk-static-provider-host/src/lib.rs | 36592c61d77526b6b12efb77e7013d7e94fea5f09011bd6cf57befcabc9ad725 | 6001 |
| 26 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/swarm-rust-sdk-static-provider-host/Cargo.toml | cbe99d18d1eb62551907fa249b6632c0ca0a691d246559b0459039c25d9339aa | 1074 |
| 27 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/crates/ss-runtime-external-capability-provider-owner/Cargo.toml | 6f9a0d10b0aa4f13049b63013f1469726de88a4adbd2ac48bc862c29b5bb9ee0 | 690 |
| 28 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/docs/PROVIDER_EXECUTION_AND_SDK_LAW.md | 0b4f90a92d75c2f4393c7200c9d2c3ab85a028304515a63e7f801f6ed0eba910 | 50618 |
| 29 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/docs/PROVIDER_VALUE_JSON_WIRE_V1.md | 01d12bdc4581f08c7ca1713b11bb51521acc18e4b0cfa3de324ac106294f6b67 | 15230 |
| 30 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/external_provider_json_text_nfc.test.ss | 3ae07a264289b52bc77c4bf8da0e88c659a3ded32db5d8ea0d3799953ca361c0 | 588 |
| 31 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/conformance/ss/provider/imported_helper_external_result_payload.test.ss | 9a26ebc4737439e4ae0e8342a3b19a357d2a8008d2d09330cc92fcae924dae43 | 771 |
| 32 | docs/reviews/libbun-w1112-20260724/adjacent-swarm-95323ff17cb29928e31467f651ef03bae2099c14/tests/negative/ss/provider/external_provider_json_nfc_duplicate_keys.test.ss | 5d238da6cfd9e1f46a46fc243409d62c3940964cb084343adae08b9079d6c99d | 493 |

No Fable session, request, response, or output exists. Fresh literal independent PART BUNDLE PASS verdicts for correction 4 remain required before launch.
