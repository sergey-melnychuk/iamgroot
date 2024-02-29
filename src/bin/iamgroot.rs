use iamgroot::{gen_code, gen_json};

fn main() {
    env_logger::init();

    let (mode, paths, flags) = {
        let mut args = std::env::args().skip(1);
        let mode = args.next().expect("Output: [JSON, CODE].");
        let mut paths = Vec::new();
        let mut flags = Vec::new();
        for arg in args {
            if arg.starts_with("--") {
                flags.push(arg.strip_prefix("--").unwrap().to_owned());
            } else {
                paths.push(arg);
            }
        }
        (mode, paths, flags)
    };

    let is_async = flags.iter().any(|flag| flag == "async");
    let gen_client = flags.iter().any(|flag| flag == "client");
    let reexport = flags.iter().any(|flag| flag == "reexport");

    match mode.as_str() {
        _ if paths.is_empty() => {
            eprintln!("No input files");
        }
        "JSON" if paths.len() == 1 => {
            let json = gen_json(&paths[0]);
            println!("{json}");
        }
        "JSON" => {
            eprintln!("JSON mode supports only single file");
        }
        "CODE" => {
            let code = gen_code(&paths, is_async, gen_client).unwrap();
            if reexport {
                println!("pub use gen::*;\n");
            }
            println!("{code}");
        }
        mode => {
            eprintln!("Unknown mode: {mode}. Supported are: JSON, CODE.");
        }
    }
}
