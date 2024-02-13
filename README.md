I am groot
==========

[Proof of Concept] Rust code generator from OpenRPC spec

[OpenRPC spec](https://spec.open-rpc.org/)

[JSON-RPC spec](https://www.jsonrpc.org/specification)

[StarkNet OpenRPC spec](https://github.com/starkware-libs/starknet-specs)

### Usage

* Generate the code and then run suggested example:

```
URL="https://starknet-mainnet.g.alchemy.com/v2/<snip>" RUST_LOG='gen::client=debug' cargo run --example gen
```

### TODO

1. [ ] `async` version of the `Rpc` trait and the client
   - using [`async_trait`](https://docs.rs/async-trait/latest/async_trait/)
1. [ ] Seamless inclusion into a build process
   - extract `-build` sub-crate for `[build-dependencies]`

#### Misc

Total lines of code: `find ./src -type f -name "*.rs" | xargs grep . | wc -l`
