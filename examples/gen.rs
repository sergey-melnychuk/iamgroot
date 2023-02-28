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

    call(
        &state,
        4,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getStateUpdate",
            "params": 42
        }),
    );

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

    let req = gen::BroadcastedDeployAccountTxn {
        broadcasted_txn_common_properties: gen::BroadcastedTxnCommonProperties {
            max_fee: "0x01".to_string(),
            version: "0x02".to_string(),
            nonce: "0x03".to_string(),
            signature: gen::Signature(vec!["0x04".to_string()]),
        },
        deploy_account_txn_properties: gen::DeployAccountTxnProperties {
            contract_address_salt: "0x05".to_string(),
            r#type: gen::DeployAccountTxnPropertiesType::DeployAccount,
            class_hash: "0x07".to_string(),
            constructor_calldata: vec!["0x08".to_string()],
        },
    };

    let json = serde_json::to_string(&req).unwrap();
    println!("\n{}", json);
    let item: gen::BroadcastedDeployAccountTxn = serde_json::from_str(&json).unwrap();
    println!("{item:#?}");

    call(
        &state,
        6,
        serde_json::json!({
            "jsonrpc":"2.0",
            "method":"starknet_addDeployAccountTransaction",
            "params":{
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
    let ret = gen::handle(rpc, req);
    println!("<<< {}", serde_json::to_string(&ret).unwrap());
}

#[allow(unused_variables)]
impl gen::Rpc for State {
    fn getBlockWithTxHashes(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::StarknetGetBlockWithTxHashesResult, jsonrpc::Error> {
        todo!()
    }

    fn getBlockWithTxs(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::StarknetGetBlockWithTxsResult, jsonrpc::Error> {
        todo!()
    }

    fn getStateUpdate(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::StarknetGetStateUpdateResult, jsonrpc::Error> {
        Ok(gen::StarknetGetStateUpdateResult::StateUpdate(
            gen::StateUpdate {
                new_root: "0xcafebabe".to_string(),
                block_hash: "0xdeadbeef".to_string(),
                pending_state_update: gen::PendingStateUpdate {
                    state_diff: gen::StateDiff {
                        nonces: vec![gen::NoncesItem {
                            nonce: Some("nonce-0".to_string()),
                            contract_address: Some("addr-0".to_string()),
                        }],
                        deprecated_declared_contract_hashes: Some(vec!["contract-0".to_string()]),
                        deployed_contracts: vec![gen::DeployedContractItem {
                            address: "addr-1".to_string(),
                            class_hash: "hash-0".to_string(),
                        }],
                        declared_contract_hashes: vec![gen::DeclaredContractHashesItem {
                            compiled_class_hash: Some("hash-1".to_string()),
                            class_hash: Some("hash-2".to_string()),
                        }],
                        storage_diffs: vec![gen::ContractStorageDiffItem {
                            address: "addr-2".to_string(),
                            storage_entries: vec![gen::StorageEntriesItem {
                                key: Some("key-1".to_string()),
                                value: Some("val-1".to_string()),
                            }],
                        }],
                    },
                    old_root: "0xFACE".to_string(),
                },
            },
        ))
    }

    fn getStorageAt(
        &self,
        contract_address: String,
        key: String,
        block_id: gen::BlockId,
    ) -> std::result::Result<String, jsonrpc::Error> {
        todo!()
    }

    fn getTransactionByHash(
        &self,
        transaction_hash: String,
    ) -> std::result::Result<gen::Txn, jsonrpc::Error> {
        todo!()
    }

    fn getTransactionByBlockIdAndIndex(
        &self,
        block_id: gen::BlockId,
        index: i64,
    ) -> std::result::Result<gen::Txn, jsonrpc::Error> {
        todo!()
    }

    fn getTransactionReceipt(
        &self,
        transaction_hash: String,
    ) -> std::result::Result<gen::TxnReceipt, jsonrpc::Error> {
        todo!()
    }

    fn getClass(
        &self,
        block_id: gen::BlockId,
        class_hash: String,
    ) -> std::result::Result<gen::StarknetGetClassResult, jsonrpc::Error> {
        todo!()
    }

    fn getClassHashAt(
        &self,
        block_id: gen::BlockId,
        contract_address: String,
    ) -> std::result::Result<String, jsonrpc::Error> {
        todo!()
    }

    fn getClassAt(
        &self,
        block_id: gen::BlockId,
        contract_address: String,
    ) -> std::result::Result<gen::StarknetGetClassAtResult, jsonrpc::Error> {
        todo!()
    }

    fn getBlockTransactionCount(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<i64, jsonrpc::Error> {
        todo!()
    }

    fn call(
        &self,
        request: gen::FunctionCall,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::StarknetCallResult, jsonrpc::Error> {
        Ok(gen::StarknetCallResult(vec!["hello".to_string()]))
    }

    fn estimateFee(
        &self,
        request: gen::BroadcastedTxn,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::FeeEstimate, jsonrpc::Error> {
        todo!()
    }

    fn blockNumber(&self) -> std::result::Result<i64, jsonrpc::Error> {
        Ok(42)
    }

    fn blockHashAndNumber(
        &self,
    ) -> std::result::Result<gen::StarknetBlockHashAndNumberResult, jsonrpc::Error> {
        todo!()
    }

    fn chainId(&self) -> std::result::Result<String, jsonrpc::Error> {
        Err(gen::error::NO_BLOCKS.into())
    }

    fn pendingTransactions(
        &self,
    ) -> std::result::Result<gen::StarknetPendingTransactionsResult, jsonrpc::Error> {
        todo!()
    }

    fn syncing(&self) -> std::result::Result<gen::StarknetSyncingSyncing, jsonrpc::Error> {
        todo!()
    }

    fn getEvents(
        &self,
        filter: gen::Filter,
    ) -> std::result::Result<gen::EventsChunk, jsonrpc::Error> {
        Ok(gen::EventsChunk {
            continuation_token: Some("token-0".to_string()),
            events: vec![gen::EmittedEvent {
                event: gen::Event {
                    from_address: "addr-0".to_string(),
                    event_content: gen::EventContent {
                        keys: vec!["key-0".to_string()],
                        data: vec!["val-0".to_string()],
                    },
                },
                block_hash: "hash-0".to_string(),
                block_number: 42,
                transaction_hash: "hash-1".to_string(),
            }],
        })
    }

    fn getNonce(
        &self,
        block_id: gen::BlockId,
        contract_address: String,
    ) -> std::result::Result<String, jsonrpc::Error> {
        todo!()
    }

    fn addInvokeTransaction(
        &self,
        invoke_transaction: gen::BroadcastedInvokeTxn,
    ) -> std::result::Result<gen::StarknetAddInvokeTransactionResult, jsonrpc::Error> {
        todo!()
    }

    fn addDeclareTransaction(
        &self,
        declare_transaction: gen::BroadcastedDeclareTxn,
    ) -> std::result::Result<gen::StarknetAddDeclareTransactionResult, jsonrpc::Error> {
        todo!()
    }

    fn addDeployAccountTransaction(
        &self,
        deploy_account_transaction: gen::BroadcastedDeployAccountTxn,
    ) -> std::result::Result<gen::StarknetAddDeployAccountTransactionResult, jsonrpc::Error> {
        Ok(gen::StarknetAddDeployAccountTransactionResult {
            transaction_hash: Some("0x01".to_string()),
            contract_address: Some("0x01".to_string()),
        })
    }
}

// NOTE: Generated code will be added below this line
