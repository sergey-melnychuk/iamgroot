I am groot
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
1. [ ] Seamless inclusion into a build process
   - extract `-build` sub-crate for `[build-dependencies]`

#### Misc

Total lines of code: `find ./src -type f -name "*.rs" | xargs grep . | wc -l`
