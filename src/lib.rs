use std::{collections::HashMap, fmt::Display, path::Path};

use cache::Cache;

pub(crate) mod binding;
pub(crate) mod cache;
pub(crate) mod codegen;
pub mod jsonrpc;
pub(crate) mod openrpc;
pub(crate) mod renders;

pub trait AsPath: AsRef<Path> + Display {}

impl AsPath for String {}
impl AsPath for &str {}

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
    let mut cache = Cache::new();
    let mut trace = Vec::with_capacity(32);

    let contracts = paths
        .iter()
        .map(|path| parse(path))
        .flat_map(|spec| binding::extract_contracts(&spec, &mut cache, &mut trace))
        .collect::<Vec<_>>();

    let mut target = String::new();
    use std::fmt::Write;

    cache
        .data
        .iter()
        .for_each(|(name, binding)| writeln!(target, "---\n{name}: {binding:#?}").unwrap());

    contracts
        .iter()
        .for_each(|contract| writeln!(target, "---\n{contract:#?}").unwrap());

    target
}

pub fn gen_code<P: AsPath>(paths: &[P]) -> String {
    let mut cache = Cache::new();
    let mut trace = Vec::with_capacity(32);

    let specs = paths.iter().map(|path| parse(path)).collect::<Vec<_>>();

    let errors = specs
        .iter()
        .flat_map(|spec| {
            spec.components
                .as_ref()
                .map(|comps| comps.errors.clone())
                .unwrap_or_default()
        })
        .collect::<HashMap<_, _>>();

    let contracts = specs
        .iter()
        .flat_map(|spec| binding::extract_contracts(spec, &mut cache, &mut trace))
        .collect::<Vec<_>>();

    let mut target = String::new();
    use std::fmt::Write;

    writeln!(target, "// vvv GENERATED CODE BELOW vvv").unwrap();
    writeln!(target, "#[allow(dead_code)]").unwrap();
    writeln!(target, "#[allow(non_snake_case)]").unwrap();
    writeln!(target, "#[allow(unused_variables)]").unwrap();
    writeln!(target, "#[allow(clippy::enum_variant_names)]").unwrap();
    writeln!(target, "pub mod gen {{").unwrap();
    writeln!(target, "use serde::{{Deserialize, Serialize}};").unwrap();
    writeln!(target, "use serde_json::Value;").unwrap();
    // TODO: Replace 'openrpc_stub_gen' with final crate name
    writeln!(target, "\nuse openrpc_stub_gen::jsonrpc;").unwrap();

    let mut ordered = cache.data.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|(name, _)| *name);

    for (name, binding) in ordered {
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

    writeln!(target, "{}", renders::render_errors(errors)).unwrap();

    writeln!(target, "{}", renders::render_client(&contracts)).unwrap();

    writeln!(target, "}}").unwrap();
    writeln!(target, "// ^^^ GENERATED CODE ABOVE ^^^").unwrap();

    target
}
