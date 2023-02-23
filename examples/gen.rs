use std::cell::RefCell;

use openrpc_stub_gen::jsonrpc;

struct State(RefCell<u64>);

fn main() {
    let state = State(RefCell::new(0));

    let json = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "starknet_call",
        "params": {
            "request": {
                "contract_address": "1",
                "calldata": ["2"],
                "entry_point_selector": "3"
            },
            "block_id": "0xFF"
        },
        "id": 42
    });
    println!(">>> {}", json.to_string());

    let req: jsonrpc::Request = serde_json::from_value(json).unwrap();
    let ret = gen::handle(&state, req.with_id(jsonrpc::Id::Number(42)));
    println!("<<< {}", serde_json::to_string(&ret).unwrap());

    let req = jsonrpc::Request::new(
        "starknet_call".to_string(),
        serde_json::json!([
            {
                "contract_address": "1",
                "calldata": ["2"],
                "entry_point_selector": "3"
            },
            "0xFF"
        ]),
    );
    println!("\n>>> {}", serde_json::to_string(&req).unwrap());
    let ret = gen::handle(&state, req);
    println!("<<< {}", serde_json::to_string(&ret).unwrap());
}

// 0. (git restore examples/gen.rs)
// 1. cargo run --release -- ./api/input.openrpc CODE >> examples/gen.rs
// 2. implement gen::Rpc for State and 'starknet_call' method (see below)
// 2. cargo run --example gen

// starknet_call:
/*
        let mut x = self.0.borrow_mut();
        *x += 1;

        if *x % 2 == 0 {
            Ok(vec![format!("x={x}")])
        } else {
            Err(jsonrpc::Error::new(-42, "Not implemented".to_string()))
        }
*/

#[allow(unused_variables)]
impl gen::Rpc for State {
    // TODO
}

// NOTE: Generated code will be added below this line
