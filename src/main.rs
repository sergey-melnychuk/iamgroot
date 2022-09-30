mod codegen;
mod openrpc;
mod binding;

use std::collections::HashMap;

use crate::openrpc::OpenRpc;

// cargo run --release -- ./api/input.openrpc JSON 2>/dev/null | jq . > debug.json
// diff <(jq --sort-keys . ./api/input.openrpc) <(jq --sort-keys . debug.json)

// cargo run -- ./api/input.openrpc AST > ast.txt 2> dbg.txt

fn main() {
    let (path, mode) = {
        let mut args = std::env::args().skip(1);
        let path = args.next().expect("Path to JSON file.");
        let mode = args.next().expect("Output: JSON or AST.");
        (path, mode)
    };
    let json = std::fs::read_to_string(path).expect("JSON file exists and is readable.");
    let spec: OpenRpc = serde_json::from_str(&json).expect("JSON is valid");
    
    if mode.as_str() == "JSON" {
        let text = serde_json::to_string(&spec).expect("JSON serialized.");
        println!("{}", text);    
    } else if mode.as_str() == "AST" {
        let mut cache = HashMap::new();
        spec.components
            .as_ref()
            .expect("Components section")
            .schemas
            .iter()
            .for_each(|(name, schema)| {
                let binding = binding::get_schema_binding(name.to_string(), schema, &spec, &mut cache);
                //println!("\n===\nname={}\nschema={:#?}\n---\nbinding={:#?}", name, schema, binding);
                println!("{:#?}", binding);
            });
    } else {
        eprintln!("Unknown mode: {}. Supported are JSON and AST.", mode);
    }
}
