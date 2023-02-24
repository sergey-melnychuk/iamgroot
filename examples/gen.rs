use openrpc_stub_gen::jsonrpc;

struct State;

fn main() {
    let state = State;

    let json = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "starknet_blockNumber",
        "params": [],
    });

    let req: jsonrpc::Request = serde_json::from_value(json).unwrap();
    let req = req.with_id(jsonrpc::Id::Number(1));
    println!(">>> {}", serde_json::to_string(&req).unwrap());
    let ret = gen::handle(&state, req);
    println!("<<< {}", serde_json::to_string(&ret).unwrap());
}

// #[allow(unused_variables)]
// impl gen::Rpc for State {}

// NOTE: Generated code will be added below this line
