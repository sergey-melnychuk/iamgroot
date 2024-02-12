use iamgroot::jsonrpc;

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
                "block_id": {"block_hash": "0xFACE"}
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
                "block_id": {"block_number": 123456}
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
                "block_id": "pending"
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
                "block_id": {"block_number": 42},
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
                "block_id": {"block_number": 42},
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
                "block_id": {"block_number": 1},
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
                "block_id": {"block_number": 42},
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
                "block_id": "pending",
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
                "block_id": "latest"
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
                {"block_number": 42}
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
                "block_id": {"block_number": 1}
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
                "block_id": {"block_number": 1}
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
                    "to_block": {"block_number": 200},
                    "from_block": {"block_number": 100},
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
                "block_id": {"block_number": 12},
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
        2301,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addDeclareTransaction",
            "params": {
                "declare_transaction": {
                    "max_fee": "0x1",
                    "version": "0x1",
                    "nonce": "0x3",
                    "signature": [
                        "0x4"
                    ],
                    "contract_class": {
                        "abi": [],
                        "entry_points_by_type": {
                            "constructor": [],
                            "external": [],
                            "l1_handler": []
                        },
                        "program": "just-some-string"
                    },
                    "sender_address": "0xA"
                }
            }
        }),
    );

    call(
        &state,
        2302,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addDeclareTransaction",
            "params": {
                "declare_transaction": {
                    "max_fee": "0x1",
                    "version": "0x1",
                    "nonce": "0x3",
                    "signature": [
                        "0x4"
                    ],
                    "compiled_class_hash": "0xB",
                    "contract_class": {
                        "abi": "just-another-string",
                        "entry_points_by_type": {
                            "constructor": [],
                            "external": [],
                            "l1_handler": []
                        },
                        "sierra_program": [
                            "0xAA",
                            "0xBB",
                            "0xCC"
                        ],
                        "sierra_version": "some-version"
                    },
                    "sender_address": "0xC",
                    "type": "DECLARE"
                }
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
                "deploy_account_transaction": {
                    "max_fee": "0x1",
                    "version": "0x2",
                    "nonce": "0x3",
                    "signature": [
                        "0x4"
                    ],
                    "contract_address_salt": "0x5",
                    "type": "DEPLOY_ACCOUNT",
                    "class_hash": "0x7",
                    "constructor_calldata": [
                        "0x8"
                    ]
                }
            }
        }),
    );

    {
        let json = r#"{
            "version": "0x100000000000000000000000000000001",
            "max_fee": "0x0",
            "contract_address_salt": "0x46c0d4abf0192a788aca261e58d7031576f7d8ea5229f452b0f23e691dd5971",
            "nonce": "0x0",
            "signature": [
                "0x296ab4b0b7cb0c6929c4fb1e04b782511dffb049f72a90efe5d53f0515eab88",
                "0x4e80d8bb98a9baf47f6f0459c2329a5401538576e76436acaf5f56c573c7d77"
            ],
            "constructor_calldata": [
                "0x63c056da088a767a6685ea0126f447681b5bceff5629789b70738bc26b5469d"
            ],
            "class_hash": "0x2b63cad399dd78efbc9938631e74079cbf19c9c08828e820e7606f46b947513",
            "type": "DEPLOY_ACCOUNT"
          }"#;
        let tx: gen::BroadcastedTxn = serde_json::from_str(json).unwrap();
        println!("{tx:#?}");
    }
    {
        let json = r#"[
            {
                "call_type": "CALL",
                "caller_address": "0x0",
                "contract_address": "0x332141f07b2081e840cd12f62fb161606a24d1d81d54549cd5fb2ed419db415",
                "calldata": [
                    "0x2b63cad399dd78efbc9938631e74079cbf19c9c08828e820e7606f46b947513",
                    "0x46c0d4abf0192a788aca261e58d7031576f7d8ea5229f452b0f23e691dd5971",
                    "0x63c056da088a767a6685ea0126f447681b5bceff5629789b70738bc26b5469d"
                ],
                "class_hash": "0x2b63cad399dd78efbc9938631e74079cbf19c9c08828e820e7606f46b947513",
                "entry_point_selector": "0x36fcbf06cd96843058359e1a75928beacfac10727dab22a3972f0af8aa92895",
                "entry_point_type": "EXTERNAL",
                "result": [],
                "execution_resources": {
                    "n_steps": 75,
                    "builtin_instance_counter": {
                    "ecdsa_builtin": 1
                    },
                    "n_memory_holes": 0
                },
                "internal_calls": [],
                "events": [],
                "messages": []
            },
            {
                "call_type": "CALL",
                "caller_address": "0x0",
                "contract_address": "0x332141f07b2081e840cd12f62fb161606a24d1d81d54549cd5fb2ed419db415",
                "calldata": [
                    "0x63c056da088a767a6685ea0126f447681b5bceff5629789b70738bc26b5469d"
                ],
                "class_hash": "0x2b63cad399dd78efbc9938631e74079cbf19c9c08828e820e7606f46b947513",
                "entry_point_selector": "0x28ffe4ff0f226a9107253e17a904099aa4f63a02a5621de0576e5aa71bc5194",
                "entry_point_type": "CONSTRUCTOR",
                "result": [],
                "execution_resources": {
                    "n_steps": 41,
                    "builtin_instance_counter": {},
                    "n_memory_holes": 0
                },
                "internal_calls": [],
                "events": [],
                "messages": []
            },
            {
                "call_type": "CALL",
                "calldata": [
                    "0x1",
                    "0x67162d776de4d50162cd2d02607cc15f3b54de48d66c0bf2994760049ce72ec",
                    "0x362398bec32bc0ebb411203221a35a0301193a96f317ebe5e40be9f60d15320",
                    "0x0",
                    "0x1",
                    "0x1",
                    "0x4d2"
                ],
                "caller_address": "0x0",
                "class_hash": "0x6f3ec04229f8f9663ee7d5bb9d2e06f213ba8c20eb34c58c25a54ef8fc591cb",
                "contract_address": "0x325bf20d89b86fafa54be01c3571d3b1bd5562e7ba13e9021e2f4be86c605a1",
                "entry_point_type": "EXTERNAL",
                "entry_point_selector": "0x15d40a3d6ca2ac30f4031e42be28da9b056fef9bb7357ac5e85627ee876e5ad",
                "events": [],
                "execution_resources": {
                    "builtin_instance_counter": {
                        "range_check_builtin": 2
                    },
                    "n_memory_holes": 3,
                    "n_steps": 206
                },
                "calls": [
                    {
                        "call_type": "CALL",
                        "calldata": [
                            "0x4d2"
                        ],
                        "caller_address": "0x325bf20d89b86fafa54be01c3571d3b1bd5562e7ba13e9021e2f4be86c605a1",
                        "class_hash": "0x6142260d223bea37ba3be22f597c0fb2faf27cb66590ca5cbd89df5d6dfdd78",
                        "contract_address": "0x67162d776de4d50162cd2d02607cc15f3b54de48d66c0bf2994760049ce72ec",
                        "entry_point_type": "EXTERNAL",
                        "entry_point_selector": "0x362398bec32bc0ebb411203221a35a0301193a96f317ebe5e40be9f60d15320",
                        "events": [],
                        "execution_resources": {
                            "builtin_instance_counter": {},
                            "n_memory_holes": 0,
                            "n_steps": 65
                        },
                        "internal_calls": [],
                        "messages": [],
                        "result": []
                    }
                ],
                "messages": [],
                "result": []
            }
          ]"#;
        // selector -> entry_point_selector
        // internal_calls -> calls
        let tx: Vec<gen::FunctionInvocation> =
            serde_json::from_str(json).unwrap();
        println!("{tx:#?}");
    }
    {
        let json = r#"{
            "overall_fee": "0x1",
            "gas_price": "0x2",
            "gas_consumed": "0x3"
          }"#;
        let fee: gen::FeeEstimate = serde_json::from_str(json).unwrap();
        println!("{fee:#?}");
    }
}

fn call<T: gen::Rpc>(rpc: &T, id: i64, json: serde_json::Value) {
    let req: jsonrpc::Request = serde_json::from_value(json).unwrap();
    let req = req.with_id(jsonrpc::Id::Number(id));
    let json = serde_json::to_string_pretty(&req).unwrap();
    println!("\n>>> {}", json);

    let req: jsonrpc::Request = serde_json::from_str(&json).unwrap();
    let res = gen::handle(rpc, &req);
    println!("<<< {}", serde_json::to_string_pretty(&res).unwrap());
}

#[allow(non_snake_case)]
impl gen::Rpc for State {
    fn getBlockWithTxHashes(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::GetBlockWithTxHashesResult, jsonrpc::Error>
    {
        let result = gen::GetBlockWithTxHashesResult::BlockWithTxHashes(
            gen::BlockWithTxHashes {
                status: gen::BlockStatus::Pending,
                block_header: gen::BlockHeader {
                    block_hash: gen::BlockHash(gen::Felt::try_new("0x1")?),
                    timestamp: 1042,
                    sequencer_address: gen::Felt::try_new("0x2")?,
                    block_number: gen::BlockNumber::try_new(42)?,
                    new_root: gen::Felt::try_new("0x3")?,
                    parent_hash: gen::BlockHash(gen::Felt::try_new("0x4")?),
                },
                block_body_with_tx_hashes: gen::BlockBodyWithTxHashes {
                    transactions: vec![
                        gen::Felt::try_new("0x5")?,
                        gen::Felt::try_new("0x6")?,
                    ],
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
        let result =
            gen::GetBlockWithTxsResult::BlockWithTxs(gen::BlockWithTxs {
                status: gen::BlockStatus::AcceptedOnL1,
                block_header: gen::BlockHeader {
                    block_hash: gen::BlockHash(gen::Felt::try_new("0x1")?),
                    timestamp: 1042,
                    sequencer_address: gen::Felt::try_new("0x2")?,
                    block_number: gen::BlockNumber::try_new(42)?,
                    new_root: gen::Felt::try_new("0x3")?,
                    parent_hash: gen::BlockHash(gen::Felt::try_new("0x4")?),
                },
                block_body_with_txs: gen::BlockBodyWithTxs {
                    transactions: vec![
                        gen::Txn::InvokeTxn(gen::InvokeTxn {
                            common_txn_properties: gen::CommonTxnProperties {
                                transaction_hash: gen::TxnHash(
                                    gen::Felt::try_new("0x1")?,
                                ),
                                broadcasted_txn_common_properties:
                                    gen::BroadcastedTxnCommonProperties {
                                        nonce: gen::Felt::try_new("0x1")?,
                                        version: gen::NumAsHex::try_new("0x1")?,
                                        max_fee: gen::Felt::try_new("0x1")?,
                                        signature: gen::Signature(vec![
                                            gen::Felt::try_new("0x1")?,
                                        ]),
                                    },
                            },
                            r#type: gen::InvokeTxnType::Invoke,
                            invoke_txn_kind: gen::InvokeTxnKind::FunctionCall(
                                gen::FunctionCall {
                                    calldata: vec![gen::Felt::try_new("0x1")?],
                                    entry_point_selector: gen::Felt::try_new(
                                        "0x1",
                                    )?,
                                    contract_address: gen::Address(
                                        gen::Felt::try_new("0x1")?,
                                    ),
                                },
                            ),
                        }),
                        gen::Txn::InvokeTxn(gen::InvokeTxn {
                            common_txn_properties: gen::CommonTxnProperties {
                                transaction_hash: gen::TxnHash(
                                    gen::Felt::try_new("0x2")?,
                                ),
                                broadcasted_txn_common_properties:
                                    gen::BroadcastedTxnCommonProperties {
                                        nonce: gen::Felt::try_new("0x2")?,
                                        version: gen::NumAsHex::try_new("0x2")?,
                                        max_fee: gen::Felt::try_new("0x2")?,
                                        signature: gen::Signature(vec![
                                            gen::Felt::try_new("0x2")?,
                                        ]),
                                    },
                            },
                            r#type: gen::InvokeTxnType::Invoke,
                            invoke_txn_kind: gen::InvokeTxnKind::InvokeTxnV1(
                                gen::InvokeTxnV1 {
                                    sender_address: gen::Address(
                                        gen::Felt::try_new("0x1")?,
                                    ),
                                    calldata: vec![gen::Felt::try_new("0x1")?],
                                },
                            ),
                        }),
                    ],
                },
            });
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
                        contract_address: Some(gen::Address(
                            gen::Felt::try_new("0x2")?,
                        )),
                    }],
                    declared_classes: vec![gen::DeclaredClassesItem {
                        class_hash: Some(gen::Felt::try_new("0x101")?),
                        compiled_class_hash: Some(gen::Felt::try_new("0x102")?),
                    }],
                    deprecated_declared_classes: vec![gen::Felt::try_new(
                        "0x3",
                    )?],
                    deployed_contracts: vec![gen::DeployedContractItem {
                        address: gen::Felt::try_new("0x4")?,
                        class_hash: gen::Felt::try_new("0x5")?,
                    }],
                    replaced_classes: vec![gen::ReplacedClassesItem {
                        contract_address: Some(gen::Address(
                            gen::Felt::try_new("0x6")?,
                        )),
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
                        transaction_hash: gen::TxnHash(gen::Felt::try_new(
                            "0x1",
                        )?),
                        broadcasted_txn_common_properties:
                            gen::BroadcastedTxnCommonProperties {
                                nonce: gen::Felt::try_new("0x1")?,
                                version: gen::NumAsHex::try_new("0x1")?,
                                max_fee: gen::Felt::try_new("0x1")?,
                                signature: gen::Signature(vec![
                                    gen::Felt::try_new("0x1")?,
                                ]),
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
                    payload: vec![
                        gen::Felt::try_new("0x1")?,
                        gen::Felt::try_new("0x1")?,
                    ],
                }],
                events: vec![gen::Event {
                    from_address: gen::Address(gen::Felt::try_new("0x1")?),
                    event_content: gen::EventContent {
                        data: vec![
                            gen::Felt::try_new("0x1")?,
                            gen::Felt::try_new("0x1")?,
                        ],
                        keys: vec![
                            gen::Felt::try_new("0x1")?,
                            gen::Felt::try_new("0x1")?,
                        ],
                    },
                }],
                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
                actual_fee: gen::Felt::try_new("0x1")?,
                status: gen::TxnStatus::AcceptedOnL2,
                block_hash: gen::BlockHash(gen::Felt::try_new("0x1")?),
                block_number: gen::BlockNumber::try_new(42)?,
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
            entry_points_by_type: gen::ContractClassEntryPoint {
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
            },
            abi: Some("abi".to_string()),
            sierra_program: vec![gen::Felt::try_new("0xABCD")?],
            contract_class_version: "0".to_string(),
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
        println!(
            "block_id={block_id:?}\ncontract_address={contract_address:?}\nresult={result:#?}"
        );
        Ok(result)
    }

    fn getClassAt(
        &self,
        block_id: gen::BlockId,
        contract_address: gen::Address,
    ) -> std::result::Result<gen::GetClassAtResult, jsonrpc::Error> {
        let result = gen::GetClassAtResult::ContractClass(gen::ContractClass {
            entry_points_by_type: gen::ContractClassEntryPoint {
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
            },
            abi: Some("abi".to_string()),
            sierra_program: vec![gen::Felt::try_new("0x44")?],
            contract_class_version: "0".to_string(),
        });
        println!(
            "block_id={block_id:?}\ncontract_address={contract_address:?}\nresult={result:#?}"
        );
        Ok(result)
    }

    fn getBlockTransactionCount(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::GetBlockTransactionCountResult, jsonrpc::Error>
    {
        let result = gen::GetBlockTransactionCountResult::try_new(42)?;
        println!("block_id={block_id:?}\nresult={result:#?}");
        Ok(result)
    }

    fn call(
        &self,
        request: gen::FunctionCall,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::CallResult, jsonrpc::Error> {
        let result = gen::CallResult(vec![gen::Felt::try_new("0x0")?]);
        println!(
            "block_id={block_id:?}\nreques={request:#?}\nresult={result:#?}"
        );
        Ok(result)
    }

    fn estimateFee(
        &self,
        request: gen::Request,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::EstimateFeeResult, jsonrpc::Error> {
        let result = gen::EstimateFeeResult(vec![gen::FeeEstimate {
            gas_consumed: Some(gen::NumAsHex::try_new("0xAA")?),
            gas_price: Some(gen::NumAsHex::try_new("0xBB")?),
            overall_fee: Some(gen::NumAsHex::try_new("0xCC")?),
        }]);
        println!(
            "block_id={block_id:?}\nreques={request:#?}\nresult={result:#?}"
        );
        Ok(result)
    }

    fn blockNumber(
        &self,
    ) -> std::result::Result<gen::BlockNumber, jsonrpc::Error> {
        Ok(gen::BlockNumber::try_new(42)?)
    }

    fn blockHashAndNumber(
        &self,
    ) -> std::result::Result<gen::BlockHashAndNumberResult, jsonrpc::Error>
    {
        Ok(gen::BlockHashAndNumberResult {
            block_number: Some(gen::BlockNumber::try_new(42)?),
            block_hash: Some(gen::BlockHash(gen::Felt::try_new("0xface")?)),
        })
    }

    fn chainId(&self) -> std::result::Result<gen::ChainId, jsonrpc::Error> {
        Ok(gen::ChainId::try_new("0xdeadbeef")?)
    }

    fn pendingTransactions(
        &self,
    ) -> std::result::Result<gen::PendingTransactionsResult, jsonrpc::Error>
    {
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

    fn syncing(
        &self,
    ) -> std::result::Result<gen::SyncingSyncing, jsonrpc::Error> {
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
                block_number: gen::BlockNumber::try_new(42)?,
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
        println!(
            "block_id={block_id:?}\ncontract_address={contract_address:?}\nresult={result:#?}"
        );
        Ok(result)
    }

    fn addInvokeTransaction(
        &self,
        invoke_transaction: gen::BroadcastedInvokeTxn,
    ) -> std::result::Result<gen::AddInvokeTransactionResult, jsonrpc::Error>
    {
        let result = gen::AddInvokeTransactionResult {
            transaction_hash: Some(gen::TxnHash(gen::Felt::try_new("0x1")?)),
        };
        println!(
            "invoke_transaction={invoke_transaction:#?}\nresult={result:#?}"
        );
        Ok(result)
    }

    fn addDeclareTransaction(
        &self,
        declare_transaction: gen::BroadcastedDeclareTxn,
    ) -> std::result::Result<gen::AddDeclareTransactionResult, jsonrpc::Error>
    {
        let result = gen::AddDeclareTransactionResult {
            class_hash: Some(gen::Felt::try_new("0x1")?),
            transaction_hash: Some(gen::TxnHash(gen::Felt::try_new("0x2")?)),
        };
        println!(
            "declare_transaction={declare_transaction:#?}\nresult={result:#?}"
        );
        Ok(result)
    }

    fn addDeployAccountTransaction(
        &self,
        deploy_account_transaction: gen::BroadcastedDeployAccountTxn,
    ) -> std::result::Result<
        gen::AddDeployAccountTransactionResult,
        jsonrpc::Error,
    > {
        let result = gen::AddDeployAccountTransactionResult {
            transaction_hash: Some(gen::TxnHash(gen::Felt::try_new("0x1")?)),
            contract_address: Some(gen::Felt::try_new("0x2")?),
        };
        println!("deploy_account_transaction={deploy_account_transaction:#?}\nresult={result:#?}");
        Ok(result)
    }

    fn traceTransaction(
        &self,
        _transaction_hash: gen::TxnHash,
    ) -> std::result::Result<gen::TransactionTrace, jsonrpc::Error> {
        Err(jsonrpc::Error {
            code: 1,
            message: "unimplemented".to_owned(),
        })
    }

    fn simulateTransaction(
        &self,
        _block_id: gen::BlockId,
        _transaction: gen::Transaction,
        _simulation_flags: gen::SimulationFlags,
    ) -> std::result::Result<
        gen::SimulateTransactionSimulatedTransactions,
        jsonrpc::Error,
    > {
        Err(jsonrpc::Error {
            code: 1,
            message: "unimplemented".to_owned(),
        })
    }

    fn traceBlockTransactions(
        &self,
        _block_hash: gen::BlockHash,
    ) -> std::result::Result<gen::TraceBlockTransactionsTraces, jsonrpc::Error>
    {
        Err(jsonrpc::Error {
            code: 1,
            message: "unimplemented".to_owned(),
        })
    }
}
