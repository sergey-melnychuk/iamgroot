#!/bin/sh
rm examples/gen.rs
cp examples/gen.txt examples/gen.rs
RUST_LOG=off cargo run -- CODE \
    ./api/0.6.0/starknet_query_api_openrpc.json \
    ./api/0.6.0/starknet_write_api_openrpc.json \
    ./api/0.6.0/starknet_trace_api_openrpc.json \
    ./api/pathfinder.json \
    --async --client >> examples/gen.rs

rm examples/demo.rs
cp examples/demo.txt examples/demo.rs
RUST_LOG=off cargo run -- CODE \
    ./api/0.6.0/starknet_query_api_openrpc.json \
    ./api/0.6.0/starknet_write_api_openrpc.json \
    ./api/0.6.0/starknet_trace_api_openrpc.json \
    ./api/pathfinder.json \
    --blocking --client >> examples/demo.rs

rm examples/proxy.rs
cp examples/proxy.txt examples/proxy.rs
RUST_LOG=off cargo run -- CODE \
    ./api/0.6.0/starknet_query_api_openrpc.json \
    ./api/0.6.0/starknet_write_api_openrpc.json \
    ./api/0.6.0/starknet_trace_api_openrpc.json \
    ./api/pathfinder.json \
    --async --client >> examples/proxy.rs

# rm examples/eth.rs
# cp examples/eth.txt examples/eth.rs
# cargo run -- CODE ./api/0.6.0/eth/ethereum.json >> examples/eth.rs

cargo fmt
cargo build --tests --examples

## Run 'demo' example (blocking)
RUST_LOG=debug cargo run --example demo

## Run 'gen' example (async)
URL="${URL}" cargo run --example gen
