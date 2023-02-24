use openrpc_stub_gen::jsonrpc;

struct State;

fn main() {
    let state = State;

    call(
        &state,
        1,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_blockNumber",
            "params": [],
        }),
    );

    call(
        &state,
        2,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_call",
            "params": [
                {
                    "entry_point_selector": "uno",
                    "calldata": ["due"],
                    "contract_address": "tre"
                },
                42
            ]
        }),
    );

    call(
        &state,
        3,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_call",
            "params": {
                "request": {
                    "entry_point_selector": "one",
                    "calldata": ["two"],
                    "contract_address": "three"
                },
                "block_id": "hi"
            }
        }),
    );
}

fn call<T: gen::Rpc>(rpc: &T, id: i64, json: serde_json::Value) {
    let req: jsonrpc::Request = serde_json::from_value(json).unwrap();
    let req = req.with_id(jsonrpc::Id::Number(id));
    let json = serde_json::to_string(&req).unwrap();
    println!("\n>>> {}", json);
    let req: jsonrpc::Request = serde_json::from_str(&json).unwrap();
    let ret = gen::handle(rpc, req);
    println!("<<< {}", serde_json::to_string(&ret).unwrap());
}

// #[allow(unused_variables)]
// impl gen::Rpc for State {}

// NOTE: Generated code will be added below this line
