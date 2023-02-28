use std::{collections::HashMap, fmt::Display, path::Path};

pub(crate) mod binding;
pub(crate) mod codegen;
pub mod jsonrpc;
pub(crate) mod openrpc;
pub(crate) mod renders;

pub trait AsPath: AsRef<Path> + Display {}

impl AsPath for String {}
impl AsPath for &str {}

fn extract_contracts(
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

fn parse<P: AsPath>(path: &P) -> openrpc::OpenRpc {
    log::info!("Processing file: {path}");
    let json = std::fs::read_to_string(path).expect("JSON file exists and is readable.");
    serde_json::from_str(&json).expect("json")
}

pub fn gen_json<P: AsPath>(path: &P) -> String {
    let spec = parse(path);
    serde_json::to_string(&spec).expect("json")
}

pub fn gen_tree<P: AsPath>(paths: &[P]) -> String {
    let mut cache = HashMap::new();

    let contracts = paths
        .iter()
        .map(|path| parse(path))
        .flat_map(|spec| extract_contracts(spec, &mut cache))
        .collect::<Vec<_>>();

    let mut target = String::new();
    use std::fmt::Write;

    cache
        .iter()
        .for_each(|(name, binding)| writeln!(target, "---\n{name}: {binding:#?}").unwrap());

    contracts
        .iter()
        .for_each(|contract| writeln!(target, "---\n{contract:#?}").unwrap());

    target
}

pub fn gen_code<P: AsPath>(paths: &[P]) -> String {
    let mut cache = HashMap::new();

    let contracts = paths
        .iter()
        .map(|path| parse(path))
        .flat_map(|spec| extract_contracts(spec, &mut cache))
        .collect::<Vec<_>>();

    let mut target = String::new();
    use std::fmt::Write;

    writeln!(target, "// vvv GENERATED CODE BELOW vvv").unwrap();
    writeln!(target, "#[allow(dead_code)]").unwrap();
    writeln!(target, "#[allow(non_snake_case)]").unwrap();
    writeln!(target, "#[allow(unused_variables)]").unwrap();
    writeln!(target, "mod gen {{").unwrap();
    writeln!(target, "use serde::{{Deserialize, Serialize}};").unwrap();
    writeln!(target, "use serde_json::Value;").unwrap();
    writeln!(target, "\nuse openrpc_stub_gen::jsonrpc;").unwrap();

    for (name, binding) in &cache {
        let code = renders::render_object(name, binding)
            .unwrap_or_else(|e| format!("// ERROR: Rendering object '{name}' failed. {e}"));

        if !code.is_empty() {
            writeln!(target, "\n// object: '{name}'\n{code}").unwrap();
        }
    }

    writeln!(target, "\npub trait Rpc {{").unwrap();
    for contract in &contracts {
        let code = renders::render_method(&contract.name, contract);
        writeln!(target, "\n{code}").unwrap();
    }
    writeln!(target, "}}").unwrap();

    for contract in &contracts {
        let code = renders::render_method_handler(&contract.name, contract);
        writeln!(target, "{code}").unwrap();
    }

    let handler = renders::render_handle_function(&contracts);
    writeln!(target, "{handler}").unwrap();
    writeln!(target, "}}").unwrap();
    writeln!(target, "// ^^^ GENERATED CODE ABOVE ^^^").unwrap();

    target
}
