use std::cell::RefCell;

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

    let req: openrpc_stub_gen::jsonrpc::Request = serde_json::from_value(json).unwrap();
    let ret = handle(
        &state,
        req.with_id(openrpc_stub_gen::jsonrpc::Id::Number(42)),
    );
    println!("<<< {}", serde_json::to_string(&ret).unwrap());

    let req = openrpc_stub_gen::jsonrpc::Request::new(
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
    let ret = handle(&state, req);
    println!("<<< {}", serde_json::to_string(&ret).unwrap());
}

// (git restore examples/gen.rs)
// cargo run --release -- ./api/input.openrpc CODE >> examples/gen.rs
// cargo run --example gen

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

// TODO Uncomment line below and update 'call' method as above
// impl Rpc for State {}

// NOTE: Generated code will be added below this line
