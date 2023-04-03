I am groot
==========

[Proof of Concept] Rust code generator from OpenRPC spec

[OpenRPC spec](https://spec.open-rpc.org/)

[JSON-RPC spec](https://www.jsonrpc.org/specification)

[StarkNet OpenRPC spec](https://github.com/starkware-libs/starknet-specs)

### Usage

* Generate the code and then run suggested example:

```
# git restore examples/gen.rs
# manually remove `mod gen` from the examples/gen.rs (generated code is commited now)
cargo run -- CODE ./api/starknet_api_openrpc.json ./api/starknet_write_api.json ./api/starknet_trace_api_openrpc.json >> examples/gen.rs && cargo fmt
cargo run --example gen
```

```
cargo run -- CODE ./api/eth.json >> examples/eth.rs && cargo fmt
```

* Dump the AST (for debugging):

```
cargo run -- TREE ./api/starknet_api_openrpc.json ./api/starknet_write_api.json > tree.txt
```

* JSON-roundtrip and JSON-aware comparison with the input file (validate bindings):

```
cargo run -- JSON ./api/test/input.openrpc 2>/dev/null | jq . > debug.json
diff <(jq --sort-keys . ./api/test/input.openrpc) <(jq --sort-keys . debug.json)
```

### TODO

1. [ ] (eth) Handle "anonymous enums"
   - params (#/components/contentDescriptors/Filter:address)
   - result (#/methods{name='eth_syncing'}:syncing)
   - try "title"? (make mandatory)
1. [ ] (eth) Add module-wise `Null` constant? ((de)serialized as `null`)
1. [ ] validation of `schema.{minimum, maximum}`
1. [ ] `async` version of trait & handlers
   - would require `async_trait` on stable rust: [`async_fn_in_trait`](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html)
1. [ ] Seamless inclusion into a build process
   - extract `-build` sub-crate for `[build-dependencies]`

### DONE

* [x] consistent ordering of generated structs
* [x] validation (against `schema.pattern` to start with)
  - for primitive types: generate named value-object wrapper when validation is necessary
  - (OK) `impl TryFrom<T>` + `#[serde(try_from = "T")]` (see example `val`)
  - (NO) generate custom (de)serializers for such properties/types?
  - (NO) use [serde_valid](https://docs.rs/serde_valid/latest/serde_valid/)?
* [x] value-objects wrappers (`Felt`, `NumAsHex` etc)
* [x] add working example for each `starknet_*` method
* [x] align errors with the [spec](https://www.jsonrpc.org/specification#error_object)
* [x] extract name-conflict-aware cache
* [x] error enum/constants
* [x] wrap non-required properties with `Option<_>`
* [x] wrap non-required arguments with `Option<_>`
* [x] add `#[serde(flatten)]` for reusable chunks included via `allOf`
* [x] resolve naming collisions (might required slightly patching the specs)
  - e.g. `starknet_getStateUpdate.result` vs `starknet_getBlockWithTxs.result`
* [x] use value-objects instead of type aliases
  - value-objects without validation do not make much sense (e.g. match against regex)
  - library (vs. framework) approach is to avoid making decisions for the client code
  - is client code allowed to send "invalid" data?
  - is client code allowed to receive "invalid" data?
  - up to client code - thus no validation (at least out of the box, at least for now)
* [x] run against most recent Starknet specs
* [x] support multiple files with specs
* [x] generate the `Rpc` trait
* [x] generate `handle` function of method handlers
* [x] provide OpenRPC and JSON-RPC bindings
* [x] add batch support by the [spec](https://www.jsonrpc.org/specification#batch)
  - out of scope: can be easily supported on web-framework level
* [x] impl notifications by the [spec](https://www.jsonrpc.org/specification#notification)
  - out of scope: on web-framework level just don't send the response
* [x] HTTP-based server generation (?)
  - nope: can be easily built around `trait Rpc` impl
* [x] HTTP-based client generation (?)
  - nope: can be easily built around `trait Rpc` impl

#### Misc

Total lines of code: `find . -type f -name "*.rs" | xargs grep . | wc -l`
