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
cargo run --release -- TREE ./api/test/input.openrpc > tree.txt 2> debug.txt
```

Generate the code and then run it:

```
git restore examples/gen.rs
cargo run --release -- CODE ./api/test/input.openrpc >> examples/gen.rs
cargo run --example gen
```

The `gen` example output:

```
>>> {"id":42,"jsonrpc":"2.0","method":"starknet_call","params":{"block_id":"0xFF","request":{"calldata":["2"],"contract_address":"1","entry_point_selector":"3"}}}
<<< {"jsonrpc":"2.0","error":{"code":-42,"message":"Not implemented"},"id":42}

>>> {"jsonrpc":"2.0","method":"starknet_call","params":[{"calldata":["2"],"contract_address":"1","entry_point_selector":"3"},"0xFF"]}
<<< {"jsonrpc":"2.0","result":["x=2"]}
```

Total lines of code (1134 clean / 2497 full): `find . -type f -name "*.rs" | xargs grep . | wc -l`

### Plans:

1. [ ] Error enum/constants
1. [ ] merge multiple spec files into single consistent spec
   - [ ] cross-file lookup for a schema
   - [ ] cross-file lookup for an error
1. [ ] validate JSON to be valid OpenRPC spec
1. [ ] `async` version of trait & handlers
   - would require `async_trait` on stable rust: [async_fn_in_trait](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html)
1. [ ] HTTP-based server generation
1. [ ] HTTP-based client generation
1. [ ] Seamless inclusion into a build process
