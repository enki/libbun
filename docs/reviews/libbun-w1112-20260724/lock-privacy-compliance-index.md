# Lock, privacy fixture, license, provenance, and compliance index

Exact product SHA: 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb

Exact product tree: cb964de8ab8162449fbe95959bf34d231570aa5c

The table below selects the bounded linked native/package closure: all four nonvendored locks, the complete six-file external privacy harness, the vendored workspace/lock, source-package licenses, vendor provenance, and licenses for the linked Bun/JSC dependencies named by that provenance. The second table inventories every tracked Cargo manifest/lock and license/notice path so the selected closure is reproducible without claiming that every vendored tool/test license is attached.

## Selected direct attachments

| Path | Git blob | SHA-256 | Bytes | Unique lock packages | Selection reason |
| --- | --- | --- | ---: | ---: | --- |
| Cargo.lock | 512ca1e971919c2dc0070124a5d4c1815f9feec5 | cb9819c613ccd991508d245de06261b03b2284d2800dca1ab26a04191952a392 | 2779 | 13 | libbun facade workspace lock |
| native/Cargo.lock | cb86ba4e1c4a1fc1e8ac75f37807390f43f05b7d | 6dd537cd346d2ee311708e4a88db8e250530d1c428a5728caab6e3bda2d03496 | 70206 | 261 | native linked engine lock |
| runtime/Cargo.lock | d0e97c3150f03e47022e09ef6d766814eab34ca4 | 7066c2adfbebf44e5b156ff1b3864b2e74976b5edce1634a11b2a99f56294617 | 70107 | 261 | worker runtime lock |
| tests/fixtures/public_api_boundary/Cargo.lock | d976926d2052442611096f90b923534c518f7eda | 685de9512211743a6430574bba5832a3175bd49ce184ab8553013a64fe38c6f2 | 1017 | 6 | external privacy fixture lock |
| tests/fixtures/public_api_boundary/Cargo.toml | 37016b9026a6237ec73e73076dad94faea066056 | c211189cf1e2c05017baec08952dce72fe6d8c372b7b887049dd52a78bbb8c96 | 409 | - | external privacy package |
| tests/fixtures/public_api_boundary/src/bin/adjacent_public_controls.rs | 2ea5a53ea124df822705961783efe3329f59e351 | b09059159bc4035e7799e881805ba30216a651b7f2a9c816d28f40f249fd8dcc | 413 | - | adjacent public-control fixture |
| tests/fixtures/public_api_boundary/src/bin/call_raw_installer.rs | 0fce7c5df6d289239288cf94b8d58f8e7b9f18a0 | 5f6bc0ec079770540e3e9f49654180da9ef74a03f6a54975e2e9a467c08df281 | 98 | - | raw installer call refusal fixture |
| tests/fixtures/public_api_boundary/src/bin/import_raw_installer.rs | 23bd541bf74de01add1f5d840344ea11876d8764 | b290426f65fcc9da855969004223d5383ae64f37bcaf3e6b0035cc3a2e0992bb | 128 | - | raw installer import refusal fixture |
| tests/public_api_boundary.rs | 1bb86384bf27653d37601e8303b66bf572bbadf5 | e53307e8fdcd0e12ed63056a32c8e1836b24e866acd00bdf4bd5da25afc9b370 | 1784 | - | privacy harness owner |
| LICENSE | 261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64 | c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4 | 11357 | - | libbun source-package license |
| vendor/README.md | 6eb15a259b7739643f7c28731639185162c8b884 | a8339961bd5659a2cdd534cd905cecca5d000fcfbdec93933f676512d2a27f0e | 825 | - | vendored-source policy |
| vendor/bun.LIBBUN_VENDOR.json | c0c92e65328d5ff20c3d0fd83a2c40d5b397234f | b5f3748b5c985b86de748777948d850045572e348d0f8e9649e74259b73a0d02 | 453 | - | Bun provenance and linked dependency declaration |
| vendor/bun/LICENSE.md | 81069ee8d3b84f21ee32b2a9766643e1de114863 | 2c6160ec8fb853f7e8f97d9b249e756c9b0ac44860a68b6bf4f1b0bcbc5c3741 | 5376 | - | Bun source-package license |
| vendor/bun/Cargo.lock | 3fe628aa5b7dea4d6c594a194ed98d5c79207a14 | b9137da3f975f37c6f225d81f96b01dda58119f5212349a9a88c746fc27147fb | 75040 | 287 | vendored Bun locked dependency graph |
| vendor/bun/Cargo.toml | e1232a4cf29435189b91182b901f9a724da999b5 | 2101849a8242a31c43d5952b663771455f21266489839fbd774d6a98283daadb | 12011 | - | vendored Bun workspace and dependency selection |
| vendor/bun/src/clap/LICENSE | cf1ab25da0349f84a3fdd40032f0ce99db813b8b | 88d9b4eb60579c191ec391ca04c16130572d7eedc4a86daa58bf28c6e14c9bcd | 1210 | - | linked Bun clap source license |
| vendor/bun/src/unicode/uucode_lib/LICENSE.md | 412454e31dfa9bac8c6ba8263cd3c49e87dfd1ce | 75b52b07e8f6ed6b1700ca6e4bcff93a59258624d4fd1ab7eae4c071c860b69b | 1298 | - | linked Bun Unicode source license |
| vendor/bun/vendor/lolhtml/LICENSE | 98b3bec0935e5c2539f70348d2151e1d9b7f00b3 | e4ddaa9d7391bb9536fcb8c59b570a8b85a0bf6da54df5b3b26f098f6f99c9cc | 1487 | - | provenance-declared linked lolhtml license |

## Exact-tree inventory

Command: git -C "$LIBBUN_REPO" ls-tree -r --name-only 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb

| Family | Path | Git blob | SHA-256 | Bytes | Disposition |
| --- | --- | --- | --- | ---: | --- |
| Cargo manifest/lock | Cargo.lock | 512ca1e971919c2dc0070124a5d4c1815f9feec5 | cb9819c613ccd991508d245de06261b03b2284d2800dca1ab26a04191952a392 | 2779 | selected |
| Cargo manifest/lock | Cargo.toml | dc704d36a0262e6ee93319b920ea7d5a1c7b0ac5 | 0fe7a603f1cc43394421f03fca41255166ca665b11c4b75357376cd98c99c496 | 956 | inventory only |
| Cargo manifest/lock | native/Cargo.lock | cb86ba4e1c4a1fc1e8ac75f37807390f43f05b7d | 6dd537cd346d2ee311708e4a88db8e250530d1c428a5728caab6e3bda2d03496 | 70206 | selected |
| Cargo manifest/lock | native/Cargo.toml | 147915a88849c0e3849546c07e2a3686389dd86e | 5d624b8bcee44bab2a2ae8f87bc0238cc15c16b5975b0f071af7e80581b0b6fd | 639 | inventory only |
| Cargo manifest/lock | runtime/Cargo.lock | d0e97c3150f03e47022e09ef6d766814eab34ca4 | 7066c2adfbebf44e5b156ff1b3864b2e74976b5edce1634a11b2a99f56294617 | 70107 | selected |
| Cargo manifest/lock | runtime/Cargo.toml | 1ec3234e9a8d8d32851852c9a2379a88bc1a88ba | c9c8a17548f5350805af9c95835cf7a8c27055a54b26992e0ffffd1fd352a7d2 | 439 | inventory only |
| Cargo manifest/lock | tests/fixtures/public_api_boundary/Cargo.lock | d976926d2052442611096f90b923534c518f7eda | 685de9512211743a6430574bba5832a3175bd49ce184ab8553013a64fe38c6f2 | 1017 | selected |
| Cargo manifest/lock | tests/fixtures/public_api_boundary/Cargo.toml | 37016b9026a6237ec73e73076dad94faea066056 | c211189cf1e2c05017baec08952dce72fe6d8c372b7b887049dd52a78bbb8c96 | 409 | selected |
| Cargo manifest/lock | vendor/bun/Cargo.lock | 3fe628aa5b7dea4d6c594a194ed98d5c79207a14 | b9137da3f975f37c6f225d81f96b01dda58119f5212349a9a88c746fc27147fb | 75040 | selected |
| Cargo manifest/lock | vendor/bun/Cargo.toml | e1232a4cf29435189b91182b901f9a724da999b5 | 2101849a8242a31c43d5952b663771455f21266489839fbd774d6a98283daadb | 12011 | selected |
| Cargo manifest/lock | vendor/bun/bench/ffi/src/Cargo.lock | ab0268b612d01085c8f4fe054d2723e40da52404 | 4fdb86ae8edfa35eb26d4c62b0de92700dcfe594792c96287ee55b0ce29cd2c6 | 5222 | inventory only |
| Cargo manifest/lock | vendor/bun/bench/ffi/src/Cargo.toml | ed1eb550f7b480bb183d83d9b054163c741d815a | fbb4603c194ebdfb2e5750a989757f76be0643709954b6caaeffed4b09385608 | 229 | inventory only |
| Cargo manifest/lock | vendor/bun/packages/bun-build-mdx-rs/Cargo.toml | 90c4753237ecdd0a099f64d929195c21bb190b13 | c55514b6c1092681e3b1e5d164924473c1d88db3b065e6f68924f6c6ae5cc696 | 472 | inventory only |
| Cargo manifest/lock | vendor/bun/packages/bun-native-plugin-rs/Cargo.lock | 734afe2a61533b06f652a6c4c1b603ce514dc6ab | ddd1964619d24b5b12c02ddd6b20798532555ab0eff62f407bf8b5f6db440811 | 6048 | inventory only |
| Cargo manifest/lock | vendor/bun/packages/bun-native-plugin-rs/Cargo.toml | b6ebc1206391f93acf25f51159a4cb487cc28692 | fe8c2482dbbd5275cf683c1aed38de3a3b9242c6b69f6d4309a7312fa327949a | 307 | inventory only |
| Cargo manifest/lock | vendor/bun/packages/bun-native-plugin-rs/bun-macro/Cargo.toml | 3fe3c7ecd61572ecb0357119230034ed173bf497 | 42bf0938b5de1c725b06e7a8fdb64f6d60284804375e7bded7499cee80bef6da | 278 | inventory only |
| Cargo manifest/lock | vendor/bun/scripts/verify-baseline-static/Cargo.lock | ba5315eb6f1c5c25867b1f3ec663807297707433 | 665d5cc6dacdff9628f28465d433bde3f0d88b38baa08d8a195184e682c1bf13 | 4961 | inventory only |
| Cargo manifest/lock | vendor/bun/scripts/verify-baseline-static/Cargo.toml | 78a0f0de34a68171844acadf24ca7d62b315ac7a | 5dd41d68d6391970b7eb650d5b4139baf18e96bae7bfb33ecc430a46a03909d0 | 504 | inventory only |
| Cargo manifest/lock | vendor/bun/src/analytics/Cargo.toml | 3b32020ad4b506d687839680542326dcb3f73142 | 168e8d5817ba306961e98994c2c4b6a61933dd53878e736377d088bac603eb50 | 450 | inventory only |
| Cargo manifest/lock | vendor/bun/src/api/Cargo.toml | 52e8bbd88663f4fa49f34fa989134f9c9d30106f | f8309ddf0459d9145763cf172dd49e1323172ca9dadba77234a0f12ffa9063ea | 521 | inventory only |
| Cargo manifest/lock | vendor/bun/src/ast/Cargo.toml | 8ef63ef52594fc513a2cf787e611f5a63eb0d095 | 2122bc74dfc3d90ff021bdbaa3fc10c1f608b5ea229dcc89a789f3e404c67900 | 655 | inventory only |
| Cargo manifest/lock | vendor/bun/src/ast_jsc/Cargo.toml | c2ed3447358111954ab4450abd5a62fc1ad347d3 | ad9017e80cb2362a08919fa1b90021063b7a5bef7401add065f17fcec0f438a7 | 472 | inventory only |
| Cargo manifest/lock | vendor/bun/src/base64/Cargo.toml | 215bac192a8b1a9c748c606e89b1b27d7734bb19 | ebc83b3c27d4481ad985a388b2b741876e7c94e53d8a6928e8b3c11188a1500a | 664 | inventory only |
| Cargo manifest/lock | vendor/bun/src/boringssl/Cargo.toml | 4b3234d6dee6af04f2a2ab7493cd8b9055697332 | e013fbc39f281166c621ec89f2c76aed9b4cf73ff2be1980b14190102fbff12c | 515 | inventory only |
| Cargo manifest/lock | vendor/bun/src/boringssl_sys/Cargo.toml | b89bdb44f472ce3ef7553dbbfbdd3b59c449f769 | aae9ff7cfdfdc0e9fca8bc34ae401879555db513af303f9c92c4a529f945e7f9 | 430 | inventory only |
| Cargo manifest/lock | vendor/bun/src/brotli/Cargo.toml | 8763a067fe5929cd4c6536c6c54ef2b70229aa54 | b55d4a56069837bfb1104eab02394ae5ae276ab222afa0fe3b2ccf2091db8b1c | 477 | inventory only |
| Cargo manifest/lock | vendor/bun/src/brotli_sys/Cargo.toml | dbc941b7942144af84bd361f24678aaca9a294fd | 568cf2880bb4104427c81fdbb900da418fa0b594345c5e221bf5a18ef2ba03d3 | 429 | inventory only |
| Cargo manifest/lock | vendor/bun/src/bun_alloc/Cargo.toml | 2ede04538ee8b1d1c7103c53f9ed037ef8a1751b | 3244ae47bb6d934ff1b9de621e1d5a3269449c37d441c50ee8040d1f4ee5c667 | 546 | inventory only |
| Cargo manifest/lock | vendor/bun/src/bun_bin/Cargo.toml | dded78f6dd5751f81e8e4cde6205fa0ea314ecfe | 975f4b466b2df55efc67e698e384499186ceebd95d705c80d9caa0471bcd9421 | 1109 | inventory only |
| Cargo manifest/lock | vendor/bun/src/bun_core/Cargo.toml | 5e0392c1d52ec21e691e6ee0c89a5c4958e6767b | 457fef8181b98f6111823886328381ba8f84480bc33ee85467b94660bcdd5514 | 689 | inventory only |
| Cargo manifest/lock | vendor/bun/src/bun_core_macros/Cargo.toml | 37c4362cf25bb3e5c43ff664076ae85274f26b8d | 8fe294d3322bc492a9e98c2176ef7e90a1dd63739b87fe35f3a4a7a4b1af6811 | 276 | inventory only |
| Cargo manifest/lock | vendor/bun/src/bun_output_tags/Cargo.toml | e063a01fc5d1992878c86c56d73bf44b420eba0d | 7f4adb6b3422275da528732678dd1c8c6842bbf04e15eb5333a1e25a64641325 | 134 | inventory only |
| Cargo manifest/lock | vendor/bun/src/bundler/Cargo.toml | 3b48c09466b087ee0f4507c5e516bcb68cbafe71 | 04fefbb0bfeba5d3ddd3fcac6d14c524b1381367435862b8250303c7e1876e75 | 2355 | inventory only |
| Cargo manifest/lock | vendor/bun/src/bundler_jsc/Cargo.toml | e76f2e846fa7aae8d7a455b9eed4af4caa97b22f | 927c8e31d9e9ec891e3a4284259549495797e0050c568d210fb0f3ec87eb17eb | 762 | inventory only |
| Cargo manifest/lock | vendor/bun/src/bunfig/Cargo.toml | b0eb238d7ba651a3fbf657af480a8850e5259efa | b0165a0595dec69e7a3386d288516f162131956e6322349e712c6417221a3863 | 1192 | inventory only |
| Cargo manifest/lock | vendor/bun/src/cares_sys/Cargo.toml | 187a1f0f70763a3573b258de0458194bdd5798b9 | 3420c61520bc44ac34e106f78038b38c0e2fe21d5defbc193b0af67b4813fdd3 | 549 | inventory only |
| Cargo manifest/lock | vendor/bun/src/clap/Cargo.toml | 289fb551688f7cc274b4a6b486003c6292cc983e | 9654a754f90dedda6011f1263058cb1fcaef74f660ed9a8d6210b3f02f716866 | 452 | inventory only |
| Cargo manifest/lock | vendor/bun/src/clap_macros/Cargo.toml | 371b778e061956aad69d73182eaa795570776cd7 | 38da3fda0e0c5831f75c9113882369deb19a1e3e69ba415616e8773c74332e58 | 497 | inventory only |
| Cargo manifest/lock | vendor/bun/src/codegen/Cargo.toml | 39e028beec3ea2b7e040952cb42400a7cf5c4f47 | fb957a07a772740332be55128c0f0646bf3bfe9b9c197d08cc8351d2c34f9944 | 395 | inventory only |
| Cargo manifest/lock | vendor/bun/src/collections/Cargo.toml | 84b5e68c7ec1f3852e3e34b4c9d58d76133cbaaf | 78db5d8aaad7f73f792a5ddbf7535dd62dfab16623908cc01c269f2e82613223 | 1131 | inventory only |
| Cargo manifest/lock | vendor/bun/src/crash_handler/Cargo.toml | 9b09896eb937c0ff645fdd92b827a8287a1b5760 | e2d812d316ef11ab338694d29fe50c63b2a890cad261584a22765840ddccf2df | 829 | inventory only |
| Cargo manifest/lock | vendor/bun/src/csrf/Cargo.toml | 8854eaee37911f4bcd5b91ac282e2cbb28f20257 | 14b5f9e6a686730a139f61a828026699b7fd3afcdadad682537c9a0642c60516 | 485 | inventory only |
| Cargo manifest/lock | vendor/bun/src/css/Cargo.toml | 98d316dc8d2241b052c93dd155da2bcff75ff5e2 | 5b819aad98be5882ceef260b0fb33a8479f0709a1b83e053cfe46347792f900c | 706 | inventory only |
| Cargo manifest/lock | vendor/bun/src/css_derive/Cargo.toml | 51beeda0cf6c82ede6da4528c845159364a40b37 | 1c26ff0adeb285a19586b206881eb31862d5116706553b48abd7672c7c0125d0 | 258 | inventory only |
| Cargo manifest/lock | vendor/bun/src/css_jsc/Cargo.toml | 61e972db8d668fe7943d7150eebf846333dfa633 | beca30ac3869390040a945beb69b7cbc76920bf8413f6d89f8f714a1510539b3 | 621 | inventory only |
| Cargo manifest/lock | vendor/bun/src/dispatch/Cargo.toml | 3e9c3f8418165364dc661e0a090e61afa9bc707c | 9f40842bb3c70d6e643e687e8a7e142ee3b5887314e1bfde3e3a380ab7100c42 | 297 | inventory only |
| Cargo manifest/lock | vendor/bun/src/dns/Cargo.toml | 0853b51151f779a48a44a4d0677ec788cba62e87 | 7c14065924890ff1e48fe72aa35877f8b09c4220c25abbc5320c785e2d409625 | 573 | inventory only |
| Cargo manifest/lock | vendor/bun/src/dotenv/Cargo.toml | 0b770eb79613016091d0d13710f1d985098379e6 | b659feaf0d525642ee8203eb7fc6dc58d0607d46f1f8fc621f586186aaa980f8 | 641 | inventory only |
| Cargo manifest/lock | vendor/bun/src/errno/Cargo.toml | e8db343f3e881e1c49d219d811480387dcbc0df3 | 997dfd862750172f0ef91f02457ed7b2f0d53b9153d453fd2c6e6e199f0b9e8a | 485 | inventory only |
| Cargo manifest/lock | vendor/bun/src/event_loop/Cargo.toml | fa6e5b97f9bb390da125c293027eb0b6f313c47d | 676e9286b473079b46a257d297b8ae20d7d33af162b4613fb811feebff1b92d5 | 646 | inventory only |
| Cargo manifest/lock | vendor/bun/src/exe_format/Cargo.toml | 32cba7591be473b0ae23f659e5162b2468c283b7 | d4d504fc1da8a735d8bfc6ba995e944b6c8e0ddc441aceb4ea6ca495badfed22 | 522 | inventory only |
| Cargo manifest/lock | vendor/bun/src/glob/Cargo.toml | 378209d8ce88bba8e05bc432b34d8f0ef91f4a91 | df089f93b9613ef0b0d542ac7831cdc87c93f2c3d1f5f1ef6a57fefbe46c16ff | 504 | inventory only |
| Cargo manifest/lock | vendor/bun/src/hash/Cargo.toml | d5dfd0634a9e075576de62c9dc65041f1ff946a7 | 1679f6ef5a2f7f3213bb6eb5b4264b763c9bd22cb028b9e44d62b22ab650f681 | 345 | inventory only |
| Cargo manifest/lock | vendor/bun/src/highway/Cargo.toml | 75c0051e4c4345f39c30b83e3c5c68286dc623b6 | c7bb859840e9cd21263b662a63bb0f5b4701637ca1d58f600441c5319490707f | 370 | inventory only |
| Cargo manifest/lock | vendor/bun/src/http/Cargo.toml | f377d46929f44a775c6bd590f332f6415e84353f | 26529047a04ef3581bc19f53ed7e76c14fd3abe274eafe48d07522b48c8febd0 | 1120 | inventory only |
| Cargo manifest/lock | vendor/bun/src/http_jsc/Cargo.toml | 86b4df1a9e59643ecc9e2c39e93732be1749f24c | 0dc662935d26a7a3c0c494dfdfb36f2e8d7aba7c8f1044a2bcae0adf08648c3c | 938 | inventory only |
| Cargo manifest/lock | vendor/bun/src/http_types/Cargo.toml | ef89ebeb39c3abe2e6378940b00a244ec7a28d55 | 1b8df70208793e3638339b4efdd0a3b5f4df594ff6dfd089741fe4438bd97e94 | 498 | inventory only |
| Cargo manifest/lock | vendor/bun/src/ini/Cargo.toml | 7582d1cde0c3bea697fa40d2daaa170ff1984e3f | 91d31ed4c6d91b8293e936f86abe4b71a99b2b2f6103f3af5060bcb34e0e0e10 | 729 | inventory only |
| Cargo manifest/lock | vendor/bun/src/install/Cargo.toml | ac7b104ac76d1aa087381e6dce0115d5924150b8 | 673d2bfcbb78e796c45491612a6f88f9f1b17f70720cb019e6e1581556df2f52 | 2018 | inventory only |
| Cargo manifest/lock | vendor/bun/src/install/windows-shim/Cargo.toml | 2501884aee83d76ea8225d187b0dd2a002270804 | c606eefd65be8d598d97f04613c8c3e0617989e19b1e2370fca146fe197819e1 | 3305 | inventory only |
| Cargo manifest/lock | vendor/bun/src/install_jsc/Cargo.toml | caf3d8c51075df48641aa943937f0372d2263f65 | d05b57b87f8148fba693489c62f357c6b5d98ef9bbd1389c4d5f28cdc553115e | 752 | inventory only |
| Cargo manifest/lock | vendor/bun/src/install_types/Cargo.toml | f094c4f4226e810e8bb2ad5837335d0b023e219d | 830b4f01f721e35e710fd4dd3c9daba6b9c09d1103dbcc02b78f47ce57ce73a6 | 542 | inventory only |
| Cargo manifest/lock | vendor/bun/src/io/Cargo.toml | 57c340e9a97cd21a878e65f74a727e1aea7da505 | 92c26b768c8af51b09976d92622b9254632996534fc2cbaafdc7637134a89e13 | 676 | inventory only |
| Cargo manifest/lock | vendor/bun/src/js/Cargo.toml | 42bea0d01a21fde5fcd804a59a888e639103530f | 8796910bc09ed9bd0b483a8a8c072dcd0423c70f51e701704aff089549881071 | 365 | inventory only |
| Cargo manifest/lock | vendor/bun/src/js_parser/Cargo.toml | 03572a930389934cb239e71d37171029d46fbcf2 | d8561c9ca99960f848ba7ff5f3746f6341989fb7140b5fea217ca2224bd36613 | 994 | inventory only |
| Cargo manifest/lock | vendor/bun/src/js_parser_jsc/Cargo.toml | 21acebc58bab590045558fc80b585a5cc7075bf7 | d8c96cde6406cc7473aa69e025ed9f0048a6576f890bf94acaac4f6996a528bf | 799 | inventory only |
| Cargo manifest/lock | vendor/bun/src/js_printer/Cargo.toml | 52de92835066a1a5c08cc4496abc67af7bd57b4c | 6f6592ddb77ea28b75d77067627d55c3c3131a5f8554de0ef64970bd8c58ac40 | 782 | inventory only |
| Cargo manifest/lock | vendor/bun/src/jsc/Cargo.toml | 6b4013a535be771f75a644a69d05e19f2ee44eee | 75b3a24c28adb2d95a51efba7f29e5c641cdfa5ac2b0a19e855c7d2d009fdf60 | 1648 | inventory only |
| Cargo manifest/lock | vendor/bun/src/jsc_macros/Cargo.toml | b2cc1e6937b639a261f0950cec1568ee36589328 | fa639743f34412f74b81f234c8dc6f815ddca5f3c8d78f397135c11eb50ae339 | 258 | inventory only |
| Cargo manifest/lock | vendor/bun/src/libarchive/Cargo.toml | 9fb1f52d6012aa0d7b40db5e2c98d4b0f0a9c3c2 | 4ebf9c0bffff95ab454bac7db5b363bb22735bdfa07695bbcad2969d6aa517b3 | 554 | inventory only |
| Cargo manifest/lock | vendor/bun/src/libarchive_sys/Cargo.toml | 13526d460e88e7b276234bf1a8426f9c0aad761f | 2f0e6be170e988b5028b5aba97f06ea6c70fc46029260b4384559c09b8918aad | 457 | inventory only |
| Cargo manifest/lock | vendor/bun/src/libdeflate_sys/Cargo.toml | cdca715002fdef3d112698fa70a55adf28730ef6 | 75bbc17b14384112e85296c5a844078838a8d0d5f88610a392906b39f51c4f91 | 431 | inventory only |
| Cargo manifest/lock | vendor/bun/src/libuv_sys/Cargo.toml | 274409b43fc2cf2b5f7d77e257de20b27aeec9bc | 24aca3b74faeb4f2a207c6b44b3d54382e407e872702c8bdc4249647be8e0152 | 496 | inventory only |
| Cargo manifest/lock | vendor/bun/src/lolhtml_sys/Cargo.toml | 39e2f9543762a6dd1ed7969567f66709f57672fd | 836d93e1d7815dba309379a74b386c602e23be5801ef343b037a9031a80b39d2 | 1034 | inventory only |
| Cargo manifest/lock | vendor/bun/src/md/Cargo.toml | 4f14094fda0573898efe6cf2a5ab9cc9d256dd6a | e2d041a9c8f735577abe0f2642fde677b360e64b9cab3fb1873d320115980b1c | 527 | inventory only |
| Cargo manifest/lock | vendor/bun/src/meta/Cargo.toml | 68485af7e3cae14de46dc32d9f5f7da4513079c4 | 4a2b2b711fd01c572d186611aee4890db4262ef4fd69b486535cdb1fb143662a | 367 | inventory only |
| Cargo manifest/lock | vendor/bun/src/mimalloc_sys/Cargo.toml | 42f9fce98e090668cee0872241372971dea5b13c | c5f2e33e2a93eb3131b87398cca4fe3c4d9b6c7f5e6f307c97bc27dd6250749a | 403 | inventory only |
| Cargo manifest/lock | vendor/bun/src/node-fallbacks/Cargo.toml | 12d2ad17b0bee2ff79edcf2e057b8b023bca386a | a34dde35e40d2aa10617188f91518f8d0511984d61f5e0822e6cd12a132418de | 377 | inventory only |
| Cargo manifest/lock | vendor/bun/src/opaque/Cargo.toml | 41a473f4d0ca8b2e0b2d613df83beecb1c66c369 | 972521044a798b52d0e4080d6d30c1a5715b91b1a3e0b1e087f63cfee1f28826 | 377 | inventory only |
| Cargo manifest/lock | vendor/bun/src/options_types/Cargo.toml | 2e17f4e177b240f5f673def116f24624b11c60cf | 4919f18418551e692bfbc45ea7b5d7ebbbdb5054cc15203070de87a6f0754b0b | 740 | inventory only |
| Cargo manifest/lock | vendor/bun/src/output/Cargo.toml | 6d0bc0acf4de86fc041b0607c0d1adca7e9acd2a | db90b0b9a059bb664bbe46e2bd239018b3a87e7d56bfa9777ab3f1e6c66d6df7 | 199 | inventory only |
| Cargo manifest/lock | vendor/bun/src/parsers/Cargo.toml | cab7d1b12dccc062381fe9664c5c21d4ffecf3e5 | b7cc6cbd86180e06d809776294b275c93d190bcb7cde8f10d443a4496a0ef51b | 734 | inventory only |
| Cargo manifest/lock | vendor/bun/src/patch/Cargo.toml | c7ffc7c3fa985d48df285ebd40470538cd949aa1 | c02dae416adbfa658cd93852173b251f23a7e267c394a60708df74314a1ea18e | 673 | inventory only |
| Cargo manifest/lock | vendor/bun/src/patch_jsc/Cargo.toml | a18a07b2b68ec09590c7efef3824413556c429a5 | 116b0c75e91d9694d979b940e2082b03f8b06a93d79829ecc573981094a25d55 | 474 | inventory only |
| Cargo manifest/lock | vendor/bun/src/paths/Cargo.toml | 84994256df41f093ffb08e3a3bc4dfd7f3495d72 | 843b74bd308330836043da6881ab7193137a4fece9ed4e9684172f016047d2b5 | 463 | inventory only |
| Cargo manifest/lock | vendor/bun/src/perf/Cargo.toml | 2d19f0ae5c1cd90139baefca0555428269aca855 | a5fff96552044f154518b2dc450f05cce6d446851875caf0cb43e013935813c4 | 444 | inventory only |
| Cargo manifest/lock | vendor/bun/src/picohttp/Cargo.toml | a20291554a1b1f76f52fe63be976bc51c019363b | f7347b6cdc9b3c3477e698ee5391912b2da51accf12ae58ae87cda8c47266600 | 396 | inventory only |
| Cargo manifest/lock | vendor/bun/src/picohttp_sys/Cargo.toml | bb8d6709a0620776e808451bcf471d9beb713d4d | 9ab5fd2e3e66d63b846541252c3c6857d3432df3a7f018a40856838e33de95e9 | 375 | inventory only |
| Cargo manifest/lock | vendor/bun/src/platform/Cargo.toml | 96a229ff5fa5fb24c17182301e4ae92f38eafa70 | 09e9c67d8dd3ab7d9a64424092bfb86f60bb1a428bdcc41fd37026ccf67a3e51 | 479 | inventory only |
| Cargo manifest/lock | vendor/bun/src/ptr/Cargo.toml | 0dc9d888f6d177ca38afce2eda61b5f1f5c20a2c | 2fcd391978c313bccfc2fb7a588b3464a8435b188d1142c764bc08de47b9fab7 | 479 | inventory only |
| Cargo manifest/lock | vendor/bun/src/resolve_builtins/Cargo.toml | e5a297cefa5c81dd0c8289c284d2679678c0a82d | 5a2414be77504dabe1ab4185e62e46dafd4a044444d771eaa063d5a35dbecd16 | 598 | inventory only |
| Cargo manifest/lock | vendor/bun/src/resolver/Cargo.toml | c869c29b66074abc4599c926ab317d13d10e764a | dc503339322844793ba4bd6c2da66fa3abf16fbf257940926a3c5652c0bffef7 | 1179 | inventory only |
| Cargo manifest/lock | vendor/bun/src/router/Cargo.toml | b7c5a2b36d2dfc24ef5377033061d5c76dfe9542 | 7d75cddbd2a7953a20f85b4b78fcc4f6e693dce7c8c77e44b4e0ddb1fb1a8a5c | 997 | inventory only |
| Cargo manifest/lock | vendor/bun/src/runtime/Cargo.toml | b015296aec46ccc321dd4d8e0bea194d2a6d6cd8 | 4632e16154223695d9dcf859933b54d00b5a401d32daf5cca1d47501f56aef2f | 3490 | inventory only |
| Cargo manifest/lock | vendor/bun/src/s3_signing/Cargo.toml | fad7f814d2d0a26af97f885cc64f37a393be6e0a | b5e23c73c53e9818312b15e0400a77a6f772d531398d170dbc0c679e2f508fcd | 666 | inventory only |
| Cargo manifest/lock | vendor/bun/src/safety/Cargo.toml | b708da62e9e35fdbc6c50ac1ef951f64899fe7a4 | 3466e24b8a0a88c5be0ca26011b44e62591d57dc2697f93128da7f20b839ae49 | 449 | inventory only |
| Cargo manifest/lock | vendor/bun/src/semver/Cargo.toml | 2b68bdcf999c26dd0d6153a95f3db1a5951a0020 | 230d4bd1d5acc401b1e890334b370f0996e870aecdffc36bbfee3f262d38f696 | 482 | inventory only |
| Cargo manifest/lock | vendor/bun/src/semver_jsc/Cargo.toml | ecd9b2e95c0f3df25291ddbf5ba532a15d778f4c | 460842534edfa8f632765a467946e8dc30f166b38235acc38fbd68c6cd49503b | 451 | inventory only |
| Cargo manifest/lock | vendor/bun/src/sha_hmac/Cargo.toml | 963283da288fe38c32c8b9209ca26558062fe03b | c4d43034e81836440dda981481e2f11a13610c1c832681e552d2f1d62eddc6af | 490 | inventory only |
| Cargo manifest/lock | vendor/bun/src/shell_parser/Cargo.toml | 1d1105aa853c8370c6192abf6d75e118b729f6f0 | 7525e8cdb268bb7deef78c8a679d677a78de6ef334927bba26e8668fec841bb6 | 567 | inventory only |
| Cargo manifest/lock | vendor/bun/src/simdutf_sys/Cargo.toml | e09a634a53ccd66ba9fadcc133134319c377487f | bf81851102c1434227240d7640e094c9daed5d9c3ddc533ab78e24a8c5c4acd7 | 374 | inventory only |
| Cargo manifest/lock | vendor/bun/src/sourcemap/Cargo.toml | 4c0a9ec287ebb87249097c5b6eaff4ce56482060 | a5f6c33fbbdbf99cb82293f395c7f0780d289e63fb80304676c6a3844fd6cb8c | 1416 | inventory only |
| Cargo manifest/lock | vendor/bun/src/sourcemap_jsc/Cargo.toml | 5723496827a2d155dbff44f7b3bab507d9e3cd99 | 1db643b3410828c001d9975e1d48a4bb64f0d045ba67c8c11acd39e692886681 | 893 | inventory only |
| Cargo manifest/lock | vendor/bun/src/spawn/Cargo.toml | 32ffd720f02e5de1413c8f0f4b73e46cc0f915ab | 5f90f7fd53e400d5d3b965cff3ca8822681de08518f7c39778ec9fca204fc295 | 589 | inventory only |
| Cargo manifest/lock | vendor/bun/src/spawn_sys/Cargo.toml | cd70659b4c4ad792e99ad481649edc1b8ed0b29a | 1f5d546931e3976af143dd2ff4b7f871c0c1314cb22af90c7e47f1970e3ccf78 | 637 | inventory only |
| Cargo manifest/lock | vendor/bun/src/sql/Cargo.toml | eacb961ea39cab57c3418030828ba730f55518df | 291f739441596de0e777c69d9abbcee3e984e2c9b7868fad2b895528476b83d6 | 562 | inventory only |
| Cargo manifest/lock | vendor/bun/src/sql_jsc/Cargo.toml | 810e8974e9caf9b04b3b1ccc4cb259e3b7f47c86 | fa744c5f3fb9d29fbe9bc4da2a9aa94c1a2d560218f59e2a282c04cec94ffc58 | 1094 | inventory only |
| Cargo manifest/lock | vendor/bun/src/standalone_graph/Cargo.toml | c513f9133eff8fe7a4021f5ba3d545deed0a71c0 | 03cb6bd55412b982365089287310910bfdc46c6274b409f15e40d823a0e2c792 | 1030 | inventory only |
| Cargo manifest/lock | vendor/bun/src/sys/Cargo.toml | f46f74475d517be5d6ea5c2816cdb75e76ce3238 | 8bb6f1d69dde40876750ee8d4bd47ba00d6653fae0912a7635a41c196a54654a | 1315 | inventory only |
| Cargo manifest/lock | vendor/bun/src/sys_jsc/Cargo.toml | a3db730603add2747b43f77a2ac73d168b721b55 | a302eb9944351892d5f645948aabeaace27d77e930a128db4b8b5c55c45cc4e9 | 222 | inventory only |
| Cargo manifest/lock | vendor/bun/src/tcc_sys/Cargo.toml | 3e094ce4b09fac5b985e92a6005d04e8ce3d9e6b | 97cc2c2f4a0490576a327f73fcf888a137b1ac2e7bd73d246b77802442bb469e | 503 | inventory only |
| Cargo manifest/lock | vendor/bun/src/threading/Cargo.toml | de1fa83d357021fc01a70ba528a2edaf7144b618 | 5dcd12677669519a871d6eaced45edd236c28bc09ffce8de337678f3cde309a7 | 562 | inventory only |
| Cargo manifest/lock | vendor/bun/src/transpiler/Cargo.toml | 1250e0dc6443d91c8cf988c32eaeae24b5432d1b | 9a74587d5bf454a936901413eb338b241c77a73c545cf9837aea7a0d40be1adb | 703 | inventory only |
| Cargo manifest/lock | vendor/bun/src/unicode/Cargo.toml | 3f118b452e28cdc5a55972fe1b519f75cf7ca3b8 | 3718e898cae68b5d50c31106330a9e999eafba3c651cc72753086dbf58612213 | 456 | inventory only |
| Cargo manifest/lock | vendor/bun/src/url/Cargo.toml | 8dbdc99207761491c4531cfd9b957ff869ba4611 | 8373b21c0c0c624428fb671f56c63425e16fd68c3d319d4a60ea35f2204a185b | 616 | inventory only |
| Cargo manifest/lock | vendor/bun/src/url_jsc/Cargo.toml | 1dd541b2c109e450d27b94d10536cdd4b33737c3 | 7aac66b71c76ffc95c5ac6d295489dd697849eb5dc846f499f8d9a308c33ec39 | 445 | inventory only |
| Cargo manifest/lock | vendor/bun/src/uws/Cargo.toml | d6d21e9ee93aaad5f0736abe93682fecad1a6c97 | e86d4dc6287dcd1f6df295cc800698661401b09dbf6da2c347672c01072a7215 | 536 | inventory only |
| Cargo manifest/lock | vendor/bun/src/uws_sys/Cargo.toml | 03970ef9677502ab984a71076cefbb0e9d4d0b65 | 6228f4fa56a7e1c4f675f66a1bf311f5820faae49cb052ae4e073793e24f0214 | 752 | inventory only |
| Cargo manifest/lock | vendor/bun/src/valkey/Cargo.toml | 132b21038edcfdd5d1ae90f36d8999069eb6bd83 | f4549c93c922c7deba373a03d8a698cb5cefbda436790f9ea6c2a625cd03c57d | 373 | inventory only |
| Cargo manifest/lock | vendor/bun/src/watcher/Cargo.toml | 99851a2e6d7788a827e8f6e458afc2676913cf9d | 46731456ca04accbf7cdcc7b608913442f8dd078bd6a7e8a0e888f9d03949cac | 564 | inventory only |
| Cargo manifest/lock | vendor/bun/src/which/Cargo.toml | add069bac71160efc5ac6612ae616ff1b14a394f | abaa18ab092935dd80134b6099ade1fc02219cb1c2cc78c5d831b9e20a024c33 | 445 | inventory only |
| Cargo manifest/lock | vendor/bun/src/windows_sys/Cargo.toml | 094af833b25fbdbba573ff1e090d735033ab2ed1 | c034e32fe06a91b4f716a7dade9d1e1d552490f83d09e36b0a5a7586bf365a24 | 587 | inventory only |
| Cargo manifest/lock | vendor/bun/src/wyhash/Cargo.toml | 23f803ebc23ca63e091b9cdd478fcacf23916a24 | 6eb618956df17f9a9f9d5f4e96f4c8dd63f10f538f56ef8990389129d23b7736 | 369 | inventory only |
| Cargo manifest/lock | vendor/bun/src/zlib/Cargo.toml | f5c34bcd12566f95a6a0366fc9307f5d56395d7a | eef523d665c2759b50b249da2c752f796483c821fb5877c790e9571a18008eb7 | 506 | inventory only |
| Cargo manifest/lock | vendor/bun/src/zlib_sys/Cargo.toml | 1a30610dc9ebc79c9cc7479fed79dd230d775b83 | 8496f4b62b44274ab5bba11813f2bb3572a82bfe1fc1f3479e14dfa78422e35c | 397 | inventory only |
| Cargo manifest/lock | vendor/bun/src/zstd/Cargo.toml | 503ce1f4b6a369eca753ffca9da4ee4ae25c02b9 | 3458eccfbdec462f4186372c44bb5d542bc51f4fba021b83a52d727ce107a8e6 | 420 | inventory only |
| Cargo manifest/lock | vendor/bun/test/js/third_party/grpc-js/fixtures/tonic-server/Cargo.toml | 7f65079bb7c681be36f75d77a92b3d2d4c34e445 | db4c68f8fefa59d0737c0be70b7bcb9d57dc621fe833b1094c8f098ef53e333b | 294 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/Cargo.toml | fc00aa9f6df5564dc45209f46e996ac71749f9e8 | 209c719c1243629484f3c38b33c00c5314e7637399292f573a8e5ae3d4e734f6 | 1917 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/c-api/Cargo.lock | 7042fe84ad34937dc2102811894e4938fb4c827a | 02d28352293be00f05be457e59e60d5b9d7e84a4cdc43bd40236a12bf8d1e53d | 10039 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/c-api/Cargo.toml | 41ebfba337e9cf61168e6dca6cba08c955d8e128 | ad427ddf31c2581bca50477400d68cdd22fa2b0890a38ae30eaedcec9495cbec | 777 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/c-api/c-tests/Cargo.toml | f94f80b5dc9e3542f7893f542bc97946696e51b5 | 8d4536005f1d9035ae23354dd498b6d252a393c9a690993a2f85fe2b2a1319b1 | 307 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/fuzz/Cargo.toml | 84202578a9782736af5a776ab4f754c2c234648a | 31901c77fc76cbcfea474eb2e85378cc1c03917670d2adc75ed0686bc6e62a0a | 443 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/fuzz/afl/Cargo.toml | 35220c27cc5b880a9edd8ce86488181e974fd611 | ed6ef26206cc6c8b1f2b6a0a5c204d59454bd3ff1b14b66a79256c5f412ae7f1 | 184 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/fuzz/hongg/Cargo.toml | c7280a784e6ce1f84a9d339701625d5f6ae88f81 | 539e739bf85d7d9476e8d53fe51ee21950766c27e76221309557504a8bcf9acc | 187 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/fuzz/test_case/Cargo.toml | b4f98e678a9a85b202ce661ebc55f2b9f582fda2 | 4a70887149264eb14631018998404e6f83cbcf072b379996f361b9f97969a4d4 | 326 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/js-api/Cargo.lock | 71f2929667ea16e5e958849a9fe65c45bba0f671 | 4df540ef083dbb7c8d4e5de2cd481ba21a788ef288a9eb96344de5615ebb6dd1 | 12120 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/js-api/Cargo.toml | c040e2f5fe3e20b15267dfdf1e32ecbd589c887f | c141228fff8357bbeef2b077b3a737c8392fc0569da9a36e9cd795b083a7c68c | 703 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/tools/parser_trace/Cargo.toml | bf94aa6010110b3de77cd2d7e36e2a15f102a6fd | d7ffe271b5487d34585f38b6238b9256ca3d61a69c06b25f9a2c641a99701a37 | 253 | inventory only |
| Cargo manifest/lock | vendor/bun/vendor/lolhtml/tools/selectors_ast/Cargo.toml | 4bee8f302369f0ad8f20a78eacc47559468abe52 | 80c6f30ece8bef154121cfce552bf85955f124e58f2e4b53b8037dd251fb2e93 | 221 | inventory only |
| Cargo manifest/lock | wire/Cargo.toml | cb1c81510326e0ef159cbbef79de75295489e1dc | a0517ebfe3c61df2f6e2516aa39f8ca56851eb18d44e21f55a47ae2e8813dc8f | 125 | inventory only |
| license/notice | LICENSE | 261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64 | c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4 | 11357 | selected |
| license/notice | vendor/bun/LICENSE.md | 81069ee8d3b84f21ee32b2a9766643e1de114863 | 2c6160ec8fb853f7e8f97d9b249e756c9b0ac44860a68b6bf4f1b0bcbc5c3741 | 5376 | selected |
| license/notice | vendor/bun/docs/project/license.mdx | 6d44dd63e89131a5eb092d38a5a73075f52e5c13 | 7305efadd1f8222666b20e8887ae22509e43e6f1484c017853d49ec738214371 | 8208 | inventory only |
| license/notice | vendor/bun/packages/@types/bun/LICENSE | 9e841e7a26e4eb057b24511e7b92d42b257a80e5 | c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383 | 1141 | inventory only |
| license/notice | vendor/bun/packages/bun-usockets/LICENSE | 261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64 | c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4 | 11357 | inventory only |
| license/notice | vendor/bun/packages/bun-uws/LICENSE | 261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64 | c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4 | 11357 | inventory only |
| license/notice | vendor/bun/packages/bun-vscode/LICENSE | e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 | e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | inventory only |
| license/notice | vendor/bun/src/clap/LICENSE | cf1ab25da0349f84a3fdd40032f0ce99db813b8b | 88d9b4eb60579c191ec391ca04c16130572d7eedc4a86daa58bf28c6e14c9bcd | 1210 | selected |
| license/notice | vendor/bun/src/sql_jsc/postgres/protocol/notice_response_jsc.rs | ca2a5cff855909d2c65d45c90a5e46bb8b51d444 | 5f5405b3f2e29f7123907d91696518a45c5a207a57960d7d4b450afae33edead | 2447 | inventory only |
| license/notice | vendor/bun/src/sql_jsc/postgres/protocol/notice_response_jsc.zig | a0b85ba882dd6574bd226ad15e168ee6cb03823d | d7f8c7f6b11b5583639a9ef2ce3b6807ea848f8d2206d433bedd5baeab66de34 | 838 | inventory only |
| license/notice | vendor/bun/src/unicode/uucode_lib/LICENSE.md | 412454e31dfa9bac8c6ba8263cd3c49e87dfd1ce | 75b52b07e8f6ed6b1700ca6e4bcff93a59258624d4fd1ab7eae4c071c860b69b | 1298 | selected |
| license/notice | vendor/bun/test/cli/install/migration/contoso-test/LICENSE | b12fb48c88fcfa95ddf73ddceb6addb0897f33ed | 24341ac14899292f68659fbd52b6dd4453cd1b304f43ef9f9cc4e8aa77e81f51 | 1077 | inventory only |
| license/notice | vendor/bun/test/js/bun/jsc-stress/LICENSE | 5130d62d0b2bf84ae85d173cc44032cd777ad67c | cc86ad6977f79dc852527747611b1c5484ca367bd7c4359d92d7f0d5318211b5 | 1578 | inventory only |
| license/notice | vendor/bun/test/js/node/test/fixtures/postject-copy/node_modules/commander/LICENSE | 10f997ab104594695189c3fca8fa6c65ae9ccdd6 | 04512a63dce4d2d506ad612dc0bd7681ccf6e3655f7b6eaef7dfac8323d1ec0b | 1098 | inventory only |
| license/notice | vendor/bun/test/js/node/test/fixtures/postject-copy/node_modules/postject/LICENSE | 862f444f43150ad1b91afcfd2d5a5643ed8f83e2 | 6546657539feb5b454f400db317e71fa39ff5cd048fa5d78ae0ada2df72f1ea7 | 13409 | inventory only |
| license/notice | vendor/bun/test/js/node/test/fixtures/test426/LICENSE.md | 39501a3b7c70dd44a41ef3eb86c9a5deca2aa4ef | b44ad77dc900d7002f3b1cb3b3968adb404d013174e024305b2917b4f8927392 | 2228 | inventory only |
| license/notice | vendor/bun/vendor/lolhtml/LICENSE | 98b3bec0935e5c2539f70348d2151e1d9b7f00b3 | e4ddaa9d7391bb9536fcb8c59b570a8b85a0bf6da54df5b3b26f098f6f99c9cc | 1487 | selected |
