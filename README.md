openrpc-stub-gen
================

!WIP!

[OpenRPC spec](https://spec.open-rpc.org/)

[JSON-RPC spec](https://www.jsonrpc.org/specification)

[StarkNet OpenRPC spec](https://github.com/starkware-libs/starknet-specs)

JSON-roundtrip and JSON-aware comparison with the input file:

```
cargo run --release -- ./api/input.openrpc JSON 2>/dev/null | jq . > debug.json
diff <(jq --sort-keys . ./api/input.openrpc) <(jq --sort-keys . debug.json)
```

Dump the AST:

```
cargo run --release -- ./api/input.openrpc TREE > tree.txt 2> debug.txt
```

Generate the code and then run it:

```
git restore examples/gen.rs
cargo run --release -- ./api/input.openrpc CODE >> examples/gen.rs
cargo run --example gen
```
