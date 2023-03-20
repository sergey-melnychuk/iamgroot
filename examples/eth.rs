fn main() {
    {
        let json = include_str!("../0x10165f5.json");
        let mut res: openrpc_stub_gen::jsonrpc::Response = serde_json::from_str(json).unwrap();
        let res: gen::Block = serde_json::from_value(res.result.take().unwrap()).unwrap();
        println!("{res:?}");
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

    use openrpc_stub_gen::jsonrpc;

    // object: 'AccessList'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct AccessList(pub Vec<AccessListEntry>); // name == binding_name

    // object: 'AccessListEntry'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct AccessListEntry {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<Address>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "storageKeys")]
        pub storagekeys: Option<Vec<Hash32>>,
    }

    // object: 'AccountProof'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct AccountProof {
        #[serde(rename = "accountProof")]
        pub accountproof: Vec<Bytes>,
        pub address: Address,
        pub balance: Uint256,
        #[serde(rename = "codeHash")]
        pub codehash: Hash32,
        pub nonce: Uint64,
        #[serde(rename = "storageHash")]
        pub storagehash: Hash32,
        #[serde(rename = "storageProof")]
        pub storageproof: Vec<StorageProof>,
    }

    // object: 'BadBlock'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BadBlock {
        pub block: Bytes,
        pub hash: Hash32,
        pub rlp: Bytes,
    }

    // object: 'Block'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Block {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "baseFeePerGas")]
        pub basefeepergas: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub difficulty: Option<Bytes>,
        #[serde(rename = "extraData")]
        pub extradata: Bytes,
        #[serde(rename = "gasLimit")]
        pub gaslimit: Uint,
        #[serde(rename = "gasUsed")]
        pub gasused: Uint,
        #[serde(rename = "logsBloom")]
        pub logsbloom: Bytes256,
        pub miner: Address,
        #[serde(rename = "mixHash")]
        pub mixhash: Hash32,
        pub nonce: Bytes8,
        pub number: Uint,
        #[serde(rename = "parentHash")]
        pub parenthash: Hash32,
        #[serde(rename = "receiptsRoot")]
        pub receiptsroot: Hash32,
        #[serde(rename = "sha3Uncles")]
        pub sha3uncles: Hash32,
        pub size: Uint,
        #[serde(rename = "stateRoot")]
        pub stateroot: Hash32,
        pub timestamp: Uint,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "totalDifficulty")]
        pub totaldifficulty: Option<Uint>,
        pub transactions: Vec<Transaction>,
        #[serde(rename = "transactionsRoot")]
        pub transactionsroot: Hash32,
        pub uncles: Vec<Hash32>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdrawals: Option<Vec<Withdrawal>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "withdrawalsRoot")]
        pub withdrawalsroot: Option<Hash32>,
    }

    // object: 'BlockNumberOrTag'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BlockNumberOrTag {
        BlockTag(BlockTag),
        Uint(Uint),
    }

    // object: 'BlockNumberOrTagOrHash'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BlockNumberOrTagOrHash {
        BlockTag(BlockTag),
        Hash32(Hash32),
        Uint(Uint),
    }

    // object: 'BlockTag'
    #[derive(Debug, Deserialize, Serialize)]
    pub enum BlockTag {
        #[serde(rename = "earliest")]
        Earliest,
        #[serde(rename = "finalized")]
        Finalized,
        #[serde(rename = "latest")]
        Latest,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "safe")]
        Safe,
    }

    // object: 'Boolean'
    // pub type Boolean = bool;

    // object: 'GenericTransaction'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct GenericTransaction {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "accessList")]
        pub accesslist: Option<AccessList>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "chainId")]
        pub chainid: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from: Option<Address>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gas: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "gasPrice")]
        pub gasprice: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub input: Option<Bytes>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "maxFeePerGas")]
        pub maxfeepergas: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "maxPriorityFeePerGas")]
        pub maxpriorityfeepergas: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub nonce: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<Address>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "type")]
        pub r#type: Option<Byte>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Uint>,
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
        pub blockhash: Option<Hash32>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "blockNumber")]
        pub blocknumber: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<Bytes>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "logIndex")]
        pub logindex: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub removed: Option<bool>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub topics: Option<Vec<Bytes32>>,
        #[serde(rename = "transactionHash")]
        pub transactionhash: Hash32,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "transactionIndex")]
        pub transactionindex: Option<Uint>,
    }

    // object: 'ReceiptInfo'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ReceiptInfo {
        #[serde(rename = "blockHash")]
        pub blockhash: Hash32,
        #[serde(rename = "blockNumber")]
        pub blocknumber: Uint,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "contractAddress")]
        pub contractaddress: Option<Address>,
        #[serde(rename = "cumulativeGasUsed")]
        pub cumulativegasused: Uint,
        #[serde(rename = "effectiveGasPrice")]
        pub effectivegasprice: Uint,
        pub from: Address,
        #[serde(rename = "gasUsed")]
        pub gasused: Uint,
        pub logs: Vec<Log>,
        #[serde(rename = "logsBloom")]
        pub logsbloom: Bytes256,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub root: Option<Hash32>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<Address>,
        #[serde(rename = "transactionHash")]
        pub transactionhash: Hash32,
        #[serde(rename = "transactionIndex")]
        pub transactionindex: Uint,
    }

    // object: 'StorageProof'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct StorageProof {
        pub key: BytesMax32,
        pub proof: Vec<Bytes>,
        pub value: Uint256,
    }

    // object: 'SyncingProgress'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SyncingProgress {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "currentBlock")]
        pub currentblock: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "highestBlock")]
        pub highestblock: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "startingBlock")]
        pub startingblock: Option<Uint>,
    }

    // object: 'SyncingStatus'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum SyncingStatus {
        Boolean(bool),
        SyncingProgress(SyncingProgress),
    }

    // object: 'Transaction'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum Transaction {
        TransactionSigned(TransactionSigned),
        Hash32(Hash32),
    }

    // object: 'Transaction1559Signed'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Transaction1559Signed {
        #[serde(flatten)]
        pub transaction1559unsigned: Transaction1559Unsigned,
        pub r: Uint,
        pub s: Uint,
        #[serde(rename = "yParity")]
        pub yparity: Uint,
    }

    // object: 'Transaction1559Unsigned'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Transaction1559Unsigned {
        #[serde(rename = "accessList")]
        pub accesslist: AccessList,
        #[serde(rename = "chainId")]
        pub chainid: Uint,
        pub gas: Uint,
        pub input: Bytes,
        #[serde(rename = "maxFeePerGas")]
        pub maxfeepergas: Uint,
        #[serde(rename = "maxPriorityFeePerGas")]
        pub maxpriorityfeepergas: Uint,
        pub nonce: Uint,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<Address>,
        #[serde(rename = "type")]
        pub r#type: Byte,
        pub value: Uint,
    }

    // object: 'Transaction2930Signed'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Transaction2930Signed {
        #[serde(flatten)]
        pub transaction2930unsigned: Transaction2930Unsigned,
        pub r: Uint,
        pub s: Uint,
        #[serde(rename = "yParity")]
        pub yparity: Uint,
    }

    // object: 'Transaction2930Unsigned'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Transaction2930Unsigned {
        #[serde(rename = "accessList")]
        pub accesslist: AccessList,
        #[serde(rename = "chainId")]
        pub chainid: Uint,
        pub gas: Uint,
        #[serde(rename = "gasPrice")]
        pub gasprice: Uint,
        pub input: Bytes,
        pub nonce: Uint,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<Address>,
        #[serde(rename = "type")]
        pub r#type: Byte,
        pub value: Uint,
    }

    // object: 'TransactionFull'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TransactionFull {
        Transaction1559Signed(Transaction1559Signed),
        Transaction2930Signed(Transaction2930Signed),
        TransactionLegacySigned(TransactionLegacySigned),
    }

    // object: 'TransactionHash'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct TransactionHash(pub Hash32); // name != binding_name

    // object: 'TransactionInfo'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct TransactionInfo {
        #[serde(flatten)]
        pub transactionsigned: TransactionSigned,
        #[serde(rename = "blockHash")]
        pub blockhash: Hash32,
        #[serde(rename = "blockNumber")]
        pub blocknumber: Uint,
        pub from: Address,
        pub hash: Hash32,
        #[serde(rename = "transactionIndex")]
        pub transactionindex: Uint,
    }

    // object: 'TransactionLegacySigned'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct TransactionLegacySigned {
        #[serde(flatten)]
        pub transactionlegacyunsigned: TransactionLegacyUnsigned,
        pub r: Uint,
        pub s: Uint,
        pub v: Uint,
    }

    // object: 'TransactionLegacyUnsigned'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct TransactionLegacyUnsigned {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "chainId")]
        pub chainid: Option<Uint>,
        pub gas: Uint,
        #[serde(rename = "gasPrice")]
        pub gasprice: Uint,
        pub input: Bytes,
        pub nonce: Uint,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<Address>,
        #[serde(rename = "type")]
        pub r#type: Byte,
        pub value: Uint,
    }

    // object: 'TransactionSigned'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TransactionSigned {
        Transaction1559Signed(Transaction1559Signed),
        Transaction2930Signed(Transaction2930Signed),
        TransactionLegacySigned(TransactionLegacySigned),
    }

    // object: 'TransactionUnsigned'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TransactionUnsigned {
        Transaction1559Unsigned(Transaction1559Unsigned),
        Transaction2930Unsigned(Transaction2930Unsigned),
        TransactionLegacyUnsigned(TransactionLegacyUnsigned),
    }

    // object: 'Withdrawal'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Withdrawal {
        pub address: Address,
        pub amount: Uint256,
        pub index: Uint64,
        #[serde(rename = "validatorIndex")]
        pub validatorindex: Uint64,
    }

    // object: 'address'
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
            Lazy::new(|| Regex::new("^0x[0-9,a-f,A-F]{40}$").unwrap());

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

    // object: 'addresses'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Addresses(pub Vec<Address>); // name == binding_name

    // object: 'byte'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Byte(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Byte(String);

    mod byte {
        use super::jsonrpc;
        use super::Byte;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static BYTE_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x([0-9,a-f,A-F]?){1,2}$").unwrap());

        impl Byte {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if BYTE_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Byte value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Byte {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Byte {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'bytes'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Bytes(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Bytes(String);

    mod bytes {
        use super::jsonrpc;
        use super::Bytes;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static BYTES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[0-9a-f]*$").unwrap());

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

    // object: 'bytes256'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Bytes256(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Bytes256(String);

    mod bytes256 {
        use super::jsonrpc;
        use super::Bytes256;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static BYTES256_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[0-9a-f]{512}$").unwrap());

        impl Bytes256 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if BYTES256_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Bytes256 value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Bytes256 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Bytes256 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'bytes32'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Bytes32(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Bytes32(String);

    mod bytes32 {
        use super::jsonrpc;
        use super::Bytes32;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static BYTES32_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[0-9a-f]{64}$").unwrap());

        impl Bytes32 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if BYTES32_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Bytes32 value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Bytes32 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Bytes32 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'bytes65'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Bytes65(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Bytes65(String);

    mod bytes65 {
        use super::jsonrpc;
        use super::Bytes65;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static BYTES65_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[0-9a-f]{65}$").unwrap());

        impl Bytes65 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if BYTES65_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Bytes65 value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Bytes65 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Bytes65 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'bytes8'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Bytes8(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Bytes8(String);

    mod bytes8 {
        use super::jsonrpc;
        use super::Bytes8;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static BYTES8_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[0-9a-f]{16}$").unwrap());

        impl Bytes8 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if BYTES8_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Bytes8 value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Bytes8 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Bytes8 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'bytesMax32'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct BytesMax32(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct BytesMax32(String);

    mod bytesmax32 {
        use super::jsonrpc;
        use super::BytesMax32;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static BYTESMAX32_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x[0-9a-f]{0,64}$").unwrap());

        impl BytesMax32 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if BYTESMAX32_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "BytesMax32 value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for BytesMax32 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for BytesMax32 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'debug_getBadBlocks_Blocks'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DebugGetBadBlocksBlocks(pub Vec<BadBlock>); // name == binding_name

    // object: 'debug_getRawBlock_Block RLP'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DebugGetRawBlockBlockRlp(pub Bytes); // name != binding_name

    // object: 'debug_getRawHeader_Header RLP'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DebugGetRawHeaderHeaderRlp(pub Bytes); // name != binding_name

    // object: 'debug_getRawReceipts_Receipts'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DebugGetRawReceiptsReceipts(pub Vec<Bytes>); // name == binding_name

    // object: 'debug_getRawTransaction_EIP-2718 binary-encoded transaction'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DebugGetRawTransactionEip2718BinaryEncodedTransaction(pub Bytes); // name != binding_name

    // object: 'eth_accounts_Accounts'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthAccountsAccounts(pub Vec<Address>); // name == binding_name

    // object: 'eth_blockNumber_Block number'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthBlockNumberBlockNumber(pub Uint); // name != binding_name

    // object: 'eth_call_Return data'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthCallReturnData(pub Bytes); // name != binding_name

    // object: 'eth_chainId_Chain ID'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthChainIdChainId(pub Uint); // name != binding_name

    // object: 'eth_coinbase_Coinbase address'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthCoinbaseCoinbaseAddress(pub Address); // name != binding_name

    // object: 'eth_createAccessList_Gas used'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthCreateAccessListGasUsed {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "accessList")]
        pub accesslist: Option<AccessList>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "gasUsed")]
        pub gasused: Option<Uint>,
    }

    // object: 'eth_estimateGas_Gas used'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthEstimateGasGasUsed(pub Uint); // name != binding_name

    // object: 'eth_feeHistory_feeHistoryResult'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthFeeHistoryFeeHistoryResult {
        #[serde(rename = "baseFeePerGas")]
        pub basefeepergas: Vec<Uint>,
        #[serde(rename = "oldestBlock")]
        pub oldestblock: Uint,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reward: Option<Vec<Vec<Uint>>>,
    }

    // object: 'eth_gasPrice_Gas price'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGasPriceGasPrice(pub Uint); // name != binding_name

    // object: 'eth_getBalance_Balance'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetBalanceBalance(pub Uint); // name != binding_name

    // object: 'eth_getBlockByHash_Block information'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetBlockByHashBlockInformation {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "baseFeePerGas")]
        pub basefeepergas: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub difficulty: Option<Bytes>,
        #[serde(rename = "extraData")]
        pub extradata: Bytes,
        #[serde(rename = "gasLimit")]
        pub gaslimit: Uint,
        #[serde(rename = "gasUsed")]
        pub gasused: Uint,
        #[serde(rename = "logsBloom")]
        pub logsbloom: Bytes256,
        pub miner: Address,
        #[serde(rename = "mixHash")]
        pub mixhash: Hash32,
        pub nonce: Bytes8,
        pub number: Uint,
        #[serde(rename = "parentHash")]
        pub parenthash: Hash32,
        #[serde(rename = "receiptsRoot")]
        pub receiptsroot: Hash32,
        #[serde(rename = "sha3Uncles")]
        pub sha3uncles: Hash32,
        pub size: Uint,
        #[serde(rename = "stateRoot")]
        pub stateroot: Hash32,
        pub timestamp: Uint,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "totalDifficulty")]
        pub totaldifficulty: Option<Uint>,
        pub transactions: Vec<Transaction>,
        #[serde(rename = "transactionsRoot")]
        pub transactionsroot: Hash32,
        pub uncles: Vec<Hash32>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdrawals: Option<Vec<Withdrawal>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "withdrawalsRoot")]
        pub withdrawalsroot: Option<Hash32>,
    }

    // object: 'eth_getBlockByNumber_Block information'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetBlockByNumberBlockInformation {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "baseFeePerGas")]
        pub basefeepergas: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub difficulty: Option<Bytes>,
        #[serde(rename = "extraData")]
        pub extradata: Bytes,
        #[serde(rename = "gasLimit")]
        pub gaslimit: Uint,
        #[serde(rename = "gasUsed")]
        pub gasused: Uint,
        #[serde(rename = "logsBloom")]
        pub logsbloom: Bytes256,
        pub miner: Address,
        #[serde(rename = "mixHash")]
        pub mixhash: Hash32,
        pub nonce: Bytes8,
        pub number: Uint,
        #[serde(rename = "parentHash")]
        pub parenthash: Hash32,
        #[serde(rename = "receiptsRoot")]
        pub receiptsroot: Hash32,
        #[serde(rename = "sha3Uncles")]
        pub sha3uncles: Hash32,
        pub size: Uint,
        #[serde(rename = "stateRoot")]
        pub stateroot: Hash32,
        pub timestamp: Uint,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "totalDifficulty")]
        pub totaldifficulty: Option<Uint>,
        pub transactions: Vec<Transaction>,
        #[serde(rename = "transactionsRoot")]
        pub transactionsroot: Hash32,
        pub uncles: Vec<Hash32>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdrawals: Option<Vec<Withdrawal>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "withdrawalsRoot")]
        pub withdrawalsroot: Option<Hash32>,
    }

    // object: 'eth_getBlockTransactionCountByHash_Transaction count'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetBlockTransactionCountByHashTransactionCount(pub Uint); // name != binding_name

    // object: 'eth_getBlockTransactionCountByNumber_Transaction count'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetBlockTransactionCountByNumberTransactionCount(pub Uint); // name != binding_name

    // object: 'eth_getCode_Bytecode'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetCodeBytecode(pub Bytes); // name != binding_name

    // object: 'eth_getProof_Account'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetProofAccount {
        #[serde(rename = "accountProof")]
        pub accountproof: Vec<Bytes>,
        pub address: Address,
        pub balance: Uint256,
        #[serde(rename = "codeHash")]
        pub codehash: Hash32,
        pub nonce: Uint64,
        #[serde(rename = "storageHash")]
        pub storagehash: Hash32,
        #[serde(rename = "storageProof")]
        pub storageproof: Vec<StorageProof>,
    }

    // object: 'eth_getStorageAt_Value'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetStorageAtValue(pub Bytes); // name != binding_name

    // object: 'eth_getTransactionByBlockHashAndIndex_Transaction information'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetTransactionByBlockHashAndIndexTransactionInformation {
        #[serde(flatten)]
        pub transactionsigned: TransactionSigned,
        #[serde(rename = "blockHash")]
        pub blockhash: Hash32,
        #[serde(rename = "blockNumber")]
        pub blocknumber: Uint,
        pub from: Address,
        pub hash: Hash32,
        #[serde(rename = "transactionIndex")]
        pub transactionindex: Uint,
    }

    // object: 'eth_getTransactionByBlockNumberAndIndex_Transaction information'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetTransactionByBlockNumberAndIndexTransactionInformation {
        #[serde(flatten)]
        pub transactionsigned: TransactionSigned,
        #[serde(rename = "blockHash")]
        pub blockhash: Hash32,
        #[serde(rename = "blockNumber")]
        pub blocknumber: Uint,
        pub from: Address,
        pub hash: Hash32,
        #[serde(rename = "transactionIndex")]
        pub transactionindex: Uint,
    }

    // object: 'eth_getTransactionByHash_Transaction information'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetTransactionByHashTransactionInformation {
        #[serde(flatten)]
        pub transactionsigned: TransactionSigned,
        #[serde(rename = "blockHash")]
        pub blockhash: Hash32,
        #[serde(rename = "blockNumber")]
        pub blocknumber: Uint,
        pub from: Address,
        pub hash: Hash32,
        #[serde(rename = "transactionIndex")]
        pub transactionindex: Uint,
    }

    // object: 'eth_getTransactionCount_Transaction count'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetTransactionCountTransactionCount(pub Uint); // name != binding_name

    // object: 'eth_getTransactionReceipt_Receipt Information'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetTransactionReceiptReceiptInformation {
        #[serde(rename = "blockHash")]
        pub blockhash: Hash32,
        #[serde(rename = "blockNumber")]
        pub blocknumber: Uint,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "contractAddress")]
        pub contractaddress: Option<Address>,
        #[serde(rename = "cumulativeGasUsed")]
        pub cumulativegasused: Uint,
        #[serde(rename = "effectiveGasPrice")]
        pub effectivegasprice: Uint,
        pub from: Address,
        #[serde(rename = "gasUsed")]
        pub gasused: Uint,
        pub logs: Vec<Log>,
        #[serde(rename = "logsBloom")]
        pub logsbloom: Bytes256,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub root: Option<Hash32>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<Uint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<Address>,
        #[serde(rename = "transactionHash")]
        pub transactionhash: Hash32,
        #[serde(rename = "transactionIndex")]
        pub transactionindex: Uint,
    }

    // object: 'eth_getUncleCountByBlockHash_Uncle count'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetUncleCountByBlockHashUncleCount(pub Uint); // name != binding_name

    // object: 'eth_getUncleCountByBlockNumber_Uncle count'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthGetUncleCountByBlockNumberUncleCount(pub Uint); // name != binding_name

    // object: 'eth_maxPriorityFeePerGas_Max priority fee per gas'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthMaxPriorityFeePerGasMaxPriorityFeePerGas(pub Uint); // name != binding_name

    // object: 'eth_newBlockFilter_Filter Identifier'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthNewBlockFilterFilterIdentifier(pub Uint); // name != binding_name

    // object: 'eth_newPendingTransactionFilter_Filter Identifier'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthNewPendingTransactionFilterFilterIdentifier(pub Uint); // name != binding_name

    // object: 'eth_sendRawTransaction_Transaction hash'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthSendRawTransactionTransactionHash(pub Hash32); // name != binding_name

    // object: 'eth_sendTransaction_Transaction hash'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthSendTransactionTransactionHash(pub Hash32); // name != binding_name

    // object: 'eth_signTransaction_Encoded transaction'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthSignTransactionEncodedTransaction(pub Bytes); // name != binding_name

    // object: 'eth_sign_Signature'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct EthSignSignature(pub Bytes65); // name != binding_name

    // object: 'eth_syncing_Syncing status'
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum EthSyncingSyncingStatus {
        Boolean(bool),
        SyncingProgress(SyncingProgress),
    }

    // object: 'eth_uninstallFilter_Success'
    // pub type EthUninstallFilterSuccess = bool;

    // object: 'hash32'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Hash32(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Hash32(String);

    mod hash32 {
        use super::jsonrpc;
        use super::Hash32;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static HASH32_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[0-9a-f]{64}$").unwrap());

        impl Hash32 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if HASH32_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Hash32 value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Hash32 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Hash32 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'rewardpercentiles'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Rewardpercentiles(pub Vec<Rewardpercentiles>); // name == binding_name

    // object: 'storagekeys'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Storagekeys(pub Vec<BytesMax32>); // name == binding_name

    // object: 'uint'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Uint(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Uint(String);

    mod uint {
        use super::jsonrpc;
        use super::Uint;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static UINT_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x([1-9a-f]+[0-9a-f]*|0)$").unwrap());

        impl Uint {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if UINT_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Uint value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Uint {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Uint {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'uint256'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Uint256(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Uint256(String);

    mod uint256 {
        use super::jsonrpc;
        use super::Uint256;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static UINT256_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x([1-9a-f]+[0-9a-f]{0,31})|0$").unwrap());

        impl Uint256 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if UINT256_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Uint256 value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Uint256 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Uint256 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'uint64'
    #[derive(Debug, Deserialize, Serialize)]
    // pub struct Uint64(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Uint64(String);

    mod uint64 {
        use super::jsonrpc;
        use super::Uint64;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static UINT64_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x([1-9a-f]+[0-9a-f]{0,15})|0$").unwrap());

        impl Uint64 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if UINT64_REGEX.is_match(&value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: "Uint64 value does not match regex".to_string(),
                    })
                }
            }
        }

        impl TryFrom<String> for Uint64 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Uint64 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    pub trait Rpc {
        /// Method: 'eth_getBlockByHash'
        /// Summary: Returns information about a block by hash.
        /// Description:
        ///
        fn eth_getBlockByHash(
            &self,
            block_hash: Hash32,
            hydrated_transactions: bool,
        ) -> std::result::Result<Block, jsonrpc::Error>;

        /// Method: 'eth_getBlockByNumber'
        /// Summary: Returns information about a block by number.
        /// Description:
        ///
        fn eth_getBlockByNumber(
            &self,
            block: BlockNumberOrTag,
            hydrated_transactions: bool,
        ) -> std::result::Result<Block, jsonrpc::Error>;

        /// Method: 'eth_getBlockTransactionCountByHash'
        /// Summary: Returns the number of transactions in a block from a block matching the given block hash.
        /// Description:
        ///
        fn eth_getBlockTransactionCountByHash(
            &self,
            block_hash: Option<Hash32>,
        ) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_getBlockTransactionCountByNumber'
        /// Summary: Returns the number of transactions in a block matching the given block number.
        /// Description:
        ///
        fn eth_getBlockTransactionCountByNumber(
            &self,
            block: Option<BlockNumberOrTag>,
        ) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_getUncleCountByBlockHash'
        /// Summary: Returns the number of uncles in a block from a block matching the given block hash.
        /// Description:
        ///
        fn eth_getUncleCountByBlockHash(
            &self,
            block_hash: Option<Hash32>,
        ) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_getUncleCountByBlockNumber'
        /// Summary: Returns the number of transactions in a block matching the given block number.
        /// Description:
        ///
        fn eth_getUncleCountByBlockNumber(
            &self,
            block: Option<BlockNumberOrTag>,
        ) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_chainId'
        /// Summary: Returns the chain ID of the current network.
        /// Description:
        ///
        fn eth_chainId(&self) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_syncing'
        /// Summary: Returns an object with data about the sync status or false.
        /// Description:
        ///
        fn eth_syncing(&self) -> std::result::Result<SyncingStatus, jsonrpc::Error>;

        /// Method: 'eth_coinbase'
        /// Summary: Returns the client coinbase address.
        /// Description:
        ///
        fn eth_coinbase(&self) -> std::result::Result<Address, jsonrpc::Error>;

        /// Method: 'eth_accounts'
        /// Summary: Returns a list of addresses owned by client.
        /// Description:
        ///
        fn eth_accounts(&self) -> std::result::Result<EthAccountsAccounts, jsonrpc::Error>;

        /// Method: 'eth_blockNumber'
        /// Summary: Returns the number of most recent block.
        /// Description:
        ///
        fn eth_blockNumber(&self) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_call'
        /// Summary: Executes a new message call immediately without creating a transaction on the block chain.
        /// Description:
        ///
        fn eth_call(
            &self,
            transaction: GenericTransaction,
            block: Option<BlockNumberOrTagOrHash>,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'eth_estimateGas'
        /// Summary: Generates and returns an estimate of how much gas is necessary to allow the transaction to complete.
        /// Description:
        ///
        fn eth_estimateGas(
            &self,
            transaction: GenericTransaction,
            block: Option<BlockNumberOrTag>,
        ) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_createAccessList'
        /// Summary: Generates an access list for a transaction.
        /// Description:
        ///
        fn eth_createAccessList(
            &self,
            transaction: GenericTransaction,
            block: Option<BlockNumberOrTag>,
        ) -> std::result::Result<EthCreateAccessListGasUsed, jsonrpc::Error>;

        /// Method: 'eth_gasPrice'
        /// Summary: Returns the current price per gas in wei.
        /// Description:
        ///
        fn eth_gasPrice(&self) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_maxPriorityFeePerGas'
        /// Summary: Returns the current maxPriorityFeePerGas per gas in wei.
        /// Description:
        ///
        fn eth_maxPriorityFeePerGas(&self) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_feeHistory'
        /// Summary: Transaction fee history
        /// Description: Returns transaction base fee per gas and effective priority fee per gas for the requested/supported block range.
        ///
        fn eth_feeHistory(
            &self,
            blockcount: Uint,
            newestblock: BlockNumberOrTag,
            rewardpercentiles: Rewardpercentiles,
        ) -> std::result::Result<EthFeeHistoryFeeHistoryResult, jsonrpc::Error>;

        /// Method: 'eth_newBlockFilter'
        /// Summary: Creates a filter in the node, to notify when a new block arrives.
        /// Description:
        ///
        fn eth_newBlockFilter(&self) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_newPendingTransactionFilter'
        /// Summary: Creates a filter in the node, to notify when new pending transactions arrive.
        /// Description:
        ///
        fn eth_newPendingTransactionFilter(&self) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_uninstallFilter'
        /// Summary: Uninstalls a filter with given id.
        /// Description:
        ///
        fn eth_uninstallFilter(
            &self,
            filter_identifier: Option<Uint>,
        ) -> std::result::Result<bool, jsonrpc::Error>;

        /// Method: 'eth_sign'
        /// Summary: Returns an EIP-191 signature over the provided data.
        /// Description:
        ///
        fn eth_sign(
            &self,
            address: Address,
            message: Bytes,
        ) -> std::result::Result<Bytes65, jsonrpc::Error>;

        /// Method: 'eth_signTransaction'
        /// Summary: Returns an RLP encoded transaction signed by the specified account.
        /// Description:
        ///
        fn eth_signTransaction(
            &self,
            transaction: GenericTransaction,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'eth_getBalance'
        /// Summary: Returns the balance of the account of given address.
        /// Description:
        ///
        fn eth_getBalance(
            &self,
            address: Address,
            block: Option<BlockNumberOrTagOrHash>,
        ) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_getStorageAt'
        /// Summary: Returns the value from a storage position at a given address.
        /// Description:
        ///
        fn eth_getStorageAt(
            &self,
            address: Address,
            storage_slot: Uint256,
            block: Option<BlockNumberOrTagOrHash>,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'eth_getTransactionCount'
        /// Summary: Returns the number of transactions sent from an address.
        /// Description:
        ///
        fn eth_getTransactionCount(
            &self,
            address: Address,
            block: Option<BlockNumberOrTagOrHash>,
        ) -> std::result::Result<Uint, jsonrpc::Error>;

        /// Method: 'eth_getCode'
        /// Summary: Returns code at a given address.
        /// Description:
        ///
        fn eth_getCode(
            &self,
            address: Address,
            block: Option<BlockNumberOrTagOrHash>,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'eth_getProof'
        /// Summary: Returns the merkle proof for a given account and optionally some storage keys.
        /// Description:
        ///
        fn eth_getProof(
            &self,
            address: Address,
            storagekeys: Storagekeys,
            block: BlockNumberOrTagOrHash,
        ) -> std::result::Result<AccountProof, jsonrpc::Error>;

        /// Method: 'eth_sendTransaction'
        /// Summary: Signs and submits a transaction.
        /// Description:
        ///
        fn eth_sendTransaction(
            &self,
            transaction: GenericTransaction,
        ) -> std::result::Result<Hash32, jsonrpc::Error>;

        /// Method: 'eth_sendRawTransaction'
        /// Summary: Submits a raw transaction.
        /// Description:
        ///
        fn eth_sendRawTransaction(
            &self,
            transaction: Bytes,
        ) -> std::result::Result<Hash32, jsonrpc::Error>;

        /// Method: 'eth_getTransactionByHash'
        /// Summary: Returns the information about a transaction requested by transaction hash.
        /// Description:
        ///
        fn eth_getTransactionByHash(
            &self,
            transaction_hash: Hash32,
        ) -> std::result::Result<TransactionInfo, jsonrpc::Error>;

        /// Method: 'eth_getTransactionByBlockHashAndIndex'
        /// Summary: Returns information about a transaction by block hash and transaction index position.
        /// Description:
        ///
        fn eth_getTransactionByBlockHashAndIndex(
            &self,
            block_hash: Hash32,
            transaction_index: Uint,
        ) -> std::result::Result<TransactionInfo, jsonrpc::Error>;

        /// Method: 'eth_getTransactionByBlockNumberAndIndex'
        /// Summary: Returns information about a transaction by block number and transaction index position.
        /// Description:
        ///
        fn eth_getTransactionByBlockNumberAndIndex(
            &self,
            block: BlockNumberOrTag,
            transaction_index: Uint,
        ) -> std::result::Result<TransactionInfo, jsonrpc::Error>;

        /// Method: 'eth_getTransactionReceipt'
        /// Summary: Returns the receipt of a transaction by transaction hash.
        /// Description:
        ///
        fn eth_getTransactionReceipt(
            &self,
            transaction_hash: Option<Hash32>,
        ) -> std::result::Result<ReceiptInfo, jsonrpc::Error>;

        /// Method: 'debug_getRawHeader'
        /// Summary: Returns an RLP-encoded header.
        /// Description:
        ///
        fn debug_getRawHeader(
            &self,
            block: BlockNumberOrTag,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'debug_getRawBlock'
        /// Summary: Returns an RLP-encoded block.
        /// Description:
        ///
        fn debug_getRawBlock(
            &self,
            block: BlockNumberOrTag,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'debug_getRawTransaction'
        /// Summary: Returns an array of EIP-2718 binary-encoded transactions.
        /// Description:
        ///
        fn debug_getRawTransaction(
            &self,
            transaction_hash: Hash32,
        ) -> std::result::Result<Bytes, jsonrpc::Error>;

        /// Method: 'debug_getRawReceipts'
        /// Summary: Returns an array of EIP-2718 binary-encoded receipts.
        /// Description:
        ///
        fn debug_getRawReceipts(
            &self,
            block: BlockNumberOrTag,
        ) -> std::result::Result<DebugGetRawReceiptsReceipts, jsonrpc::Error>;

        /// Method: 'debug_getBadBlocks'
        /// Summary: Returns an array of recent bad blocks that the client has seen on the network.
        /// Description:
        ///
        fn debug_getBadBlocks(
            &self,
        ) -> std::result::Result<DebugGetBadBlocksBlocks, jsonrpc::Error>;
    }

    fn handle_eth_getBlockByHash<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Hash32, bool);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_hash: Hash32,
            hydrated_transactions: bool,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_hash, hydrated_transactions) = args_by_pos;
                ArgByName {
                    block_hash,
                    hydrated_transactions,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block_hash,
            hydrated_transactions,
        } = args;

        match rpc.eth_getBlockByHash(block_hash, hydrated_transactions) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getBlockByNumber<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockNumberOrTag, bool);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block: BlockNumberOrTag,
            hydrated_transactions: bool,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block, hydrated_transactions) = args_by_pos;
                ArgByName {
                    block,
                    hydrated_transactions,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block,
            hydrated_transactions,
        } = args;

        match rpc.eth_getBlockByNumber(block, hydrated_transactions) {
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
        struct ArgByPos(Option<Hash32>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_hash: Option<Hash32>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_hash) = args_by_pos;
                ArgByName { block_hash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_hash } = args;

        match rpc.eth_getBlockTransactionCountByHash(block_hash) {
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
        struct ArgByPos(Option<BlockNumberOrTag>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block: Option<BlockNumberOrTag>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block) = args_by_pos;
                ArgByName { block }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block } = args;

        match rpc.eth_getBlockTransactionCountByNumber(block) {
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
        struct ArgByPos(Option<Hash32>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_hash: Option<Hash32>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_hash) = args_by_pos;
                ArgByName { block_hash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_hash } = args;

        match rpc.eth_getUncleCountByBlockHash(block_hash) {
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
        struct ArgByPos(Option<BlockNumberOrTag>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block: Option<BlockNumberOrTag>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block) = args_by_pos;
                ArgByName { block }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block } = args;

        match rpc.eth_getUncleCountByBlockNumber(block) {
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

    fn handle_eth_syncing<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_syncing() {
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

    fn handle_eth_accounts<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_accounts() {
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
        struct ArgByPos(GenericTransaction, Option<BlockNumberOrTagOrHash>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction: GenericTransaction,
            block: Option<BlockNumberOrTagOrHash>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transaction, block) = args_by_pos;
                ArgByName { transaction, block }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transaction, block } = args;

        match rpc.eth_call(transaction, block) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_estimateGas<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(GenericTransaction, Option<BlockNumberOrTag>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction: GenericTransaction,
            block: Option<BlockNumberOrTag>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transaction, block) = args_by_pos;
                ArgByName { transaction, block }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transaction, block } = args;

        match rpc.eth_estimateGas(transaction, block) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_createAccessList<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(GenericTransaction, Option<BlockNumberOrTag>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction: GenericTransaction,
            block: Option<BlockNumberOrTag>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transaction, block) = args_by_pos;
                ArgByName { transaction, block }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transaction, block } = args;

        match rpc.eth_createAccessList(transaction, block) {
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

    fn handle_eth_maxPriorityFeePerGas<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.eth_maxPriorityFeePerGas() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_feeHistory<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Uint, BlockNumberOrTag, Rewardpercentiles);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            blockcount: Uint,
            newestblock: BlockNumberOrTag,
            rewardpercentiles: Rewardpercentiles,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(blockcount, newestblock, rewardpercentiles) = args_by_pos;
                ArgByName {
                    blockcount,
                    newestblock,
                    rewardpercentiles,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            blockcount,
            newestblock,
            rewardpercentiles,
        } = args;

        match rpc.eth_feeHistory(blockcount, newestblock, rewardpercentiles) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
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

    fn handle_eth_uninstallFilter<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Option<Uint>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            filter_identifier: Option<Uint>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(filter_identifier) = args_by_pos;
                ArgByName { filter_identifier }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { filter_identifier } = args;

        match rpc.eth_uninstallFilter(filter_identifier) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_sign<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, Bytes);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            address: Address,
            message: Bytes,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(address, message) = args_by_pos;
                ArgByName { address, message }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { address, message } = args;

        match rpc.eth_sign(address, message) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_signTransaction<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(GenericTransaction);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction: GenericTransaction,
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

        match rpc.eth_signTransaction(transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getBalance<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, Option<BlockNumberOrTagOrHash>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            address: Address,
            block: Option<BlockNumberOrTagOrHash>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(address, block) = args_by_pos;
                ArgByName { address, block }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { address, block } = args;

        match rpc.eth_getBalance(address, block) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getStorageAt<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, Uint256, Option<BlockNumberOrTagOrHash>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            address: Address,
            storage_slot: Uint256,
            block: Option<BlockNumberOrTagOrHash>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(address, storage_slot, block) = args_by_pos;
                ArgByName {
                    address,
                    storage_slot,
                    block,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            address,
            storage_slot,
            block,
        } = args;

        match rpc.eth_getStorageAt(address, storage_slot, block) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getTransactionCount<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, Option<BlockNumberOrTagOrHash>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            address: Address,
            block: Option<BlockNumberOrTagOrHash>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(address, block) = args_by_pos;
                ArgByName { address, block }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { address, block } = args;

        match rpc.eth_getTransactionCount(address, block) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getCode<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, Option<BlockNumberOrTagOrHash>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            address: Address,
            block: Option<BlockNumberOrTagOrHash>,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(address, block) = args_by_pos;
                ArgByName { address, block }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { address, block } = args;

        match rpc.eth_getCode(address, block) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getProof<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, Storagekeys, BlockNumberOrTagOrHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            address: Address,
            storagekeys: Storagekeys,
            block: BlockNumberOrTagOrHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(address, storagekeys, block) = args_by_pos;
                ArgByName {
                    address,
                    storagekeys,
                    block,
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
            block,
        } = args;

        match rpc.eth_getProof(address, storagekeys, block) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_sendTransaction<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(GenericTransaction);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction: GenericTransaction,
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

        match rpc.eth_sendTransaction(transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_sendRawTransaction<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Bytes);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction: Bytes,
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

        match rpc.eth_sendRawTransaction(transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getTransactionByHash<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Hash32);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: Hash32,
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

        match rpc.eth_getTransactionByHash(transaction_hash) {
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
        struct ArgByPos(Hash32, Uint);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_hash: Hash32,
            transaction_index: Uint,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_hash, transaction_index) = args_by_pos;
                ArgByName {
                    block_hash,
                    transaction_index,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block_hash,
            transaction_index,
        } = args;

        match rpc.eth_getTransactionByBlockHashAndIndex(block_hash, transaction_index) {
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
        struct ArgByPos(BlockNumberOrTag, Uint);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block: BlockNumberOrTag,
            transaction_index: Uint,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block, transaction_index) = args_by_pos;
                ArgByName {
                    block,
                    transaction_index,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block,
            transaction_index,
        } = args;

        match rpc.eth_getTransactionByBlockNumberAndIndex(block, transaction_index) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_eth_getTransactionReceipt<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Option<Hash32>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: Option<Hash32>,
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

        match rpc.eth_getTransactionReceipt(transaction_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_debug_getRawHeader<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockNumberOrTag);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block: BlockNumberOrTag,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block) = args_by_pos;
                ArgByName { block }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block } = args;

        match rpc.debug_getRawHeader(block) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_debug_getRawBlock<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockNumberOrTag);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block: BlockNumberOrTag,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block) = args_by_pos;
                ArgByName { block }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block } = args;

        match rpc.debug_getRawBlock(block) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_debug_getRawTransaction<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Hash32);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: Hash32,
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

        match rpc.debug_getRawTransaction(transaction_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_debug_getRawReceipts<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockNumberOrTag);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block: BlockNumberOrTag,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block) = args_by_pos;
                ArgByName { block }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block } = args;

        match rpc.debug_getRawReceipts(block) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_debug_getBadBlocks<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.debug_getBadBlocks() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    pub fn handle<RPC: Rpc>(rpc: &RPC, req: &jsonrpc::Request) -> jsonrpc::Response {
        let params = &req.params.clone().unwrap_or_default();

        let response = match req.method.as_str() {
            "eth_getBlockByHash" => handle_eth_getBlockByHash(rpc, params),
            "eth_getBlockByNumber" => handle_eth_getBlockByNumber(rpc, params),
            "eth_getBlockTransactionCountByHash" => {
                handle_eth_getBlockTransactionCountByHash(rpc, params)
            }
            "eth_getBlockTransactionCountByNumber" => {
                handle_eth_getBlockTransactionCountByNumber(rpc, params)
            }
            "eth_getUncleCountByBlockHash" => handle_eth_getUncleCountByBlockHash(rpc, params),
            "eth_getUncleCountByBlockNumber" => handle_eth_getUncleCountByBlockNumber(rpc, params),
            "eth_chainId" => handle_eth_chainId(rpc, params),
            "eth_syncing" => handle_eth_syncing(rpc, params),
            "eth_coinbase" => handle_eth_coinbase(rpc, params),
            "eth_accounts" => handle_eth_accounts(rpc, params),
            "eth_blockNumber" => handle_eth_blockNumber(rpc, params),
            "eth_call" => handle_eth_call(rpc, params),
            "eth_estimateGas" => handle_eth_estimateGas(rpc, params),
            "eth_createAccessList" => handle_eth_createAccessList(rpc, params),
            "eth_gasPrice" => handle_eth_gasPrice(rpc, params),
            "eth_maxPriorityFeePerGas" => handle_eth_maxPriorityFeePerGas(rpc, params),
            "eth_feeHistory" => handle_eth_feeHistory(rpc, params),
            "eth_newBlockFilter" => handle_eth_newBlockFilter(rpc, params),
            "eth_newPendingTransactionFilter" => {
                handle_eth_newPendingTransactionFilter(rpc, params)
            }
            "eth_uninstallFilter" => handle_eth_uninstallFilter(rpc, params),
            "eth_sign" => handle_eth_sign(rpc, params),
            "eth_signTransaction" => handle_eth_signTransaction(rpc, params),
            "eth_getBalance" => handle_eth_getBalance(rpc, params),
            "eth_getStorageAt" => handle_eth_getStorageAt(rpc, params),
            "eth_getTransactionCount" => handle_eth_getTransactionCount(rpc, params),
            "eth_getCode" => handle_eth_getCode(rpc, params),
            "eth_getProof" => handle_eth_getProof(rpc, params),
            "eth_sendTransaction" => handle_eth_sendTransaction(rpc, params),
            "eth_sendRawTransaction" => handle_eth_sendRawTransaction(rpc, params),
            "eth_getTransactionByHash" => handle_eth_getTransactionByHash(rpc, params),
            "eth_getTransactionByBlockHashAndIndex" => {
                handle_eth_getTransactionByBlockHashAndIndex(rpc, params)
            }
            "eth_getTransactionByBlockNumberAndIndex" => {
                handle_eth_getTransactionByBlockNumberAndIndex(rpc, params)
            }
            "eth_getTransactionReceipt" => handle_eth_getTransactionReceipt(rpc, params),
            "debug_getRawHeader" => handle_debug_getRawHeader(rpc, params),
            "debug_getRawBlock" => handle_debug_getRawBlock(rpc, params),
            "debug_getRawTransaction" => handle_debug_getRawTransaction(rpc, params),
            "debug_getRawReceipts" => handle_debug_getRawReceipts(rpc, params),
            "debug_getBadBlocks" => handle_debug_getBadBlocks(rpc, params),
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
            fn eth_getBlockByHash(
                &self,
                block_hash: Hash32,
                hydrated_transactions: bool,
            ) -> std::result::Result<Block, jsonrpc::Error> {
                let args = (block_hash, hydrated_transactions);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getBlockByHash".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: Block = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getBlockByNumber(
                &self,
                block: BlockNumberOrTag,
                hydrated_transactions: bool,
            ) -> std::result::Result<Block, jsonrpc::Error> {
                let args = (block, hydrated_transactions);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getBlockByNumber".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: Block = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getBlockTransactionCountByHash(
                &self,
                block_hash: Option<Hash32>,
            ) -> std::result::Result<Uint, jsonrpc::Error> {
                let args = (block_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req =
                    jsonrpc::Request::new("eth_getBlockTransactionCountByHash".to_string(), params)
                        .with_id(jsonrpc::Id::Number(1));

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getBlockTransactionCountByNumber(
                &self,
                block: Option<BlockNumberOrTag>,
            ) -> std::result::Result<Uint, jsonrpc::Error> {
                let args = (block,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new(
                    "eth_getBlockTransactionCountByNumber".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getUncleCountByBlockHash(
                &self,
                block_hash: Option<Hash32>,
            ) -> std::result::Result<Uint, jsonrpc::Error> {
                let args = (block_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getUncleCountByBlockHash".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getUncleCountByBlockNumber(
                &self,
                block: Option<BlockNumberOrTag>,
            ) -> std::result::Result<Uint, jsonrpc::Error> {
                let args = (block,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req =
                    jsonrpc::Request::new("eth_getUncleCountByBlockNumber".to_string(), params)
                        .with_id(jsonrpc::Id::Number(1));

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_chainId(&self) -> std::result::Result<Uint, jsonrpc::Error> {
                let req =
                    jsonrpc::Request::new("eth_chainId".to_string(), serde_json::Value::default());

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_syncing(&self) -> std::result::Result<SyncingStatus, jsonrpc::Error> {
                let req =
                    jsonrpc::Request::new("eth_syncing".to_string(), serde_json::Value::default());

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
                    let out: SyncingStatus = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_coinbase(&self) -> std::result::Result<Address, jsonrpc::Error> {
                let req =
                    jsonrpc::Request::new("eth_coinbase".to_string(), serde_json::Value::default());

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
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_accounts(&self) -> std::result::Result<EthAccountsAccounts, jsonrpc::Error> {
                let req =
                    jsonrpc::Request::new("eth_accounts".to_string(), serde_json::Value::default());

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
                    let out: EthAccountsAccounts = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_blockNumber(&self) -> std::result::Result<Uint, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_blockNumber".to_string(),
                    serde_json::Value::default(),
                );

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_call(
                &self,
                transaction: GenericTransaction,
                block: Option<BlockNumberOrTagOrHash>,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (transaction, block);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_call".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_estimateGas(
                &self,
                transaction: GenericTransaction,
                block: Option<BlockNumberOrTag>,
            ) -> std::result::Result<Uint, jsonrpc::Error> {
                let args = (transaction, block);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_estimateGas".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_createAccessList(
                &self,
                transaction: GenericTransaction,
                block: Option<BlockNumberOrTag>,
            ) -> std::result::Result<EthCreateAccessListGasUsed, jsonrpc::Error> {
                let args = (transaction, block);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_createAccessList".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: EthCreateAccessListGasUsed =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_gasPrice(&self) -> std::result::Result<Uint, jsonrpc::Error> {
                let req =
                    jsonrpc::Request::new("eth_gasPrice".to_string(), serde_json::Value::default());

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_maxPriorityFeePerGas(&self) -> std::result::Result<Uint, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_maxPriorityFeePerGas".to_string(),
                    serde_json::Value::default(),
                );

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_feeHistory(
                &self,
                blockcount: Uint,
                newestblock: BlockNumberOrTag,
                rewardpercentiles: Rewardpercentiles,
            ) -> std::result::Result<EthFeeHistoryFeeHistoryResult, jsonrpc::Error> {
                let args = (blockcount, newestblock, rewardpercentiles);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_feeHistory".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: EthFeeHistoryFeeHistoryResult = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_newBlockFilter(&self) -> std::result::Result<Uint, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_newBlockFilter".to_string(),
                    serde_json::Value::default(),
                );

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_newPendingTransactionFilter(&self) -> std::result::Result<Uint, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "eth_newPendingTransactionFilter".to_string(),
                    serde_json::Value::default(),
                );

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_uninstallFilter(
                &self,
                filter_identifier: Option<Uint>,
            ) -> std::result::Result<bool, jsonrpc::Error> {
                let args = (filter_identifier,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_uninstallFilter".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_sign(
                &self,
                address: Address,
                message: Bytes,
            ) -> std::result::Result<Bytes65, jsonrpc::Error> {
                let args = (address, message);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_sign".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: Bytes65 = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_signTransaction(
                &self,
                transaction: GenericTransaction,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (transaction,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_signTransaction".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getBalance(
                &self,
                address: Address,
                block: Option<BlockNumberOrTagOrHash>,
            ) -> std::result::Result<Uint, jsonrpc::Error> {
                let args = (address, block);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getBalance".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getStorageAt(
                &self,
                address: Address,
                storage_slot: Uint256,
                block: Option<BlockNumberOrTagOrHash>,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (address, storage_slot, block);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getStorageAt".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getTransactionCount(
                &self,
                address: Address,
                block: Option<BlockNumberOrTagOrHash>,
            ) -> std::result::Result<Uint, jsonrpc::Error> {
                let args = (address, block);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getTransactionCount".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: Uint = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getCode(
                &self,
                address: Address,
                block: Option<BlockNumberOrTagOrHash>,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (address, block);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getCode".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getProof(
                &self,
                address: Address,
                storagekeys: Storagekeys,
                block: BlockNumberOrTagOrHash,
            ) -> std::result::Result<AccountProof, jsonrpc::Error> {
                let args = (address, storagekeys, block);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getProof".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: AccountProof = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_sendTransaction(
                &self,
                transaction: GenericTransaction,
            ) -> std::result::Result<Hash32, jsonrpc::Error> {
                let args = (transaction,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_sendTransaction".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: Hash32 = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_sendRawTransaction(
                &self,
                transaction: Bytes,
            ) -> std::result::Result<Hash32, jsonrpc::Error> {
                let args = (transaction,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_sendRawTransaction".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: Hash32 = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getTransactionByHash(
                &self,
                transaction_hash: Hash32,
            ) -> std::result::Result<TransactionInfo, jsonrpc::Error> {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getTransactionByHash".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: TransactionInfo = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getTransactionByBlockHashAndIndex(
                &self,
                block_hash: Hash32,
                transaction_index: Uint,
            ) -> std::result::Result<TransactionInfo, jsonrpc::Error> {
                let args = (block_hash, transaction_index);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new(
                    "eth_getTransactionByBlockHashAndIndex".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

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
                    let out: TransactionInfo = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getTransactionByBlockNumberAndIndex(
                &self,
                block: BlockNumberOrTag,
                transaction_index: Uint,
            ) -> std::result::Result<TransactionInfo, jsonrpc::Error> {
                let args = (block, transaction_index);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new(
                    "eth_getTransactionByBlockNumberAndIndex".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

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
                    let out: TransactionInfo = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn eth_getTransactionReceipt(
                &self,
                transaction_hash: Option<Hash32>,
            ) -> std::result::Result<ReceiptInfo, jsonrpc::Error> {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("eth_getTransactionReceipt".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: ReceiptInfo = serde_json::from_value(value).map_err(|e| {
                        jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                    })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn debug_getRawHeader(
                &self,
                block: BlockNumberOrTag,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (block,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("debug_getRawHeader".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn debug_getRawBlock(
                &self,
                block: BlockNumberOrTag,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (block,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("debug_getRawBlock".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn debug_getRawTransaction(
                &self,
                transaction_hash: Hash32,
            ) -> std::result::Result<Bytes, jsonrpc::Error> {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("debug_getRawTransaction".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn debug_getRawReceipts(
                &self,
                block: BlockNumberOrTag,
            ) -> std::result::Result<DebugGetRawReceiptsReceipts, jsonrpc::Error> {
                let args = (block,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
                let req = jsonrpc::Request::new("debug_getRawReceipts".to_string(), params)
                    .with_id(jsonrpc::Id::Number(1));

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
                    let out: DebugGetRawReceiptsReceipts =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }

            fn debug_getBadBlocks(
                &self,
            ) -> std::result::Result<DebugGetBadBlocksBlocks, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "debug_getBadBlocks".to_string(),
                    serde_json::Value::default(),
                );

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
                    let out: DebugGetBadBlocksBlocks =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
                        })?;
                    Ok(out)
                } else {
                    Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
                }
            }
        }
    }
}
// ^^^ GENERATED CODE ABOVE ^^^
