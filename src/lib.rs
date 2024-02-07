use std::{collections::HashMap, fmt::Display, path::Path};

use codegen::{Object, Primitive, Property, Struct, Type};
use openrpc::{ErrorOrRef, Schema, SchemaOrRef};

use crate::codegen::Rule;

pub(crate) mod codegen;
pub mod jsonrpc;
pub(crate) mod openrpc;
pub(crate) mod renders;

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

fn x(
    schemas: &HashMap<String, SchemaOrRef>,
    errors: &HashMap<String, ErrorOrRef>,
    methods: Vec<openrpc::Method>,
) -> (
    Vec<codegen::Object>,
    HashMap<String, openrpc::Error>,
    Vec<codegen::Method>,
) {
    // TODO: impl

    let objects = Vec::new();
    let errors = HashMap::new();
    let methods = Vec::new();

    (objects, errors, methods)
}

fn resolve_schema<'a>(
    name: &'a str,
    schemas: &'a HashMap<String, SchemaOrRef>,
) -> Option<&'a Schema> {
    match schemas.get(name)? {
        SchemaOrRef::Schema(schema) => Some(schema),
        r @ SchemaOrRef::Ref { .. } => {
            let r = r.get_ref()?;
            match schemas.get(r) {
                Some(SchemaOrRef::Schema(ret)) => Some(ret),
                _ => None,
            }
        }
    }
}

fn capitalize(name: &str) -> String {
    let head = name.chars().next().unwrap();
    let tail = name.chars().skip(1).collect::<String>();
    format!("{}{}", head.to_ascii_uppercase(), tail.to_ascii_lowercase())
}

fn normalize(name: &str) -> String {
    name.split(|c| c == '_')
        .map(capitalize)
        .collect::<Vec<_>>()
        .join("")
}

fn bind_primitive(
    name: &str,
    ty: &openrpc::Type,
    schema: &Schema,
    schemas: &HashMap<String, SchemaOrRef>,
) -> Option<Object> {
    match ty {
        openrpc::Type::String => {
            let rules = if let Some(regex) = schema.pattern.as_ref() {
                vec![Rule::Regex(regex.to_owned())]
            } else {
                vec![]
            };
            let object = Struct {
                name: normalize(name),
                properties: vec![Property::of(
                    Default::default(),
                    Type::Primitive(Primitive::String, rules),
                )],
                decorators: vec![],
                visibility: codegen::Visibility::Public,
            };
            Some(Object::Struct(object))
        }
        openrpc::Type::Integer => {
            let mut rules = Vec::with_capacity(2);
            if let Some(min) = schema.minimum {
                rules.push(Rule::Min(min));
            }
            if let Some(max) = schema.maximum {
                rules.push(Rule::Max(max));
            }
            let object = Struct {
                name: normalize(name),
                properties: vec![Property::of(
                    Default::default(),
                    Type::Primitive(Primitive::Integer, rules),
                )],
                decorators: vec![],
                visibility: codegen::Visibility::Public,
            };
            Some(Object::Struct(object))
        }
        openrpc::Type::Boolean => Some(Object::Struct(Struct {
            name: normalize(name),
            properties: vec![Property::of(
                Default::default(),
                codegen::Type::Primitive(Primitive::Boolean, vec![]),
            )],
            decorators: vec![],
            visibility: codegen::Visibility::Public,
        })),
        openrpc::Type::Array => todo!(),
        openrpc::Type::Object => todo!(),
        openrpc::Type::Null => todo!(),
    }
}

fn bind_schema(name: &str, schemas: &HashMap<String, SchemaOrRef>) -> Option<Object> {
    let schema = resolve_schema(name, schemas)?;

    if let Some(ty) = schema.r#type.as_ref() {
        return bind_primitive(name, ty, schema, schemas);
    }

    None
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
        .collect::<HashMap<_, _>>();

    let errors = specs
        .iter()
        .flat_map(|spec| {
            spec.components
                .as_ref()
                .map(|components| components.errors.clone())
                .unwrap_or_default()
        })
        .collect::<HashMap<_, _>>();

    let methods = specs
        .iter()
        .flat_map(|spec| spec.methods.clone())
        .collect::<Vec<_>>();

    let (schemas, errors, methods) = x(&schemas, &errors, methods);

    let mut target = String::new();
    use std::fmt::Write;

    writeln!(target, "// vvv GENERATED CODE BELOW vvv")?;
    writeln!(target, "#[allow(dead_code)]")?;
    writeln!(target, "#[allow(non_snake_case)]")?;
    writeln!(target, "#[allow(unused_variables)]")?;
    writeln!(target, "#[allow(clippy::enum_variant_names)]")?;
    writeln!(target, "pub mod gen {{")?;
    writeln!(target, "use serde::{{Deserialize, Serialize}};")?;
    writeln!(target, "use serde_json::Value;")?;
    writeln!(target, "\nuse iamgroot::jsonrpc;")?;

    // TODO: render `schemas`

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
