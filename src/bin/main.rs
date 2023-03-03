use openrpc_stub_gen::{gen_code, gen_json, gen_tree};

fn main() {
    env_logger::init();

    let (mode, paths) = {
        let mut args = std::env::args().skip(1);
        let mode = args.next().expect("Output: [JSON, TREE, CODE].");
        let paths = args.collect::<Vec<_>>();
        (mode, paths)
    };

    if paths.is_empty() {
        eprintln!("No input files");
        return;
    }

    if mode.as_str() == "JSON" {
        if paths.len() > 1 {
            eprintln!("JSON mode supports only single file");
            return;
        }
        println!("{}", gen_json(&paths[0]));
    } else if mode.as_str() == "TREE" {
        println!("{}", gen_tree(&paths));
    } else if mode.as_str() == "CODE" {
        println!("{}", gen_code(&paths));
    } else {
        eprintln!("Unknown mode: {mode}. Supported are: JSON, TREE, CODE.");
    }
}

/* Validation Example:

// once_cell = "1.17.1"
// regex = "1.7.1"

    // object: 'FELT'
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Felt(String);

    static FELT_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new("^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$").unwrap()
    });

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
        type Error = jsonrpc::Error;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            Self::try_new(&value)
        }
    }

*/
