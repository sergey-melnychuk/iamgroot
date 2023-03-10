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
                "block_id": "PENDING"
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
                "contract_address": "0x1",
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
                "transaction_hash": "0x1"
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
                "class_hash": "0x1"
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
                "block_id": "PENDING",
                "contract_address": "0x1"
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
                "block_id": "LATEST"
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
                    "entry_point_selector": "0x1",
                    "calldata": ["0x2"],
                    "contract_address": "0x3"
                },
                42
            ]
        }),
    );

    call(
        &state,
        1401,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_estimateFee",
            "params": {
                "request": {
                    "nonce": "0x1",
                    "version": "0x2",
                    "max_fee": "0x3",
                    "signature": [
                        "0x4",
                        "0x5"
                    ],
                    "calldata": [
                        "0x6",
                        "0x7"
                    ],
                    "entry_point_selector": "0x8",
                    "contract_address": "0x9",
                    "type": "INVOKE"
                },
                "block_id": 1
            },
        }),
    );

    call(
        &state,
        1402,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_estimateFee",
            "params": {
                "request": {
                    "nonce": "0x1",
                    "version": "0x2",
                    "max_fee": "0x3",
                    "signature": [
                        "0x4",
                        "0x5"
                    ],
                    "sender_address": "0xA",
                    "calldata": [
                        "0xB",
                        "0xC"
                    ],
                    "type": "INVOKE"
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
            "method": "starknet_blockNumber"
        }),
    );

    call(
        &state,
        16,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_blockHashAndNumber"
        }),
    );

    call(
        &state,
        17,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_chainId"
        }),
    );

    call(
        &state,
        18,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_pendingTransactions"
        }),
    );

    call(
        &state,
        19,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_syncing"
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
                    "to_block": 200,
                    "from_block": 100,
                    "address": "0xA",
                    "keys": [
                        ["0x1", "0x2"],
                        ["0x3", "0x4"]
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
                "contract_address": "0x1"
            }
        }),
    );

    call(
        &state,
        2201,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addInvokeTransaction",
            "params": {
                "invoke_transaction": {
                    "max_fee": "0x1",
                    "version": "0x2",
                    "nonce": "0x3",
                    "signature": [
                        "0x4"
                    ],
                    "type": "INVOKE",
                    "calldata": [
                        "0x6",
                        "0x7"
                    ],
                    "entry_point_selector": "0x8",
                    "contract_address": "0x9"
                }
            }
        }),
    );

    call(
        &state,
        2202,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addInvokeTransaction",
            "params": {
                "invoke_transaction": {
                    "max_fee": "0x1",
                    "version": "0x2",
                    "nonce": "0x3",
                    "signature": [
                        "0x4"
                    ],
                    "type": "INVOKE",
                    "sender_address": "0xA",
                    "calldata": [
                        "0xB",
                        "0xC"
                    ]
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
                "max_fee": "0x1",
                "version": "0x2",
                "nonce": "0x3",
                "signature": [
                    "0x4"
                ],
                "contract_address_salt": "0x5",
                "type": "DEPLOYACCOUNT",
                "class_hash": "0x7",
                "constructor_calldata": [
                    "0x8"
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
                "max_fee": "0x1",
                "version": "0x2",
                "nonce": "0x3",
                "signature": [
                    "0x4"
                ],
                "contract_address_salt": "0x5",
                "type": "DEPLOYACCOUNT",
                "class_hash": "0x7",
                "constructor_calldata": [
                    "0x8"
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
    println!("<<< {}", serde_json::to_string(&res).unwrap());
}

impl gen::Rpc for State {
    fn getBlockWithTxHashes(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::GetBlockWithTxHashesResult, jsonrpc::Error> {
        let result = gen::GetBlockWithTxHashesResult::BlockWithTxHashes(
            gen::BlockWithTxHashes {
                status: gen::BlockStatus::Pending,
                block_header: gen::BlockHeader {
                    block_hash: gen::BlockHash(gen::Felt::try_new("0x1")?),
                    timestamp: 1042,
                    sequencer_address: gen::Felt::try_new("0x2")?,
                    block_number: gen::BlockNumber(42),
                    new_root: gen::Felt::try_new("0x3")?,
                    parent_hash: gen::BlockHash(gen::Felt::try_new("0x4")?),
                },
                block_body_with_tx_hashes: gen::BlockBodyWithTxHashes {
                    transactions: vec![gen::Felt::try_new("0x5")?, gen::Felt::try_new("0x6")?],
                },
            },
        );
        println!("block_id={block_id:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getBlockWithTxs(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::GetBlockWithTxsResult, jsonrpc::Error> {
        let result = gen::GetBlockWithTxsResult::BlockWithTxs(
            gen::BlockWithTxs {
                status: gen::BlockStatus::AcceptedOnL1,
                block_header: gen::BlockHeader {
                    block_hash: gen::BlockHash(gen::Felt::try_new("0x1")?),
                    timestamp: 1042,
                    sequencer_address: gen::Felt::try_new("0x2")?,
                    block_number: gen::BlockNumber(42),
                    new_root: gen::Felt::try_new("0x3")?,
                    parent_hash: gen::BlockHash(gen::Felt::try_new("0x4")?),
                },
                block_body_with_txs: gen::BlockBodyWithTxs {
                    transactions: vec![
                        gen::Txn::InvokeTxn(gen::InvokeTxn {
                            common_txn_properties: gen::CommonTxnProperties {
                                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
                                broadcasted_txn_common_properties:
                                    gen::BroadcastedTxnCommonProperties {
                                        nonce: gen::Felt::try_new("0x1")?,
                                        version: gen::NumAsHex::try_new("0x1")?,
                                        max_fee: gen::Felt::try_new("0x1")?,
                                        signature: gen::Signature(vec![gen::Felt::try_new("0x1")?]),
                                    },
                            },
                            r#type: gen::InvokeTxnType::Invoke,
                            invoke_txn_kind: gen::InvokeTxnKind::FunctionCall(gen::FunctionCall {
                                calldata: vec![gen::Felt::try_new("0x1")?],
                                entry_point_selector: gen::Felt::try_new("0x1")?,
                                contract_address: gen::Address(gen::Felt::try_new("0x1")?),
                            }),
                        }),
                        gen::Txn::InvokeTxn(gen::InvokeTxn {
                            common_txn_properties: gen::CommonTxnProperties {
                                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x2")?),
                                broadcasted_txn_common_properties:
                                    gen::BroadcastedTxnCommonProperties {
                                        nonce: gen::Felt::try_new("0x2")?,
                                        version: gen::NumAsHex::try_new("0x2")?,
                                        max_fee: gen::Felt::try_new("0x2")?,
                                        signature: gen::Signature(vec![gen::Felt::try_new("0x2")?]),
                                    },
                            },
                            r#type: gen::InvokeTxnType::Invoke,
                            invoke_txn_kind: gen::InvokeTxnKind::InvokeTxnV1(gen::InvokeTxnV1 {
                                sender_address: gen::Address(gen::Felt::try_new("0x1")?),
                                calldata: vec![gen::Felt::try_new("0x1")?],
                            }),
                        }),
                    ],
                },
            },
        );
        println!("block_id={block_id:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getStateUpdate(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::GetStateUpdateResult, jsonrpc::Error> {
        let result = gen::GetStateUpdateResult::StateUpdate(gen::StateUpdate {
            new_root: gen::Felt::try_new("0xcafebabe")?,
            block_hash: gen::BlockHash(gen::Felt::try_new("0xdeadbeef")?),
            pending_state_update: gen::PendingStateUpdate {
                state_diff: gen::StateDiff {
                    nonces: vec![gen::NoncesItem {
                        nonce: Some(gen::Felt::try_new("0x1")?),
                        contract_address: Some(gen::Address(gen::Felt::try_new("0x2")?)),
                    }],
                    deprecated_declared_contract_hashes: Some(vec![gen::Felt::try_new("0x3")?]),
                    deployed_contracts: vec![gen::DeployedContractItem {
                        address: gen::Felt::try_new("0x4")?,
                        class_hash: gen::Felt::try_new("0x5")?,
                    }],
                    declared_contract_hashes: vec![gen::DeclaredContractHashesItem {
                        compiled_class_hash: Some(gen::Felt::try_new("0x6")?),
                        class_hash: Some(gen::Felt::try_new("0x7")?),
                    }],
                    storage_diffs: vec![gen::ContractStorageDiffItem {
                        address: gen::Felt::try_new("0x8")?,
                        storage_entries: vec![gen::StorageEntriesItem {
                            key: Some(gen::Felt::try_new("0x9")?),
                            value: Some(gen::Felt::try_new("0xA")?),
                        }],
                    }],
                },
                old_root: gen::Felt::try_new("0xFACE")?,
            },
        });
        println!("block_id={block_id:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getStorageAt(
        &self,
        contract_address: gen::Address,
        key: gen::StorageKey,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        let result = gen::Felt::try_new("0xcafebabe")?;
        println!("contract_address={contract_address:?}\nkey={key:?}\nblock_id={block_id:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getTransactionByHash(
        &self,
        transaction_hash: gen::TxnHash,
    ) -> std::result::Result<gen::Txn, jsonrpc::Error> {
        let result = gen::Txn::L1HandlerTxn(gen::L1HandlerTxn {
            r#type: gen::L1HandlerTxnType::L1Handler,
            transaction_hash: gen::TxnHash(gen::Felt::try_new("0xA")?),
            version: gen::NumAsHex::try_new("0x0")?,
            nonce: gen::NumAsHex::try_new("0x0")?,
            function_call: gen::FunctionCall {
                calldata: vec![gen::Felt::try_new("0x1")?],
                entry_point_selector: gen::Felt::try_new("0x1")?,
                contract_address: gen::Address(gen::Felt::try_new("0x1")?),
            },
        });
        println!("transaction_hash={transaction_hash:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getTransactionByBlockIdAndIndex(
        &self,
        block_id: gen::BlockId,
        index: gen::Index,
    ) -> std::result::Result<gen::Txn, jsonrpc::Error> {
        let result = gen::Txn::DeclareTxn(gen::DeclareTxn::DeclareTxnV2(
            gen::DeclareTxnV2 {
                declare_txn_v1: gen::DeclareTxnV1 {
                    common_txn_properties: gen::CommonTxnProperties {
                        transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
                        broadcasted_txn_common_properties: gen::BroadcastedTxnCommonProperties {
                            nonce: gen::Felt::try_new("0x1")?,
                            version: gen::NumAsHex::try_new("0x1")?,
                            max_fee: gen::Felt::try_new("0x1")?,
                            signature: gen::Signature(vec![gen::Felt::try_new("0x1")?]),
                        },
                    },
                    class_hash: gen::Felt::try_new("0x1")?,
                    sender_address: gen::Address(gen::Felt::try_new("0x1")?),
                    r#type: gen::DeclareTxnV1Type::Declare,
                },
                compiled_class_hash: Some(gen::Felt::try_new("0x1")?),
            },
        ));
        println!("block_id={block_id:?}\nindex={index:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getTransactionReceipt(
        &self,
        transaction_hash: gen::TxnHash,
    ) -> std::result::Result<gen::TxnReceipt, jsonrpc::Error> {
        let result = gen::TxnReceipt::DeployTxnReceipt(gen::DeployTxnReceipt {
            common_receipt_properties: gen::CommonReceiptProperties {
                messages_sent: vec![gen::MsgToL1 {
                    to_address: gen::Felt::try_new("0x1")?,
                    payload: vec![gen::Felt::try_new("0x1")?, gen::Felt::try_new("0x1")?],
                }],
                events: vec![gen::Event {
                    from_address: gen::Address(gen::Felt::try_new("0x1")?),
                    event_content: gen::EventContent {
                        data: vec![gen::Felt::try_new("0x1")?, gen::Felt::try_new("0x1")?],
                        keys: vec![gen::Felt::try_new("0x1")?, gen::Felt::try_new("0x1")?],
                    },
                }],
                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
                actual_fee: gen::Felt::try_new("0x1")?,
                status: gen::TxnStatus::AcceptedOnL2,
                block_hash: gen::BlockHash(gen::Felt::try_new("0x1")?),
                block_number: gen::BlockNumber(42),
            },
            contract_address: gen::Felt::try_new("0x1")?,
            r#type: gen::DeployTxnReceiptType::Deploy,
        });
        println!("transaction_hash={transaction_hash:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getClass(
        &self,
        block_id: gen::BlockId,
        class_hash: gen::Felt,
    ) -> std::result::Result<gen::GetClassResult, jsonrpc::Error> {
        let result = gen::GetClassResult::ContractClass(gen::ContractClass {
            entry_points_by_type: Some(gen::EntryPointsByTypeItem {
                constructor: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x11")?),
                    function_idx: Some(1),
                }]),
                external: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x22")?),
                    function_idx: Some(2),
                }]),
                l1_handler: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x33")?),
                    function_idx: Some(3),
                }]),
            }),
            abi: Some("abi".to_string()),
            sierra_program: Some(vec![gen::Felt::try_new("0xABCD")?]),
            sierra_version: Some("0".to_string()),
        });
        println!("block_id={block_id:?}\nclass_hash={class_hash:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getClassHashAt(
        &self,
        block_id: gen::BlockId,
        contract_address: gen::Address,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        let result = gen::Felt::try_new("0xF")?;
        println!("block_id={block_id:?}\ncontract_address={contract_address:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getClassAt(
        &self,
        block_id: gen::BlockId,
        contract_address: gen::Address,
    ) -> std::result::Result<gen::GetClassAtResult, jsonrpc::Error> {
        let result = gen::GetClassAtResult::ContractClass(gen::ContractClass {
            entry_points_by_type: Some(gen::EntryPointsByTypeItem {
                constructor: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x11")?),
                    function_idx: Some(1),
                }]),
                external: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x22")?),
                    function_idx: Some(2),
                }]),
                l1_handler: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x33")?),
                    function_idx: Some(3),
                }]),
            }),
            abi: Some("abi".to_string()),
            sierra_program: Some(vec![gen::Felt::try_new("0x44")?]),
            sierra_version: Some("0".to_string()),
        });
        println!("block_id={block_id:?}\ncontract_address={contract_address:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getBlockTransactionCount(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::GetBlockTransactionCountResult, jsonrpc::Error> {
        let result = gen::GetBlockTransactionCountResult(42);
        println!("block_id={block_id:?}\nresult={result:#?}");
        Ok(result)
    }

    fn call(
        &self,
        request: gen::FunctionCall,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::CallResult, jsonrpc::Error> {
        let result = gen::CallResult(vec![gen::Felt::try_new("0x0")?]);
        println!("block_id={block_id:?}\nreques={request:#?}\nresult={result:#?}");
        Ok(result)
    }

    fn estimateFee(
        &self,
        request: gen::BroadcastedTxn,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::FeeEstimate, jsonrpc::Error> {
        let result = gen::FeeEstimate {
            gas_consumed: Some(gen::NumAsHex::try_new("0xAA")?),
            gas_price: Some(gen::NumAsHex::try_new("0xBB")?),
            overall_fee: Some(gen::NumAsHex::try_new("0xCC")?),
        };
        println!("block_id={block_id:?}\nreques={request:#?}\nresult={result:#?}");
        Ok(result)
    }

    fn blockNumber(&self) -> std::result::Result<gen::BlockNumber, jsonrpc::Error> {
        Ok(gen::BlockNumber(42))
    }

    fn blockHashAndNumber(
        &self,
    ) -> std::result::Result<gen::BlockHashAndNumberResult, jsonrpc::Error> {
        Ok(gen::BlockHashAndNumberResult {
            block_number: Some(gen::BlockNumber(42)),
            block_hash: Some(gen::BlockHash(gen::Felt::try_new("0xface")?)),
        })
    }

    fn chainId(&self) -> std::result::Result<gen::ChainId, jsonrpc::Error> {
        Ok(gen::ChainId::try_new("0xdeadbeef")?)
    }

    fn pendingTransactions(
        &self,
    ) -> std::result::Result<gen::PendingTransactionsResult, jsonrpc::Error> {
        Ok(gen::PendingTransactionsResult(vec![gen::Txn::DeployTxn(
            gen::DeployTxn {
                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
                class_hash: gen::Felt::try_new("0x2")?,
                deploy_txn_properties: gen::DeployTxnProperties {
                    r#type: gen::DeployTxnPropertiesType::Deploy,
                    version: gen::NumAsHex::try_new("0x3")?,
                    contract_address_salt: gen::Felt::try_new("0x4")?,
                    constructor_calldata: vec![gen::Felt::try_new("0x5")?],
                },
            },
        )]))
    }

    fn syncing(&self) -> std::result::Result<gen::SyncingSyncing, jsonrpc::Error> {
        Ok(gen::SyncingSyncing::SyncStatus(gen::SyncStatus {
            starting_block_num: gen::NumAsHex::try_new("0x1")?,
            current_block_hash: gen::BlockHash(gen::Felt::try_new("0x2")?),
            starting_block_hash: gen::BlockHash(gen::Felt::try_new("0x3")?),
            current_block_num: gen::NumAsHex::try_new("0x4")?,
            highest_block_hash: gen::BlockHash(gen::Felt::try_new("0x5")?),
            highest_block_num: gen::NumAsHex::try_new("0x6")?,
        }))
    }

    fn getEvents(
        &self,
        filter: gen::Filter,
    ) -> std::result::Result<gen::EventsChunk, jsonrpc::Error> {
        let result = gen::EventsChunk {
            continuation_token: Some("token-0".to_string()),
            events: vec![gen::EmittedEvent {
                event: gen::Event {
                    from_address: gen::Address(gen::Felt::try_new("0x5")?),
                    event_content: gen::EventContent {
                        keys: vec![gen::Felt::try_new("0x4")?],
                        data: vec![gen::Felt::try_new("0x3")?],
                    },
                },
                block_hash: gen::BlockHash(gen::Felt::try_new("0x2")?),
                block_number: gen::BlockNumber(42),
                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
            }],
        };
        println!("filter={filter:#?}\nresult={result:#?}");
        Ok(result)
    }

    fn getNonce(
        &self,
        block_id: gen::BlockId,
        contract_address: gen::Address,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        let result = gen::Felt::try_new("0xA")?;
        println!("block_id={block_id:?}\ncontract_address={contract_address:?}\nresult={result:#?}");
        Ok(result)
    }

    fn addInvokeTransaction(
        &self,
        invoke_transaction: gen::BroadcastedInvokeTxn,
    ) -> std::result::Result<gen::AddInvokeTransactionResult, jsonrpc::Error> {
        let result = gen::AddInvokeTransactionResult {
            transaction_hash: Some(gen::TxnHash(gen::Felt::try_new("0x1")?)),
        };
        println!("invoke_transaction={invoke_transaction:#?}\nresult={result:#?}");
        Ok(result)
    }

    fn addDeclareTransaction(
        &self,
        declare_transaction: gen::BroadcastedDeclareTxn,
    ) -> std::result::Result<gen::AddDeclareTransactionResult, jsonrpc::Error> {
        let result = gen::AddDeclareTransactionResult {
            class_hash: Some(gen::Felt::try_new("0x1")?),
            transaction_hash: Some(gen::TxnHash(gen::Felt::try_new("0x2")?)),
        };
        println!("declare_transaction={declare_transaction:#?}\nresult={result:#?}");
        Ok(result)
    }

    fn addDeployAccountTransaction(
        &self,
        deploy_account_transaction: gen::BroadcastedDeployAccountTxn,
    ) -> std::result::Result<gen::AddDeployAccountTransactionResult, jsonrpc::Error> {
        let result = gen::AddDeployAccountTransactionResult {
            transaction_hash: Some(gen::TxnHash(gen::Felt::try_new("0x1")?)),
            contract_address: Some(gen::Felt::try_new("0x2")?),
        };
        println!("deploy_account_transaction={deploy_account_transaction:#?}\nresult={result:#?}");
        Ok(result)
    }
}

// NOTE: Generated code will be added below this line
// vvv GENERATED CODE BELOW vvv
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(clippy::enum_variant_names)]
pub mod gen {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use openrpc_stub_gen::jsonrpc;

    // object: 'ADDRESS'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Address(pub Felt); // name != binding_name

    // object: 'BLOCK_BODY_WITH_TXS'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockBodyWithTxs {
        pub transactions: Vec<Txn>,
    }

    // object: 'BLOCK_BODY_WITH_TX_HASHES'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockBodyWithTxHashes {
        pub transactions: Vec<Felt>,
    }

    // object: 'BLOCK_HASH'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockHash(pub Felt); // name != binding_name

    // object: 'BLOCK_HEADER'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockHeader {
        pub block_hash: BlockHash,
        pub block_number: BlockNumber,
        pub new_root: Felt,
        pub parent_hash: BlockHash,
        pub sequencer_address: Felt,
        pub timestamp: i64,
    }

    // object: 'BLOCK_ID'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum BlockId {
        BlockHash(BlockHash),
        BlockNumber(BlockNumber),
        BlockTag(BlockTag),
    }

    // object: 'BLOCK_NUMBER'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockNumber(pub i64); // name == binding_name

    // object: 'BLOCK_STATUS'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum BlockStatus {
        AcceptedOnL1,
        AcceptedOnL2,
        Pending,
        Rejected,
    }

    // object: 'BLOCK_TAG'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum BlockTag {
        Latest,
        Pending,
    }

    // object: 'BLOCK_WITH_TXS'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockWithTxs {
        #[serde(flatten)]
        pub block_body_with_txs: BlockBodyWithTxs,
        #[serde(flatten)]
        pub block_header: BlockHeader,
        pub status: BlockStatus,
    }

    // object: 'BLOCK_WITH_TX_HASHES'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockWithTxHashes {
        #[serde(flatten)]
        pub block_body_with_tx_hashes: BlockBodyWithTxHashes,
        #[serde(flatten)]
        pub block_header: BlockHeader,
        pub status: BlockStatus,
    }

    // object: 'BROADCASTED_DECLARE_TXN'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum BroadcastedDeclareTxn {
        BroadcastedDeclareTxnV1(BroadcastedDeclareTxnV1),
        BroadcastedDeclareTxnV2(BroadcastedDeclareTxnV2),
    }

    // object: 'BROADCASTED_DECLARE_TXN_V1'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeclareTxnV1 {
        #[serde(flatten)]
        pub broadcasted_txn_common_properties: BroadcastedTxnCommonProperties,
        pub contract_class: DeprecatedContractClass,
        pub sender_address: Address,
    }

    // object: 'BROADCASTED_DECLARE_TXN_V2'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeclareTxnV2 {
        #[serde(flatten)]
        pub broadcasted_txn_common_properties: BroadcastedTxnCommonProperties,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compiled_class_hash: Option<Felt>,
        pub contract_class: ContractClass,
        pub sender_address: Address,
        pub r#type: BroadcastedDeclareTxnV2Type,
    }

    // object: 'BROADCASTED_DECLARE_TXN_V2_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum BroadcastedDeclareTxnV2Type {
        Declare,
    }

    // object: 'BROADCASTED_DEPLOY_ACCOUNT_TXN'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeployAccountTxn {
        #[serde(flatten)]
        pub broadcasted_txn_common_properties: BroadcastedTxnCommonProperties,
        #[serde(flatten)]
        pub deploy_account_txn_properties: DeployAccountTxnProperties,
    }

    // object: 'BROADCASTED_INVOKE_TXN'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BroadcastedInvokeTxn {
        #[serde(flatten)]
        pub broadcasted_invoke_txn_kind: BroadcastedInvokeTxnKind,
        #[serde(flatten)]
        pub broadcasted_txn_common_properties: BroadcastedTxnCommonProperties,
        pub r#type: BroadcastedInvokeTxnType,
    }

    // object: 'BROADCASTED_INVOKE_TXN_KIND'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum BroadcastedInvokeTxnKind {
        FunctionCall(FunctionCall),
        InvokeTxnV1(InvokeTxnV1),
    }

    // object: 'BROADCASTED_INVOKE_TXN_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum BroadcastedInvokeTxnType {
        Invoke,
    }

    // object: 'BROADCASTED_TXN'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum BroadcastedTxn {
        BroadcastedDeclareTxn(BroadcastedDeclareTxn),
        BroadcastedDeployAccountTxn(BroadcastedDeployAccountTxn),
        BroadcastedInvokeTxn(BroadcastedInvokeTxn),
    }

    // object: 'BROADCASTED_TXN_COMMON_PROPERTIES'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BroadcastedTxnCommonProperties {
        pub max_fee: Felt,
        pub nonce: Felt,
        pub signature: Signature,
        pub version: NumAsHex,
    }

    // object: 'CHAIN_ID'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct ChainId(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct ChainId(String);

    mod chainid {
        use super::jsonrpc;
        use super::ChainId;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static CHAINID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[a-fA-F0-9]+$").unwrap());

        impl ChainId {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if CHAINID_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "ChainId value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for ChainId {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for ChainId {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'COMMON_RECEIPT_PROPERTIES'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct CommonReceiptProperties {
        pub actual_fee: Felt,
        pub block_hash: BlockHash,
        pub block_number: BlockNumber,
        pub events: Vec<Event>,
        pub messages_sent: Vec<MsgToL1>,
        pub status: TxnStatus,
        pub transaction_hash: TxnHash,
    }

    // object: 'COMMON_TXN_PROPERTIES'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct CommonTxnProperties {
        #[serde(flatten)]
        pub broadcasted_txn_common_properties: BroadcastedTxnCommonProperties,
        pub transaction_hash: TxnHash,
    }

    // object: 'CONTRACT_ABI'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ContractAbi(pub Vec<ContractAbiEntry>); // name == binding_name

    // object: 'CONTRACT_ABI_ENTRY'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum ContractAbiEntry {
        EventAbiEntry(EventAbiEntry),
        FunctionAbiEntry(FunctionAbiEntry),
        StructAbiEntry(StructAbiEntry),
    }

    // object: 'CONTRACT_CLASS'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ContractClass {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub abi: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub entry_points_by_type: Option<EntryPointsByTypeItem>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sierra_program: Option<Vec<Felt>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sierra_version: Option<String>,
    }

    // object: 'CONTRACT_STORAGE_DIFF_ITEM'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ContractStorageDiffItem {
        pub address: Felt,
        pub storage_entries: Vec<StorageEntriesItem>,
    }

    // object: 'DECLARED_CONTRACT_HASHES_ITEM'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeclaredContractHashesItem {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub class_hash: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compiled_class_hash: Option<Felt>,
    }

    // object: 'DECLARE_TXN'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum DeclareTxn {
        DeclareTxnV1(DeclareTxnV1),
        DeclareTxnV2(DeclareTxnV2),
    }

    // object: 'DECLARE_TXN_RECEIPT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeclareTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        pub r#type: DeclareTxnReceiptType,
    }

    // object: 'DECLARE_TXN_RECEIPT_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum DeclareTxnReceiptType {
        Declare,
    }

    // object: 'DECLARE_TXN_V1'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV1 {
        pub class_hash: Felt,
        #[serde(flatten)]
        pub common_txn_properties: CommonTxnProperties,
        pub sender_address: Address,
        pub r#type: DeclareTxnV1Type,
    }

    // object: 'DECLARE_TXN_V1_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum DeclareTxnV1Type {
        Declare,
    }

    // object: 'DECLARE_TXN_V2'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV2 {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compiled_class_hash: Option<Felt>,
        #[serde(flatten)]
        pub declare_txn_v1: DeclareTxnV1,
    }

    // object: 'DEPLOYED_CONTRACT_ITEM'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeployedContractItem {
        pub address: Felt,
        pub class_hash: Felt,
    }

    // object: 'DEPLOY_ACCOUNT_TXN'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxn {
        #[serde(flatten)]
        pub common_txn_properties: CommonTxnProperties,
        #[serde(flatten)]
        pub deploy_account_txn_properties: DeployAccountTxnProperties,
    }

    // object: 'DEPLOY_ACCOUNT_TXN_PROPERTIES'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnProperties {
        pub class_hash: Felt,
        pub constructor_calldata: Vec<Felt>,
        pub contract_address_salt: Felt,
        pub r#type: DeployAccountTxnPropertiesType,
    }

    // object: 'DEPLOY_ACCOUNT_TXN_PROPERTIES_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum DeployAccountTxnPropertiesType {
        DeployAccount,
    }

    // object: 'DEPLOY_ACCOUNT_TXN_RECEIPT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        pub contract_address: Felt,
        pub r#type: DeployAccountTxnReceiptType,
    }

    // object: 'DEPLOY_ACCOUNT_TXN_RECEIPT_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum DeployAccountTxnReceiptType {
        DeployAccount,
    }

    // object: 'DEPLOY_TXN'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeployTxn {
        pub class_hash: Felt,
        #[serde(flatten)]
        pub deploy_txn_properties: DeployTxnProperties,
        pub transaction_hash: TxnHash,
    }

    // object: 'DEPLOY_TXN_PROPERTIES'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeployTxnProperties {
        pub constructor_calldata: Vec<Felt>,
        pub contract_address_salt: Felt,
        pub r#type: DeployTxnPropertiesType,
        pub version: NumAsHex,
    }

    // object: 'DEPLOY_TXN_PROPERTIES_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum DeployTxnPropertiesType {
        Deploy,
    }

    // object: 'DEPLOY_TXN_RECEIPT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeployTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        pub contract_address: Felt,
        pub r#type: DeployTxnReceiptType,
    }

    // object: 'DEPLOY_TXN_RECEIPT_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum DeployTxnReceiptType {
        Deploy,
    }

    // object: 'DEPRECATED_CAIRO_ENTRY_POINT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeprecatedCairoEntryPoint {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<NumAsHex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub selector: Option<Felt>,
    }

    // object: 'DEPRECATED_CONTRACT_CLASS'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeprecatedContractClass {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub abi: Option<ContractAbi>,
        pub entry_points_by_type: EntryPointsByType,
        pub program: String,
    }

    // object: 'EMITTED_EVENT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EmittedEvent {
        pub block_hash: BlockHash,
        pub block_number: BlockNumber,
        #[serde(flatten)]
        pub event: Event,
        pub transaction_hash: TxnHash,
    }

    // object: 'ENTRY_POINTS_BY_TYPE'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EntryPointsByType {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub constructor: Option<Vec<DeprecatedCairoEntryPoint>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external: Option<Vec<DeprecatedCairoEntryPoint>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub l1_handler: Option<Vec<DeprecatedCairoEntryPoint>>,
    }

    // object: 'ENTRY_POINTS_BY_TYPE_ITEM'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EntryPointsByTypeItem {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub constructor: Option<Vec<SierraEntryPoint>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external: Option<Vec<SierraEntryPoint>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub l1_handler: Option<Vec<SierraEntryPoint>>,
    }

    // object: 'ETH_ADDRESS'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct EthAddress(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct EthAddress(String);

    mod ethaddress {
        use super::jsonrpc;
        use super::EthAddress;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static ETHADDRESS_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x[a-fA-F0-9]{40}$").unwrap());

        impl EthAddress {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if ETHADDRESS_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "EthAddress value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for EthAddress {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for EthAddress {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'EVENT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Event {
        #[serde(flatten)]
        pub event_content: EventContent,
        pub from_address: Address,
    }

    // object: 'EVENTS_CHUNK'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EventsChunk {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub continuation_token: Option<String>,
        pub events: Vec<EmittedEvent>,
    }

    // object: 'EVENT_ABI_ENTRY'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EventAbiEntry {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<Vec<TypedParameter>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keys: Option<Vec<TypedParameter>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<EventAbiType>,
    }

    // object: 'EVENT_ABI_TYPE'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum EventAbiType {
        Event,
    }

    // object: 'EVENT_CONTENT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EventContent {
        pub data: Vec<Felt>,
        pub keys: Vec<Felt>,
    }

    // object: 'EVENT_FILTER'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EventFilter {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<Address>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_block: Option<BlockId>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keys: Option<Vec<Vec<Felt>>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to_block: Option<BlockId>,
    }

    // object: 'FEE_ESTIMATE'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct FeeEstimate {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gas_consumed: Option<NumAsHex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gas_price: Option<NumAsHex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub overall_fee: Option<NumAsHex>,
    }

    // object: 'FELT'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Felt(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Felt(String);

    mod felt {
        use super::jsonrpc;
        use super::Felt;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static FELT_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$").unwrap());

        impl Felt {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if FELT_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Felt value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Felt {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Felt {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'FUNCTION_ABI_ENTRY'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct FunctionAbiEntry {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inputs: Option<Vec<TypedParameter>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub outputs: Option<Vec<TypedParameter>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<FunctionAbiType>,
    }

    // object: 'FUNCTION_ABI_TYPE'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum FunctionAbiType {
        Constructor,
        Function,
        L1Handler,
    }

    // object: 'FUNCTION_CALL'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct FunctionCall {
        pub calldata: Vec<Felt>,
        pub contract_address: Address,
        pub entry_point_selector: Felt,
    }

    // object: 'INVOKE_TXN'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct InvokeTxn {
        #[serde(flatten)]
        pub common_txn_properties: CommonTxnProperties,
        #[serde(flatten)]
        pub invoke_txn_kind: InvokeTxnKind,
        pub r#type: InvokeTxnType,
    }

    // object: 'INVOKE_TXN_KIND'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum InvokeTxnKind {
        FunctionCall(FunctionCall),
        InvokeTxnV1(InvokeTxnV1),
    }

    // object: 'INVOKE_TXN_RECEIPT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct InvokeTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        pub r#type: InvokeTxnReceiptType,
    }

    // object: 'INVOKE_TXN_RECEIPT_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum InvokeTxnReceiptType {
        Invoke,
    }

    // object: 'INVOKE_TXN_V0'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct InvokeTxnV0 {
        pub calldata: Vec<Felt>,
        pub contract_address: Address,
        pub entry_point_selector: Felt,
    }

    // object: 'INVOKE_TXN_V1'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct InvokeTxnV1 {
        pub calldata: Vec<Felt>,
        pub sender_address: Address,
    }

    // object: 'INVOKE_TXN_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum InvokeTxnType {
        Invoke,
    }

    // object: 'L1_HANDLER_TXN'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct L1HandlerTxn {
        #[serde(flatten)]
        pub function_call: FunctionCall,
        pub nonce: NumAsHex,
        pub transaction_hash: TxnHash,
        pub r#type: L1HandlerTxnType,
        pub version: NumAsHex,
    }

    // object: 'L1_HANDLER_TXN_RECEIPT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct L1HandlerTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        pub r#type: L1HandlerTxnReceiptType,
    }

    // object: 'L1_HANDLER_TXN_RECEIPT_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum L1HandlerTxnReceiptType {
        L1Handler,
    }

    // object: 'L1_HANDLER_TXN_type'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum L1HandlerTxnType {
        L1Handler,
    }

    // object: 'MSG_TO_L1'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct MsgToL1 {
        pub payload: Vec<Felt>,
        pub to_address: Felt,
    }

    // object: 'NONCES_ITEM'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct NoncesItem {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contract_address: Option<Address>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub nonce: Option<Felt>,
    }

    // object: 'NUM_AS_HEX'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct NumAsHex(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct NumAsHex(String);

    mod numashex {
        use super::jsonrpc;
        use super::NumAsHex;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static NUMASHEX_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[a-fA-F0-9]+$").unwrap());

        impl NumAsHex {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if NUMASHEX_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "NumAsHex value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for NumAsHex {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for NumAsHex {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'PENDING_BLOCK_WITH_TXS'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct PendingBlockWithTxs {
        #[serde(flatten)]
        pub block_body_with_txs: BlockBodyWithTxs,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_hash: Option<BlockHash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sequencer_address: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<i64>,
    }

    // object: 'PENDING_BLOCK_WITH_TX_HASHES'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct PendingBlockWithTxHashes {
        #[serde(flatten)]
        pub block_body_with_tx_hashes: BlockBodyWithTxHashes,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_hash: Option<BlockHash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sequencer_address: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<i64>,
    }

    // object: 'PENDING_COMMON_RECEIPT_PROPERTIES'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct PendingCommonReceiptProperties {
        pub actual_fee: Felt,
        pub events: Vec<Event>,
        pub messages_sent: Vec<MsgToL1>,
        pub transaction_hash: TxnHash,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<TxnType>,
    }

    // object: 'PENDING_DEPLOY_TXN_RECEIPT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct PendingDeployTxnReceipt {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contract_address: Option<Felt>,
        #[serde(flatten)]
        pub pending_common_receipt_properties: PendingCommonReceiptProperties,
    }

    // object: 'PENDING_STATE_UPDATE'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct PendingStateUpdate {
        pub old_root: Felt,
        pub state_diff: StateDiff,
    }

    // object: 'PENDING_TXN_RECEIPT'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum PendingTxnReceipt {
        PendingCommonReceiptProperties(PendingCommonReceiptProperties),
        PendingDeployTxnReceipt(PendingDeployTxnReceipt),
    }

    // object: 'RESULT_PAGE_REQUEST'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ResultPageRequest {
        pub chunk_size: i64,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub continuation_token: Option<String>,
    }

    // object: 'SIERRA_ENTRY_POINT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SierraEntryPoint {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub function_idx: Option<i64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub selector: Option<Felt>,
    }

    // object: 'SIGNATURE'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Signature(pub Vec<Felt>); // name == binding_name

    // object: 'STATE_DIFF'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct StateDiff {
        pub declared_contract_hashes: Vec<DeclaredContractHashesItem>,
        pub deployed_contracts: Vec<DeployedContractItem>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deprecated_declared_contract_hashes: Option<Vec<Felt>>,
        pub nonces: Vec<NoncesItem>,
        pub storage_diffs: Vec<ContractStorageDiffItem>,
    }

    // object: 'STATE_UPDATE'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct StateUpdate {
        pub block_hash: BlockHash,
        pub new_root: Felt,
        #[serde(flatten)]
        pub pending_state_update: PendingStateUpdate,
    }

    // object: 'STORAGE_ENTRIES_ITEM'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct StorageEntriesItem {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Felt>,
    }

    // object: 'STORAGE_KEY'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct StorageKey(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct StorageKey(String);

    mod storagekey {
        use super::jsonrpc;
        use super::StorageKey;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static STORAGEKEY_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x0[0-7]{1}[a-fA-F0-9]{0,62}$").unwrap());

        impl StorageKey {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if STORAGEKEY_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "StorageKey value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for StorageKey {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for StorageKey {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'STRUCT_ABI_ENTRY'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct StructAbiEntry {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub members: Option<Vec<StructMember>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<i64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<StructAbiType>,
    }

    // object: 'STRUCT_ABI_TYPE'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum StructAbiType {
        Struct,
    }

    // object: 'STRUCT_MEMBER'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct StructMember {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<i64>,
        #[serde(flatten)]
        pub typed_parameter: TypedParameter,
    }

    // object: 'SYNC_STATUS'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SyncStatus {
        pub current_block_hash: BlockHash,
        pub current_block_num: NumAsHex,
        pub highest_block_hash: BlockHash,
        pub highest_block_num: NumAsHex,
        pub starting_block_hash: BlockHash,
        pub starting_block_num: NumAsHex,
    }

    // object: 'TXN'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum Txn {
        DeclareTxn(DeclareTxn),
        DeployAccountTxn(DeployAccountTxn),
        DeployTxn(DeployTxn),
        InvokeTxn(InvokeTxn),
        L1HandlerTxn(L1HandlerTxn),
    }

    // object: 'TXN_HASH'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct TxnHash(pub Felt); // name != binding_name

    // object: 'TXN_RECEIPT'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum TxnReceipt {
        DeclareTxnReceipt(DeclareTxnReceipt),
        DeployAccountTxnReceipt(DeployAccountTxnReceipt),
        DeployTxnReceipt(DeployTxnReceipt),
        InvokeTxnReceipt(InvokeTxnReceipt),
        L1HandlerTxnReceipt(L1HandlerTxnReceipt),
        PendingTxnReceipt(PendingTxnReceipt),
    }

    // object: 'TXN_STATUS'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum TxnStatus {
        AcceptedOnL1,
        AcceptedOnL2,
        Pending,
        Rejected,
    }

    // object: 'TXN_TYPE'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum TxnType {
        Declare,
        Deploy,
        DeployAccount,
        Invoke,
        L1Handler,
    }

    // object: 'TYPED_PARAMETER'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct TypedParameter {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
    }

    // object: 'addDeclareTransaction_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct AddDeclareTransactionResult {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub class_hash: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_hash: Option<TxnHash>,
    }

    // object: 'addDeployAccountTransaction_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct AddDeployAccountTransactionResult {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contract_address: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_hash: Option<TxnHash>,
    }

    // object: 'addInvokeTransaction_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct AddInvokeTransactionResult {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_hash: Option<TxnHash>,
    }

    // object: 'blockHashAndNumber_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockHashAndNumberResult {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_hash: Option<BlockHash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_number: Option<BlockNumber>,
    }

    // object: 'blockNumber_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockNumberResult(pub BlockNumber); // name != binding_name

    // object: 'call_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct CallResult(pub Vec<Felt>); // name == binding_name

    // object: 'chainId_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ChainIdResult(pub ChainId); // name != binding_name

    // object: 'estimateFee_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EstimateFeeResult {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gas_consumed: Option<NumAsHex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gas_price: Option<NumAsHex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub overall_fee: Option<NumAsHex>,
    }

    // object: 'filter'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Filter {
        #[serde(flatten)]
        pub event_filter: EventFilter,
        #[serde(flatten)]
        pub result_page_request: ResultPageRequest,
    }

    // object: 'getBlockTransactionCount_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct GetBlockTransactionCountResult(pub i64); // name == binding_name

    // object: 'getBlockWithTxHashes_result'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum GetBlockWithTxHashesResult {
        BlockWithTxHashes(BlockWithTxHashes),
        PendingBlockWithTxHashes(PendingBlockWithTxHashes),
    }

    // object: 'getBlockWithTxs_result'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum GetBlockWithTxsResult {
        BlockWithTxs(BlockWithTxs),
        PendingBlockWithTxs(PendingBlockWithTxs),
    }

    // object: 'getClassAt_result'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum GetClassAtResult {
        ContractClass(ContractClass),
        DeprecatedContractClass(DeprecatedContractClass),
    }

    // object: 'getClassHashAt_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct GetClassHashAtResult(pub Felt); // name != binding_name

    // object: 'getClass_result'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum GetClassResult {
        ContractClass(ContractClass),
        DeprecatedContractClass(DeprecatedContractClass),
    }

    // object: 'getEvents_events'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct GetEventsEvents {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub continuation_token: Option<String>,
        pub events: Vec<EmittedEvent>,
    }

    // object: 'getNonce_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct GetNonceResult(pub Felt); // name != binding_name

    // object: 'getStateUpdate_result'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum GetStateUpdateResult {
        PendingStateUpdate(PendingStateUpdate),
        StateUpdate(StateUpdate),
    }

    // object: 'getStorageAt_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct GetStorageAtResult(pub Felt); // name != binding_name

    // object: 'getTransactionByBlockIdAndIndex_transactionResult'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum GetTransactionByBlockIdAndIndexTransactionResult {
        DeclareTxn(DeclareTxn),
        DeployAccountTxn(DeployAccountTxn),
        DeployTxn(DeployTxn),
        InvokeTxn(InvokeTxn),
        L1HandlerTxn(L1HandlerTxn),
    }

    // object: 'getTransactionByHash_result'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum GetTransactionByHashResult {
        DeclareTxn(DeclareTxn),
        DeployAccountTxn(DeployAccountTxn),
        DeployTxn(DeployTxn),
        InvokeTxn(InvokeTxn),
        L1HandlerTxn(L1HandlerTxn),
    }

    // object: 'getTransactionReceipt_result'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum GetTransactionReceiptResult {
        DeclareTxnReceipt(DeclareTxnReceipt),
        DeployAccountTxnReceipt(DeployAccountTxnReceipt),
        DeployTxnReceipt(DeployTxnReceipt),
        InvokeTxnReceipt(InvokeTxnReceipt),
        L1HandlerTxnReceipt(L1HandlerTxnReceipt),
        PendingTxnReceipt(PendingTxnReceipt),
    }

    // object: 'index'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Index(pub i64); // name == binding_name

    // object: 'pendingTransactions_result'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct PendingTransactionsResult(pub Vec<Txn>); // name == binding_name

    // object: 'syncing_syncing'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(untagged)]
    pub enum SyncingSyncing {
        Boolean(bool),
        SyncStatus(SyncStatus),
    }

    pub trait Rpc {
        /// Method: 'starknet_getBlockWithTxHashes'
        /// Summary: Get block information with transaction hashes given the block id
        /// Description:
        ///
        fn getBlockWithTxHashes(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockWithTxHashesResult, jsonrpc::Error>;

        /// Method: 'starknet_getBlockWithTxs'
        /// Summary: Get block information with full transactions given the block id
        /// Description:
        ///
        fn getBlockWithTxs(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockWithTxsResult, jsonrpc::Error>;

        /// Method: 'starknet_getStateUpdate'
        /// Summary: Get the information about the result of executing the requested block
        /// Description:
        ///
        fn getStateUpdate(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetStateUpdateResult, jsonrpc::Error>;

        /// Method: 'starknet_getStorageAt'
        /// Summary: Get the value of the storage at the given address and key
        /// Description:
        ///
        fn getStorageAt(
            &self,
            contract_address: Address,
            key: StorageKey,
            block_id: BlockId,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Method: 'starknet_getTransactionByHash'
        /// Summary: Get the details and status of a submitted transaction
        /// Description:
        ///
        fn getTransactionByHash(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<Txn, jsonrpc::Error>;

        /// Method: 'starknet_getTransactionByBlockIdAndIndex'
        /// Summary: Get the details of a transaction by a given block id and index
        /// Description: Get the details of the transaction given by the identified block and index in that block. If no transaction is found, null is returned.
        ///
        fn getTransactionByBlockIdAndIndex(
            &self,
            block_id: BlockId,
            index: Index,
        ) -> std::result::Result<Txn, jsonrpc::Error>;

        /// Method: 'starknet_getTransactionReceipt'
        /// Summary: Get the transaction receipt by the transaction hash
        /// Description:
        ///
        fn getTransactionReceipt(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<TxnReceipt, jsonrpc::Error>;

        /// Method: 'starknet_getClass'
        /// Summary: Get the contract class definition in the given block associated with the given hash
        /// Description:
        ///
        fn getClass(
            &self,
            block_id: BlockId,
            class_hash: Felt,
        ) -> std::result::Result<GetClassResult, jsonrpc::Error>;

        /// Method: 'starknet_getClassHashAt'
        /// Summary: Get the contract class hash in the given block for the contract deployed at the given address
        /// Description:
        ///
        fn getClassHashAt(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Method: 'starknet_getClassAt'
        /// Summary: Get the contract class definition in the given block at the given address
        /// Description:
        ///
        fn getClassAt(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<GetClassAtResult, jsonrpc::Error>;

        /// Method: 'starknet_getBlockTransactionCount'
        /// Summary: Get the number of transactions in a block given a block id
        /// Description: Returns the number of transactions in the designated block.
        ///
        fn getBlockTransactionCount(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockTransactionCountResult, jsonrpc::Error>;

        /// Method: 'starknet_call'
        /// Summary: call a starknet function without creating a StarkNet transaction
        /// Description: Calls a function in a contract and returns the return value.  Using this call will not create a transaction; hence, will not change the state
        ///
        fn call(
            &self,
            request: FunctionCall,
            block_id: BlockId,
        ) -> std::result::Result<CallResult, jsonrpc::Error>;

        /// Method: 'starknet_estimateFee'
        /// Summary: estimate the fee for a given StarkNet transaction
        /// Description: estimates the resources required by a transaction relative to a given state
        ///
        fn estimateFee(
            &self,
            request: BroadcastedTxn,
            block_id: BlockId,
        ) -> std::result::Result<FeeEstimate, jsonrpc::Error>;

        /// Method: 'starknet_blockNumber'
        /// Summary: Get the most recent accepted block number
        /// Description:
        ///
        fn blockNumber(&self) -> std::result::Result<BlockNumber, jsonrpc::Error>;

        /// Method: 'starknet_blockHashAndNumber'
        /// Summary: Get the most recent accepted block hash and number
        /// Description:
        ///
        fn blockHashAndNumber(
            &self,
        ) -> std::result::Result<BlockHashAndNumberResult, jsonrpc::Error>;

        /// Method: 'starknet_chainId'
        /// Summary: Return the currently configured StarkNet chain id
        /// Description:
        ///
        fn chainId(&self) -> std::result::Result<ChainId, jsonrpc::Error>;

        /// Method: 'starknet_pendingTransactions'
        /// Summary: Returns the transactions in the transaction pool, recognized by this sequencer
        /// Description:
        ///
        fn pendingTransactions(
            &self,
        ) -> std::result::Result<PendingTransactionsResult, jsonrpc::Error>;

        /// Method: 'starknet_syncing'
        /// Summary: Returns an object about the sync status, or false if the node is not synching
        /// Description:
        ///
        fn syncing(&self) -> std::result::Result<SyncingSyncing, jsonrpc::Error>;

        /// Method: 'starknet_getEvents'
        /// Summary: Returns all events matching the given filter
        /// Description: Returns all event objects matching the conditions in the provided filter
        ///
        fn getEvents(&self, filter: Filter) -> std::result::Result<EventsChunk, jsonrpc::Error>;

        /// Method: 'starknet_getNonce'
        /// Summary: Get the nonce associated with the given address in the given block
        /// Description:
        ///
        fn getNonce(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Method: 'starknet_addInvokeTransaction'
        /// Summary: Submit a new transaction to be added to the chain
        /// Description:
        ///
        fn addInvokeTransaction(
            &self,
            invoke_transaction: BroadcastedInvokeTxn,
        ) -> std::result::Result<AddInvokeTransactionResult, jsonrpc::Error>;

        /// Method: 'starknet_addDeclareTransaction'
        /// Summary: Submit a new class declaration transaction
        /// Description:
        ///
        fn addDeclareTransaction(
            &self,
            declare_transaction: BroadcastedDeclareTxn,
        ) -> std::result::Result<AddDeclareTransactionResult, jsonrpc::Error>;

        /// Method: 'starknet_addDeployAccountTransaction'
        /// Summary: Submit a new deploy account transaction
        /// Description:
        ///
        fn addDeployAccountTransaction(
            &self,
            deploy_account_transaction: BroadcastedDeployAccountTxn,
        ) -> std::result::Result<AddDeployAccountTransactionResult, jsonrpc::Error>;
    }

    fn handle_starknet_getBlockWithTxHashes<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id) = args_by_pos;
                ArgByName { block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockWithTxHashes(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getBlockWithTxs<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id) = args_by_pos;
                ArgByName { block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockWithTxs(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getStateUpdate<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id) = args_by_pos;
                ArgByName { block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_id } = args;

        match rpc.getStateUpdate(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getStorageAt<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, StorageKey, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            contract_address: Address,
            key: StorageKey,
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(contract_address, key, block_id) = args_by_pos;
                ArgByName {
                    contract_address,
                    key,
                    block_id,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            contract_address,
            key,
            block_id,
        } = args;

        match rpc.getStorageAt(contract_address, key, block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getTransactionByHash<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transaction_hash) = args_by_pos;
                ArgByName { transaction_hash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transaction_hash } = args;

        match rpc.getTransactionByHash(transaction_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getTransactionByBlockIdAndIndex<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Index);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            index: Index,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id, index) = args_by_pos;
                ArgByName { block_id, index }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_id, index } = args;

        match rpc.getTransactionByBlockIdAndIndex(block_id, index) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getTransactionReceipt<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transaction_hash) = args_by_pos;
                ArgByName { transaction_hash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transaction_hash } = args;

        match rpc.getTransactionReceipt(transaction_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getClass<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Felt);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            class_hash: Felt,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id, class_hash) = args_by_pos;
                ArgByName {
                    block_id,
                    class_hash,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block_id,
            class_hash,
        } = args;

        match rpc.getClass(block_id, class_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getClassHashAt<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id, contract_address) = args_by_pos;
                ArgByName {
                    block_id,
                    contract_address,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block_id,
            contract_address,
        } = args;

        match rpc.getClassHashAt(block_id, contract_address) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getClassAt<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id, contract_address) = args_by_pos;
                ArgByName {
                    block_id,
                    contract_address,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block_id,
            contract_address,
        } = args;

        match rpc.getClassAt(block_id, contract_address) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getBlockTransactionCount<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id) = args_by_pos;
                ArgByName { block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockTransactionCount(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_call<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(FunctionCall, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            request: FunctionCall,
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(request, block_id) = args_by_pos;
                ArgByName { request, block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { request, block_id } = args;

        match rpc.call(request, block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_estimateFee<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedTxn, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            request: BroadcastedTxn,
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(request, block_id) = args_by_pos;
                ArgByName { request, block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { request, block_id } = args;

        match rpc.estimateFee(request, block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_blockNumber<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.blockNumber() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_blockHashAndNumber<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.blockHashAndNumber() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_chainId<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.chainId() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_pendingTransactions<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.pendingTransactions() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_syncing<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.syncing() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getEvents<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Filter);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            filter: Filter,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(filter) = args_by_pos;
                ArgByName { filter }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { filter } = args;

        match rpc.getEvents(filter) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getNonce<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id, contract_address) = args_by_pos;
                ArgByName {
                    block_id,
                    contract_address,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block_id,
            contract_address,
        } = args;

        match rpc.getNonce(block_id, contract_address) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_addInvokeTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedInvokeTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            invoke_transaction: BroadcastedInvokeTxn,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(invoke_transaction) = args_by_pos;
                ArgByName { invoke_transaction }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { invoke_transaction } = args;

        match rpc.addInvokeTransaction(invoke_transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_addDeclareTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedDeclareTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            declare_transaction: BroadcastedDeclareTxn,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(declare_transaction) = args_by_pos;
                ArgByName {
                    declare_transaction,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            declare_transaction,
        } = args;

        match rpc.addDeclareTransaction(declare_transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_addDeployAccountTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedDeployAccountTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            deploy_account_transaction: BroadcastedDeployAccountTxn,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(deploy_account_transaction) = args_by_pos;
                ArgByName {
                    deploy_account_transaction,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            deploy_account_transaction,
        } = args;

        match rpc.addDeployAccountTransaction(deploy_account_transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    pub fn handle<RPC: Rpc>(rpc: &RPC, req: &jsonrpc::Request) -> jsonrpc::Response {
        let params = &req.params.clone().unwrap_or_default();

        let response = match req.method.as_str() {
            "starknet_getBlockWithTxHashes" => handle_starknet_getBlockWithTxHashes(rpc, params),
            "starknet_getBlockWithTxs" => handle_starknet_getBlockWithTxs(rpc, params),
            "starknet_getStateUpdate" => handle_starknet_getStateUpdate(rpc, params),
            "starknet_getStorageAt" => handle_starknet_getStorageAt(rpc, params),
            "starknet_getTransactionByHash" => handle_starknet_getTransactionByHash(rpc, params),
            "starknet_getTransactionByBlockIdAndIndex" => {
                handle_starknet_getTransactionByBlockIdAndIndex(rpc, params)
            }
            "starknet_getTransactionReceipt" => handle_starknet_getTransactionReceipt(rpc, params),
            "starknet_getClass" => handle_starknet_getClass(rpc, params),
            "starknet_getClassHashAt" => handle_starknet_getClassHashAt(rpc, params),
            "starknet_getClassAt" => handle_starknet_getClassAt(rpc, params),
            "starknet_getBlockTransactionCount" => {
                handle_starknet_getBlockTransactionCount(rpc, params)
            }
            "starknet_call" => handle_starknet_call(rpc, params),
            "starknet_estimateFee" => handle_starknet_estimateFee(rpc, params),
            "starknet_blockNumber" => handle_starknet_blockNumber(rpc, params),
            "starknet_blockHashAndNumber" => handle_starknet_blockHashAndNumber(rpc, params),
            "starknet_chainId" => handle_starknet_chainId(rpc, params),
            "starknet_pendingTransactions" => handle_starknet_pendingTransactions(rpc, params),
            "starknet_syncing" => handle_starknet_syncing(rpc, params),
            "starknet_getEvents" => handle_starknet_getEvents(rpc, params),
            "starknet_getNonce" => handle_starknet_getNonce(rpc, params),
            "starknet_addInvokeTransaction" => handle_starknet_addInvokeTransaction(rpc, params),
            "starknet_addDeclareTransaction" => handle_starknet_addDeclareTransaction(rpc, params),
            "starknet_addDeployAccountTransaction" => {
                handle_starknet_addDeployAccountTransaction(rpc, params)
            }
            _ => jsonrpc::Response::error(-32601, "Method not found"),
        };

        return if let Some(id) = req.id.as_ref() {
            response.with_id(id.clone())
        } else {
            response
        };
    }

    pub mod error {
        pub const BLOCK_NOT_FOUND: Error = Error(24, "Block not found");
        pub const CLASS_HASH_NOT_FOUND: Error = Error(28, "Class hash not found");
        pub const CONTRACT_ERROR: Error = Error(40, "Contract error");
        pub const CONTRACT_NOT_FOUND: Error = Error(20, "Contract not found");
        pub const FAILED_TO_RECEIVE_TXN: Error = Error(1, "Failed to write transaction");
        pub const INVALID_CALL_DATA: Error = Error(22, "Invalid call data");
        pub const INVALID_CONTINUATION_TOKEN: Error =
            Error(33, "The supplied continuation token is invalid or unknown");
        pub const INVALID_CONTRACT_CLASS: Error = Error(50, "Invalid contract class");
        pub const INVALID_MESSAGE_SELECTOR: Error = Error(21, "Invalid message selector");
        pub const INVALID_TXN_INDEX: Error = Error(27, "Invalid transaction index in a block");
        pub const NO_BLOCKS: Error = Error(32, "There are no blocks");
        pub const PAGE_SIZE_TOO_BIG: Error = Error(31, "Requested page size is too big");
        pub const TOO_MANY_KEYS_IN_FILTER: Error = Error(34, "Too many keys provided in a filter");
        pub const TXN_HASH_NOT_FOUND: Error = Error(25, "Transaction hash not found");

        pub struct Error(i64, &'static str);

        impl From<Error> for super::jsonrpc::Error {
            fn from(Error(code, message): Error) -> Self {
                Self {
                    code,
                    message: message.to_string(),
                }
            }
        }
    }
}
// ^^^ GENERATED CODE ABOVE ^^^
