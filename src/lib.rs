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
    tracing::info!(file = path.to_string(), "processing");
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
        Vec<(String, openrpc::Error)>,
        Vec<codegen::Method>,
    ) {
        let mut objects = Vec::new();
        let processed = self
            .schemas
            .iter()
            .filter_map(|(name, _)| {
                bind_object(name, &self.schemas, &mut objects)
            })
            .collect::<Vec<_>>();
        for object in processed {
            objects.push(object);
        }
        objects.sort_by_cached_key(|object| object.get_name().to_owned());

        let mut errors = self
            .errors
            .iter()
            .filter_map(|(name, _)| {
                let error = get_error(name, &self.errors)?;
                Some((name.to_owned(), error.clone()))
            })
            .collect::<Vec<_>>();
        errors.sort_by_cached_key(|(name, _)| name.to_owned());

        let mut methods = self
            .methods
            .iter()
            .filter_map(|method| {
                bind_method(/* parent_name */ "", method, &mut objects)
            })
            .collect::<Vec<_>>();
        methods.sort_by_cached_key(|method| method.name.to_owned());

        (objects, errors, methods)
    }
}

pub fn gen_code<P: AsPath>(
    paths: &[P],
    gen_async: bool,
    gen_blocking: bool,
    gen_client: bool,
) -> Result<String, std::fmt::Error> {
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
    writeln!(target, "#[allow(clippy::module_inception)]")?;
    writeln!(target, "#[allow(non_snake_case)]")?;
    writeln!(target, "#[allow(clippy::enum_variant_names)]")?;
    writeln!(target, "pub mod gen {{")?;
    writeln!(target, "use serde::{{Deserialize, Serialize}};")?;
    writeln!(target, "use serde_json::Value;")?;
    writeln!(target, "\nuse iamgroot::jsonrpc;\n")?;

    for object in schemas {
        let code = renders::render_object(&object)?;
        writeln!(target, "{code}")?;
    }

    writeln!(target, "\n")?;

    writeln!(target, "{}", renders::render_errors(&errors))?;

    if gen_async {
        writeln!(target, "#[async_trait::async_trait]")?;
        writeln!(target, "pub trait Rpc {{")?;
        for method in &methods {
            let code = renders::render_method(method, true);
            writeln!(target, "\n{code}")?;
        }
        writeln!(target, "}}")?;
        for contract in &methods {
            let code = renders::render_method_handler(contract, true);
            writeln!(target, "{code}")?;
        }
        writeln!(
            target,
            "{}",
            renders::render_handle_function(&methods, true)
        )?;
    }

    if gen_blocking {
        writeln!(target, "pub mod blocking {{\nuse super::*;")?;

        writeln!(target, "pub trait Rpc {{")?;
        for method in &methods {
            let code = renders::render_method(method, false);
            writeln!(target, "\n{code}")?;
        }
        writeln!(target, "}}")?; // trait Rpc
        for contract in &methods {
            let code = renders::render_method_handler(contract, false);
            writeln!(target, "{code}")?;
        }
        writeln!(
            target,
            "{}",
            renders::render_handle_function(&methods, false)
        )?;

        writeln!(target, "\n}}")?; // mod blocking
    }

    if gen_client {
        writeln!(target, "pub mod client {{\nuse super::*;")?;

        if gen_async {
            writeln!(target, "{}", renders::render_client(&methods, true))?;
        }

        if gen_blocking {
            let code = renders::render_client(&methods, false);
            writeln!(
                target,
                "pub mod blocking {{\nuse super::*;\n {code} \n}}"
            )?;
        }

        writeln!(target, "\n}}")?; // mod client
    }

    writeln!(target, "}}")?; // mod gen
    writeln!(target, "// ^^^ GENERATED CODE ABOVE ^^^")?;
    Ok(target)
}
