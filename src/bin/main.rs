use std::collections::HashMap;

use openrpc_stub_gen::{binding, openrpc, renders};

fn run(
    spec: openrpc::OpenRpc,
    cache: &mut HashMap<String, binding::Binding>,
) -> Vec<binding::Contract> {
    let bindings = spec
        .components
        .as_ref()
        .expect("components")
        .schemas
        .iter()
        .map(|(name, schema)| binding::get_schema_binding(name.to_string(), schema, &spec, cache))
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
            binding::get_method_contract(name, &spec, cache)
        })
        .collect::<Vec<_>>();

    contracts
}

fn parse(path: &str) -> openrpc::OpenRpc {
    log::info!("Processing file: {path}");
    let json = std::fs::read_to_string(path).expect("JSON file exists and is readable.");
    serde_json::from_str(&json).expect("json")
}

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
        let spec = parse(&paths[0]);
        let text = serde_json::to_string(&spec).expect("json");
        println!("{text}");
    } else if mode.as_str() == "TREE" {
        let mut cache = HashMap::new();

        let contracts = paths
            .into_iter()
            .map(|path| parse(&path))
            .flat_map(|spec| run(spec, &mut cache))
            .collect::<Vec<_>>();

        cache
            .iter()
            .for_each(|(name, binding)| println!("---\n{name}: {binding:#?}"));

        contracts
            .iter()
            .for_each(|contract| println!("---\n{contract:#?}"));
    } else if mode.as_str() == "CODE" {
        let mut cache = HashMap::new();

        let contracts = paths
            .into_iter()
            .map(|path| parse(&path))
            .flat_map(|spec| run(spec, &mut cache))
            .collect::<Vec<_>>();

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
                .unwrap_or_else(|e| format!("// ERROR: Rendering object '{name}' failed: {e}"));

            if !code.is_empty() {
                println!("\n// object: '{name}'\n{code}");
            }
        }

        println!("\npub trait Rpc {{");
        for contract in &contracts {
            let code = renders::render_method(&contract.name, contract);
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
