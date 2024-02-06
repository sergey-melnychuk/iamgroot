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
cargo run --example eth
```

* Dump the AST (for debugging):

```
cargo run -- TREE ./api/starknet_api_openrpc.json ./api/starknet_write_api.json > tree.txt
```

* JSON-roundtrip and JSON-aware comparison with the input file (validate bindings):

```
### STARKNET

cargo run -- JSON ./api/starknet_api_openrpc.json 2>/dev/null | jq . > debug.json
diff <(jq --sort-keys . ./api/starknet_api_openrpc.json) <(jq --sort-keys . debug.json)

cargo run -- JSON ./api/starknet_trace_api_openrpc.json 2>/dev/null | jq . > debug.json
diff <(jq --sort-keys . ./api/starknet_trace_api_openrpc.json) <(jq --sort-keys . debug.json)

cargo run -- JSON ./api/starknet_write_api.json 2>/dev/null | jq . > debug.json
diff <(jq --sort-keys . ./api/starknet_write_api.json) <(jq --sort-keys . debug.json)
```

```
for FILE in api/*.json; 
do
echo "\n========== $FILE ==========:\n"
jq --sort-keys . $FILE > a.json
cargo run -- JSON $FILE 2>/dev/null | jq --sort-keys . > b.json
diff a.json b.json
;
done
rm a.json b.json
```

```
### ETHEREUM

jq --sort-keys . ./api/eth/ethereum.json > a.json
cargo run -- JSON ./api/eth/ethereum.json 2>/dev/null | jq  --sort-keys . > b.json
diff a.json b.json
rm a.json b.json
```

### TODO

1. [ ] `async` version of the `Rpc` trait
   - using [`async_trait`](https://docs.rs/async-trait/latest/async_trait/)
   - async client as well
1. [ ] Seamless inclusion into a build process
   - extract `-build` sub-crate for `[build-dependencies]`

#### Misc

Total lines of code: `find . -type f -name "*.rs" | xargs grep . | wc -l`
