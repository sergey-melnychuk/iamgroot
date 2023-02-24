use std::collections::HashMap;

use openrpc_stub_gen::{binding, openrpc, renders};

fn run(spec: openrpc::OpenRpc) -> (HashMap<String, binding::Binding>, Vec<binding::Contract>) {
    let mut cache = HashMap::new();
    let bindings = spec
        .components
        .as_ref()
        .expect("Components section")
        .schemas
        .iter()
        .map(|(name, schema)| {
            binding::get_schema_binding(name.to_string(), schema, &spec, &mut cache)
        })
        .flat_map(binding::unfold_binding)
        // Make a second pass (extra unfolding might be necessary)
        .flat_map(binding::unfold_binding)
        .collect::<Vec<_>>();

    for b in bindings {
        cache.insert(b.get_name(), b);
    }

    let contracts = spec
        .methods
        .iter()
        .filter_map(|method| {
            let name = method.name.clone();
            binding::get_method_contract(name, &spec, &mut cache)
        })
        .collect::<Vec<_>>();

    (cache, contracts)
}

fn main() {
    let (path, mode) = {
        let mut args = std::env::args().skip(1);
        let mode = args.next().expect("Output: [JSON, TREE, CODE].");
        let path = args.next().expect("Path to JSON file.");
        (path, mode)
    };
    let json = std::fs::read_to_string(path).expect("JSON file exists and is readable.");
    let spec: openrpc::OpenRpc = serde_json::from_str(&json).expect("JSON is valid");

    if mode.as_str() == "JSON" {
        let text = serde_json::to_string(&spec).expect("JSON serialized.");
        println!("{text}");
    } else if mode.as_str() == "TREE" {
        let (cache, contracts) = run(spec);

        cache
            .iter()
            .for_each(|(name, binding)| println!("---\n{name}: {binding:#?}"));

        contracts
            .iter()
            .for_each(|contract| println!("---\n{contract:#?}"));
    } else if mode.as_str() == "CODE" {
        let (cache, contracts) = run(spec);

        println!("// vvv GENERATED CODE BELOW vvv");
        println!("#[allow(dead_code)]");
        println!("#[allow(non_snake_case)]");
        println!("#[allow(unused_variables)]");
        println!("mod gen {{");
        println!("use serde::{{Deserialize, Serialize}};");
        println!("use serde_json::Value;");
        println!("\nuse openrpc_stub_gen::jsonrpc;");

        for (name, binding) in &cache {
            let code = renders::render_object(name, binding)
                .unwrap_or_else(|e| format!("//! Rendering object '{name}' failed: {e}"));

            if !code.is_empty() {
                println!("\n// object: '{name}'\n{code}");
            }
        }

        println!("pub trait Rpc {{");
        for contract in &contracts {
            let code = renders::render_method(&contract.name, contract, &cache);
            println!("\n{code}");
        }
        println!("}}");

        for contract in &contracts {
            let code = renders::render_method_handler(&contract.name, contract);
            println!("{code}");
        }

        let handler = renders::render_handle_function(&contracts);
        println!("{handler}");
        println!("}}");
        println!("// ^^^ GENERATED CODE ABOVE ^^^");
    } else {
        eprintln!("Unknown mode: {mode}. Supported are: JSON, TREE, CODE.");
    }
}
