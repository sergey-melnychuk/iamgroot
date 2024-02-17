#!/bin/sh
rm examples/gen.rs
cp examples/gen.txt examples/gen.rs
cargo run -- CODE \
    ./api/starknet_query_api_openrpc.json \
    ./api/starknet_write_api_openrpc.json \
    ./api/starknet_trace_api_openrpc.json \
    --async --client >> examples/gen.rs

rm examples/demo.rs
cp examples/demo.txt examples/demo.rs
cargo run -- CODE \
    ./api/starknet_query_api_openrpc.json \
    ./api/starknet_write_api_openrpc.json \
    ./api/starknet_trace_api_openrpc.json \
    --client >> examples/demo.rs

rm examples/server.rs
cp examples/server.txt examples/server.rs
cargo run -- CODE \
    ./api/starknet_query_api_openrpc.json \
    ./api/starknet_write_api_openrpc.json \
    ./api/starknet_trace_api_openrpc.json \
    --async --client >> examples/server.rs

# rm examples/eth.rs
# cp examples/eth.txt examples/eth.rs
# cargo run -- CODE ./api/eth/ethereum.json >> examples/eth.rs

cargo fmt
cargo build --tests --examples

## Run 'demo' example (blocking)
RUST_LOG=debug cargo run --example demo

## Run 'gen' example (async)
URL="${URL}" cargo run --example gen
