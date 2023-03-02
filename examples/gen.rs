use openrpc_stub_gen::jsonrpc;

struct State;

fn main() {
    let state = State;

    call(
        &state,
        1,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getBlockWithTxHashes",
            "params": {
                "block_id": "0xFACE"
            }
        }),
    );

    call(
        &state,
        2,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getBlockWithTxs",
            "params": {
                "block_id": 123456789
            }
        }),
    );

    call(
        &state,
        3,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getStateUpdate",
            "params": {
                "block_id": "Pending"
            }
        }),
    );

    call(
        &state,
        4,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getStorageAt",
            "params": {
                "contract_address": "0x01",
                "key": "0x02",
                "block_id": 42,
            }
        }),
    );

    call(
        &state,
        5,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionByHash",
            "params": {
                "transaction_hash": "0xcafebabe",
            }
        }),
    );

    call(
        &state,
        6,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionByHash",
            "params": {
                "transaction_hash": "0xcafebabe",
            }
        }),
    );

    call(
        &state,
        7,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionByBlockIdAndIndex",
            "params": {
                "block_id": 42,
                "index": 24
            }
        }),
    );

    call(
        &state,
        8,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionReceipt",
            "params": {
                "transaction_hash": "0x01"
            }
        }),
    );

    call(
        &state,
        9,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getClass",
            "params": {
                "block_id": 1,
                "class_hash": "0x01"
            }
        }),
    );

    call(
        &state,
        10,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getClassAt",
            "params": {
                "block_id": 42,
                "contract_address": "0xFF"
            }
        }),
    );

    call(
        &state,
        11,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getClassHashAt",
            "params": {
                "block_id": "Pending",
                "contract_address": "0x01"
            }
        }),
    );

    call(
        &state,
        12,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getBlockTransactionCount",
            "params": {
                "block_id": "Latest"
            }
        }),
    );

    call(
        &state,
        13,
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
        14,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_estimateFee",
            "params": {
                "request": {
                    "nonce": "0x01",
                    "version": "0x02",
                    "max_fee": "0x03",
                    "signature": [
                        "0x04",
                        "0x05"
                    ],
                    "function_call": {
                        "calldata": [
                            "0x06",
                            "0x07"
                        ],
                        "entry_point_selector": "0x08",
                        "contract_address": "0x09"
                    },
                    "invoke_txn_v1": {
                        "sender_address": "0x0A",
                        "calldata": [
                            "0x0B",
                            "0x0C"
                        ]
                    },
                    "type": "Invoke"
                },
                "block_id": 1
            },
        }),
    );

    call(
        &state,
        15,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_blockNumber",
            "params": []
        }),
    );

    call(
        &state,
        16,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_blockHashAndNumber",
            "params": []
        }),
    );

    call(
        &state,
        17,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_chainId",
            "params": []
        }),
    );

    call(
        &state,
        18,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_pendingTransactions",
            "params": {}
        }),
    );

    call(
        &state,
        19,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_syncing",
            "params": []
        }),
    );

    call(
        &state,
        20,
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

    call(
        &state,
        21,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getNonce",
            "params": {
                "block_id": 12,
                "contract_address": "0x01"
            }
        }),
    );

    call(
        &state,
        22,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addInvokeTransaction",
            "params": {
                "invoke_transaction": {
                    "max_fee": "0x01",
                    "version": "0x02",
                    "nonce": "0x03",
                    "signature": [
                        "0x04"
                    ],
                    "type": "Invoke",
                    "function_call": {
                        "calldata": [
                            "0x06",
                            "0x07"
                        ],
                        "entry_point_selector": "0x08",
                        "contract_address": "0x09"
                    },
                    "invoke_txn_v1": {
                        "sender_address": "0x0A",
                        "calldata": [
                            "0x0B",
                            "0x0C"
                        ]
                    }
                }
            }
        }),
    );

    call(
        &state,
        23,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addDeclareTransaction",
            "params": {
                "max_fee": "0x01",
                "version": "0x02",
                "nonce": "0x03",
                "signature": [
                    "0x04"
                ],
                "contract_address_salt": "0x05",
                "type": "DeployAccount",
                "class_hash": "0x07",
                "constructor_calldata": [
                    "0x08"
                ]
            }
        }),
    );

    call(
        &state,
        24,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addDeployAccountTransaction",
            "params": {
                "max_fee": "0x01",
                "version": "0x02",
                "nonce": "0x03",
                "signature": [
                    "0x04"
                ],
                "contract_address_salt": "0x05",
                "type": "DeployAccount",
                "class_hash": "0x07",
                "constructor_calldata": [
                    "0x08"
                ]
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
    let res = gen::handle(rpc, &req);
    println!("<<<[A] {}", serde_json::to_string(&res).unwrap());

    let client = reqwest::blocking::Client::new();
    let res: jsonrpc::Response = client
        .post("http://localhost:3000/api")
        .json(&req)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("<<<[B] {}", serde_json::to_string(&res).unwrap());
}

impl gen::Rpc for State {
    fn getBlockWithTxHashes(
        &self,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::StarknetGetBlockWithTxHashesResult, jsonrpc::Error> {
        Ok(gen::StarknetGetBlockWithTxHashesResult::BlockWithTxHashes(
            gen::BlockWithTxHashes {
                status: gen::BlockStatus::Pending,
                block_header: gen::BlockHeader {
                    block_hash: gen::Felt("0x01".to_string()),
                    timestamp: 1042,
                    sequencer_address: gen::Felt("0x02".to_string()),
                    block_number: 42,
                    new_root: gen::Felt("0x03".to_string()),
                    parent_hash: gen::Felt("0x04".to_string()),
                },
                block_body_with_tx_hashes: gen::BlockBodyWithTxHashes {
                    transactions: vec![
                        gen::Felt("0x05".to_string()),
                        gen::Felt("0x06".to_string()),
                    ],
                },
            },
        ))
    }

    fn getBlockWithTxs(
        &self,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::StarknetGetBlockWithTxsResult, jsonrpc::Error> {
        Ok(gen::StarknetGetBlockWithTxsResult::BlockWithTxs(
            gen::BlockWithTxs {
                status: gen::BlockStatus::AcceptedOnL1,
                block_header: gen::BlockHeader {
                    block_hash: gen::Felt("0x01".to_string()),
                    timestamp: 1042,
                    sequencer_address: gen::Felt("0x02".to_string()),
                    block_number: 42,
                    new_root: gen::Felt("0x03".to_string()),
                    parent_hash: gen::Felt("0x04".to_string()),
                },
                block_body_with_txs: gen::BlockBodyWithTxs {
                    transactions: vec![gen::Txn::InvokeTxn(gen::InvokeTxn {
                        common_txn_properties: gen::CommonTxnProperties {
                            transaction_hash: gen::Felt("0x01".to_string()),
                            broadcasted_txn_common_properties:
                                gen::BroadcastedTxnCommonProperties {
                                    nonce: gen::Felt("0x01".to_string()),
                                    version: gen::NumAsHex("0x01".to_string()),
                                    max_fee: gen::Felt("0x01".to_string()),
                                    signature: gen::Signature(vec![gen::Felt("0x01".to_string())]),
                                },
                        },
                        r#type: gen::InvokeTxnType::Invoke,
                        function_call: gen::FunctionCall {
                            calldata: vec![gen::Felt("0x01".to_string())],
                            entry_point_selector: gen::Felt("0x01".to_string()),
                            contract_address: gen::Felt("0x01".to_string()),
                        },
                        invoke_txn_v1: gen::InvokeTxnV1 {
                            sender_address: gen::Felt("0x01".to_string()),
                            calldata: vec![gen::Felt("0x01".to_string())],
                        },
                    })],
                },
            },
        ))
    }

    fn getStateUpdate(
        &self,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::StarknetGetStateUpdateResult, jsonrpc::Error> {
        Ok(gen::StarknetGetStateUpdateResult::StateUpdate(
            gen::StateUpdate {
                new_root: gen::Felt("0xcafebabe".to_string()),
                block_hash: gen::Felt("0xdeadbeef".to_string()),
                pending_state_update: gen::PendingStateUpdate {
                    state_diff: gen::StateDiff {
                        nonces: vec![gen::NoncesItem {
                            nonce: Some(gen::Felt("nonce-0".to_string())),
                            contract_address: Some(gen::Felt("addr-0".to_string())),
                        }],
                        deprecated_declared_contract_hashes: Some(vec![gen::Felt(
                            "contract-0".to_string(),
                        )]),
                        deployed_contracts: vec![gen::DeployedContractItem {
                            address: gen::Felt("addr-1".to_string()),
                            class_hash: gen::Felt("hash-0".to_string()),
                        }],
                        declared_contract_hashes: vec![gen::DeclaredContractHashesItem {
                            compiled_class_hash: Some(gen::Felt("hash-1".to_string())),
                            class_hash: Some(gen::Felt("hash-2".to_string())),
                        }],
                        storage_diffs: vec![gen::ContractStorageDiffItem {
                            address: gen::Felt("addr-2".to_string()),
                            storage_entries: vec![gen::StorageEntriesItem {
                                key: Some(gen::Felt("key-1".to_string())),
                                value: Some(gen::Felt("val-1".to_string())),
                            }],
                        }],
                    },
                    old_root: gen::Felt("0xFACE".to_string()),
                },
            },
        ))
    }

    fn getStorageAt(
        &self,
        _contract_address: gen::Felt,
        _key: gen::StorageKey,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        Ok(gen::Felt("some-storage".to_string()))
    }

    fn getTransactionByHash(
        &self,
        _transaction_hash: gen::Felt,
    ) -> std::result::Result<gen::Txn, jsonrpc::Error> {
        Ok(gen::Txn::L1HandlerTxn(gen::L1HandlerTxn {
            r#type: gen::L1HandlerTxnType::L1Handler,
            transaction_hash: gen::Felt("0x00".to_string()),
            version: gen::NumAsHex("0x00".to_string()),
            nonce: gen::NumAsHex("0x00".to_string()),
            function_call: gen::FunctionCall {
                calldata: vec![gen::Felt("0x01".to_string())],
                entry_point_selector: gen::Felt("0x01".to_string()),
                contract_address: gen::Felt("0x01".to_string()),
            },
        }))
    }

    fn getTransactionByBlockIdAndIndex(
        &self,
        _block_id: gen::BlockId,
        _index: i64,
    ) -> std::result::Result<gen::Txn, jsonrpc::Error> {
        Ok(gen::Txn::DeclareTxn(gen::DeclareTxn::DeclareTxnV2(
            gen::DeclareTxnV2 {
                declare_txn_v1: gen::DeclareTxnV1 {
                    common_txn_properties: gen::CommonTxnProperties {
                        transaction_hash: gen::Felt("0x01".to_string()),
                        broadcasted_txn_common_properties: gen::BroadcastedTxnCommonProperties {
                            nonce: gen::Felt("0x01".to_string()),
                            version: gen::NumAsHex("0x01".to_string()),
                            max_fee: gen::Felt("0x01".to_string()),
                            signature: gen::Signature(vec![gen::Felt("0x01".to_string())]),
                        },
                    },
                    class_hash: gen::Felt("0x01".to_string()),
                    sender_address: gen::Felt("0x01".to_string()),
                    r#type: gen::DeclareTxnV1Type::Declare,
                },
                compiled_class_hash: Some(gen::Felt("0x01".to_string())),
            },
        )))
    }

    fn getTransactionReceipt(
        &self,
        _transaction_hash: gen::Felt,
    ) -> std::result::Result<gen::TxnReceipt, jsonrpc::Error> {
        Ok(gen::TxnReceipt::DeployTxnReceipt(gen::DeployTxnReceipt {
            common_receipt_properties: gen::CommonReceiptProperties {
                messages_sent: vec![gen::MsgToL1 {
                    to_address: gen::Felt("0x01".to_string()),
                    payload: vec![gen::Felt("0x01".to_string()), gen::Felt("0x01".to_string())],
                }],
                events: vec![gen::Event {
                    from_address: gen::Felt("0x01".to_string()),
                    event_content: gen::EventContent {
                        data: vec![gen::Felt("0x01".to_string()), gen::Felt("0x01".to_string())],
                        keys: vec![gen::Felt("0x01".to_string()), gen::Felt("0x01".to_string())],
                    },
                }],
                transaction_hash: gen::Felt("0x01".to_string()),
                actual_fee: gen::Felt("0x01".to_string()),
                status: gen::TxnStatus::AcceptedOnL2,
                block_hash: gen::Felt("0x01".to_string()),
                block_number: 42,
            },
            contract_address: gen::Felt("0x01".to_string()),
            r#type: gen::DeployTxnReceiptType::Deploy,
        }))
    }

    fn getClass(
        &self,
        _block_id: gen::BlockId,
        _class_hash: gen::Felt,
    ) -> std::result::Result<gen::StarknetGetClassResult, jsonrpc::Error> {
        Ok(gen::StarknetGetClassResult::ContractClass(
            gen::ContractClass {
                entry_points_by_type: Some(gen::EntryPointsByTypeItem {
                    constructor: Some(vec![gen::SierraEntryPoint {
                        selector: Some(gen::Felt("0x11".to_string())),
                        function_idx: Some(1),
                    }]),
                    external: Some(vec![gen::SierraEntryPoint {
                        selector: Some(gen::Felt("0x22".to_string())),
                        function_idx: Some(2),
                    }]),
                    l1_handler: Some(vec![gen::SierraEntryPoint {
                        selector: Some(gen::Felt("0x33".to_string())),
                        function_idx: Some(3),
                    }]),
                }),
                abi: Some("abi".to_string()),
                sierra_program: Some(vec![gen::Felt("program".to_string())]),
                sierra_version: Some("0".to_string()),
            },
        ))
    }

    fn getClassHashAt(
        &self,
        _block_id: gen::BlockId,
        _contract_address: gen::Felt,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        Ok(gen::Felt("some-hash".to_string()))
    }

    fn getClassAt(
        &self,
        _block_id: gen::BlockId,
        _contract_address: gen::Felt,
    ) -> std::result::Result<gen::StarknetGetClassAtResult, jsonrpc::Error> {
        Ok(gen::StarknetGetClassAtResult::ContractClass(
            gen::ContractClass {
                entry_points_by_type: Some(gen::EntryPointsByTypeItem {
                    constructor: Some(vec![gen::SierraEntryPoint {
                        selector: Some(gen::Felt("0x11".to_string())),
                        function_idx: Some(1),
                    }]),
                    external: Some(vec![gen::SierraEntryPoint {
                        selector: Some(gen::Felt("0x22".to_string())),
                        function_idx: Some(2),
                    }]),
                    l1_handler: Some(vec![gen::SierraEntryPoint {
                        selector: Some(gen::Felt("0x33".to_string())),
                        function_idx: Some(3),
                    }]),
                }),
                abi: Some("abi".to_string()),
                sierra_program: Some(vec![gen::Felt("program".to_string())]),
                sierra_version: Some("0".to_string()),
            },
        ))
    }

    fn getBlockTransactionCount(
        &self,
        _block_id: gen::BlockId,
    ) -> std::result::Result<i64, jsonrpc::Error> {
        Ok(42)
    }

    fn call(
        &self,
        _request: gen::FunctionCall,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::StarknetCallResult, jsonrpc::Error> {
        Ok(gen::StarknetCallResult(vec![gen::Felt(
            "hello".to_string(),
        )]))
    }

    fn estimateFee(
        &self,
        _request: gen::BroadcastedTxn,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::FeeEstimate, jsonrpc::Error> {
        Ok(gen::FeeEstimate {
            gas_consumed: Some(gen::NumAsHex("0xAA".to_string())),
            gas_price: Some(gen::NumAsHex("0xBB".to_string())),
            overall_fee: Some(gen::NumAsHex("0xCC".to_string())),
        })
    }

    fn blockNumber(&self) -> std::result::Result<i64, jsonrpc::Error> {
        Ok(42)
    }

    fn blockHashAndNumber(
        &self,
    ) -> std::result::Result<gen::StarknetBlockHashAndNumberResult, jsonrpc::Error> {
        Ok(gen::StarknetBlockHashAndNumberResult {
            block_number: Some(42),
            block_hash: Some(gen::Felt("0xface".to_string())),
        })
    }

    fn chainId(&self) -> std::result::Result<gen::ChainId, jsonrpc::Error> {
        Ok(gen::ChainId("some-chain".to_string()))
    }

    fn pendingTransactions(
        &self,
    ) -> std::result::Result<gen::StarknetPendingTransactionsResult, jsonrpc::Error> {
        Ok(gen::StarknetPendingTransactionsResult(vec![
            gen::Txn::DeployTxn(gen::DeployTxn {
                transaction_hash: gen::Felt("0x01".to_string()),
                class_hash: gen::Felt("0x01".to_string()),
                deploy_txn_properties: gen::DeployTxnProperties {
                    r#type: gen::DeployTxnPropertiesType::Deploy,
                    version: gen::NumAsHex("0x01".to_string()),
                    contract_address_salt: gen::Felt("0x01".to_string()),
                    constructor_calldata: vec![gen::Felt("0x01".to_string())],
                },
            }),
        ]))
    }

    fn syncing(&self) -> std::result::Result<gen::StarknetSyncingSyncing, jsonrpc::Error> {
        Ok(gen::StarknetSyncingSyncing::SyncStatus(gen::SyncStatus {
            starting_block_num: gen::NumAsHex("0x01".to_string()),
            current_block_hash: gen::Felt("0x02".to_string()),
            starting_block_hash: gen::Felt("0x03".to_string()),
            current_block_num: gen::NumAsHex("0x04".to_string()),
            highest_block_hash: gen::Felt("0x05".to_string()),
            highest_block_num: gen::NumAsHex("0x06".to_string()),
        }))
    }

    fn getEvents(
        &self,
        _filter: gen::Filter,
    ) -> std::result::Result<gen::EventsChunk, jsonrpc::Error> {
        Ok(gen::EventsChunk {
            continuation_token: Some("token-0".to_string()),
            events: vec![gen::EmittedEvent {
                event: gen::Event {
                    from_address: gen::Felt("addr-0".to_string()),
                    event_content: gen::EventContent {
                        keys: vec![gen::Felt("key-0".to_string())],
                        data: vec![gen::Felt("val-0".to_string())],
                    },
                },
                block_hash: gen::Felt("hash-0".to_string()),
                block_number: 42,
                transaction_hash: gen::Felt("hash-1".to_string()),
            }],
        })
    }

    fn getNonce(
        &self,
        _block_id: gen::BlockId,
        _contract_address: gen::Felt,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        Ok(gen::Felt("some-nonce".to_string()))
    }

    fn addInvokeTransaction(
        &self,
        _invoke_transaction: gen::BroadcastedInvokeTxn,
    ) -> std::result::Result<gen::StarknetAddInvokeTransactionResult, jsonrpc::Error> {
        Ok(gen::StarknetAddInvokeTransactionResult {
            transaction_hash: Some(gen::Felt("0x01".to_string())),
        })
    }

    fn addDeclareTransaction(
        &self,
        _declare_transaction: gen::BroadcastedDeclareTxn,
    ) -> std::result::Result<gen::StarknetAddDeclareTransactionResult, jsonrpc::Error> {
        Ok(gen::StarknetAddDeclareTransactionResult {
            class_hash: Some(gen::Felt("0x01".to_string())),
            transaction_hash: Some(gen::Felt("0x02".to_string())),
        })
    }

    fn addDeployAccountTransaction(
        &self,
        _deploy_account_transaction: gen::BroadcastedDeployAccountTxn,
    ) -> std::result::Result<gen::StarknetAddDeployAccountTransactionResult, jsonrpc::Error> {
        Ok(gen::StarknetAddDeployAccountTransactionResult {
            transaction_hash: Some(gen::Felt("0x01".to_string())),
            contract_address: Some(gen::Felt("0x02".to_string())),
        })
    }
}

// NOTE: Generated code will be added below this line
