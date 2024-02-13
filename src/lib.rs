use std::collections::HashMap as Map;
use std::{fmt::Display, path::Path};

use binding::{bind_method, bind_object, get_error};
use openrpc::{ErrorOrRef, SchemaOrRef};

pub(crate) mod binding;
pub(crate) mod codegen;
pub mod jsonrpc;
pub(crate) mod openrpc;
pub(crate) mod renders;

#[cfg(test)]
mod tests;

pub trait AsPath: AsRef<Path> + Display {}

impl AsPath for String {}
impl AsPath for &str {}

fn parse<P: AsPath>(path: &P) -> openrpc::Spec {
    log::info!("Processing file: {path}");
    let json = std::fs::read_to_string(path).expect("file");
    serde_json::from_str(&json).expect("json")
}

pub fn gen_json<P: AsPath>(path: &P) -> String {
    let spec = parse(path);
    serde_json::to_string(&spec).expect("json")
}

pub struct Gen {
    schemas: Map<String, SchemaOrRef>,
    errors: Map<String, ErrorOrRef>,
    methods: Vec<openrpc::Method>,
}

impl Gen {
    pub fn new(
        schemas: Map<String, SchemaOrRef>,
        errors: Map<String, ErrorOrRef>,
        methods: Vec<openrpc::Method>,
    ) -> Self {
        Self {
            schemas,
            errors,
            methods,
        }
    }

    pub fn apply(
        &mut self,
    ) -> (
        Vec<codegen::Object>,
        Map<String, openrpc::Error>,
        Vec<codegen::Method>,
    ) {
        let mut objects = Vec::new();
        {
            let processed = self
                .schemas
                .iter()
                .filter_map(|(name, _)| {
                    bind_object(name, &self.schemas, &mut objects)
                })
                .collect::<Vec<_>>();
            objects.extend_from_slice(&processed);
        }

        let errors = self
            .errors
            .iter()
            .filter_map(|(name, _)| {
                let error = get_error(name, &self.errors)?;
                Some((name.to_owned(), error.clone()))
            })
            .collect();

        let methods = self
            .methods
            .iter()
            .filter_map(|method| {
                bind_method(/* parent_name */ "", method, &mut objects)
            })
            .collect();

        (objects, errors, methods)
    }
}

pub fn gen_code<P: AsPath>(paths: &[P]) -> Result<String, std::fmt::Error> {
    let specs = paths.iter().map(parse).collect::<Vec<_>>();

    let schemas = specs
        .iter()
        .flat_map(|spec| {
            spec.components
                .as_ref()
                .map(|components| components.schemas.clone())
                .unwrap_or_default()
        })
        .filter(|(name, schema)| match schema {
            SchemaOrRef::Schema(_) => true,
            r @ SchemaOrRef::Ref { .. } => {
                r.get_ref().map(|n| name != n).unwrap_or_default()
            }
        })
        .collect::<Map<_, _>>();

    let errors = specs
        .iter()
        .flat_map(|spec| {
            spec.components
                .as_ref()
                .map(|components| components.errors.clone())
                .unwrap_or_default()
        })
        .filter(|(name, error)| match error {
            ErrorOrRef::Err(_) => true,
            r @ ErrorOrRef::Ref { .. } => {
                r.get_ref().map(|n| name != n).unwrap_or_default()
            }
        })
        .collect::<Map<_, _>>();

    let methods = specs
        .iter()
        .flat_map(|spec| spec.methods.clone())
        .collect::<Vec<_>>();

    let mut gen = Gen::new(schemas, errors, methods);
    let (schemas, errors, methods) = gen.apply();

    let mut target = String::new();
    use std::fmt::Write;
    writeln!(target, "\n// vvv GENERATED CODE BELOW vvv")?;
    writeln!(target, "#[allow(dead_code)]")?;
    writeln!(target, "#[allow(non_snake_case)]")?;
    writeln!(target, "#[allow(unused_variables)]")?;
    writeln!(target, "#[allow(clippy::enum_variant_names)]")?;
    writeln!(target, "pub mod gen {{")?;
    writeln!(target, "use serde::{{Deserialize, Serialize}};")?;
    writeln!(target, "use serde_json::Value;")?;
    writeln!(target, "\nuse iamgroot::jsonrpc;\n")?;

    for object in schemas {
        let code = renders::render_object(&object)?;
        writeln!(target, "{code}")?;
    }

    writeln!(target, "\npub trait Rpc {{")?;
    for method in &methods {
        let code = renders::render_method(method);
        writeln!(target, "\n{code}")?;
    }
    writeln!(target, "}}")?;
    for contract in &methods {
        let code = renders::render_method_handler(contract);
        writeln!(target, "{code}")?;
    }
    writeln!(target, "{}", renders::render_handle_function(&methods))?;
    writeln!(target, "{}", renders::render_errors(errors))?;
    writeln!(target, "{}", renders::render_client(&methods))?;
    writeln!(target, "}}")?;
    writeln!(target, "// ^^^ GENERATED CODE ABOVE ^^^")?;
    Ok(target)
}
