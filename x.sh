#!/bin/sh
rm examples/gen.rs
cp examples/gen.txt examples/gen.rs
cargo run -- CODE \
    ./api/starknet_query_api_openrpc.json \
    ./api/starknet_write_api_openrpc.json \
    ./api/starknet_trace_api_openrpc.json >> examples/gen.rs

# rm examples/eth.rs
# cp examples/eth.txt examples/eth.rs
# cargo run -- CODE ./api/eth/ethereum.json >> examples/eth.rs

cargo fmt
cargo run --example gen
