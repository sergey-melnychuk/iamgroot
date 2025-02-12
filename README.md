I AM GROOT
==========

[Proof of Concept] Rust code generator from OpenRPC spec

[OpenRPC spec](https://spec.open-rpc.org/)

[JSON-RPC spec](https://www.jsonrpc.org/specification)

[StarkNet OpenRPC spec](https://github.com/starkware-libs/starknet-specs)

### Usage

* Generate the code and then run suggested example:

```
export KEY="..."
export URL="https://starknet-mainnet.g.alchemy.com/v2/$KEY"

./x.sh

RUST_LOG=debug cargo run --example demo

cargo run --example gen > data.json
```

```
## Sample call
curl \
-H 'Content-Type: application/json' \
-d '{"jsonrpc":"2.0","method":"starknet_getStateUpdate","params":[{"block_hash": "0x4684a9257747388a70848ccf222fd4c7e0bde27b84457e829ee48cac28ea21d"}],"id":1}' \
$URL
```

### TODO

1. [X] `async` version of the `Rpc` trait and the client
   - using [`async_trait`](https://docs.rs/async-trait/latest/async_trait/)
   - add an option to generate blocking client (use case: Beerus WebAssembly)
1. [X] Seamless inclusion into a build process
   - extract `-build` sub-crate for `[build-dependencies]`
   - see [iamgroot-demo](https://github.com/sergey-melnychuk/iamgroot-demo/tree/main/gen-build-proxy)
1. [X] Extract generic HTTP client trait and uncouple specific HTTP client dependencies

#### Misc

Total lines of code: `find ./src -type f -name "*.rs" | xargs grep . | wc -l`

---

**Why "I am groot"?**

Most people keep crafting hand-made RPC clients from formalized and machine-readable specs.

Such people don't want to make specs codegen-friendly and usable for reasons unknown.

Such people don't want to turn spec & client upgrades work from `O(N)` into `O(1)`.

Such people can't explain why do they do it ¯\\\_(ツ)_/¯.

Such people's excuses all sound like "I am groot" to me. That's why.
