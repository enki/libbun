# ProviderValue Canonical JSON Wire V1

Status: canonical current wire law and implementation roadmap. This document
does not claim that the parser correction, generated TypeScript runtime,
shared limits, duplicate-key admission, or libbun migration has landed.

Decision provenance: corrected Oracle/Fable review recorded on 2026-07-23 by
Oracle session `provider-wire-law-correction`, which reviewed source at
`ecbe8f791e122fc8d05a8ebb8639c7522b1de730`. The ruling was rechecked while
writing this record at repository HEAD
`f46699464ae55ad465c1373f82e8143322062a23`; current source still has the
missing implementation listed below.

This law is subordinate to
[Provider Execution And SDK Law](PROVIDER_EXECUTION_AND_SDK_LAW.md) for
installation, execution, authority, and settlement, and to [TSON](TSON.md) for
contract-shaped records, sums, and optionals. It owns only authored semantic
`ProviderValue` admission and its canonical JSON V1 transport/observation
projection.

## Decision

FrameV2 does **not** supersede V1. The canonical V1 law is:

- exact integers in `[-2^63, 2^64 - 1]` encode as bare JSON integers;
- every other exact integer encodes as
  `{"kind":"swarm.integer.v1","text":"<canonical decimal>"}`;
- finite, mathematically integral `f64`, including either zero sign, becomes
  exact `Integer`;
- finite non-integral `f64` encodes as an ordinary JSON number; and
- NaN and either infinity refuse. V1 has no binary64 bit/hex envelope.

The highest semantic owner is `swarm-provider-value-model`, with exact-number
semantics supplied by `swarmscript-number-model`. Serde, Contract-TSON,
generated TypeScript, Rust/static hosts, the native ABI/SDK, libbun, Mesh,
manifests, and DTOs are downstream consumers and cannot reinterpret this law.
The live source anchors are `integer_json_projection_value_v1`,
`provider_value_to_canonical_json_v1`,
`provider_value_from_canonical_json_v1`, and
`ProviderValue::number_from_f64_canonical_v1`.

Any future FrameV2 numeric law requires an explicitly negotiated version and
migration. No endpoint may infer V1 versus V2 from value shape. Generic ordered
object or sum envelopes are the wrong `ProviderValue` scope: ordinary object
order is not semantic, and closed sums belong to TSON.

## Exact Integer Law

Canonical bare admission is based on the JSON token's exact lexical integer,
before conversion to `serde_json::Value`. It is not based on JavaScript safety
and must never pass an oversized token through `f64`.

| Semantic integer | Exact decimal | Canonical V1 encoding | Correct V1 bare-token admission | Generated JS value |
| --- | ---: | --- | --- | --- |
| `-2^127` / `i128::MIN` | `-170141183460469231731687303715884105728` | wide envelope | refuse bare before serde | `bigint` |
| `i64::MIN - 1` | `-9223372036854775809` | wide envelope | refuse bare | `bigint` |
| `i64::MIN` | `-9223372036854775808` | bare | accept exactly | `bigint` |
| first below the JS-safe interval | `-9007199254740992` | bare | accept exactly | `bigint` |
| next value | `-9007199254740993` | bare | accept exactly | `bigint` |
| JS-safe minimum | `-9007199254740991` | bare | accept exactly | `number` |
| zero | `0` | `0` | accept | `number` |
| JS-safe maximum | `9007199254740991` | bare | accept exactly | `number` |
| first above the JS-safe interval | `9007199254740992` | bare | accept exactly | `bigint` |
| next value | `9007199254740993` | bare | accept exactly | `bigint` |
| `i64::MAX` | `9223372036854775807` | bare | accept exactly | `bigint` |
| `i64::MAX + 1` | `9223372036854775808` | bare through the unsigned lane | accept exactly | `bigint` |
| `u64::MAX` | `18446744073709551615` | bare | accept exactly | `bigint` |
| `2^64` | `18446744073709551616` | wide envelope | refuse bare | `bigint` |
| `i128::MAX` | `170141183460469231731687303715884105727` | wide envelope | refuse bare | `bigint` |
| `u128::MAX` | `340282366920938463463374607431768211455` | wide envelope | refuse bare | `bigint` |
| any wider exact integer | canonical decimal | wide envelope | refuse bare | `bigint` |

Bare `-0`, `-0.0`, `-0e0`, and equivalent zero spellings canonicalize to
`Integer(0)` and re-encode as `0`; signed zero is not a semantic distinction.
Envelope text `"0"` is forbidden because in-range integers must be bare, and
`"-0"` is not canonical decimal.

The envelope has exactly two fields, `kind` and `text`. Missing, extra,
duplicate, or wrongly typed fields refuse. Its decimal text refuses a leading
`+`, leading zeroes, whitespace, exponent or decimal-point syntax, junk, and
in-range values. An authored ordinary object with reserved kind
`swarm.integer.v1` also refuses. There is one representation per exact integer;
no compatibility reader may accept both an in-range envelope and bare form or
an oversized bare integer and envelope.

## Exact Float Law

| Input | Semantic and wire result |
| --- | --- |
| finite and mathematically integral | exact `Integer`, then the integer law above |
| `+0.0` or `-0.0` | `Integer(0)`, canonical text `0` |
| finite and non-integral | `Number(FiniteProviderNumber)`, ordinary JSON number |
| finite subnormal | admitted as an ordinary JSON number |
| NaN, `+Infinity`, or `-Infinity` | typed refusal |

Valid fraction/exponent spellings enter the binary64 lane. If their parsed
binary64 value is integral, they canonicalize to `Integer`; otherwise canonical
output is the owner-selected shortest binary64 round-trip spelling, not the
input spelling. A nonzero decimal that underflows to binary64 zero consequently
canonicalizes to `Integer(0)`; a spelling that overflows binary64 refuses rather
than producing infinity. V1 preserves neither signed-zero bits nor NaN payloads
and has no float bit/hex envelope.

Current public construction can still create an integral
`FiniteProviderNumber` directly even though the canonical numeric mint would
make an `Integer`. Hardening that construction or validation is owner follow-up
work after the lexical integer correction; it is not a reason to change V1.

## Structural Scope

| Concern | ProviderValue V1 law | TSON/generated-binding scope |
| --- | --- | --- |
| objects | `BTreeMap<String, ProviderValue>`; insertion order is not semantic; canonical Rust text uses Rust string order | generated TypeScript must produce the same order by UTF-8 bytes, not default UTF-16 `.sort()` |
| duplicate keys | typed refusal before map construction, including escape-equivalent duplicates | schema bindings may impose stronger exact-field rules |
| arrays | order and length are semantic | holes and sparse arrays refuse |
| sums | no generic sum envelope; tag-shaped objects are ordinary authored data | exact closed sums and their tags belong to TSON |
| optional/null | omitted key and present `null` differ; generic ProviderValue has no `undefined` | schema optional may map `undefined` to omission, never silently to `null` |
| strings | exact Rust strings; V1 does not normalize NFC | codecs cannot normalize independently; lone UTF-16 surrogates refuse |
| bytes | exact semantic byte sequence with no canonical JSON V1 representation | no consumer may invent base64 or hex |

Canonical JSON text is transport/observation, not provider selection, route,
contract, manifest, identity, receipt, settlement, or execution authority.
Generated code may validate and mechanically translate values only.

## Owner Product And Current Missing Work

The owner-issued V1 product is one Rust implementation, one mechanically
generated TypeScript/libbun runtime, and one shared hostile-vector corpus:

```text
crates/swarm-provider-value-model/src/json_wire_v1.rs
crates/swarm-provider-value-model/generated/provider-value-json-wire-v1.generated.ts
crates/swarm-provider-value-model/tests/vectors/provider-value-json-wire-v1.jsonl
```

The generated TypeScript surface uses `number | bigint`: safe admitted
integers may surface as `number`, and every other admitted exact integer as
`bigint`. It must parse bare integers lexically rather than through
`JSON.parse`/`Number`, encode `bigint` using the same V1 threshold, format
non-integral binary64 identically to Rust, sort keys by UTF-8 bytes, reject
duplicates, cycles, sparse arrays, accessors requiring execution, `undefined`,
symbols, functions, nonfinite numbers, malformed envelopes, unsupported bytes,
and resource-limit violations. It remains adapter-private and is not exported
as raw wire authority from `@swarm/provider`.

Its adapter-private public shape is mechanically equivalent to:

```ts
type ProviderIntegerV1 = number | bigint;
type ProviderJsonValueV1 =
  | null
  | boolean
  | string
  | ProviderIntegerV1
  | readonly ProviderJsonValueV1[]
  | { readonly [key: string]: ProviderJsonValueV1 };

decodeProviderValueJsonWireV1(text: string): ProviderJsonValueV1;
encodeProviderValueJsonWireV1(value: ProviderJsonValueV1): string;
```

Failures are typed `ProviderValueWireFaultV1` values with stable fault codes,
path/offset where applicable, and bounded input observation. They are not
authored `{ kind: "err" }` settlement cargo.

The owner must also issue typed wire faults retaining bounded input observation
for syntax, resource limits, duplicate keys, oversized bare integers,
malformed wide projections, nonfinite numbers, reserved carrier objects, and
unsupported semantic values. Observation text cannot become authority.

Exact shared byte, output, depth, node, member, item, string, and integer-digit
limits are not yet recorded in source. Rust and TypeScript must compile the
same owner-issued constants; serde, Bun, libbun, and individual consumers may
not select their own defaults. This document deliberately does not invent the
numbers.

At the reviewed source and at the documentation HEAD named above:

- `provider_value_from_canonical_json_v1` still invokes
  `serde_json::from_str` before exact integer-token admission, so oversized
  bare integer lexemes can be rounded through `f64` instead of refusing;
- serde-first object decoding cannot detect duplicate keys;
- the shared limits, generated runtime, and vector corpus do not exist;
- the external libbun path still translates canonical text through
  `serde_json::Value`/structural values rather than preserving lexical text;
  and
- integral direct `FiniteProviderNumber` construction remains possible.

These are implementation gaps, not alternate laws.

## First Edit, Current WAIT, And Dependent Adoption

The first compile-coherent edit is owner-private lexical JSON admission in
`crates/swarm-provider-value-model/src/lib.rs`, at
`provider_value_from_canonical_json_v1`, before
`serde_json::from_str`. A grammar-aware private parser/preflight must classify
each integer token exactly: negative magnitude through `2^63`, nonnegative
through `2^64 - 1`, otherwise a typed
`BareIntegerRequiresWideProjection` retaining byte offset, token length, and a
bounded exact token observation. A regex scan, public token carrier, or
post-serde range check does not satisfy the law.

The focused red/green proof covers `u64::MAX`, `2^64`, `i64::MIN`,
`i64::MIN - 1`, and `9007199254740993`, proving both exact non-JS-safe bare
admission and pre-serde oversized refusal.

The current provider-value wire WAIT has exactly two ordered steps:

1. `swarm-provider-value-model` fails closed on oversized bare integer lexemes
   before serde conversion while retaining exact admission of the full
   i64/u64 bare interval.
2. The same owner issues the fixed TypeScript/libbun V1 runtime, exact shared
   limits, duplicate-key admission, and hostile vectors; the external-provider
   owner then carries canonical text as `StructuralValue(String)` into and out
   of that runtime rather than carrying unsafe structural numbers.

Contract-TSON optional, null, record, or closed-sum generation is not a
prerequisite and is not part of this WAIT lane. TSON owns those schema meanings;
that semantic scope statement does not add current implementation work here.

After the two owner steps exist, dependent adoption proceeds without changing
the wire:

1. Rust static providers continue to carry typed `ProviderValue` directly;
   generated Rust bindings add no JSON round trip.
2. Native provider ABI/SDK retains the V1 C layout and raw-byte ownership, adds
   only owner-codec conveniences, and keeps parsing out of providers/macros.
3. Mesh keeps its distinct framing, delegates numeric admission to the owner,
   and stops directly minting integral `Number` values.
4. Consumer-local thresholds, parsers, serde bridges, duplicate vectors,
   alternate numeric fixtures, and public raw codec/projection surfaces are
   deleted after their consumers move.

Python, Go, WASM, and other hosts are future compatibility consumers only.
They do not delay this Rust/TypeScript/libbun sequence and may not establish
their own wire profile.

The fail-closed V1 parser correction changes acceptance of malformed or
noncanonical input, not canonical emitted bytes. It requires no ProviderValue
wire bump and no native ABI bump. Publishing generated TypeScript types that
change `number` to `number | bigint` requires the applicable binding/package
version bump. FrameV2 would require explicit wire negotiation; changing a
native V1 function pointer's meaning to V2 would require a new ABI entrypoint.

## Acceptance Matrix

| Owner | Required hostile evidence |
| --- | --- |
| `swarm-provider-value-model` | every integer boundary above; malformed and dual envelopes; exact lexical attacks; duplicate and escape-equivalent keys; negative zero; exponent spellings; subnormals; random arbitrary integers and finite-f64 bit vectors; object order; shared depth/size limits |
| generated TypeScript / `packages/provider` | `number | bigint`, cycles, sparse arrays, lone surrogates, UTF-8 key order, nonfinite numbers, hostile envelopes, and the owner vector corpus without authoritative `JSON.parse`, `parseInt`, or `parseFloat` shortcuts |
| external-provider owner/libbun | Rust encode -> Bun decode -> Bun encode -> Rust decode and the reverse through actual canonical-text string carriage |
| Rust static host | typed binding parity without JSON reserialization |
| native SDK/loader | the same V1 bytes/vectors and typed host refusal for malformed output, with unchanged ABI V1 layout |
| Mesh | owner canonical numeric mint, explicit outer bytes framing, and no claim that the mesh DTO is ProviderValue JSON V1 |

Acceptance searches must leave no provider-payload serde structural bridge in
the external libbun owner; no consumer-local V1 threshold/decimal parser in
external, static, native, or Mesh consumers; no live V1 `FrameV2`, bit-hex
float, ordered-object, or generic-sum implementation; and `swarm.integer.v1`
only in the owner, its generated runtime/vectors, and explicit conformance
tests.

## Forbidden Forks

Consumers must not add a JS-safe bare-integer threshold, accept oversized bare
integers, wrap in-range integers, introduce float bit envelopes, preserve
signed-zero bits, normalize strings, invent byte or sum envelopes, choose
consumer-local limits, parse then guess, accept ambiguous dual shapes, or turn
wire DTOs into authority. Rust/native, libbun/TypeScript, Mesh, WASM, Python,
and Go all consume the same owner-issued V1 law.
