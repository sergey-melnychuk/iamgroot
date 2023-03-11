mod private {
    use once_cell::sync::Lazy;
    use openrpc_stub_gen::jsonrpc;
    use regex::Regex;
    use serde::{Deserialize, Serialize};

    // Field `Felt.0` is private (not exposed from the module)
    // Thus creating `Felt` instances is only possible with `Felt::try_new(&str)`

    // object: 'FELT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct Felt(String);

    static FELT_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new("^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$").unwrap());

    impl Felt {
        pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
            if FELT_REGEX.is_match(&value) {
                Ok(Felt(value.to_string()))
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

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Envelope {
        pub felt: Felt,
    }
}

use private::{Envelope, Felt};

fn check_json(value: &str) -> Result<(), String> {
    let envelope: Envelope = serde_json::from_str(&format!("{{\"felt\":\"{}\"}}", value))
        .map_err(|e| format!("{:?}", e))?;
    assert_eq!(envelope.felt.as_ref(), value);
    Ok(())
}

fn check_felt(value: &str) -> Result<(), String> {
    Felt::try_new(value).map_err(|e| format!("{:?}", e))?;
    Ok(())
}

fn check(value: &str) -> Result<(), String> {
    check_json(value)?;
    check_felt(value)?;
    Ok(())
}

mod block {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockHash(pub super::Felt);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockNumber(pub i64);

    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
    // #[serde(rename_all = "lowercase")]
    pub enum BlockTag {
        #[serde(rename = "latest")]
        Latest,
        #[serde(rename = "pending")]
        Pending,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BlockId {
        BlockHash { block_hash: BlockHash },
        BlockNumber { block_number: BlockNumber },
        BlockTag(BlockTag),
    }

    fn check(json: &str, block: BlockId) {
        let parsed: BlockId = serde_json::from_str(json).unwrap();
        let line = format!("json={json} block={block:?}");
        match (block, parsed) {
            (BlockId::BlockHash { block_hash: h1 }, BlockId::BlockHash { block_hash: h2 }) => {
                assert_eq!(h1.0.as_ref(), h2.0.as_ref())
            }
            (
                BlockId::BlockNumber { block_number: n1 },
                BlockId::BlockNumber { block_number: n2 },
            ) => assert_eq!(n1.0, n2.0),
            (BlockId::BlockTag(t1), BlockId::BlockTag(t2)) => assert_eq!(t1, t2),
            _ => panic!("mismatch"),
        }
        println!("{}: OK", line);
    }

    pub fn check_all() {
        for (json, block) in [
            (
                "{\"block_hash\": \"0xFACE\"}",
                BlockId::BlockHash {
                    block_hash: BlockHash(super::Felt::try_new("0xFACE").unwrap()),
                },
            ),
            (
                "{\"block_hash\": \"0x0\"}",
                BlockId::BlockHash {
                    block_hash: BlockHash(super::Felt::try_new("0x0").unwrap()),
                },
            ),
            (
                "{\"block_number\": 12345}",
                BlockId::BlockNumber {
                    block_number: BlockNumber(12345),
                },
            ),
            (
                "{\"block_number\": 0}",
                BlockId::BlockNumber {
                    block_number: BlockNumber(0),
                },
            ),
            (
                "{\"block_number\": -42}",
                BlockId::BlockNumber {
                    block_number: BlockNumber(-42),
                },
            ),
            ("\"latest\"", BlockId::BlockTag(BlockTag::Latest)),
            ("\"pending\"", BlockId::BlockTag(BlockTag::Pending)),
        ] {
            check(json, block);
        }
    }
}

fn main() {
    let values = vec![
        "0x0",
        "0xA",
        "0x9",
        "hello",
        "0xFACE",
        "0x0FACE", // leading zeros are not valid (except "0x0" for zero)
        // .........1.........2.........3.........4.........5.........6.. (len=62)
        "0xFEDCBA9876543210FEDCBA9876543210FEDCBA9876543210FEDCBA98765432",
        // .........1.........2.........3.........4.........5.........6... (len=63)
        "0xFEDCBA9876543210FEDCBA9876543210FEDCBA9876543210FEDCBA987654321",
        // .........1.........2.........3.........4.........5.........6... (len=63)
        "0xFEDCBA9876543210FEDCBA9876543210FEDCBA9876543210FEDCBA98765432X",
        // .........1.........2.........3.........4.........5.........6... (len=63)
        "0x0EDCBA9876543210FEDCBA9876543210FEDCBA9876543210FEDCBA987654321",
        // .........1.........2.........3.........4.........5.........6.... (len=64)
        "0xFEDCBA9876543210FEDCBA9876543210FEDCBA9876543210FEDCBA987654321F",
    ];

    for value in values {
        let result = match check(value) {
            Ok(_) => "OK".to_string(),
            Err(message) => message,
        };
        println!("Felt(\"{value}\"): {result}");
    }

    block::check_all();
}
