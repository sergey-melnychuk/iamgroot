use std::collections::HashMap;

mod binding;
mod codegen;
mod openrpc;
mod renders;

// cargo run --release -- ./api/input.openrpc JSON 2>/dev/null | jq . > debug.json
// diff <(jq --sort-keys . ./api/input.openrpc) <(jq --sort-keys . debug.json)

// cargo run --release -- ./api/input.openrpc TREE > tree.txt 2> debug.txt

// rm -rf examples && mkdir examples
// cargo run --release -- ./api/input.openrpc CODE > examples/gen.rs
// echo '\nfn main() { println!("OK"); }' >> examples/gen.rs
// cargo run --example gen 2> debug.txt

// Total lines of code:
// find . -type f -name "*.rs" | xargs grep . | wc -l

// Changes necessary to make existing spec a valid OpenRPC spec:
// https://github.com/starkware-libs/starknet-specs/pull/56

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
        let path = args.next().expect("Path to JSON file.");
        let mode = args.next().expect("Output: [JSON, TREE, CODE].");
        (path, mode)
    };
    let json = std::fs::read_to_string(path).expect("JSON file exists and is readable.");
    let spec: openrpc::OpenRpc = serde_json::from_str(&json).expect("JSON is valid");

    if mode.as_str() == "JSON" {
        let text = serde_json::to_string(&spec).expect("JSON serialized.");
        println!("{}", text);
    } else if mode.as_str() == "TREE" {
        let (cache, contracts) = run(spec);

        cache
            .iter()
            .for_each(|(name, binding)| println!("---\n{}: {:#?}", name, binding));

        contracts
            .iter()
            .for_each(|contract| println!("---\n{:#?}", contract));
    } else if mode.as_str() == "CODE" {
        let (cache, contracts) = run(spec);

        println!("/// === GENERATED CODE ===");
        for (name, binding) in &cache {
            let code = renders::render_object(name, binding)
                .unwrap_or_else(|e| format!("//! Rendering object '{}' failed: {}", name, e));
            println!("\n// object: '{}'\n{}", name, code);
        }

        for contract in contracts {
            let code = renders::render_method(&contract.name, &contract, &cache);
            println!("\n{}", code);
        }
    } else {
        eprintln!("Unknown mode: {}. Supported are: JSON, TREE, CODE.", mode);
    }
}
