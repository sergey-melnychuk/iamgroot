use iamgroot::jsonrpc;

fn main() {
    env_logger::init();

    demo::demo();
    client::run();
}

struct State;

mod client {
    use super::*;
    pub fn run() {
        use crate::gen::Rpc;
        let url: Option<&'static str> = option_env!("URL");
        if let Some(url) = url {
            let client = gen::client::Client::new(url);
            let block = client
                .getBlockWithTxs(gen::BlockId::BlockTag(gen::BlockTag::Latest))
                .unwrap();
            if let gen::GetBlockWithTxsResult::BlockWithTxs(block) = block {
                log::debug!(
                    "block hash: {}",
                    block.block_header.block_hash.0.as_ref()
                );
                log::debug!(
                    "block number: {}",
                    block.block_header.block_number.as_ref()
                );
            } else {
                log::error!("got pending block");
            }
        }
    }
}

mod demo {
    use super::*;
    pub fn demo() {
        let state = State;

        call(
            &state,
            1,
            serde_json::json!({
                "jsonrpc": "2.0",
                "method": "starknet_getBlockWithTxHashes",
                "params": {
                    "block_id": {
                        "block_hash": "0xFACE"
                    }
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
                    "block_id": {
                        "block_number": 123456
                    }
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
                    "block_id": {
                        "block_number": 42
                    },
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
                    "block_id": {
                        "block_number": 42
                    },
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
                    "block_id": {
                        "block_number": 1
                    },
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
                    "block_id": {
                        "block_number": 42
                    },
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
                    {
                        "block_number": 42
                    }
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
                    "request": [{
                        "version": "0x0",
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
                    }],
                    "simulation_flags": [],
                    "block_id": {
                        "block_number": 1
                    }
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
                    "request": [
                        {
                            "nonce": "0x01",
                            "version": "0x0",
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
                            "entry_point_selector": "0x8",
                            "contract_address": "0x9",
                            "type": "INVOKE"
                        }
                    ],
                    "simulation_flags": [],
                    "block_id": {
                        "block_number": 1
                    }
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
                "method": "starknet_syncing"
            }),
        );

        call(
            &state,
            19,
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
            20,
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
            201,
            serde_json::json!({
                "jsonrpc": "2.0",
                "method": "starknet_addInvokeTransaction",
                "params": {
                    "invoke_transaction": {
                        "max_fee": "0x1",
                        "version": "0x0",
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
            202,
            serde_json::json!({
                "jsonrpc": "2.0",
                "method": "starknet_addInvokeTransaction",
                "params": {
                    "invoke_transaction": {
                        "max_fee": "0x1",
                        "version": "0x1",
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
            21,
            serde_json::json!({
                "jsonrpc": "2.0",
                "method": "starknet_addDeclareTransaction",
                "params": {
                    "declare_transaction": {
                        "type": "DECLARE",
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
                            "program": "CAFEBABE"
                        },
                        "sender_address": "0xA"
                    }
                }
            }),
        );

        call(
            &state,
            22,
            serde_json::json!({
                "jsonrpc": "2.0",
                "method": "starknet_addDeclareTransaction",
                "params": {
                    "declare_transaction": {
                        "max_fee": "0x1",
                        "version": "0x2",
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
                            "contract_class_version": "some-version"
                        },
                        "sender_address": "0xC",
                        "type": "DECLARE"
                    }
                }
            }),
        );

        call(
            &state,
            23,
            serde_json::json!({
                "jsonrpc": "2.0",
                "method": "starknet_addDeployAccountTransaction",
                "params": {
                    "deploy_account_transaction": {
                        "max_fee": "0x1",
                        "version": "0x1",
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
    }

    fn call<T: gen::Rpc>(rpc: &T, id: i64, json: serde_json::Value) {
        let req: jsonrpc::Request = serde_json::from_value(json).unwrap();
        let req = req.with_id(jsonrpc::Id::Number(id));
        let json = serde_json::to_string_pretty(&req).unwrap();
        log::debug!(">>> {}", json);

        let req: jsonrpc::Request = serde_json::from_str(&json).unwrap();
        let res = gen::handle(rpc, &req);
        log::debug!("<<< {}", serde_json::to_string_pretty(&res).unwrap());
    }
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
                    timestamp: gen::BlockHeaderTimestamp::try_new(1042)?,
                    sequencer_address: gen::Felt::try_new("0x2")?,
                    block_number: gen::BlockNumber::try_new(42)?,
                    new_root: gen::Felt::try_new("0x3")?,
                    parent_hash: gen::BlockHash(gen::Felt::try_new("0x4")?),
                    l1_gas_price: gen::ResourcePrice {
                        price_in_fri: Some(gen::Felt::try_new("0x4")?),
                        price_in_wei: gen::Felt::try_new("0x4")?,
                    },
                    starknet_version: "starknet-version".to_string(),
                },
                block_body_with_tx_hashes: gen::BlockBodyWithTxHashes {
                    transactions: vec![
                        gen::TxnHash(gen::Felt::try_new("0x5")?),
                        gen::TxnHash(gen::Felt::try_new("0x6")?),
                    ],
                },
            },
        );
        log::debug!("block_id={block_id:?}\nresult={result:#?}");
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
                    timestamp: gen::BlockHeaderTimestamp::try_new(1042)?,
                    sequencer_address: gen::Felt::try_new("0x2")?,
                    block_number: gen::BlockNumber::try_new(42)?,
                    new_root: gen::Felt::try_new("0x3")?,
                    parent_hash: gen::BlockHash(gen::Felt::try_new("0x4")?),
                    l1_gas_price: gen::ResourcePrice {
                        price_in_fri: Some(gen::Felt::try_new("0x1111")?),
                        price_in_wei: gen::Felt::try_new("0x2222")?,
                    },
                    starknet_version: "starknet-version".to_owned(),
                },
                block_body_with_txs: gen::BlockBodyWithTxs {
                    transactions: vec![gen::TxnWithHash {
                        txn: gen::Txn::InvokeTxn(gen::InvokeTxn::InvokeTxnV0(
                            gen::InvokeTxnV0 {
                                r#type: gen::InvokeTxnV0Type::Invoke,
                                max_fee: gen::Felt::try_new("0x1111")?,
                                version: gen::InvokeTxnV0Version::V0x0,
                                signature: vec![gen::Felt::try_new("0x1111")?],
                                contract_address: gen::Address(
                                    gen::Felt::try_new("0x1111")?,
                                ),
                                entry_point_selector: gen::Felt::try_new(
                                    "0x1111",
                                )?,
                                calldata: vec![gen::Felt::try_new("0x1111")?],
                            },
                        )),
                        transaction_hash: gen::TxnHash(gen::Felt::try_new(
                            "0x1111",
                        )?),
                    }],
                },
            });
        log::debug!("block_id={block_id:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getStateUpdate(
        &self,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::GetStateUpdateResult, jsonrpc::Error> {
        let result = gen::GetStateUpdateResult::StateUpdate(gen::StateUpdate {
            new_root: gen::Felt::try_new("0xcafebabe")?,
            block_hash: gen::BlockHash(gen::Felt::try_new("0xdeadbeef")?),
            state_diff: gen::StateDiff {
                nonces: vec![gen::NonceUpdate {
                    nonce: Some(gen::Felt::try_new("0x1")?),
                    contract_address: Some(gen::Address(gen::Felt::try_new(
                        "0x2",
                    )?)),
                }],
                declared_classes: vec![gen::DeclaredClass {
                    class_hash: Some(gen::Felt::try_new("0x101")?),
                    compiled_class_hash: Some(gen::Felt::try_new("0x102")?),
                }],
                deprecated_declared_classes: vec![gen::Felt::try_new("0x3")?],
                deployed_contracts: vec![gen::DeployedContractItem {
                    address: gen::Felt::try_new("0x4")?,
                    class_hash: gen::Felt::try_new("0x5")?,
                }],
                replaced_classes: vec![gen::ReplacedClass {
                    contract_address: Some(gen::Address(gen::Felt::try_new(
                        "0x6",
                    )?)),
                    class_hash: Some(gen::Felt::try_new("0x7")?),
                }],
                storage_diffs: vec![gen::ContractStorageDiffItem {
                    address: gen::Felt::try_new("0x8")?,
                    storage_entries: vec![gen::StorageDiffItem {
                        key: Some(gen::Felt::try_new("0x9")?),
                        value: Some(gen::Felt::try_new("0xA")?),
                    }],
                }],
            },
            old_root: gen::Felt::try_new("0xFACE")?,
        });
        log::debug!("block_id={block_id:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getStorageAt(
        &self,
        contract_address: gen::Address,
        key: gen::StorageKey,
        block_id: gen::BlockId,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        let result = gen::Felt::try_new("0xcafebabe")?;
        log::debug!("contract_address={contract_address:?}\nkey={key:?}\nblock_id={block_id:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getTransactionByHash(
        &self,
        transaction_hash: gen::TxnHash,
    ) -> std::result::Result<gen::GetTransactionByHashResult, jsonrpc::Error>
    {
        let result = gen::GetTransactionByHashResult {
            txn: gen::Txn::L1HandlerTxn(gen::L1HandlerTxn {
                version: gen::Felt::try_new("0xcafebabe")?,
                r#type: gen::L1HandlerTxnType::L1Handler,
                nonce: gen::NumAsHex::try_new("0x1")?,
                function_call: gen::FunctionCall {
                    calldata: vec![gen::Felt::try_new("0x1")?],
                    entry_point_selector: gen::Felt::try_new("0x1")?,
                    contract_address: gen::Address(gen::Felt::try_new("0x1")?),
                },
            }),
            transaction_hash: transaction_hash.clone(),
        };
        log::debug!(
            "transaction_hash={transaction_hash:?}\nresult={result:#?}"
        );
        Ok(result)
    }

    fn getTransactionByBlockIdAndIndex(
        &self,
        block_id: gen::BlockId,
        index: gen::GetTransactionByBlockIdAndIndexIndex,
    ) -> std::result::Result<
        gen::GetTransactionByBlockIdAndIndexResult,
        jsonrpc::Error,
    > {
        let result = gen::GetTransactionByBlockIdAndIndexResult {
            txn: gen::Txn::DeclareTxn(gen::DeclareTxn::DeclareTxnV2(
                gen::DeclareTxnV2 {
                    compiled_class_hash: gen::Felt::try_new("0x1")?,
                    r#type: gen::DeclareTxnV2Type::Declare,
                    sender_address: gen::Address(gen::Felt::try_new("0x1")?),
                    max_fee: gen::Felt::try_new("0x1")?,
                    version: gen::DeclareTxnV2Version::V0x2,
                    signature: vec![gen::Felt::try_new("0x1")?],
                    nonce: gen::Felt::try_new("0x1")?,
                    class_hash: gen::Felt::try_new("0x1")?,
                },
            )),
            transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
        };
        log::debug!(
            "block_id={block_id:?}\nindex={index:?}\nresult={result:#?}"
        );
        Ok(result)
    }

    fn getTransactionReceipt(
        &self,
        transaction_hash: gen::TxnHash,
    ) -> std::result::Result<gen::GetTransactionReceiptResult, jsonrpc::Error>
    {
        let result = gen::TxnReceipt::DeployTxnReceipt(gen::DeployTxnReceipt {
            common_receipt_properties: gen::CommonReceiptProperties {
                messages_sent: vec![gen::MsgToL1 {
                    to_address: gen::Felt::try_new("0x1")?,
                    payload: vec![
                        gen::Felt::try_new("0x1")?,
                        gen::Felt::try_new("0x1")?,
                    ],
                    from_address: gen::Felt::try_new("0x1")?,
                }],
                events: vec![gen::Event(gen::EventContent {
                    data: vec![
                        gen::Felt::try_new("0x1")?,
                        gen::Felt::try_new("0x1")?,
                    ],
                    keys: vec![
                        gen::Felt::try_new("0x1")?,
                        gen::Felt::try_new("0x1")?,
                    ],
                })],
                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
                actual_fee: gen::FeePayment {
                    amount: gen::Felt::try_new("0x1")?,
                    unit: gen::PriceUnit::Wei,
                },
                block_hash: gen::BlockHash(gen::Felt::try_new("0x1")?),
                block_number: gen::BlockNumber::try_new(42)?,
                execution_status: gen::TxnExecutionStatus::Succeeded,
                finality_status: gen::TxnFinalityStatus::AcceptedOnL2,
                revert_reason: Some("reason".to_string()),
                execution_resources: gen::ExecutionResources {
                    steps: 42,
                    memory_holes: Some(42),
                    range_check_builtin_applications: Some(1),
                    pedersen_builtin_applications: Some(1),
                    poseidon_builtin_applications: Some(1),
                    ec_op_builtin_applications: Some(1),
                    ecdsa_builtin_applications: Some(1),
                    bitwise_builtin_applications: Some(1),
                    keccak_builtin_applications: Some(1),
                    segment_arena_builtin: Some(1),
                },
            },
            contract_address: gen::Felt::try_new("0x1")?,
            r#type: gen::DeployTxnReceiptType::Deploy,
        });
        let result = gen::GetTransactionReceiptResult::TxnReceipt(result);
        log::debug!(
            "transaction_hash={transaction_hash:?}\nresult={result:#?}"
        );
        Ok(result)
    }

    fn getClass(
        &self,
        block_id: gen::BlockId,
        class_hash: gen::Felt,
    ) -> std::result::Result<gen::GetClassResult, jsonrpc::Error> {
        let result = gen::GetClassResult::ContractClass(gen::ContractClass {
            entry_points_by_type: gen::ContractClassEntryPointsByType {
                constructor: vec![gen::SierraEntryPoint {
                    selector: gen::Felt::try_new("0x11")?,
                    function_idx: 1,
                }],
                external: vec![gen::SierraEntryPoint {
                    selector: gen::Felt::try_new("0x22")?,
                    function_idx: 2,
                }],
                l1_handler: vec![gen::SierraEntryPoint {
                    selector: gen::Felt::try_new("0x33")?,
                    function_idx: 3,
                }],
            },
            abi: Some("abi".to_string()),
            sierra_program: vec![gen::Felt::try_new("0xABCD")?],
            contract_class_version: "0".to_string(),
        });
        log::debug!("block_id={block_id:?}\nclass_hash={class_hash:?}\nresult={result:#?}");
        Ok(result)
    }

    fn getClassHashAt(
        &self,
        block_id: gen::BlockId,
        contract_address: gen::Address,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        let result = gen::Felt::try_new("0xF")?;
        log::debug!(
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
            entry_points_by_type: gen::ContractClassEntryPointsByType {
                constructor: vec![gen::SierraEntryPoint {
                    selector: gen::Felt::try_new("0x11")?,
                    function_idx: 1,
                }],
                external: vec![gen::SierraEntryPoint {
                    selector: gen::Felt::try_new("0x22")?,
                    function_idx: 2,
                }],
                l1_handler: vec![gen::SierraEntryPoint {
                    selector: gen::Felt::try_new("0x33")?,
                    function_idx: 3,
                }],
            },
            abi: Some("abi".to_string()),
            sierra_program: vec![gen::Felt::try_new("0x44")?],
            contract_class_version: "0".to_string(),
        });
        log::debug!(
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
        log::debug!("block_id={block_id:?}\nresult={result:#?}");
        Ok(result)
    }

    fn call(
        &self,
        request: gen::FunctionCall,
        block_id: gen::BlockId,
    ) -> std::result::Result<Vec<gen::Felt>, jsonrpc::Error> {
        let result = vec![gen::Felt::try_new("0x0")?];
        log::debug!(
            "block_id={block_id:?}\nreques={request:#?}\nresult={result:#?}"
        );
        Ok(result)
    }

    fn estimateFee(
        &self,
        request: Vec<gen::BroadcastedTxn>,
        simulation_flags: Vec<gen::SimulationFlagForEstimateFee>,
        block_id: gen::BlockId,
    ) -> std::result::Result<Vec<gen::FeeEstimate>, jsonrpc::Error> {
        let result = vec![gen::FeeEstimate {
            gas_consumed: gen::Felt::try_new("0xAA")?,
            gas_price: gen::Felt::try_new("0xBB")?,
            overall_fee: gen::Felt::try_new("0xCC")?,
            unit: gen::PriceUnit::Wei,
        }];
        log::debug!(
            "block_id={block_id:?}\nsimulation_flags={simulation_flags:?}\nreques={request:#?}\nresult={result:#?}"
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
            block_number: gen::BlockNumber::try_new(42)?,
            block_hash: gen::BlockHash(gen::Felt::try_new("0xface")?),
        })
    }

    fn chainId(&self) -> std::result::Result<gen::ChainId, jsonrpc::Error> {
        Ok(gen::ChainId::try_new("0xdeadbeef")?)
    }

    fn syncing(
        &self,
    ) -> std::result::Result<gen::SyncingResult, jsonrpc::Error> {
        Ok(gen::SyncingResult::SyncStatus(gen::SyncStatus {
            starting_block_num: gen::BlockNumber::try_new(42)?,
            current_block_hash: gen::BlockHash(gen::Felt::try_new("0x2")?),
            starting_block_hash: gen::BlockHash(gen::Felt::try_new("0x3")?),
            current_block_num: gen::BlockNumber::try_new(42)?,
            highest_block_hash: gen::BlockHash(gen::Felt::try_new("0x5")?),
            highest_block_num: gen::BlockNumber::try_new(42)?,
        }))
    }

    fn getEvents(
        &self,
        filter: gen::GetEventsFilter,
    ) -> std::result::Result<gen::EventsChunk, jsonrpc::Error> {
        let result = gen::EventsChunk {
            continuation_token: Some("token-0".to_string()),
            events: vec![gen::EmittedEvent {
                event: gen::Event(gen::EventContent {
                    keys: vec![gen::Felt::try_new("0x4")?],
                    data: vec![gen::Felt::try_new("0x3")?],
                }),
                block_hash: Some(gen::BlockHash(gen::Felt::try_new("0x2")?)),
                block_number: Some(gen::BlockNumber::try_new(42)?),
                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
            }],
        };
        log::debug!("filter={filter:#?}\nresult={result:#?}");
        Ok(result)
    }

    fn getNonce(
        &self,
        block_id: gen::BlockId,
        contract_address: gen::Address,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        let result = gen::Felt::try_new("0xA")?;
        log::debug!(
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
            transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
        };
        log::debug!(
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
            class_hash: gen::Felt::try_new("0x1")?,
            transaction_hash: gen::TxnHash(gen::Felt::try_new("0x2")?),
        };
        log::debug!(
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
            transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
            contract_address: gen::Felt::try_new("0x2")?,
        };
        log::debug!("deploy_account_transaction={deploy_account_transaction:#?}\nresult={result:#?}");
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

    fn traceBlockTransactions(
        &self,
        _block_hash: gen::BlockId,
    ) -> std::result::Result<Vec<gen::BlockTransaction>, jsonrpc::Error> {
        Err(jsonrpc::Error {
            code: 1,
            message: "unimplemented".to_owned(),
        })
    }

    fn specVersion(&self) -> std::result::Result<String, jsonrpc::Error> {
        Ok("0.0.0".to_string())
    }

    fn getTransactionStatus(
        &self,
        _transaction_hash: gen::TxnHash,
    ) -> std::result::Result<gen::GetTransactionStatusResult, jsonrpc::Error>
    {
        Err(jsonrpc::Error {
            code: 1,
            message: "unimplemented".to_owned(),
        })
    }

    fn estimateMessageFee(
        &self,
        _message: gen::MsgFromL1,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::FeeEstimate, jsonrpc::Error> {
        Err(jsonrpc::Error {
            code: 1,
            message: "unimplemented".to_owned(),
        })
    }

    fn simulateTransactions(
        &self,
        _block_id: gen::BlockId,
        _transactions: Vec<gen::BroadcastedTxn>,
        _simulation_flags: Vec<gen::SimulationFlag>,
    ) -> std::result::Result<Vec<gen::SimulatedTransaction>, jsonrpc::Error>
    {
        Err(jsonrpc::Error {
            code: 1,
            message: "unimplemented".to_owned(),
        })
    }
}
// vvv GENERATED CODE BELOW vvv
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(clippy::enum_variant_names)]
pub mod gen {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use iamgroot::jsonrpc;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV0Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV0Version {
        #[serde(rename = "0x0")]
        V0x0,
        #[serde(rename = "0x100000000000000000000000000000000")]
        V0x100000000000000000000000000000000,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum PendingDeployAccountTxnReceiptType {
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum InvokeTxnTraceExecuteInvocation {
        FunctionInvocation(FunctionInvocation),
        RevertReason {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            revert_reason: Option<String>,
        },
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnTraceType {
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct BlockHeaderTimestamp(i64);

    mod blockheadertimestamp {
        use super::jsonrpc;
        use super::BlockHeaderTimestamp;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl BlockHeaderTimestamp {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("BlockHeaderTimestamp value {value} must be > {MIN}"),
                });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("BlockHeaderTimestamp value {value} must be < {MAX}"),
                });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for BlockHeaderTimestamp {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for BlockHeaderTimestamp {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ContractClassEntryPointsByType {
        pub constructor: Vec<SierraEntryPoint>,
        pub external: Vec<SierraEntryPoint>,
        pub l1_handler: Vec<SierraEntryPoint>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV3Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV3Version {
        #[serde(rename = "0x3")]
        V0x3,
        #[serde(rename = "0x100000000000000000000000000000003")]
        V0x100000000000000000000000000000003,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct StructAbiEntrySize(i64);

    mod structabientrysize {
        use super::jsonrpc;
        use super::StructAbiEntrySize;

        static MIN: i64 = 1;
        static MAX: i64 = 9223372036854775807;

        impl StructAbiEntrySize {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "StructAbiEntrySize value {value} must be > {MIN}"
                        ),
                    });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "StructAbiEntrySize value {value} must be < {MAX}"
                        ),
                    });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for StructAbiEntrySize {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for StructAbiEntrySize {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct ResultPageRequestChunkSize(i64);

    mod resultpagerequestchunksize {
        use super::jsonrpc;
        use super::ResultPageRequestChunkSize;

        static MIN: i64 = 1;
        static MAX: i64 = 9223372036854775807;

        impl ResultPageRequestChunkSize {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("ResultPageRequestChunkSize value {value} must be > {MIN}"),
                });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("ResultPageRequestChunkSize value {value} must be < {MAX}"),
                });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for ResultPageRequestChunkSize {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for ResultPageRequestChunkSize {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV0Type {
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV0Version {
        #[serde(rename = "0x0")]
        V0x0,
        #[serde(rename = "0x100000000000000000000000000000000")]
        V0x100000000000000000000000000000000,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV2Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV2Version {
        #[serde(rename = "0x2")]
        V0x2,
        #[serde(rename = "0x100000000000000000000000000000002")]
        V0x100000000000000000000000000000002,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum PendingDeclareTxnReceiptType {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV1Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV1Version {
        #[serde(rename = "0x1")]
        V0x1,
        #[serde(rename = "0x100000000000000000000000000000001")]
        V0x100000000000000000000000000000001,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnReceiptType {
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployTxnType {
        #[serde(rename = "DEPLOY")]
        Deploy,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnReceiptType {
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV2Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV2Version {
        #[serde(rename = "0x2")]
        V0x2,
        #[serde(rename = "0x100000000000000000000000000000002")]
        V0x100000000000000000000000000000002,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum L1HandlerTxnReceiptType {
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV1Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV1Version {
        #[serde(rename = "0x1")]
        V0x1,
        #[serde(rename = "0x100000000000000000000000000000001")]
        V0x100000000000000000000000000000001,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV3Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV3Version {
        #[serde(rename = "0x3")]
        V0x3,
        #[serde(rename = "0x100000000000000000000000000000003")]
        V0x100000000000000000000000000000003,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum PendingL1HandlerTxnReceiptType {
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum PendingInvokeTxnReceiptType {
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV1Type {
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV1Version {
        #[serde(rename = "0x1")]
        V0x1,
        #[serde(rename = "0x100000000000000000000000000000001")]
        V0x100000000000000000000000000000001,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployTxnReceiptType {
        #[serde(rename = "DEPLOY")]
        Deploy,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum L1HandlerTxnTraceType {
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct PendingBlockHeaderTimestamp(i64);

    mod pendingblockheadertimestamp {
        use super::jsonrpc;
        use super::PendingBlockHeaderTimestamp;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl PendingBlockHeaderTimestamp {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("PendingBlockHeaderTimestamp value {value} must be > {MIN}"),
                });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("PendingBlockHeaderTimestamp value {value} must be < {MAX}"),
                });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for PendingBlockHeaderTimestamp {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for PendingBlockHeaderTimestamp {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnV3Type {
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnV3Version {
        #[serde(rename = "0x3")]
        V0x3,
        #[serde(rename = "0x100000000000000000000000000000003")]
        V0x100000000000000000000000000000003,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum PendingCommonReceiptPropertiesFinalityStatus {
        #[serde(rename = "ACCEPTED_ON_L2")]
        AcceptedOnL2,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV3Type {
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV3Version {
        #[serde(rename = "0x3")]
        V0x3,
        #[serde(rename = "0x100000000000000000000000000000003")]
        V0x100000000000000000000000000000003,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnTraceType {
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnReceiptType {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnTraceType {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnV1Type {
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnV1Version {
        #[serde(rename = "0x1")]
        V0x1,
        #[serde(rename = "0x100000000000000000000000000000001")]
        V0x100000000000000000000000000000001,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum L1HandlerTxnType {
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV0 {
        pub r#type: DeclareTxnV0Type,
        pub sender_address: Address,
        pub max_fee: Felt,
        pub version: DeclareTxnV0Version,
        pub signature: Signature,
        pub class_hash: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NestedCall(pub FunctionInvocation);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StorageDiffItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub key: Option<Felt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub value: Option<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum InvokeTxn {
        InvokeTxnV0(InvokeTxnV0),
        InvokeTxnV1(InvokeTxnV1),
        InvokeTxnV3(InvokeTxnV3),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BlockStatus {
        #[serde(rename = "PENDING")]
        Pending,
        #[serde(rename = "ACCEPTED_ON_L2")]
        AcceptedOnL2,
        #[serde(rename = "ACCEPTED_ON_L1")]
        AcceptedOnL1,
        #[serde(rename = "REJECTED")]
        Rejected,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TxnHash(pub Felt);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingDeployAccountTxnReceipt {
        #[serde(flatten)]
        pub pending_common_receipt_properties: PendingCommonReceiptProperties,
        pub r#type: PendingDeployAccountTxnReceiptType,
        pub contract_address: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BlockId {
        BlockHash { block_hash: BlockHash },
        BlockNumber { block_number: BlockNumber },
        BlockTag(BlockTag),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BlockTag {
        #[serde(rename = "latest")]
        Latest,
        #[serde(rename = "pending")]
        Pending,
    }

    type ContractAbi = Vec<ContractAbiEntry>;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FeeEstimate {
        pub gas_consumed: Felt,
        pub gas_price: Felt,
        pub overall_fee: Felt,
        pub unit: PriceUnit,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SyncStatus {
        pub starting_block_hash: BlockHash,
        pub starting_block_num: BlockNumber,
        pub current_block_hash: BlockHash,
        pub current_block_num: BlockNumber,
        pub highest_block_hash: BlockHash,
        pub highest_block_num: BlockNumber,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FeePayment {
        pub amount: Felt,
        pub unit: PriceUnit,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StateUpdate {
        pub block_hash: BlockHash,
        pub old_root: Felt,
        pub new_root: Felt,
        pub state_diff: StateDiff,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Event(pub EventContent);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct U64(String);

    mod u64 {
        use super::jsonrpc;
        use super::U64;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static U64_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,15})$").unwrap()
        });

        impl U64 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if U64_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "U64 value does not match regex: {value}"
                        ),
                    })
                }
            }
        }

        impl TryFrom<String> for U64 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for U64 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnTrace {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub validate_invocation: Option<FunctionInvocation>,
        pub execute_invocation: InvokeTxnTraceExecuteInvocation,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub fee_transfer_invocation: Option<FunctionInvocation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub state_diff: Option<StateDiff>,
        pub r#type: InvokeTxnTraceType,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MsgFromL1 {
        pub from_address: EthAddress,
        pub to_address: Address,
        pub entry_point_selector: Felt,
        pub payload: Vec<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventsChunk {
        pub events: Vec<EmittedEvent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub continuation_token: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TypedParameter {
        pub name: String,
        pub r#type: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockHeader {
        pub block_hash: BlockHash,
        pub parent_hash: BlockHash,
        pub block_number: BlockNumber,
        pub new_root: Felt,
        pub timestamp: BlockHeaderTimestamp,
        pub sequencer_address: Felt,
        pub l1_gas_price: ResourcePrice,
        pub starknet_version: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ContractClass {
        pub sierra_program: Vec<Felt>,
        pub contract_class_version: String,
        pub entry_points_by_type: ContractClassEntryPointsByType,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub abi: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockBodyWithTxHashes {
        pub transactions: Vec<TxnHash>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingStateUpdate {
        pub old_root: Felt,
        pub state_diff: StateDiff,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventFilter {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub from_block: Option<BlockId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub to_block: Option<BlockId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub address: Option<Address>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub keys: Option<Vec<Vec<Felt>>>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedInvokeTxn(pub InvokeTxn);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum FunctionAbiType {
        #[serde(rename = "function")]
        Function,
        #[serde(rename = "l1_handler")]
        L1Handler,
        #[serde(rename = "constructor")]
        Constructor,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeclareTxnV3 {
        pub r#type: BroadcastedDeclareTxnV3Type,
        pub sender_address: Address,
        pub compiled_class_hash: Felt,
        pub version: BroadcastedDeclareTxnV3Version,
        pub signature: Signature,
        pub nonce: Felt,
        pub contract_class: ContractClass,
        pub resource_bounds: ResourceBoundsMapping,
        pub tip: U64,
        pub paymaster_data: Vec<Felt>,
        pub account_deployment_data: Vec<Felt>,
        pub nonce_data_availability_mode: DaMode,
        pub fee_data_availability_mode: DaMode,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct U128(String);

    mod u128 {
        use super::jsonrpc;
        use super::U128;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static U128_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,31})$").unwrap()
        });

        impl U128 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if U128_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "U128 value does not match regex: {value}"
                        ),
                    })
                }
            }
        }

        impl TryFrom<String> for U128 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for U128 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StructAbiEntry {
        pub r#type: StructAbiType,
        pub name: String,
        pub size: StructAbiEntrySize,
        pub members: Vec<StructMember>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResultPageRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub continuation_token: Option<String>,
        pub chunk_size: ResultPageRequestChunkSize,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnV0 {
        pub r#type: InvokeTxnV0Type,
        pub max_fee: Felt,
        pub version: InvokeTxnV0Version,
        pub signature: Signature,
        pub contract_address: Address,
        pub entry_point_selector: Felt,
        pub calldata: Vec<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StateDiff {
        pub storage_diffs: Vec<ContractStorageDiffItem>,
        pub deprecated_declared_classes: Vec<Felt>,
        pub declared_classes: Vec<DeclaredClass>,
        pub deployed_contracts: Vec<DeployedContractItem>,
        pub replaced_classes: Vec<ReplacedClass>,
        pub nonces: Vec<NonceUpdate>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeployAccountTxn(pub DeployAccountTxn);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum StructAbiType {
        #[serde(rename = "struct")]
        Struct,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingBlockWithTxHashes {
        #[serde(flatten)]
        pub block_body_with_tx_hashes: BlockBodyWithTxHashes,
        #[serde(flatten)]
        pub pending_block_header: PendingBlockHeader,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct Felt(String);

    mod felt {
        use super::jsonrpc;
        use super::Felt;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static FELT_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$").unwrap()
        });

        impl Felt {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if FELT_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "Felt value does not match regex: {value}"
                        ),
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

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockWithTxHashes {
        pub status: BlockStatus,
        #[serde(flatten)]
        pub block_header: BlockHeader,
        #[serde(flatten)]
        pub block_body_with_tx_hashes: BlockBodyWithTxHashes,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV2 {
        pub r#type: DeclareTxnV2Type,
        pub sender_address: Address,
        pub compiled_class_hash: Felt,
        pub max_fee: Felt,
        pub version: DeclareTxnV2Version,
        pub signature: Signature,
        pub nonce: Felt,
        pub class_hash: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum PendingTxnReceipt {
        PendingInvokeTxnReceipt(PendingInvokeTxnReceipt),
        PendingL1HandlerTxnReceipt(PendingL1HandlerTxnReceipt),
        PendingDeclareTxnReceipt(PendingDeclareTxnReceipt),
        PendingDeployAccountTxnReceipt(PendingDeployAccountTxnReceipt),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DaMode {
        #[serde(rename = "L1")]
        L1,
        #[serde(rename = "L2")]
        L2,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TxnExecutionStatus {
        #[serde(rename = "SUCCEEDED")]
        Succeeded,
        #[serde(rename = "REVERTED")]
        Reverted,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum Txn {
        InvokeTxn(InvokeTxn),
        L1HandlerTxn(L1HandlerTxn),
        DeclareTxn(DeclareTxn),
        DeployTxn(DeployTxn),
        DeployAccountTxn(DeployAccountTxn),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockBodyWithTxs {
        pub transactions: Vec<TxnWithHash>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockWithTxs {
        pub status: BlockStatus,
        #[serde(flatten)]
        pub block_header: BlockHeader,
        #[serde(flatten)]
        pub block_body_with_txs: BlockBodyWithTxs,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BroadcastedDeclareTxn {
        BroadcastedDeclareTxnV1(BroadcastedDeclareTxnV1),
        BroadcastedDeclareTxnV2(BroadcastedDeclareTxnV2),
        BroadcastedDeclareTxnV3(BroadcastedDeclareTxnV3),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResourcePrice {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub price_in_fri: Option<Felt>,
        pub price_in_wei: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployedContractItem {
        pub address: Felt,
        pub class_hash: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum DeployAccountTxn {
        DeployAccountTxnV1(DeployAccountTxnV1),
        DeployAccountTxnV3(DeployAccountTxnV3),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct OrderedMessage {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub order: Option<i64>,
        #[serde(flatten)]
        pub msg_to_l1: MsgToL1,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingDeclareTxnReceipt {
        pub r#type: PendingDeclareTxnReceiptType,
        #[serde(flatten)]
        pub pending_common_receipt_properties: PendingCommonReceiptProperties,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclaredClass {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub class_hash: Option<Felt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub compiled_class_hash: Option<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV1 {
        pub r#type: DeclareTxnV1Type,
        pub sender_address: Address,
        pub max_fee: Felt,
        pub version: DeclareTxnV1Version,
        pub signature: Signature,
        pub nonce: Felt,
        pub class_hash: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum EventAbiType {
        #[serde(rename = "event")]
        Event,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResourceBounds {
        pub max_amount: U64,
        pub max_price_per_unit: U128,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EntryPoints {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub constructor: Option<Vec<DeprecatedCairoEntryPoint>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub external: Option<Vec<DeprecatedCairoEntryPoint>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub l1_handler: Option<Vec<DeprecatedCairoEntryPoint>>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FunctionAbiEntry {
        pub r#type: FunctionAbiType,
        pub name: String,
        pub inputs: Vec<TypedParameter>,
        pub outputs: Vec<TypedParameter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub statemutability: Option<FunctionStateMutability>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        pub r#type: DeployAccountTxnReceiptType,
        pub contract_address: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CommonReceiptProperties {
        pub transaction_hash: TxnHash,
        pub actual_fee: FeePayment,
        pub execution_status: TxnExecutionStatus,
        pub finality_status: TxnFinalityStatus,
        pub block_hash: BlockHash,
        pub block_number: BlockNumber,
        pub messages_sent: Vec<MsgToL1>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub revert_reason: Option<String>,
        pub events: Vec<Event>,
        pub execution_resources: ExecutionResources,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct OrderedEvent {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub order: Option<i64>,
        #[serde(flatten)]
        pub event: Event,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployTxn {
        pub version: Felt,
        pub r#type: DeployTxnType,
        pub contract_address_salt: Felt,
        pub constructor_calldata: Vec<Felt>,
        pub class_hash: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnReceipt {
        pub r#type: InvokeTxnReceiptType,
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventContent {
        pub keys: Vec<Felt>,
        pub data: Vec<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TxnWithHash {
        #[serde(flatten)]
        pub txn: Txn,
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
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
                if ETHADDRESS_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "EthAddress value does not match regex: {value}"
                        ),
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

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FunctionCall {
        pub contract_address: Address,
        pub entry_point_selector: Felt,
        pub calldata: Vec<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MsgToL1 {
        pub from_address: Felt,
        pub to_address: Felt,
        pub payload: Vec<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TxnStatus {
        #[serde(rename = "RECEIVED")]
        Received,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "ACCEPTED_ON_L2")]
        AcceptedOnL2,
        #[serde(rename = "ACCEPTED_ON_L1")]
        AcceptedOnL1,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TxnType {
        #[serde(rename = "DECLARE")]
        Declare,
        #[serde(rename = "DEPLOY")]
        Deploy,
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
        #[serde(rename = "INVOKE")]
        Invoke,
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FunctionInvocation {
        #[serde(flatten)]
        pub function_call: FunctionCall,
        pub caller_address: Felt,
        pub class_hash: Felt,
        pub entry_point_type: EntryPointType,
        pub call_type: CallType,
        pub result: Vec<Felt>,
        pub calls: Vec<NestedCall>,
        pub events: Vec<OrderedEvent>,
        pub messages: Vec<OrderedMessage>,
        pub execution_resources: ExecutionResources,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExecutionResources {
        pub steps: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub memory_holes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub range_check_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub pedersen_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub poseidon_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub ec_op_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub ecdsa_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub bitwise_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub keccak_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub segment_arena_builtin: Option<i64>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TransactionTrace {
        InvokeTxnTrace(InvokeTxnTrace),
        DeclareTxnTrace(DeclareTxnTrace),
        DeployAccountTxnTrace(DeployAccountTxnTrace),
        L1HandlerTxnTrace(L1HandlerTxnTrace),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeclareTxnV2 {
        pub r#type: BroadcastedDeclareTxnV2Type,
        pub sender_address: Address,
        pub compiled_class_hash: Felt,
        pub max_fee: Felt,
        pub version: BroadcastedDeclareTxnV2Version,
        pub signature: Signature,
        pub nonce: Felt,
        pub contract_class: ContractClass,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Address(pub Felt);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum FunctionStateMutability {
        #[serde(rename = "view")]
        View,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum PriceUnit {
        #[serde(rename = "WEI")]
        Wei,
        #[serde(rename = "FRI")]
        Fri,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1HandlerTxnReceipt {
        pub r#type: L1HandlerTxnReceiptType,
        pub message_hash: NumAsHex,
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SierraEntryPoint {
        pub selector: Felt,
        pub function_idx: i64,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EmittedEvent {
        #[serde(flatten)]
        pub event: Event,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub block_hash: Option<BlockHash>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub block_number: Option<BlockNumber>,
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum EntryPointType {
        #[serde(rename = "EXTERNAL")]
        External,
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
        #[serde(rename = "CONSTRUCTOR")]
        Constructor,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BroadcastedTxn {
        BroadcastedInvokeTxn(BroadcastedInvokeTxn),
        BroadcastedDeclareTxn(BroadcastedDeclareTxn),
        BroadcastedDeployAccountTxn(BroadcastedDeployAccountTxn),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct ChainId(String);

    mod chainid {
        use super::jsonrpc;
        use super::ChainId;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static CHAINID_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x[a-fA-F0-9]+$").unwrap());

        impl ChainId {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if CHAINID_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "ChainId value does not match regex: {value}"
                        ),
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

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeclareTxnV1 {
        pub r#type: BroadcastedDeclareTxnV1Type,
        pub sender_address: Address,
        pub max_fee: Felt,
        pub version: BroadcastedDeclareTxnV1Version,
        pub signature: Signature,
        pub nonce: Felt,
        pub contract_class: DeprecatedContractClass,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResourceBoundsMapping {
        pub l1_gas: ResourceBounds,
        pub l2_gas: ResourceBounds,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockHash(pub Felt);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ContractStorageDiffItem {
        pub address: Felt,
        pub storage_entries: Vec<StorageDiffItem>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum SimulationFlag {
        #[serde(rename = "SKIP_VALIDATE")]
        SkipValidate,
        #[serde(rename = "SKIP_FEE_CHARGE")]
        SkipFeeCharge,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct NumAsHex(String);

    mod numashex {
        use super::jsonrpc;
        use super::NumAsHex;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static NUMASHEX_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x[a-fA-F0-9]+$").unwrap());

        impl NumAsHex {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if NUMASHEX_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "NumAsHex value does not match regex: {value}"
                        ),
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

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeprecatedCairoEntryPoint {
        pub offset: NumAsHex,
        pub selector: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV3 {
        pub r#type: DeclareTxnV3Type,
        pub sender_address: Address,
        pub compiled_class_hash: Felt,
        pub version: DeclareTxnV3Version,
        pub signature: Signature,
        pub nonce: Felt,
        pub class_hash: Felt,
        pub resource_bounds: ResourceBoundsMapping,
        pub tip: U64,
        pub paymaster_data: Vec<Felt>,
        pub account_deployment_data: Vec<Felt>,
        pub nonce_data_availability_mode: DaMode,
        pub fee_data_availability_mode: DaMode,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingL1HandlerTxnReceipt {
        pub r#type: PendingL1HandlerTxnReceiptType,
        pub message_hash: NumAsHex,
        #[serde(flatten)]
        pub pending_common_receipt_properties: PendingCommonReceiptProperties,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ReplacedClass {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub contract_address: Option<Address>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub class_hash: Option<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
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
                if STORAGEKEY_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "StorageKey value does not match regex: {value}"
                        ),
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

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingInvokeTxnReceipt {
        pub r#type: PendingInvokeTxnReceiptType,
        #[serde(flatten)]
        pub pending_common_receipt_properties: PendingCommonReceiptProperties,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnV1 {
        pub r#type: InvokeTxnV1Type,
        pub sender_address: Address,
        pub calldata: Vec<Felt>,
        pub max_fee: Felt,
        pub version: InvokeTxnV1Version,
        pub signature: Signature,
        pub nonce: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NonceUpdate {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub contract_address: Option<Address>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub nonce: Option<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct BlockNumber(i64);

    mod blocknumber {
        use super::jsonrpc;
        use super::BlockNumber;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl BlockNumber {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "BlockNumber value {value} must be > {MIN}"
                        ),
                    });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "BlockNumber value {value} must be < {MAX}"
                        ),
                    });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for BlockNumber {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for BlockNumber {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        pub r#type: DeployTxnReceiptType,
        pub contract_address: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TxnReceipt {
        InvokeTxnReceipt(InvokeTxnReceipt),
        L1HandlerTxnReceipt(L1HandlerTxnReceipt),
        DeclareTxnReceipt(DeclareTxnReceipt),
        DeployTxnReceipt(DeployTxnReceipt),
        DeployAccountTxnReceipt(DeployAccountTxnReceipt),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum CallType {
        #[serde(rename = "LIBRARY_CALL")]
        LibraryCall,
        #[serde(rename = "CALL")]
        Call,
        #[serde(rename = "DELEGATE")]
        Delegate,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1HandlerTxnTrace {
        pub function_invocation: FunctionInvocation,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub state_diff: Option<StateDiff>,
        pub r#type: L1HandlerTxnTraceType,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingBlockHeader {
        pub parent_hash: BlockHash,
        pub timestamp: PendingBlockHeaderTimestamp,
        pub sequencer_address: Felt,
        pub l1_gas_price: ResourcePrice,
        pub starknet_version: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct Program(String);

    mod program {
        use super::jsonrpc;
        use super::Program;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static PROGRAM_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{3}=|[A-Za-z0-9+/]{2}==)?$").unwrap()
        });

        impl Program {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if PROGRAM_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "Program value does not match regex: {value}"
                        ),
                    })
                }
            }
        }

        impl TryFrom<String> for Program {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Program {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum SimulationFlagForEstimateFee {
        #[serde(rename = "SKIP_VALIDATE")]
        SkipValidate,
    }

    type Signature = Vec<Felt>;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TxnFinalityStatus {
        #[serde(rename = "ACCEPTED_ON_L2")]
        AcceptedOnL2,
        #[serde(rename = "ACCEPTED_ON_L1")]
        AcceptedOnL1,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnV3 {
        pub r#type: DeployAccountTxnV3Type,
        pub version: DeployAccountTxnV3Version,
        pub signature: Signature,
        pub nonce: Felt,
        pub contract_address_salt: Felt,
        pub constructor_calldata: Vec<Felt>,
        pub class_hash: Felt,
        pub resource_bounds: ResourceBoundsMapping,
        pub tip: U64,
        pub paymaster_data: Vec<Felt>,
        pub nonce_data_availability_mode: DaMode,
        pub fee_data_availability_mode: DaMode,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingCommonReceiptProperties {
        pub transaction_hash: TxnHash,
        pub actual_fee: FeePayment,
        pub messages_sent: Vec<MsgToL1>,
        pub events: Vec<Event>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub revert_reason: Option<String>,
        pub finality_status: PendingCommonReceiptPropertiesFinalityStatus,
        pub execution_status: TxnExecutionStatus,
        pub execution_resources: ExecutionResources,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingBlockWithTxs {
        #[serde(flatten)]
        pub block_body_with_txs: BlockBodyWithTxs,
        #[serde(flatten)]
        pub pending_block_header: PendingBlockHeader,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum ContractAbiEntry {
        FunctionAbiEntry(FunctionAbiEntry),
        EventAbiEntry(EventAbiEntry),
        StructAbiEntry(StructAbiEntry),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnV3 {
        pub r#type: InvokeTxnV3Type,
        pub sender_address: Address,
        pub calldata: Vec<Felt>,
        pub version: InvokeTxnV3Version,
        pub signature: Signature,
        pub nonce: Felt,
        pub resource_bounds: ResourceBoundsMapping,
        pub tip: U64,
        pub paymaster_data: Vec<Felt>,
        pub account_deployment_data: Vec<Felt>,
        pub nonce_data_availability_mode: DaMode,
        pub fee_data_availability_mode: DaMode,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum DeclareTxn {
        DeclareTxnV0(DeclareTxnV0),
        DeclareTxnV1(DeclareTxnV1),
        DeclareTxnV2(DeclareTxnV2),
        DeclareTxnV3(DeclareTxnV3),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnTrace {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub validate_invocation: Option<FunctionInvocation>,
        pub constructor_invocation: FunctionInvocation,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub fee_transfer_invocation: Option<FunctionInvocation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub state_diff: Option<StateDiff>,
        pub r#type: DeployAccountTxnTraceType,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StructMember {
        #[serde(flatten)]
        pub typed_parameter: TypedParameter,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub offset: Option<i64>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnReceipt {
        pub r#type: DeclareTxnReceiptType,
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeprecatedContractClass {
        pub program: Program,
        pub entry_points_by_type: EntryPoints,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub abi: Option<ContractAbi>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnTrace {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub validate_invocation: Option<FunctionInvocation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub fee_transfer_invocation: Option<FunctionInvocation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub state_diff: Option<StateDiff>,
        pub r#type: DeclareTxnTraceType,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnV1 {
        pub r#type: DeployAccountTxnV1Type,
        pub max_fee: Felt,
        pub version: DeployAccountTxnV1Version,
        pub signature: Signature,
        pub nonce: Felt,
        pub contract_address_salt: Felt,
        pub constructor_calldata: Vec<Felt>,
        pub class_hash: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1HandlerTxn {
        pub version: Felt,
        pub r#type: L1HandlerTxnType,
        pub nonce: NumAsHex,
        #[serde(flatten)]
        pub function_call: FunctionCall,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventAbiEntry {
        pub r#type: EventAbiType,
        pub name: String,
        pub keys: Vec<TypedParameter>,
        pub data: Vec<TypedParameter>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetBlockWithTxHashesResult {
        BlockWithTxHashes(BlockWithTxHashes),
        PendingBlockWithTxHashes(PendingBlockWithTxHashes),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetBlockWithTxsResult {
        BlockWithTxs(BlockWithTxs),
        PendingBlockWithTxs(PendingBlockWithTxs),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetStateUpdateResult {
        StateUpdate(StateUpdate),
        PendingStateUpdate(PendingStateUpdate),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetTransactionStatusResult {
        pub finality_status: TxnStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub execution_status: Option<TxnExecutionStatus>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetTransactionByHashResult {
        #[serde(flatten)]
        pub txn: Txn,
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetTransactionByBlockIdAndIndexResult {
        #[serde(flatten)]
        pub txn: Txn,
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct GetTransactionByBlockIdAndIndexIndex(i64);

    mod gettransactionbyblockidandindexindex {
        use super::jsonrpc;
        use super::GetTransactionByBlockIdAndIndexIndex;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl GetTransactionByBlockIdAndIndexIndex {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("GetTransactionByBlockIdAndIndexIndex value {value} must be > {MIN}"),
                });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("GetTransactionByBlockIdAndIndexIndex value {value} must be < {MAX}"),
                });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for GetTransactionByBlockIdAndIndexIndex {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for GetTransactionByBlockIdAndIndexIndex {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetTransactionReceiptResult {
        TxnReceipt(TxnReceipt),
        PendingTxnReceipt(PendingTxnReceipt),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetClassResult {
        DeprecatedContractClass(DeprecatedContractClass),
        ContractClass(ContractClass),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetClassAtResult {
        DeprecatedContractClass(DeprecatedContractClass),
        ContractClass(ContractClass),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct GetBlockTransactionCountResult(i64);

    mod getblocktransactioncountresult {
        use super::jsonrpc;
        use super::GetBlockTransactionCountResult;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl GetBlockTransactionCountResult {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("GetBlockTransactionCountResult value {value} must be > {MIN}"),
                });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("GetBlockTransactionCountResult value {value} must be < {MAX}"),
                });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for GetBlockTransactionCountResult {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for GetBlockTransactionCountResult {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockHashAndNumberResult {
        pub block_hash: BlockHash,
        pub block_number: BlockNumber,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum SyncingResult {
        False(bool),
        SyncStatus(SyncStatus),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetEventsFilter {
        #[serde(flatten)]
        pub event_filter: EventFilter,
        #[serde(flatten)]
        pub result_page_request: ResultPageRequest,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddInvokeTransactionResult {
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddDeclareTransactionResult {
        pub transaction_hash: TxnHash,
        pub class_hash: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddDeployAccountTransactionResult {
        pub transaction_hash: TxnHash,
        pub contract_address: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SimulatedTransaction {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub transaction_trace: Option<TransactionTrace>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub fee_estimation: Option<FeeEstimate>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockTransaction {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub transaction_hash: Option<Felt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub trace_root: Option<TransactionTrace>,
    }

    pub trait Rpc {
        /// Returns the version of the Starknet JSON-RPC specification being used
        fn specVersion(&self) -> std::result::Result<String, jsonrpc::Error>;

        /// Get block information with transaction hashes given the block id
        fn getBlockWithTxHashes(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockWithTxHashesResult, jsonrpc::Error>;

        /// Get block information with full transactions given the block id
        fn getBlockWithTxs(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockWithTxsResult, jsonrpc::Error>;

        /// Get the information about the result of executing the requested block
        fn getStateUpdate(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetStateUpdateResult, jsonrpc::Error>;

        /// Get the value of the storage at the given address and key
        fn getStorageAt(
            &self,
            contract_address: Address,
            key: StorageKey,
            block_id: BlockId,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Gets the transaction status (possibly reflecting that the tx is still in the mempool, or dropped from it)
        fn getTransactionStatus(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<GetTransactionStatusResult, jsonrpc::Error>;

        /// Get the details and status of a submitted transaction
        fn getTransactionByHash(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<GetTransactionByHashResult, jsonrpc::Error>;

        /// Get the details of a transaction by a given block id and index
        fn getTransactionByBlockIdAndIndex(
            &self,
            block_id: BlockId,
            index: GetTransactionByBlockIdAndIndexIndex,
        ) -> std::result::Result<
            GetTransactionByBlockIdAndIndexResult,
            jsonrpc::Error,
        >;

        /// Get the transaction receipt by the transaction hash
        fn getTransactionReceipt(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<GetTransactionReceiptResult, jsonrpc::Error>;

        /// Get the contract class definition in the given block associated with the given hash
        fn getClass(
            &self,
            block_id: BlockId,
            class_hash: Felt,
        ) -> std::result::Result<GetClassResult, jsonrpc::Error>;

        /// Get the contract class hash in the given block for the contract deployed at the given address
        fn getClassHashAt(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Get the contract class definition in the given block at the given address
        fn getClassAt(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<GetClassAtResult, jsonrpc::Error>;

        /// Get the number of transactions in a block given a block id
        fn getBlockTransactionCount(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockTransactionCountResult, jsonrpc::Error>;

        /// call a starknet function without creating a StarkNet transaction
        fn call(
            &self,
            request: FunctionCall,
            block_id: BlockId,
        ) -> std::result::Result<Vec<Felt>, jsonrpc::Error>;

        /// estimate the fee for of StarkNet transactions
        fn estimateFee(
            &self,
            request: Vec<BroadcastedTxn>,
            simulation_flags: Vec<SimulationFlagForEstimateFee>,
            block_id: BlockId,
        ) -> std::result::Result<Vec<FeeEstimate>, jsonrpc::Error>;

        /// estimate the L2 fee of a message sent on L1
        fn estimateMessageFee(
            &self,
            message: MsgFromL1,
            block_id: BlockId,
        ) -> std::result::Result<FeeEstimate, jsonrpc::Error>;

        /// Get the most recent accepted block number
        fn blockNumber(
            &self,
        ) -> std::result::Result<BlockNumber, jsonrpc::Error>;

        /// Get the most recent accepted block hash and number
        fn blockHashAndNumber(
            &self,
        ) -> std::result::Result<BlockHashAndNumberResult, jsonrpc::Error>;

        /// Return the currently configured StarkNet chain id
        fn chainId(&self) -> std::result::Result<ChainId, jsonrpc::Error>;

        /// Returns an object about the sync status, or false if the node is not synching
        fn syncing(&self)
            -> std::result::Result<SyncingResult, jsonrpc::Error>;

        /// Returns all events matching the given filter
        fn getEvents(
            &self,
            filter: GetEventsFilter,
        ) -> std::result::Result<EventsChunk, jsonrpc::Error>;

        /// Get the nonce associated with the given address in the given block
        fn getNonce(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Submit a new transaction to be added to the chain
        fn addInvokeTransaction(
            &self,
            invoke_transaction: BroadcastedInvokeTxn,
        ) -> std::result::Result<AddInvokeTransactionResult, jsonrpc::Error>;

        /// Submit a new class declaration transaction
        fn addDeclareTransaction(
            &self,
            declare_transaction: BroadcastedDeclareTxn,
        ) -> std::result::Result<AddDeclareTransactionResult, jsonrpc::Error>;

        /// Submit a new deploy account transaction
        fn addDeployAccountTransaction(
            &self,
            deploy_account_transaction: BroadcastedDeployAccountTxn,
        ) -> std::result::Result<
            AddDeployAccountTransactionResult,
            jsonrpc::Error,
        >;

        /// For a given executed transaction, return the trace of its execution, including internal calls
        fn traceTransaction(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<TransactionTrace, jsonrpc::Error>;

        /// Simulate a given sequence of transactions on the requested state, and generate the execution traces. Note that some of the transactions may revert, in which case no error is thrown, but revert details can be seen on the returned trace object. . Note that some of the transactions may revert, this will be reflected by the revert_error property in the trace. Other types of failures (e.g. unexpected error or failure in the validation phase) will result in TRANSACTION_EXECUTION_ERROR.
        fn simulateTransactions(
            &self,
            block_id: BlockId,
            transactions: Vec<BroadcastedTxn>,
            simulation_flags: Vec<SimulationFlag>,
        ) -> std::result::Result<Vec<SimulatedTransaction>, jsonrpc::Error>;

        /// Retrieve traces for all transactions in the given block
        fn traceBlockTransactions(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<Vec<BlockTransaction>, jsonrpc::Error>;
    }

    fn handle_specVersion<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.specVersion() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getBlockWithTxHashes<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id) = args_by_pos;
                        ArgByName { block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockWithTxHashes(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getBlockWithTxs<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id) = args_by_pos;
                        ArgByName { block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockWithTxs(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getStateUpdate<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id) = args_by_pos;
                        ArgByName { block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { block_id } = args;

        match rpc.getStateUpdate(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getStorageAt<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, StorageKey, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            contract_address: Address,
            key: StorageKey,
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(contract_address, key, block_id) =
                            args_by_pos;
                        ArgByName {
                            contract_address,
                            key,
                            block_id,
                        }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName {
            contract_address,
            key,
            block_id,
        } = args;

        match rpc.getStorageAt(contract_address, key, block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getTransactionStatus<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(transaction_hash) = args_by_pos;
                        ArgByName { transaction_hash }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { transaction_hash } = args;

        match rpc.getTransactionStatus(transaction_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getTransactionByHash<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(transaction_hash) = args_by_pos;
                        ArgByName { transaction_hash }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { transaction_hash } = args;

        match rpc.getTransactionByHash(transaction_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getTransactionByBlockIdAndIndex<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, GetTransactionByBlockIdAndIndexIndex);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            index: GetTransactionByBlockIdAndIndexIndex,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, index) = args_by_pos;
                        ArgByName { block_id, index }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { block_id, index } = args;

        match rpc.getTransactionByBlockIdAndIndex(block_id, index) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getTransactionReceipt<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(transaction_hash) = args_by_pos;
                        ArgByName { transaction_hash }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { transaction_hash } = args;

        match rpc.getTransactionReceipt(transaction_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getClass<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Felt);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            class_hash: Felt,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, class_hash) = args_by_pos;
                        ArgByName {
                            block_id,
                            class_hash,
                        }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName {
            block_id,
            class_hash,
        } = args;

        match rpc.getClass(block_id, class_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getClassHashAt<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, contract_address) = args_by_pos;
                        ArgByName {
                            block_id,
                            contract_address,
                        }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName {
            block_id,
            contract_address,
        } = args;

        match rpc.getClassHashAt(block_id, contract_address) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getClassAt<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, contract_address) = args_by_pos;
                        ArgByName {
                            block_id,
                            contract_address,
                        }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName {
            block_id,
            contract_address,
        } = args;

        match rpc.getClassAt(block_id, contract_address) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getBlockTransactionCount<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id) = args_by_pos;
                        ArgByName { block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockTransactionCount(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_call<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(FunctionCall, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            request: FunctionCall,
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(request, block_id) = args_by_pos;
                        ArgByName { request, block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { request, block_id } = args;

        match rpc.call(request, block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_estimateFee<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(
            Vec<BroadcastedTxn>,
            Vec<SimulationFlagForEstimateFee>,
            BlockId,
        );

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            request: Vec<BroadcastedTxn>,
            simulation_flags: Vec<SimulationFlagForEstimateFee>,
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(request, simulation_flags, block_id) =
                            args_by_pos;
                        ArgByName {
                            request,
                            simulation_flags,
                            block_id,
                        }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName {
            request,
            simulation_flags,
            block_id,
        } = args;

        match rpc.estimateFee(request, simulation_flags, block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_estimateMessageFee<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(MsgFromL1, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            message: MsgFromL1,
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(message, block_id) = args_by_pos;
                        ArgByName { message, block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { message, block_id } = args;

        match rpc.estimateMessageFee(message, block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_blockNumber<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.blockNumber() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_blockHashAndNumber<RPC: Rpc>(
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

    fn handle_chainId<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.chainId() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_syncing<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.syncing() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getEvents<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(GetEventsFilter);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            filter: GetEventsFilter,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(filter) = args_by_pos;
                        ArgByName { filter }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { filter } = args;

        match rpc.getEvents(filter) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_getNonce<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, contract_address) = args_by_pos;
                        ArgByName {
                            block_id,
                            contract_address,
                        }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName {
            block_id,
            contract_address,
        } = args;

        match rpc.getNonce(block_id, contract_address) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_addInvokeTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedInvokeTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            invoke_transaction: BroadcastedInvokeTxn,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(invoke_transaction) = args_by_pos;
                        ArgByName { invoke_transaction }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { invoke_transaction } = args;

        match rpc.addInvokeTransaction(invoke_transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_addDeclareTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedDeclareTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            declare_transaction: BroadcastedDeclareTxn,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(declare_transaction) = args_by_pos;
                        ArgByName {
                            declare_transaction,
                        }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName {
            declare_transaction,
        } = args;

        match rpc.addDeclareTransaction(declare_transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_addDeployAccountTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedDeployAccountTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            deploy_account_transaction: BroadcastedDeployAccountTxn,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(deploy_account_transaction) = args_by_pos;
                        ArgByName {
                            deploy_account_transaction,
                        }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName {
            deploy_account_transaction,
        } = args;

        match rpc.addDeployAccountTransaction(deploy_account_transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_traceTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(transaction_hash) = args_by_pos;
                        ArgByName { transaction_hash }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { transaction_hash } = args;

        match rpc.traceTransaction(transaction_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_simulateTransactions<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Vec<BroadcastedTxn>, Vec<SimulationFlag>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            transactions: Vec<BroadcastedTxn>,
            simulation_flags: Vec<SimulationFlag>,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, transactions, simulation_flags) =
                            args_by_pos;
                        ArgByName {
                            block_id,
                            transactions,
                            simulation_flags,
                        }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName {
            block_id,
            transactions,
            simulation_flags,
        } = args;

        match rpc.simulateTransactions(block_id, transactions, simulation_flags)
        {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_traceBlockTransactions<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id) = args_by_pos;
                        ArgByName { block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(_) => {
                return jsonrpc::Response::error(-32602, "Invalid params")
            }
        };

        let ArgByName { block_id } = args;

        match rpc.traceBlockTransactions(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(_) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    pub fn handle<RPC: Rpc>(
        rpc: &RPC,
        req: &jsonrpc::Request,
    ) -> jsonrpc::Response {
        let params = &req.params.clone().unwrap_or_default();

        let response = match req.method.as_str() {
            "starknet_specVersion" => handle_specVersion(rpc, params),
            "starknet_getBlockWithTxHashes" => {
                handle_getBlockWithTxHashes(rpc, params)
            }
            "starknet_getBlockWithTxs" => handle_getBlockWithTxs(rpc, params),
            "starknet_getStateUpdate" => handle_getStateUpdate(rpc, params),
            "starknet_getStorageAt" => handle_getStorageAt(rpc, params),
            "starknet_getTransactionStatus" => {
                handle_getTransactionStatus(rpc, params)
            }
            "starknet_getTransactionByHash" => {
                handle_getTransactionByHash(rpc, params)
            }
            "starknet_getTransactionByBlockIdAndIndex" => {
                handle_getTransactionByBlockIdAndIndex(rpc, params)
            }
            "starknet_getTransactionReceipt" => {
                handle_getTransactionReceipt(rpc, params)
            }
            "starknet_getClass" => handle_getClass(rpc, params),
            "starknet_getClassHashAt" => handle_getClassHashAt(rpc, params),
            "starknet_getClassAt" => handle_getClassAt(rpc, params),
            "starknet_getBlockTransactionCount" => {
                handle_getBlockTransactionCount(rpc, params)
            }
            "starknet_call" => handle_call(rpc, params),
            "starknet_estimateFee" => handle_estimateFee(rpc, params),
            "starknet_estimateMessageFee" => {
                handle_estimateMessageFee(rpc, params)
            }
            "starknet_blockNumber" => handle_blockNumber(rpc, params),
            "starknet_blockHashAndNumber" => {
                handle_blockHashAndNumber(rpc, params)
            }
            "starknet_chainId" => handle_chainId(rpc, params),
            "starknet_syncing" => handle_syncing(rpc, params),
            "starknet_getEvents" => handle_getEvents(rpc, params),
            "starknet_getNonce" => handle_getNonce(rpc, params),
            "starknet_addInvokeTransaction" => {
                handle_addInvokeTransaction(rpc, params)
            }
            "starknet_addDeclareTransaction" => {
                handle_addDeclareTransaction(rpc, params)
            }
            "starknet_addDeployAccountTransaction" => {
                handle_addDeployAccountTransaction(rpc, params)
            }
            "starknet_traceTransaction" => handle_traceTransaction(rpc, params),
            "starknet_simulateTransactions" => {
                handle_simulateTransactions(rpc, params)
            }
            "starknet_traceBlockTransactions" => {
                handle_traceBlockTransactions(rpc, params)
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
        pub const CLASS_ALREADY_DECLARED: Error =
            Error(51, "Class already declared");
        pub const CLASS_HASH_NOT_FOUND: Error =
            Error(28, "Class hash not found");
        pub const COMPILATION_FAILED: Error = Error(56, "Compilation failed");
        pub const COMPILED_CLASS_HASH_MISMATCH: Error = Error(60, "the compiled class hash did not match the one supplied in the transaction");
        pub const CONTRACT_CLASS_SIZE_IS_TOO_LARGE: Error =
            Error(57, "Contract class size it too large");
        pub const CONTRACT_ERROR: Error = Error(40, "Contract error");
        pub const CONTRACT_NOT_FOUND: Error = Error(20, "Contract not found");
        pub const DUPLICATE_TX: Error = Error(
            59,
            "A transaction with the same hash already exists in the mempool",
        );
        pub const FAILED_TO_RECEIVE_TXN: Error =
            Error(1, "Failed to write transaction");
        pub const INSUFFICIENT_ACCOUNT_BALANCE: Error = Error(
            54,
            "Account balance is smaller than the transaction's max_fee",
        );
        pub const INSUFFICIENT_MAX_FEE: Error = Error(53, "Max fee is smaller than the minimal transaction cost (validation plus fee transfer)");
        pub const INVALID_CONTINUATION_TOKEN: Error =
            Error(33, "The supplied continuation token is invalid or unknown");
        pub const INVALID_TRANSACTION_NONCE: Error =
            Error(52, "Invalid transaction nonce");
        pub const INVALID_TXN_INDEX: Error =
            Error(27, "Invalid transaction index in a block");
        pub const NON_ACCOUNT: Error =
            Error(58, "Sender address in not an account contract");
        pub const NO_BLOCKS: Error = Error(32, "There are no blocks");
        pub const NO_TRACE_AVAILABLE: Error =
            Error(10, "No trace available for transaction");
        pub const PAGE_SIZE_TOO_BIG: Error =
            Error(31, "Requested page size is too big");
        pub const TOO_MANY_KEYS_IN_FILTER: Error =
            Error(34, "Too many keys provided in a filter");
        pub const TRANSACTION_EXECUTION_ERROR: Error =
            Error(41, "Transaction execution error");
        pub const TXN_HASH_NOT_FOUND: Error =
            Error(29, "Transaction hash not found");
        pub const UNEXPECTED_ERROR: Error =
            Error(63, "An unexpected error occurred");
        pub const UNSUPPORTED_CONTRACT_CLASS_VERSION: Error =
            Error(62, "the contract class version is not supported");
        pub const UNSUPPORTED_TX_VERSION: Error =
            Error(61, "the transaction version is not supported");
        pub const VALIDATION_FAILURE: Error =
            Error(55, "Account validation failed");

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

    pub mod client {
        use super::*;

        pub struct Client {
            client: reqwest::blocking::Client,
            url: String,
        }

        impl Client {
            pub fn new(url: &str) -> Self {
                Self {
                    url: url.to_string(),
                    client: reqwest::blocking::Client::new(),
                }
            }
        }

        impl super::Rpc for Client {
            fn specVersion(
                &self,
            ) -> std::result::Result<String, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "starknet_specVersion".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: String =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getBlockWithTxHashes(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<GetBlockWithTxHashesResult, jsonrpc::Error>
            {
                let args = (block_id,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getBlockWithTxHashes".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetBlockWithTxHashesResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getBlockWithTxs(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<GetBlockWithTxsResult, jsonrpc::Error>
            {
                let args = (block_id,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getBlockWithTxs".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetBlockWithTxsResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getStateUpdate(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<GetStateUpdateResult, jsonrpc::Error>
            {
                let args = (block_id,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getStateUpdate".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetStateUpdateResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getStorageAt(
                &self,
                contract_address: Address,
                key: StorageKey,
                block_id: BlockId,
            ) -> std::result::Result<Felt, jsonrpc::Error> {
                let args = (contract_address, key, block_id);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getStorageAt".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Felt =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getTransactionStatus(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<GetTransactionStatusResult, jsonrpc::Error>
            {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getTransactionStatus".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetTransactionStatusResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getTransactionByHash(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<GetTransactionByHashResult, jsonrpc::Error>
            {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getTransactionByHash".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetTransactionByHashResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getTransactionByBlockIdAndIndex(
                &self,
                block_id: BlockId,
                index: GetTransactionByBlockIdAndIndexIndex,
            ) -> std::result::Result<
                GetTransactionByBlockIdAndIndexResult,
                jsonrpc::Error,
            > {
                let args = (block_id, index);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getTransactionByBlockIdAndIndex".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetTransactionByBlockIdAndIndexResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getTransactionReceipt(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<GetTransactionReceiptResult, jsonrpc::Error>
            {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getTransactionReceipt".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetTransactionReceiptResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getClass(
                &self,
                block_id: BlockId,
                class_hash: Felt,
            ) -> std::result::Result<GetClassResult, jsonrpc::Error>
            {
                let args = (block_id, class_hash);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getClass".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetClassResult = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getClassHashAt(
                &self,
                block_id: BlockId,
                contract_address: Address,
            ) -> std::result::Result<Felt, jsonrpc::Error> {
                let args = (block_id, contract_address);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getClassHashAt".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Felt =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getClassAt(
                &self,
                block_id: BlockId,
                contract_address: Address,
            ) -> std::result::Result<GetClassAtResult, jsonrpc::Error>
            {
                let args = (block_id, contract_address);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getClassAt".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetClassAtResult = serde_json::from_value(value)
                        .map_err(|e| {
                        jsonrpc::Error::new(
                            5002,
                            format!("Invalid response object: {e}."),
                        )
                    })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getBlockTransactionCount(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<
                GetBlockTransactionCountResult,
                jsonrpc::Error,
            > {
                let args = (block_id,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getBlockTransactionCount".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetBlockTransactionCountResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn call(
                &self,
                request: FunctionCall,
                block_id: BlockId,
            ) -> std::result::Result<Vec<Felt>, jsonrpc::Error> {
                let args = (request, block_id);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req =
                    jsonrpc::Request::new("starknet_call".to_string(), params)
                        .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Vec<Felt> = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn estimateFee(
                &self,
                request: Vec<BroadcastedTxn>,
                simulation_flags: Vec<SimulationFlagForEstimateFee>,
                block_id: BlockId,
            ) -> std::result::Result<Vec<FeeEstimate>, jsonrpc::Error>
            {
                let args = (request, simulation_flags, block_id);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_estimateFee".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Vec<FeeEstimate> = serde_json::from_value(value)
                        .map_err(|e| {
                        jsonrpc::Error::new(
                            5002,
                            format!("Invalid response object: {e}."),
                        )
                    })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn estimateMessageFee(
                &self,
                message: MsgFromL1,
                block_id: BlockId,
            ) -> std::result::Result<FeeEstimate, jsonrpc::Error> {
                let args = (message, block_id);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_estimateMessageFee".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: FeeEstimate = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn blockNumber(
                &self,
            ) -> std::result::Result<BlockNumber, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "starknet_blockNumber".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: BlockNumber = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn blockHashAndNumber(
                &self,
            ) -> std::result::Result<BlockHashAndNumberResult, jsonrpc::Error>
            {
                let req = jsonrpc::Request::new(
                    "starknet_blockHashAndNumber".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: BlockHashAndNumberResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn chainId(&self) -> std::result::Result<ChainId, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "starknet_chainId".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: ChainId =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn syncing(
                &self,
            ) -> std::result::Result<SyncingResult, jsonrpc::Error>
            {
                let req = jsonrpc::Request::new(
                    "starknet_syncing".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: SyncingResult = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getEvents(
                &self,
                filter: GetEventsFilter,
            ) -> std::result::Result<EventsChunk, jsonrpc::Error> {
                let args = (filter,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getEvents".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: EventsChunk = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn getNonce(
                &self,
                block_id: BlockId,
                contract_address: Address,
            ) -> std::result::Result<Felt, jsonrpc::Error> {
                let args = (block_id, contract_address);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getNonce".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Felt =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn addInvokeTransaction(
                &self,
                invoke_transaction: BroadcastedInvokeTxn,
            ) -> std::result::Result<AddInvokeTransactionResult, jsonrpc::Error>
            {
                let args = (invoke_transaction,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_addInvokeTransaction".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: AddInvokeTransactionResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn addDeclareTransaction(
                &self,
                declare_transaction: BroadcastedDeclareTxn,
            ) -> std::result::Result<AddDeclareTransactionResult, jsonrpc::Error>
            {
                let args = (declare_transaction,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_addDeclareTransaction".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: AddDeclareTransactionResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn addDeployAccountTransaction(
                &self,
                deploy_account_transaction: BroadcastedDeployAccountTxn,
            ) -> std::result::Result<
                AddDeployAccountTransactionResult,
                jsonrpc::Error,
            > {
                let args = (deploy_account_transaction,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_addDeployAccountTransaction".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: AddDeployAccountTransactionResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn traceTransaction(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<TransactionTrace, jsonrpc::Error>
            {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_traceTransaction".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: TransactionTrace = serde_json::from_value(value)
                        .map_err(|e| {
                        jsonrpc::Error::new(
                            5002,
                            format!("Invalid response object: {e}."),
                        )
                    })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn simulateTransactions(
                &self,
                block_id: BlockId,
                transactions: Vec<BroadcastedTxn>,
                simulation_flags: Vec<SimulationFlag>,
            ) -> std::result::Result<Vec<SimulatedTransaction>, jsonrpc::Error>
            {
                let args = (block_id, transactions, simulation_flags);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_simulateTransactions".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Vec<SimulatedTransaction> =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            fn traceBlockTransactions(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<Vec<BlockTransaction>, jsonrpc::Error>
            {
                let args = (block_id,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_traceBlockTransactions".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("REQ: {req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4002,
                            format!("Request failed: {e}."),
                        )
                    })?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            5001,
                            format!("Invalid response JSON: {e}."),
                        )
                    })?;

                log::debug!("RES: {res:#?}");

                if let Some(err) = res.error.take() {
                    log::error!("{err:#?}");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Vec<BlockTransaction> =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    log::debug!("RET: {ret:#?}");

                    Ok(ret)
                } else {
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }
        }
    }
}
// ^^^ GENERATED CODE ABOVE ^^^
