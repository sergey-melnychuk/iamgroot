mod private {
    use once_cell::sync::Lazy;
    use openrpc_stub_gen::jsonrpc;
    use regex::Regex;
    use serde::{Deserialize, Serialize};

    // Field `Felt.0` is private (not exposed from the module)
    // Thus creating `Felt` instances is only possible with `Felt::try_new(&str)`

    // object: 'FELT'
    #[derive(Debug, Deserialize, Serialize)]
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
}
