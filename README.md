openrpc-stub-gen
================

!WIP!

[OpenRPC spec](https://spec.open-rpc.org/)

[JSON-RPC spec](https://www.jsonrpc.org/specification)

[StarkNet OpenRPC spec](https://github.com/starkware-libs/starknet-specs)

NOTE: The [fix](https://github.com/starkware-libs/starknet-specs/pull/56) is necessary to make existing spec a valid OpenRPC spec.

JSON-roundtrip and JSON-aware comparison with the input file:

```
cargo run --release -- JSON ./api/test/input.openrpc 2>/dev/null | jq . > debug.json
diff <(jq --sort-keys . ./api/test/input.openrpc) <(jq --sort-keys . debug.json)
```

Dump the AST:

```
cargo run --release -- TREE ./api/starknet_api_openrpc.json ./api/starknet_write_api.json > tree.txt
```

Generate the code and then run it:

```
git restore examples/gen.rs
cargo run --release -- CODE ./api/starknet_api_openrpc.json ./api/starknet_write_api.json >> examples/gen.rs
cargo run --example gen
```

Total lines of code (1105): `find . -type f -name "*.rs" | xargs grep . | wc -l`

### TODO

1. [X] wrap non-required properties with `Option<_>`
1. [X] wrap non-required arguments with `Option<_>`
1. [ ] add `#[serde(flatten)]` for reusable chunks included via `allOf`
1. [ ] error enum/constants
1. [ ] verify each method against pathfinder
1. [ ] `async` version of trait & handlers (?)
   - would require `async_trait` on stable rust: [`async_fn_in_trait`](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html)
1. [ ] HTTP-based server generation (?)
1. [ ] HTTP-based client generation (?)
1. [ ] Seamless inclusion into a build process
   - publish as a library/binary on crates.io (?)

### DONE

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
