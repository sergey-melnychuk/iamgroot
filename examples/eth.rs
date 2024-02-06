fn main() {
    let json = include_str!("../api/eth/0x10165f5.json");
    let mut res: iamgroot::jsonrpc::Response = serde_json::from_str(json).unwrap();
    let res: gen::Block = serde_json::from_value(res.result.take().unwrap()).unwrap();
    println!("{res:#?}");
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

    // object: 'Address'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Address(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Address(String);

    mod address {
        use super::jsonrpc;
        use super::Address;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static ADDRESS_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x[a-fA-F\\d]{40}$").unwrap());

        impl Address {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if ADDRESS_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Address value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Address {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Address {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'AddressOrNull'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum AddressOrNull {
        Address(Address),
        Null(Null),
    }

    // object: 'Addresses'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Addresses(pub Vec<Address>); // name == binding_name

    // object: 'Block'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Block {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub difficulty: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "extraData")]
        pub extradata: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "gasLimit")]
        pub gaslimit: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "gasUsed")]
        pub gasused: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hash: Option<BlockHashOrNull>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "logsBloom")]
        pub logsbloom: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub miner: Option<AddressOrNull>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub nonce: Option<NonceOrNull>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub number: Option<BlockNumberOrNull>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "parentHash")]
        pub parenthash: Option<BlockHash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "receiptsRoot")]
        pub receiptsroot: Option<Keccak>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "sha3Uncles")]
        pub sha3uncles: Option<Keccak>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "stateRoot")]
        pub stateroot: Option<Keccak>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "totalDifficulty")]
        pub totaldifficulty: Option<IntegerOrNull>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transactions: Option<Vec<TransactionOrTransactionHash>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "transactionsRoot")]
        pub transactionsroot: Option<Keccak>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub uncles: Option<Vec<Keccak>>,
    }

    // object: 'BlockHash'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct BlockHash(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct BlockHash(String);

    mod blockhash {
        use super::jsonrpc;
        use super::BlockHash;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static BLOCKHASH_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x[a-fA-F\\d]{64}$").unwrap());

        impl BlockHash {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if BLOCKHASH_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "BlockHash value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for BlockHash {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for BlockHash {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'BlockHashOrNull'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BlockHashOrNull {
        Keccak(Keccak),
        Null(Null),
    }

    // object: 'BlockNumber'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockNumber(pub Integer); // name != binding_name

    // object: 'BlockNumberOrNull'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BlockNumberOrNull {
        Integer(Integer),
        Null(Null),
    }

    // object: 'BlockNumberTag'
    #[derive(Debug, Deserialize, Serialize)]
    pub enum BlockNumberTag {
        #[serde(rename = "earliest")]
        Earliest,
        #[serde(rename = "latest")]
        Latest,
        #[serde(rename = "pending")]
        Pending,
    }

    // object: 'BlockOrNull'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BlockOrNull {
        Block(Block),
        Null(Null),
    }

    // object: 'BloomFilter'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BloomFilter(pub String); // name == binding_name

    // object: 'Boolean'
    // pub type Boolean = bool;

    // object: 'Bytes'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Bytes(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Bytes(String);

    mod bytes {
        use super::jsonrpc;
        use super::Bytes;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static BYTES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x([a-fA-F0-9]?)+$").unwrap());

        impl Bytes {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if BYTES_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Bytes value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Bytes {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Bytes {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'DataWord'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct DataWord(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct DataWord(String);

    mod dataword {
        use super::jsonrpc;
        use super::DataWord;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static DATAWORD_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x([a-fA-F\\d]{64})?$").unwrap());

        impl DataWord {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if DATAWORD_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "DataWord value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for DataWord {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for DataWord {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'Difficulty'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Difficulty(pub DataWord); // name != binding_name

    // object: 'FilterId'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct FilterId(pub String); // name == binding_name

    // object: 'From'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct From(pub Address); // name != binding_name

    // object: 'Integer'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Integer(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Integer(String);

    mod integer {
        use super::jsonrpc;
        use super::Integer;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static INTEGER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[a-fA-F0-9]+$").unwrap());

        impl Integer {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if INTEGER_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Integer value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Integer {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Integer {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'IntegerOrNull'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum IntegerOrNull {
        Integer(Integer),
        Null(Null),
    }

    // object: 'Keccak'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Keccak(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Keccak(String);

    mod keccak {
        use super::jsonrpc;
        use super::Keccak;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static KECCAK_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x[a-fA-F\\d]{1,64}$").unwrap());

        impl Keccak {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if KECCAK_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Keccak value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Keccak {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Keccak {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'KeccakOrPending'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum KeccakOrPending {
        Keccak(Keccak),
        Null(Null),
    }

    // object: 'Log'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Log {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<Address>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "blockHash")]
        pub blockhash: Option<BlockHash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "blockNumber")]
        pub blocknumber: Option<BlockNumber>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<Bytes>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "logIndex")]
        pub logindex: Option<Integer>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub removed: Option<bool>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub topics: Option<Topics>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "transactionHash")]
        pub transactionhash: Option<TransactionHash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "transactionIndex")]
        pub transactionindex: Option<TransactionIndex>,
    }

    // object: 'MixHash'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct MixHash(pub DataWord); // name != binding_name

    // object: 'Nonce'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Nonce(pub Integer); // name != binding_name

    // object: 'NonceOrNull'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum NonceOrNull {
        Integer(Integer),
        Null(Null),
    }

    // object: 'Null'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Null;

    // object: 'Position'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Position(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Position(String);

    mod position {
        use super::jsonrpc;
        use super::Position;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static POSITION_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x([a-fA-F0-9]?)+$").unwrap());

        impl Position {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if POSITION_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Position value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Position {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Position {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'PowHash'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct PowHash(pub DataWord); // name != binding_name

    // object: 'ProofNode'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ProofNode(pub Bytes); // name != binding_name

    // object: 'ProofNodes'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ProofNodes(pub Vec<Bytes>); // name == binding_name

    // object: 'Receipt'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Receipt {
        #[serde(rename = "blockHash")]
        pub blockhash: BlockHash,
        #[serde(rename = "blockNumber")]
        pub blocknumber: BlockNumber,
        #[serde(rename = "contractAddress")]
        pub contractaddress: AddressOrNull,
        #[serde(rename = "cumulativeGasUsed")]
        pub cumulativegasused: Integer,
        pub from: From,
        #[serde(rename = "gasUsed")]
        pub gasused: Integer,
        pub logs: Vec<Log>,
        #[serde(rename = "logsBloom")]
        pub logsbloom: BloomFilter,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "postTransactionState")]
        pub posttransactionstate: Option<Keccak>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<bool>,
        pub to: To,
        #[serde(rename = "transactionHash")]
        pub transactionhash: TransactionHash,
        #[serde(rename = "transactionIndex")]
        pub transactionindex: TransactionIndex,
    }

    // object: 'SeedHash'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SeedHash(pub DataWord); // name != binding_name

    // object: 'StorageProof'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct StorageProof(pub Vec<StorageProof>); // name == binding_name

    // object: 'StorageProofKey'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct StorageProofKey(pub Integer); // name != binding_name

    // object: 'To'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum To {
        Address(Address),
        Null(Null),
    }

    // object: 'Topic'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Topic(pub DataWord); // name != binding_name

    // object: 'Topics'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Topics(pub Vec<DataWord>); // name == binding_name

    // object: 'Transaction'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Transaction {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "blockHash")]
        pub blockhash: Option<BlockHashOrNull>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "blockNumber")]
        pub blocknumber: Option<BlockNumberOrNull>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from: Option<From>,
        pub gas: String,
        #[serde(rename = "gasPrice")]
        pub gasprice: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hash: Option<TransactionHash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
        pub nonce: Nonce,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub s: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<To>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "transactionIndex")]
        pub transactionindex: Option<TransactionIndex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub v: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Keccak>,
    }

    // object: 'TransactionHash'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct TransactionHash(pub Keccak); // name != binding_name

    // object: 'TransactionIndex'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TransactionIndex {
        Integer(Integer),
        Null(Null),
    }

    // object: 'Transactions'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Transactions(pub Vec<Transaction>); // name == binding_name

    // object: 'blocknumber'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum Blocknumber {
        BlockNumberTag(BlockNumberTag),
        Integer(Integer),
    }

    // object: 'data'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Data(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Data(String);

    mod data {
        use super::jsonrpc;
        use super::Data;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static DATA_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[a-fA-F\\d]+$").unwrap());

        impl Data {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if DATA_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Data value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Data {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Data {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'eth_blockNumber_blockNumber'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthBlockNumberBlockNumber {
        BlockNumberTag(BlockNumberTag),
        Integer(Integer),
    }

    // object: 'eth_call_returnValue'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthCallReturnValue(pub Bytes); // name != binding_name

    // object: 'eth_chainId_chainId'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct EthChainIdChainId(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct EthChainIdChainId(String);

    mod ethchainidchainid {
        use super::jsonrpc;
        use super::EthChainIdChainId;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static ETHCHAINIDCHAINID_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x[a-fA-F\\d]+$").unwrap());

        impl EthChainIdChainId {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if ETHCHAINIDCHAINID_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "EthChainIdChainId value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for EthChainIdChainId {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for EthChainIdChainId {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'eth_coinbase_address'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthCoinbaseAddress(pub Address); // name != binding_name

    // object: 'eth_estimateGas_gasUsed'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthEstimateGasGasUsed(pub Integer); // name != binding_name

    // object: 'eth_gasPrice_gasPrice'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGasPriceGasPrice(pub Integer); // name != binding_name

    // object: 'eth_getBalance_getBalanceResult'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetBalanceGetBalanceResult {
        Integer(Integer),
        Null(Null),
    }

    // object: 'eth_getBlockByHash_getBlockByHashResult'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetBlockByHashGetBlockByHashResult {
        Block(Block),
        Null(Null),
    }

    // object: 'eth_getBlockByNumber_getBlockByNumberResult'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetBlockByNumberGetBlockByNumberResult {
        Block(Block),
        Null(Null),
    }

    // object: 'eth_getBlockTransactionCountByHash_blockTransactionCountByHash'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetBlockTransactionCountByHashBlockTransactionCountByHash {
        Integer(Integer),
        Null(Null),
    }

    // object: 'eth_getBlockTransactionCountByNumber_blockTransactionCountByHash'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetBlockTransactionCountByNumberBlockTransactionCountByHash {
        Integer(Integer),
        Null(Null),
    }

    // object: 'eth_getCode_bytes'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetCodeBytes(pub Bytes); // name != binding_name

    // object: 'eth_getFilterChanges_logResult'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetFilterChangesLogResult(pub Vec<Log>); // name == binding_name

    // object: 'eth_getFilterLogs_logs'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetFilterLogsLogs(pub Vec<Log>); // name == binding_name

    // object: 'eth_getLogs_logs'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetLogsLogs(pub Vec<Log>); // name == binding_name

    // object: 'eth_getProof_account'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetProofAccount {
        Null(Null),
        ProofAccount(ProofAccount),
    }

    // object: 'eth_getRawTransactionByBlockHashAndIndex_rawTransaction'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetRawTransactionByBlockHashAndIndexRawTransaction(pub Bytes); // name != binding_name

    // object: 'eth_getRawTransactionByBlockNumberAndIndex_rawTransaction'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetRawTransactionByBlockNumberAndIndexRawTransaction(pub Bytes); // name != binding_name

    // object: 'eth_getRawTransactionByHash_rawTransactionByHash'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetRawTransactionByHashRawTransactionByHash(pub Bytes); // name != binding_name

    // object: 'eth_getStorageAt_dataWord'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetStorageAtDataWord(pub DataWord); // name != binding_name

    // object: 'eth_getTransactionByBlockHashAndIndex_transactionResult'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetTransactionByBlockHashAndIndexTransactionResult {
        Null(Null),
        Transaction(Transaction),
    }

    // object: 'eth_getTransactionByBlockNumberAndIndex_transactionResult'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetTransactionByBlockNumberAndIndexTransactionResult {
        Null(Null),
        Transaction(Transaction),
    }

    // object: 'eth_getTransactionByHash_transactionResult'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetTransactionByHashTransactionResult {
        Null(Null),
        Transaction(Transaction),
    }

    // object: 'eth_getTransactionCount_transactionCount'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetTransactionCountTransactionCount {
        Integer(Integer),
        Null(Null),
    }

    // object: 'eth_getTransactionReceipt_transactionReceiptResult'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetTransactionReceiptTransactionReceiptResult {
        Null(Null),
        Receipt(Receipt),
    }

    // object: 'eth_getUncleByBlockHashAndIndex_uncle'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetUncleByBlockHashAndIndexUncle {
        Block(Block),
        Null(Null),
    }

    // object: 'eth_getUncleByBlockNumberAndIndex_uncleResult'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetUncleByBlockNumberAndIndexUncleResult {
        Block(Block),
        Null(Null),
    }

    // object: 'eth_getUncleCountByBlockHash_uncleCountResult'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetUncleCountByBlockHashUncleCountResult {
        Integer(Integer),
        Null(Null),
    }

    // object: 'eth_getUncleCountByBlockNumber_uncleCountResult'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthGetUncleCountByBlockNumberUncleCountResult {
        Integer(Integer),
        Null(Null),
    }

    // object: 'eth_getWork_work'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetWorkWork(pub Vec<EthGetWorkWork>); // name == binding_name

    // object: 'eth_hashrate_hashesPerSecond'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthHashrateHashesPerSecond(pub Integer); // name != binding_name

    // object: 'eth_mining_mining'
    // pub type EthMiningMining = bool;

    // object: 'eth_newBlockFilter_filterId'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthNewBlockFilterFilterId(pub FilterId); // name != binding_name

    // object: 'eth_newFilter_filterId'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthNewFilterFilterId(pub Integer); // name != binding_name

    // object: 'eth_newPendingTransactionFilter_filterId'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthNewPendingTransactionFilterFilterId(pub FilterId); // name != binding_name

    // object: 'eth_pendingTransactions_pendingTransactions'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthPendingTransactionsPendingTransactions(pub Transactions); // name != binding_name

    // object: 'eth_protocolVersion_protocolVersion'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthProtocolVersionProtocolVersion(pub Integer); // name != binding_name

    // object: 'eth_sendRawTransaction_transactionHash'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthSendRawTransactionTransactionHash(pub Keccak); // name != binding_name

    // object: 'eth_submitHashrate_submitHashRateSuccess'
    // pub type EthSubmitHashrateSubmitHashRateSuccess = bool;

    // object: 'eth_submitWork_solutionValid'
    // pub type EthSubmitWorkSolutionValid = bool;

    // object: 'eth_syncing_syncing'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthSyncingSyncing {
        Boolean(bool),
        SyncingData(SyncingData),
    }

    // object: 'eth_uninstallFilter_filterUninstalledSuccess'
    // pub type EthUninstallFilterFilterUninstalledSuccess = bool;

    // object: 'filter'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Filter {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<OneOrArrayOfAddresses>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "fromBlock")]
        pub fromblock: Option<BlockNumber>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "toBlock")]
        pub toblock: Option<BlockNumber>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub topics: Option<Topics>,
    }

    // object: 'net_listening_netListeningResult'
    // pub type NetListeningNetListeningResult = bool;

    // object: 'net_peerCount_quantity'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct NetPeerCountQuantity(pub String); // name == binding_name

    // object: 'net_version_networkId'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct NetVersionNetworkId(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct NetVersionNetworkId(String);

    mod netversionnetworkid {
        use super::jsonrpc;
        use super::NetVersionNetworkId;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static NETVERSIONNETWORKID_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^[\\d]+$").unwrap());

        impl NetVersionNetworkId {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if NETVERSIONNETWORKID_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "NetVersionNetworkId value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for NetVersionNetworkId {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for NetVersionNetworkId {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'oneOrArrayOfAddresses'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum OneOrArrayOfAddresses {
        Address(Address),
        Addresses(Addresses),
    }

    // object: 'proofAccount'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ProofAccount {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "accountProof")]
        pub accountproof: Option<ProofNodes>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<Address>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub balance: Option<Integer>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "codeHash")]
        pub codehash: Option<Keccak>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub nonce: Option<Nonce>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "storageHash")]
        pub storagehash: Option<Keccak>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "storageProof")]
        pub storageproof: Option<StorageProof>,
    }

    // object: 'storagekeys'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Storagekeys(pub Vec<Integer>); // name == binding_name

    // object: 'syncingData'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SyncingData {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "currentBlock")]
        pub currentblock: Option<Integer>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "highestBlock")]
        pub highestblock: Option<Integer>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "knownStates")]
        pub knownstates: Option<Integer>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "pulledStates")]
        pub pulledstates: Option<Integer>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "startingBlock")]
        pub startingblock: Option<Integer>,
    }

    // object: 'transactionOrTransactionHash'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TransactionOrTransactionHash {
        Keccak(Keccak),
        Transaction(Transaction),
    }

    // object: 'web3_clientVersion_clientVersion'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Web3ClientVersionClientVersion(pub String); // name == binding_name

    // object: 'web3_sha3_hashedData'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Web3Sha3HashedData(pub Keccak); // name != binding_name

    pub trait Rpc {
        /// Method: 'web3_clientVersion'
        /// Summary: current client version
        /// Description: Returns the version of the current client
        ///
        fn web3_clientVersion(
            &self,
        ) -> std::result::Result<Web3ClientVersionClientVersion, jsonrpc::Error>;

        /// Method: 'web3_sha3'
        /// Summary: Hashes data
        /// Description: Hashes data using the Keccak-256 algorithm
        ///
        fn web3_sha3(&self, data: Option<Data>) -> std::result::Result<Keccak, jsonrpc::Error>;

        /// Method: 'net_listening'
        /// Summary: returns listening status
        /// Description: Determines if this client is listening for new network connections.
        ///
        fn net_listening(&self) -> std::result::Result<bool, jsonrpc::Error>;

        /// Method: 'net_peerCount'
        /// Summary: number of peers
        /// Description: Returns the number of peers currently connected to this client.
        ///
        fn net_peerCount(&self) -> std::result::Result<NetPeerCountQuantity, jsonrpc::Error>;

        /// Method: 'net_version'
        /// Summary: Network identifier associated with network
        /// Description: Returns the network ID associated with the current network.
        ///
        fn net_version(&self) -> std::result::Result<NetVersionNetworkId, jsonrpc::Error>;

        /// Method: 'eth_blockNumber'
        /// Summary: Returns the number of most recent block.
        /// Description:
        ///
        fn eth_blockNumber(&self)
            -> std::result::Result<EthBlockNumberBlockNumber, jsonrpc::Error>;

        /// Method: 'eth_call'
        /// Summary: Executes a new message call (locally) immediately without creating a transaction on the block chain.
        /// Description:
        ///
        fn eth_call(
            &self,
            transaction: Transaction,
            blocknumber: Blocknumber,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'eth_chainId'
        /// Summary: Returns the currently configured chain id
        /// Description: Returns the currently configured chain id, a value used in replay-protected transaction signing as introduced by [EIP-155](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-155.md).
        ///
        fn eth_chainId(&self) -> std::result::Result<EthChainIdChainId, jsonrpc::Error>;

        /// Method: 'eth_coinbase'
        /// Summary: Returns the client coinbase address.
        /// Description:
        ///
        fn eth_coinbase(&self) -> std::result::Result<Address, jsonrpc::Error>;

        /// Method: 'eth_estimateGas'
        /// Summary: Generates and returns an estimate of how much gas is necessary to allow the transaction to complete. The transaction will not be added to the blockchain. Note that the estimate may be significantly more than the amount of gas actually used by the transaction, for a variety of reasons including EVM mechanics and node performance.
        /// Description:
        ///
        fn eth_estimateGas(
            &self,
            transaction: Transaction,
        ) -> std::result::Result<Integer, jsonrpc::Error>;

        /// Method: 'eth_gasPrice'
        /// Summary: Returns the current price per gas in wei
        /// Description:
        ///
        fn eth_gasPrice(&self) -> std::result::Result<Integer, jsonrpc::Error>;

        /// Method: 'eth_getBalance'
        /// Summary: Returns Ether balance of a given or account or contract
        /// Description:
        ///
        fn eth_getBalance(
            &self,
            address: Address,
            blocknumber: BlockNumber,
        ) -> std::result::Result<IntegerOrNull, jsonrpc::Error>;

        /// Method: 'eth_getBlockByHash'
        /// Summary: Gets a block for a given hash
        /// Description:
        ///
        fn eth_getBlockByHash(
            &self,
            blockhash: BlockHash,
            includetransactions: bool,
        ) -> std::result::Result<BlockOrNull, jsonrpc::Error>;

        /// Method: 'eth_getBlockByNumber'
        /// Summary: Gets a block for a given number
        /// Description:
        ///
        fn eth_getBlockByNumber(
            &self,
            blocknumber: Blocknumber,
            includetransactions: bool,
        ) -> std::result::Result<BlockOrNull, jsonrpc::Error>;

        /// Method: 'eth_getBlockTransactionCountByHash'
        /// Summary: Returns the number of transactions in a block from a block matching the given block hash.
        /// Description:
        ///
        fn eth_getBlockTransactionCountByHash(
            &self,
            blockhash: BlockHash,
        ) -> std::result::Result<IntegerOrNull, jsonrpc::Error>;

        /// Method: 'eth_getBlockTransactionCountByNumber'
        /// Summary: Returns the number of transactions in a block from a block matching the given block number.
        /// Description:
        ///
        fn eth_getBlockTransactionCountByNumber(
            &self,
            blocknumber: Blocknumber,
        ) -> std::result::Result<IntegerOrNull, jsonrpc::Error>;

        /// Method: 'eth_getCode'
        /// Summary: Returns code at a given contract address
        /// Description:
        ///
        fn eth_getCode(
            &self,
            address: Address,
            blocknumber: BlockNumber,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'eth_getFilterChanges'
        /// Summary: Polling method for a filter, which returns an array of logs which occurred since last poll.
        /// Description:
        ///
        fn eth_getFilterChanges(
            &self,
            filterid: FilterId,
        ) -> std::result::Result<EthGetFilterChangesLogResult, jsonrpc::Error>;

        /// Method: 'eth_getFilterLogs'
        /// Summary: Returns an array of all logs matching filter with given id.
        /// Description:
        ///
        fn eth_getFilterLogs(
            &self,
            filterid: FilterId,
        ) -> std::result::Result<EthGetFilterLogsLogs, jsonrpc::Error>;

        /// Method: 'eth_getRawTransactionByHash'
        /// Summary: Returns raw transaction data of a transaction with the given hash.
        /// Description:
        ///
        fn eth_getRawTransactionByHash(
            &self,
            transactionhash: TransactionHash,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'eth_getRawTransactionByBlockHashAndIndex'
        /// Summary: Returns raw transaction data of a transaction with the block hash and index of which it was mined.
        /// Description:
        ///
        fn eth_getRawTransactionByBlockHashAndIndex(
            &self,
            blockhash: BlockHash,
            index: Integer,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'eth_getRawTransactionByBlockNumberAndIndex'
        /// Summary: Returns raw transaction data of a transaction with the block number and index of which it was mined.
        /// Description:
        ///
        fn eth_getRawTransactionByBlockNumberAndIndex(
            &self,
            blocknumber: Blocknumber,
            index: Integer,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'eth_getLogs'
        /// Summary: Returns an array of all logs matching a given filter object.
        /// Description:
        ///
        fn eth_getLogs(
            &self,
            filter: Filter,
        ) -> std::result::Result<EthGetLogsLogs, jsonrpc::Error>;

        /// Method: 'eth_getStorageAt'
        /// Summary: Gets a storage value from a contract address, a position, and an optional blockNumber
        /// Description:
        ///
        fn eth_getStorageAt(
            &self,
            address: Address,
            key: Position,
            blocknumber: Blocknumber,
        ) -> std::result::Result<DataWord, jsonrpc::Error>;

        /// Method: 'eth_getTransactionByBlockHashAndIndex'
        /// Summary: Returns the information about a transaction requested by the block hash and index of which it was mined.
        /// Description:
        ///
        fn eth_getTransactionByBlockHashAndIndex(
            &self,
            blockhash: BlockHash,
            index: Integer,
        ) -> std::result::Result<
            EthGetTransactionByBlockHashAndIndexTransactionResult,
            jsonrpc::Error,
        >;

        /// Method: 'eth_getTransactionByBlockNumberAndIndex'
        /// Summary: Returns the information about a transaction requested by the block number and index of which it was mined.
        /// Description:
        ///
        fn eth_getTransactionByBlockNumberAndIndex(
            &self,
            blocknumber: Blocknumber,
            index: Integer,
        ) -> std::result::Result<
            EthGetTransactionByBlockNumberAndIndexTransactionResult,
            jsonrpc::Error,
        >;

        /// Method: 'eth_getTransactionByHash'
        /// Summary: Returns the information about a transaction requested by transaction hash.
        /// Description:
        ///
        fn eth_getTransactionByHash(
            &self,
            transactionhash: TransactionHash,
        ) -> std::result::Result<EthGetTransactionByHashTransactionResult, jsonrpc::Error>;

        /// Method: 'eth_getTransactionCount'
        /// Summary: Returns the number of transactions sent from an address
        /// Description:
        ///
        fn eth_getTransactionCount(
            &self,
            address: Address,
            blocknumber: Blocknumber,
        ) -> std::result::Result<EthGetTransactionCountTransactionCount, jsonrpc::Error>;

        /// Method: 'eth_getTransactionReceipt'
        /// Summary: Returns the receipt information of a transaction by its hash.
        /// Description:
        ///
        fn eth_getTransactionReceipt(
            &self,
            transactionhash: TransactionHash,
        ) -> std::result::Result<EthGetTransactionReceiptTransactionReceiptResult, jsonrpc::Error>;

        /// Method: 'eth_getUncleByBlockHashAndIndex'
        /// Summary: Returns information about a uncle of a block by hash and uncle index position.
        /// Description:
        ///
        fn eth_getUncleByBlockHashAndIndex(
            &self,
            blockhash: BlockHash,
            index: Integer,
        ) -> std::result::Result<BlockOrNull, jsonrpc::Error>;

        /// Method: 'eth_getUncleByBlockNumberAndIndex'
        /// Summary: Returns information about a uncle of a block by hash and uncle index position.
        /// Description:
        ///
        fn eth_getUncleByBlockNumberAndIndex(
            &self,
            uncleblocknumber: BlockNumber,
            index: Integer,
        ) -> std::result::Result<BlockOrNull, jsonrpc::Error>;

        /// Method: 'eth_getUncleCountByBlockHash'
        /// Summary: Returns the number of uncles in a block from a block matching the given block hash.
        /// Description:
        ///
        fn eth_getUncleCountByBlockHash(
            &self,
            blockhash: BlockHash,
        ) -> std::result::Result<IntegerOrNull, jsonrpc::Error>;

        /// Method: 'eth_getUncleCountByBlockNumber'
        /// Summary: Returns the number of uncles in a block from a block matching the given block number.
        /// Description:
        ///
        fn eth_getUncleCountByBlockNumber(
            &self,
            blocknumber: Blocknumber,
        ) -> std::result::Result<IntegerOrNull, jsonrpc::Error>;

        /// Method: 'eth_getProof'
        /// Summary: Returns the account- and storage-values of the specified account including the Merkle-proof.
        /// Description:
        ///
        fn eth_getProof(
            &self,
            address: Address,
            storagekeys: Storagekeys,
            blocknumber: Blocknumber,
        ) -> std::result::Result<EthGetProofAccount, jsonrpc::Error>;

        /// Method: 'eth_getWork'
        /// Summary: Returns the hash of the current block, the seedHash, and the boundary condition to be met ('target').
        /// Description:
        ///
        fn eth_getWork(&self) -> std::result::Result<EthGetWorkWork, jsonrpc::Error>;

        /// Method: 'eth_hashrate'
        /// Summary: Returns the number of hashes per second that the node is mining with.
        /// Description:
        ///
        fn eth_hashrate(&self) -> std::result::Result<Integer, jsonrpc::Error>;

        /// Method: 'eth_mining'
        /// Summary: Returns true if client is actively mining new blocks.
        /// Description:
        ///
        fn eth_mining(&self) -> std::result::Result<bool, jsonrpc::Error>;

        /// Method: 'eth_newBlockFilter'
        /// Summary: Creates a filter in the node, to notify when a new block arrives. To check if the state has changed, call eth_getFilterChanges.
        /// Description:
        ///
        fn eth_newBlockFilter(&self) -> std::result::Result<FilterId, jsonrpc::Error>;

        /// Method: 'eth_newFilter'
        /// Summary: Creates a filter object, based on filter options, to notify when the state changes (logs). To check if the state has changed, call eth_getFilterChanges.
        /// Description:
        ///
        fn eth_newFilter(&self, filter: Filter) -> std::result::Result<Integer, jsonrpc::Error>;

        /// Method: 'eth_newPendingTransactionFilter'
        /// Summary: Creates a filter in the node, to notify when new pending transactions arrive. To check if the state has changed, call eth_getFilterChanges.
        /// Description:
        ///
        fn eth_newPendingTransactionFilter(&self) -> std::result::Result<FilterId, jsonrpc::Error>;

        /// Method: 'eth_pendingTransactions'
        /// Summary: Returns the transactions that are pending in the transaction pool and have a from address that is one of the accounts this node manages.
        /// Description:
        ///
        fn eth_pendingTransactions(&self) -> std::result::Result<Transactions, jsonrpc::Error>;

        /// Method: 'eth_protocolVersion'
        /// Summary: Returns the current ethereum protocol version.
        /// Description:
        ///
        fn eth_protocolVersion(&self) -> std::result::Result<Integer, jsonrpc::Error>;

        /// Method: 'eth_sendRawTransaction'
        /// Summary: Creates new message call transaction or a contract creation for signed transactions.
        /// Description:
        ///
        fn eth_sendRawTransaction(
            &self,
            signedtransactiondata: Bytes,
        ) -> std::result::Result<Keccak, jsonrpc::Error>;

        /// Method: 'eth_submitHashrate'
        /// Summary: Used for submitting mining hashrate.
        /// Description:
        ///
        fn eth_submitHashrate(
            &self,
            hashrate: DataWord,
            id: DataWord,
        ) -> std::result::Result<bool, jsonrpc::Error>;

        /// Method: 'eth_submitWork'
        /// Summary: Used for submitting a proof-of-work solution.
        /// Description:
        ///
        fn eth_submitWork(
            &self,
            nonce: Nonce,
            powhash: PowHash,
            mixhash: MixHash,
        ) -> std::result::Result<bool, jsonrpc::Error>;

        /// Method: 'eth_syncing'
        /// Summary: Returns an object with data about the sync status or false.
        /// Description:
        ///
        fn eth_syncing(&self) -> std::result::Result<EthSyncingSyncing, jsonrpc::Error>;

        /// Method: 'eth_uninstallFilter'
        /// Summary: Uninstalls a filter with given id. Should always be called when watch is no longer needed. Additionally Filters timeout when they aren't requested with eth_getFilterChanges for a period of time.
        /// Description:
        ///
        fn eth_uninstallFilter(
            &self,
            filterid: FilterId,
        ) -> std::result::Result<bool, jsonrpc::Error>;
    }

    fn handle_web3_clientVersion<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.web3_clientVersion() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_web3_sha3<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Option<Data>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            data: Option<Data>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(data) = args_by_pos;
                ArgByName { data }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { data } = args;

        match rpc.web3_sha3(data) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_net_listening<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.net_listening() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_net_peerCount<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.net_peerCount() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_net_version<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.net_version() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_blockNumber<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_blockNumber() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_call<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Transaction, Blocknumber);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction: Transaction,
            blocknumber: Blocknumber,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transaction, blocknumber) = args_by_pos;
                ArgByName {
                    transaction,
                    blocknumber,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            transaction,
            blocknumber,
        } = args;

        match rpc.eth_call(transaction, blocknumber) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_chainId<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_chainId() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_coinbase<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_coinbase() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_estimateGas<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Transaction);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction: Transaction,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transaction) = args_by_pos;
                ArgByName { transaction }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transaction } = args;

        match rpc.eth_estimateGas(transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_gasPrice<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_gasPrice() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getBalance<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, BlockNumber);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            address: Address,
            blocknumber: BlockNumber,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(address, blocknumber) = args_by_pos;
                ArgByName {
                    address,
                    blocknumber,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            address,
            blocknumber,
        } = args;

        match rpc.eth_getBalance(address, blocknumber) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getBlockByHash<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockHash, bool);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blockhash: BlockHash,
            includetransactions: bool,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blockhash, includetransactions) = args_by_pos;
                ArgByName {
                    blockhash,
                    includetransactions,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            blockhash,
            includetransactions,
        } = args;

        match rpc.eth_getBlockByHash(blockhash, includetransactions) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getBlockByNumber<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Blocknumber, bool);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blocknumber: Blocknumber,
            includetransactions: bool,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blocknumber, includetransactions) = args_by_pos;
                ArgByName {
                    blocknumber,
                    includetransactions,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            blocknumber,
            includetransactions,
        } = args;

        match rpc.eth_getBlockByNumber(blocknumber, includetransactions) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getBlockTransactionCountByHash<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blockhash: BlockHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blockhash) = args_by_pos;
                ArgByName { blockhash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { blockhash } = args;

        match rpc.eth_getBlockTransactionCountByHash(blockhash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getBlockTransactionCountByNumber<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Blocknumber);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blocknumber: Blocknumber,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blocknumber) = args_by_pos;
                ArgByName { blocknumber }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { blocknumber } = args;

        match rpc.eth_getBlockTransactionCountByNumber(blocknumber) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getCode<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, BlockNumber);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            address: Address,
            blocknumber: BlockNumber,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(address, blocknumber) = args_by_pos;
                ArgByName {
                    address,
                    blocknumber,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            address,
            blocknumber,
        } = args;

        match rpc.eth_getCode(address, blocknumber) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getFilterChanges<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(FilterId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            filterid: FilterId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(filterid) = args_by_pos;
                ArgByName { filterid }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { filterid } = args;

        match rpc.eth_getFilterChanges(filterid) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getFilterLogs<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(FilterId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            filterid: FilterId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(filterid) = args_by_pos;
                ArgByName { filterid }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { filterid } = args;

        match rpc.eth_getFilterLogs(filterid) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getRawTransactionByHash<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TransactionHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transactionhash: TransactionHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transactionhash) = args_by_pos;
                ArgByName { transactionhash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transactionhash } = args;

        match rpc.eth_getRawTransactionByHash(transactionhash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getRawTransactionByBlockHashAndIndex<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockHash, Integer);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blockhash: BlockHash,
            index: Integer,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blockhash, index) = args_by_pos;
                ArgByName { blockhash, index }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { blockhash, index } = args;

        match rpc.eth_getRawTransactionByBlockHashAndIndex(blockhash, index) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getRawTransactionByBlockNumberAndIndex<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Blocknumber, Integer);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blocknumber: Blocknumber,
            index: Integer,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blocknumber, index) = args_by_pos;
                ArgByName { blocknumber, index }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { blocknumber, index } = args;

        match rpc.eth_getRawTransactionByBlockNumberAndIndex(blocknumber, index) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getLogs<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
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

        match rpc.eth_getLogs(filter) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getStorageAt<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, Position, Blocknumber);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            address: Address,
            key: Position,
            blocknumber: Blocknumber,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(address, key, blocknumber) = args_by_pos;
                ArgByName {
                    address,
                    key,
                    blocknumber,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            address,
            key,
            blocknumber,
        } = args;

        match rpc.eth_getStorageAt(address, key, blocknumber) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getTransactionByBlockHashAndIndex<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockHash, Integer);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blockhash: BlockHash,
            index: Integer,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blockhash, index) = args_by_pos;
                ArgByName { blockhash, index }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { blockhash, index } = args;

        match rpc.eth_getTransactionByBlockHashAndIndex(blockhash, index) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getTransactionByBlockNumberAndIndex<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Blocknumber, Integer);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blocknumber: Blocknumber,
            index: Integer,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blocknumber, index) = args_by_pos;
                ArgByName { blocknumber, index }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { blocknumber, index } = args;

        match rpc.eth_getTransactionByBlockNumberAndIndex(blocknumber, index) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getTransactionByHash<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TransactionHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transactionhash: TransactionHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transactionhash) = args_by_pos;
                ArgByName { transactionhash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transactionhash } = args;

        match rpc.eth_getTransactionByHash(transactionhash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getTransactionCount<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, Blocknumber);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            address: Address,
            blocknumber: Blocknumber,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(address, blocknumber) = args_by_pos;
                ArgByName {
                    address,
                    blocknumber,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            address,
            blocknumber,
        } = args;

        match rpc.eth_getTransactionCount(address, blocknumber) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getTransactionReceipt<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TransactionHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transactionhash: TransactionHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transactionhash) = args_by_pos;
                ArgByName { transactionhash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transactionhash } = args;

        match rpc.eth_getTransactionReceipt(transactionhash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getUncleByBlockHashAndIndex<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockHash, Integer);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blockhash: BlockHash,
            index: Integer,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blockhash, index) = args_by_pos;
                ArgByName { blockhash, index }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { blockhash, index } = args;

        match rpc.eth_getUncleByBlockHashAndIndex(blockhash, index) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getUncleByBlockNumberAndIndex<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockNumber, Integer);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            uncleblocknumber: BlockNumber,
            index: Integer,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(uncleblocknumber, index) = args_by_pos;
                ArgByName {
                    uncleblocknumber,
                    index,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            uncleblocknumber,
            index,
        } = args;

        match rpc.eth_getUncleByBlockNumberAndIndex(uncleblocknumber, index) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getUncleCountByBlockHash<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blockhash: BlockHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blockhash) = args_by_pos;
                ArgByName { blockhash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { blockhash } = args;

        match rpc.eth_getUncleCountByBlockHash(blockhash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getUncleCountByBlockNumber<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Blocknumber);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blocknumber: Blocknumber,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blocknumber) = args_by_pos;
                ArgByName { blocknumber }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { blocknumber } = args;

        match rpc.eth_getUncleCountByBlockNumber(blocknumber) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getProof<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, Storagekeys, Blocknumber);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            address: Address,
            storagekeys: Storagekeys,
            blocknumber: Blocknumber,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(address, storagekeys, blocknumber) = args_by_pos;
                ArgByName {
                    address,
                    storagekeys,
                    blocknumber,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            address,
            storagekeys,
            blocknumber,
        } = args;

        match rpc.eth_getProof(address, storagekeys, blocknumber) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getWork<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_getWork() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_hashrate<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_hashrate() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_mining<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_mining() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_newBlockFilter<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_newBlockFilter() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_newFilter<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
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

        match rpc.eth_newFilter(filter) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_newPendingTransactionFilter<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.eth_newPendingTransactionFilter() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_pendingTransactions<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_pendingTransactions() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_protocolVersion<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_protocolVersion() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_sendRawTransaction<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Bytes);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            signedtransactiondata: Bytes,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(signedtransactiondata) = args_by_pos;
                ArgByName {
                    signedtransactiondata,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            signedtransactiondata,
        } = args;

        match rpc.eth_sendRawTransaction(signedtransactiondata) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_submitHashrate<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(DataWord, DataWord);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            hashrate: DataWord,
            id: DataWord,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(hashrate, id) = args_by_pos;
                ArgByName { hashrate, id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { hashrate, id } = args;

        match rpc.eth_submitHashrate(hashrate, id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_submitWork<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Nonce, PowHash, MixHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            nonce: Nonce,
            powhash: PowHash,
            mixhash: MixHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(nonce, powhash, mixhash) = args_by_pos;
                ArgByName {
                    nonce,
                    powhash,
                    mixhash,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            nonce,
            powhash,
            mixhash,
        } = args;

        match rpc.eth_submitWork(nonce, powhash, mixhash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_syncing<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_syncing() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_uninstallFilter<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(FilterId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            filterid: FilterId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(filterid) = args_by_pos;
                ArgByName { filterid }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { filterid } = args;

        match rpc.eth_uninstallFilter(filterid) {
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
            "web3_clientVersion" => handle_web3_clientVersion(rpc, params),
            "web3_sha3" => handle_web3_sha3(rpc, params),
            "net_listening" => handle_net_listening(rpc, params),
            "net_peerCount" => handle_net_peerCount(rpc, params),
            "net_version" => handle_net_version(rpc, params),
            "eth_blockNumber" => handle_eth_blockNumber(rpc, params),
            "eth_call" => handle_eth_call(rpc, params),
            "eth_chainId" => handle_eth_chainId(rpc, params),
            "eth_coinbase" => handle_eth_coinbase(rpc, params),
            "eth_estimateGas" => handle_eth_estimateGas(rpc, params),
            "eth_gasPrice" => handle_eth_gasPrice(rpc, params),
            "eth_getBalance" => handle_eth_getBalance(rpc, params),
            "eth_getBlockByHash" => handle_eth_getBlockByHash(rpc, params),
            "eth_getBlockByNumber" => handle_eth_getBlockByNumber(rpc, params),
            "eth_getBlockTransactionCountByHash" => {
                handle_eth_getBlockTransactionCountByHash(rpc, params)
            }
            "eth_getBlockTransactionCountByNumber" => {
                handle_eth_getBlockTransactionCountByNumber(rpc, params)
            }
            "eth_getCode" => handle_eth_getCode(rpc, params),
            "eth_getFilterChanges" => handle_eth_getFilterChanges(rpc, params),
            "eth_getFilterLogs" => handle_eth_getFilterLogs(rpc, params),
            "eth_getRawTransactionByHash" => handle_eth_getRawTransactionByHash(rpc, params),
            "eth_getRawTransactionByBlockHashAndIndex" => {
                handle_eth_getRawTransactionByBlockHashAndIndex(rpc, params)
            }
            "eth_getRawTransactionByBlockNumberAndIndex" => {
                handle_eth_getRawTransactionByBlockNumberAndIndex(rpc, params)
            }
            "eth_getLogs" => handle_eth_getLogs(rpc, params),
            "eth_getStorageAt" => handle_eth_getStorageAt(rpc, params),
            "eth_getTransactionByBlockHashAndIndex" => {
                handle_eth_getTransactionByBlockHashAndIndex(rpc, params)
            }
            "eth_getTransactionByBlockNumberAndIndex" => {
                handle_eth_getTransactionByBlockNumberAndIndex(rpc, params)
            }
            "eth_getTransactionByHash" => handle_eth_getTransactionByHash(rpc, params),
            "eth_getTransactionCount" => handle_eth_getTransactionCount(rpc, params),
            "eth_getTransactionReceipt" => handle_eth_getTransactionReceipt(rpc, params),
            "eth_getUncleByBlockHashAndIndex" => {
                handle_eth_getUncleByBlockHashAndIndex(rpc, params)
            }
            "eth_getUncleByBlockNumberAndIndex" => {
                handle_eth_getUncleByBlockNumberAndIndex(rpc, params)
            }
            "eth_getUncleCountByBlockHash" => handle_eth_getUncleCountByBlockHash(rpc, params),
            "eth_getUncleCountByBlockNumber" => handle_eth_getUncleCountByBlockNumber(rpc, params),
            "eth_getProof" => handle_eth_getProof(rpc, params),
            "eth_getWork" => handle_eth_getWork(rpc, params),
            "eth_hashrate" => handle_eth_hashrate(rpc, params),
            "eth_mining" => handle_eth_mining(rpc, params),
            "eth_newBlockFilter" => handle_eth_newBlockFilter(rpc, params),
            "eth_newFilter" => handle_eth_newFilter(rpc, params),
            "eth_newPendingTransactionFilter" => {
                handle_eth_newPendingTransactionFilter(rpc, params)
            }
            "eth_pendingTransactions" => handle_eth_pendingTransactions(rpc, params),
            "eth_protocolVersion" => handle_eth_protocolVersion(rpc, params),
            "eth_sendRawTransaction" => handle_eth_sendRawTransaction(rpc, params),
            "eth_submitHashrate" => handle_eth_submitHashrate(rpc, params),
            "eth_submitWork" => handle_eth_submitWork(rpc, params),
            "eth_syncing" => handle_eth_syncing(rpc, params),
            "eth_uninstallFilter" => handle_eth_uninstallFilter(rpc, params),
            _ => jsonrpc::Response::error(-32601, "Method not found"),
        };

        return if let Some(id) = req.id.as_ref() {
            response.with_id(id.clone())
        } else {
            response
        };
    }

    pub mod error {

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
            fn web3_clientVersion(
                &self,
            ) -> std::result::Result<Web3ClientVersionClientVersion, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "web3_clientVersion".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Web3ClientVersionClientVersion = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn web3_sha3(&self, data: Option<Data>) -> std::result::Result<Keccak, jsonrpc::Error> {
                let args = (data,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("web3_sha3".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Keccak = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn net_listening(&self) -> std::result::Result<bool, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "net_listening".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: bool = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn net_peerCount(&self) -> std::result::Result<NetPeerCountQuantity, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "net_peerCount".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: NetPeerCountQuantity = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn net_version(&self) -> std::result::Result<NetVersionNetworkId, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "net_version".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: NetVersionNetworkId = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_blockNumber(
                &self,
            ) -> std::result::Result<EthBlockNumberBlockNumber, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_blockNumber".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthBlockNumberBlockNumber =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_call(
                &self,
                transaction: Transaction,
                blocknumber: Blocknumber,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (transaction, blocknumber);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_call".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Bytes = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_chainId(&self) -> std::result::Result<EthChainIdChainId, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_chainId".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthChainIdChainId = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_coinbase(&self) -> std::result::Result<Address, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_coinbase".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Address = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_estimateGas(
                &self,
                transaction: Transaction,
            ) -> std::result::Result<Integer, jsonrpc::Error> {
                let args = (transaction,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_estimateGas".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Integer = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_gasPrice(&self) -> std::result::Result<Integer, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_gasPrice".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Integer = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getBalance(
                &self,
                address: Address,
                blocknumber: BlockNumber,
            ) -> std::result::Result<IntegerOrNull, jsonrpc::Error> {
                let args = (address, blocknumber);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getBalance".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: IntegerOrNull = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getBlockByHash(
                &self,
                blockhash: BlockHash,
                includetransactions: bool,
            ) -> std::result::Result<BlockOrNull, jsonrpc::Error> {
                let args = (blockhash, includetransactions);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getBlockByHash".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: BlockOrNull = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getBlockByNumber(
                &self,
                blocknumber: Blocknumber,
                includetransactions: bool,
            ) -> std::result::Result<BlockOrNull, jsonrpc::Error> {
                let args = (blocknumber, includetransactions);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getBlockByNumber".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: BlockOrNull = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getBlockTransactionCountByHash(
                &self,
                blockhash: BlockHash,
            ) -> std::result::Result<IntegerOrNull, jsonrpc::Error> {
                let args = (blockhash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req =
                    jsonrpc::Request::new("eth_getBlockTransactionCountByHash".to_string(), params)
                        .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: IntegerOrNull = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getBlockTransactionCountByNumber(
                &self,
                blocknumber: Blocknumber,
            ) -> std::result::Result<IntegerOrNull, jsonrpc::Error> {
                let args = (blocknumber,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new(
                    "eth_getBlockTransactionCountByNumber".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: IntegerOrNull = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getCode(
                &self,
                address: Address,
                blocknumber: BlockNumber,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (address, blocknumber);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getCode".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Bytes = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getFilterChanges(
                &self,
                filterid: FilterId,
            ) -> std::result::Result<EthGetFilterChangesLogResult, jsonrpc::Error> {
                let args = (filterid,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getFilterChanges".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthGetFilterChangesLogResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getFilterLogs(
                &self,
                filterid: FilterId,
            ) -> std::result::Result<EthGetFilterLogsLogs, jsonrpc::Error> {
                let args = (filterid,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getFilterLogs".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthGetFilterLogsLogs = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getRawTransactionByHash(
                &self,
                transactionhash: TransactionHash,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (transactionhash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getRawTransactionByHash".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Bytes = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getRawTransactionByBlockHashAndIndex(
                &self,
                blockhash: BlockHash,
                index: Integer,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (blockhash, index);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new(
                    "eth_getRawTransactionByBlockHashAndIndex".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Bytes = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getRawTransactionByBlockNumberAndIndex(
                &self,
                blocknumber: Blocknumber,
                index: Integer,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (blocknumber, index);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new(
                    "eth_getRawTransactionByBlockNumberAndIndex".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Bytes = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getLogs(
                &self,
                filter: Filter,
            ) -> std::result::Result<EthGetLogsLogs, jsonrpc::Error> {
                let args = (filter,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getLogs".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthGetLogsLogs = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getStorageAt(
                &self,
                address: Address,
                key: Position,
                blocknumber: Blocknumber,
            ) -> std::result::Result<DataWord, jsonrpc::Error> {
                let args = (address, key, blocknumber);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getStorageAt".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: DataWord = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getTransactionByBlockHashAndIndex(
                &self,
                blockhash: BlockHash,
                index: Integer,
            ) -> std::result::Result<
                EthGetTransactionByBlockHashAndIndexTransactionResult,
                jsonrpc::Error,
            > {
                let args = (blockhash, index);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new(
                    "eth_getTransactionByBlockHashAndIndex".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthGetTransactionByBlockHashAndIndexTransactionResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getTransactionByBlockNumberAndIndex(
                &self,
                blocknumber: Blocknumber,
                index: Integer,
            ) -> std::result::Result<
                EthGetTransactionByBlockNumberAndIndexTransactionResult,
                jsonrpc::Error,
            > {
                let args = (blocknumber, index);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new(
                    "eth_getTransactionByBlockNumberAndIndex".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthGetTransactionByBlockNumberAndIndexTransactionResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getTransactionByHash(
                &self,
                transactionhash: TransactionHash,
            ) -> std::result::Result<EthGetTransactionByHashTransactionResult, jsonrpc::Error>
            {
                let args = (transactionhash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getTransactionByHash".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthGetTransactionByHashTransactionResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getTransactionCount(
                &self,
                address: Address,
                blocknumber: Blocknumber,
            ) -> std::result::Result<EthGetTransactionCountTransactionCount, jsonrpc::Error>
            {
                let args = (address, blocknumber);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getTransactionCount".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthGetTransactionCountTransactionCount = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getTransactionReceipt(
                &self,
                transactionhash: TransactionHash,
            ) -> std::result::Result<EthGetTransactionReceiptTransactionReceiptResult, jsonrpc::Error>
            {
                let args = (transactionhash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getTransactionReceipt".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthGetTransactionReceiptTransactionReceiptResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getUncleByBlockHashAndIndex(
                &self,
                blockhash: BlockHash,
                index: Integer,
            ) -> std::result::Result<BlockOrNull, jsonrpc::Error> {
                let args = (blockhash, index);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req =
                    jsonrpc::Request::new("eth_getUncleByBlockHashAndIndex".to_string(), params)
                        .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: BlockOrNull = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getUncleByBlockNumberAndIndex(
                &self,
                uncleblocknumber: BlockNumber,
                index: Integer,
            ) -> std::result::Result<BlockOrNull, jsonrpc::Error> {
                let args = (uncleblocknumber, index);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req =
                    jsonrpc::Request::new("eth_getUncleByBlockNumberAndIndex".to_string(), params)
                        .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: BlockOrNull = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getUncleCountByBlockHash(
                &self,
                blockhash: BlockHash,
            ) -> std::result::Result<IntegerOrNull, jsonrpc::Error> {
                let args = (blockhash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getUncleCountByBlockHash".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: IntegerOrNull = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getUncleCountByBlockNumber(
                &self,
                blocknumber: Blocknumber,
            ) -> std::result::Result<IntegerOrNull, jsonrpc::Error> {
                let args = (blocknumber,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req =
                    jsonrpc::Request::new("eth_getUncleCountByBlockNumber".to_string(), params)
                        .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: IntegerOrNull = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getProof(
                &self,
                address: Address,
                storagekeys: Storagekeys,
                blocknumber: Blocknumber,
            ) -> std::result::Result<EthGetProofAccount, jsonrpc::Error> {
                let args = (address, storagekeys, blocknumber);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getProof".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthGetProofAccount = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getWork(&self) -> std::result::Result<EthGetWorkWork, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_getWork".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthGetWorkWork = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_hashrate(&self) -> std::result::Result<Integer, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_hashrate".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Integer = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_mining(&self) -> std::result::Result<bool, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_mining".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: bool = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_newBlockFilter(&self) -> std::result::Result<FilterId, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_newBlockFilter".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: FilterId = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_newFilter(
                &self,
                filter: Filter,
            ) -> std::result::Result<Integer, jsonrpc::Error> {
                let args = (filter,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_newFilter".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Integer = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_newPendingTransactionFilter(
                &self,
            ) -> std::result::Result<FilterId, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_newPendingTransactionFilter".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: FilterId = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_pendingTransactions(&self) -> std::result::Result<Transactions, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_pendingTransactions".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Transactions = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_protocolVersion(&self) -> std::result::Result<Integer, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_protocolVersion".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Integer = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_sendRawTransaction(
                &self,
                signedtransactiondata: Bytes,
            ) -> std::result::Result<Keccak, jsonrpc::Error> {
                let args = (signedtransactiondata,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_sendRawTransaction".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: Keccak = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_submitHashrate(
                &self,
                hashrate: DataWord,
                id: DataWord,
            ) -> std::result::Result<bool, jsonrpc::Error> {
                let args = (hashrate, id);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_submitHashrate".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: bool = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_submitWork(
                &self,
                nonce: Nonce,
                powhash: PowHash,
                mixhash: MixHash,
            ) -> std::result::Result<bool, jsonrpc::Error> {
                let args = (nonce, powhash, mixhash);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_submitWork".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: bool = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_syncing(&self) -> std::result::Result<EthSyncingSyncing, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_syncing".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: EthSyncingSyncing = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_uninstallFilter(
                &self,
                filterid: FilterId,
            ) -> std::result::Result<bool, jsonrpc::Error> {
                let args = (filterid,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_uninstallFilter".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

                log::debug!("{req:#?}");

                let mut res: jsonrpc::Response = self
                    .client
                    .post(&self.url)
                    .json(&req)
                    .send()
                    .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
                    .json()
                    .map_err(|e| {
                        jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}."))
                    })?;

                if let Some(err) = res.error.take() {
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let out: bool = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;

                    log::debug!("{out:#?}");

                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }
        }
    }
}
// ^^^ GENERATED CODE ABOVE ^^^
