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

    /* 
    fn getStateUpdate(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::StarknetGetStateUpdateResult, jsonrpc::Error> {
        Ok(gen::StarknetGetStateUpdateResult::StateUpdate(gen::StateUpdate {
            new_root: "0xcafebabe".to_string(),
            block_hash: "0xdeadbeef".to_string(),
            pending_state_update: gen::PendingStateUpdate {
                state_diff: gen::StateDiff {
                    nonces: vec![
                        gen::NoncesItem {
                            nonce: Some("nonce-0".to_string()),
                            contract_address: Some("addr-0".to_string()),
                        }
                    ],
                    deprecated_declared_contract_hashes: Some(vec!["contract-0".to_string()]),
                    deployed_contracts: vec![gen::DeployedContractItem {
                        address: "addr-1".to_string(),
                        class_hash: "hash-0".to_string(),
                    }],
                    declared_contract_hashes: vec![gen::DeclaredContractHashesItem {
                        compiled_class_hash: Some("hash-1".to_string()),
                        class_hash: Some("hash-2".to_string())
                    }],
                    storage_diffs: vec![gen::ContractStorageDiffItem {
                        address: "addr-2".to_string(),
                        storage_entries: vec![gen::StorageEntriesItem {
                            key: Some("key-1".to_string()),
                            value: Some("val-1".to_string()),
                        }]
                    }],
                },
                old_root: "0xFACE".to_string(),
            }
        }))
    }
    */
    call(
        &state,
        4,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getStateUpdate",
            "params": 42
        }),
    );

    /*
    fn getEvents(
        &self,
        filter: gen::Filter,
    ) -> std::result::Result<gen::EventsChunk, jsonrpc::Error> {
        Ok(gen::EventsChunk {
            continuation_token: Some("token-0".to_string()),
            events: vec![
                gen::EmittedEvent {
                    event: gen::Event {
                        from_address: "addr-0".to_string(),
                        event_content: gen::EventContent {
                            keys: vec!["key-0".to_string()],
                            data: vec!["val-0".to_string()],
                        }
                    },
                    block_hash: "hash-0".to_string(),
                    block_number: 42,
                    transaction_hash: "hash-1".to_string(),
                }
            ]
        })
    }
    */
    call(
        &state,
        5,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getEvents",
            "params": {
                "filter": {
                    "to_block": "200",
                    "from_block": 100,
                    "address": "req-addr-0",
                    "keys": [
                        ["req-key-0-0", "req-key-0-1"],
                        ["req-key-1-0", "req-key-1-1"]
                    ],
                    "continuation_token": "req-token-0",
                    "chunk_size": 42
                }
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
