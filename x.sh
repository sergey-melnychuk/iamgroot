#!/bin/sh
rm examples/x.rs

cp examples/gen.rs examples/x.rs
cargo run -- CODE \
    ./api/starknet_query_api_openrpc.json \
    ./api/starknet_write_api_openrpc.json \
    ./api/starknet_trace_api_openrpc.json >> examples/x.rs

# cp examples/eth.rs examples/x.rs
# cargo run -- CODE ./api/eth/ethereum.json >> examples/x.rs

cargo fmt
cargo run --example x
