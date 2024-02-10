use std::collections::HashMap as Map;
use std::{fmt::Display, path::Path};

use codegen::{Enum, Object, Primitive, Property, Struct, Type, Variant};
use openrpc::{Content, Error, ErrorOrRef, Schema, SchemaOrRef};

use crate::codegen::Rule;

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

fn x(
    schemas: &Map<String, SchemaOrRef>,
    errors: &Map<String, ErrorOrRef>,
    methods: Vec<openrpc::Method>,
) -> (
    Vec<codegen::Object>,
    Map<String, openrpc::Error>,
    Vec<codegen::Method>,
) {
    let objects = schemas
        .iter()
        .filter_map(|(name, _)| bind_object(name, schemas))
        .collect();

    let errors = bind_errors(errors);

    let methods = methods.into_iter().filter_map(bind_method).collect();

    (objects, errors, methods)
}

fn bind_method(method: openrpc::Method) -> Option<codegen::Method> {
    let ret = get_type(method.result.as_ref()?.schema.as_ref()?, true)?;
    Some(codegen::Method {
        doc: method.summary,
        name: method.name,
        args: method.params.iter().flat_map(bind_param).collect(),
        ret,
    })
}

fn bind_param(param: &Content) -> Option<(String, Type)> {
    let name = get_prop_name(param.name.as_ref()?);
    let required = param.required.unwrap_or_default();
    let ty = get_type(param.schema.as_ref()?, required)?;
    Some((name, ty))
}

fn get_error<'a>(name: &'a str, errors: &'a Map<String, ErrorOrRef>) -> Option<&'a Error> {
    match errors.get(name)? {
        ErrorOrRef::Err(e) => Some(e),
        r @ ErrorOrRef::Ref { .. } => {
            let r = r.get_ref()?;
            match errors.get(r) {
                Some(ErrorOrRef::Err(ret)) => Some(ret),
                _ => None,
            }
        }
    }
}

fn bind_errors(_errors: &Map<String, ErrorOrRef>) -> Map<String, openrpc::Error> {
    Default::default() // TODO: errors
}

fn get_schema<'a>(name: &'a str, schemas: &'a Map<String, SchemaOrRef>) -> Option<&'a Schema> {
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
    if name.is_empty() {
        return name.to_owned();
    }
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

fn bind_schema(schema: &Schema) -> Option<Object> {
    if let Some(variants) = schema.r#enum.as_ref() {
        return bind_enum(variants, schema);
    }
    if let Some(ty) = schema.r#type.as_ref() {
        return bind_type(ty, schema);
    }

    // TODO: allOf (see BLOCK_BODY_WITH_TXS.transactions)

    // TODO: array of arrays (see EVENT_FILTER.keys)

    // TODO: oneOf (see BLOCK_ID)

    None
}

fn bind_enum(variants: &[String], _schema: &Schema) -> Option<Object> {
    Some(Object::Enum(Enum {
        variants: variants
            .iter()
            .map(|value| Variant {
                name: normalize(value),
                value: value.to_owned(),
            })
            .collect(),
        ..Default::default()
    }))
}

fn bind_type(ty: &openrpc::Type, schema: &Schema) -> Option<Object> {
    match ty {
        openrpc::Type::String => {
            let mut rules = Vec::with_capacity(1);
            if let Some(regex) = schema.pattern.as_ref() {
                rules.push(Rule::Regex(regex.to_owned()));
            };
            if rules.is_empty() {
                return Some(Object::Type(Type::Primitive(Primitive::String, vec![])));
            }
            let object = Struct {
                properties: vec![Property::of(Type::Primitive(Primitive::String, rules))],
                ..Default::default()
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
            if rules.is_empty() {
                return Some(Object::Type(Type::Primitive(Primitive::Integer, vec![])));
            }
            let object = Struct {
                properties: vec![Property::of(Type::Primitive(Primitive::Integer, rules))],
                ..Default::default()
            };
            Some(Object::Struct(object))
        }
        openrpc::Type::Boolean => Some(Object::Struct(Struct {
            properties: vec![Property::of(codegen::Type::Primitive(
                Primitive::Boolean,
                vec![],
            ))],
            ..Default::default()
        })),
        openrpc::Type::Null => Some(Object::Struct(Struct::default())),
        openrpc::Type::Object => {
            let properties = schema
                .properties
                .as_ref()
                .map(|props| {
                    props
                        .iter()
                        .filter_map(|(prop_name, prop_schema)| {
                            let required = schema
                                .required
                                .as_ref()
                                .map(|rs: &Vec<String>| rs.contains(prop_name))
                                .unwrap_or_default();
                            bind_prop(prop_name, prop_schema, required)
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let object = Struct {
                properties,
                ..Default::default()
            };
            Some(Object::Struct(object))
        }
        openrpc::Type::Array => {
            let items = schema.items.as_ref()?;
            let ty = match &**items {
                SchemaOrRef::Schema(param) => {
                    eprintln!("anonymous array param type definition: {param:#?}");
                    return None;
                }
                r @ SchemaOrRef::Ref { .. } => {
                    let name = r.get_ref()?;
                    let name = normalize(name);
                    Type::Named(name)
                }
            };
            Some(Object::Type(Type::Array(Box::new(ty))))
        }
    }
}

fn get_prop_name(name: &str) -> String {
    if name == "enum" || name == "struct" || name == "type" {
        format!("r#{}", name)
    } else {
        name.to_owned()
    }
}

fn bind_prop(prop_name: &str, schema: &SchemaOrRef, required: bool) -> Option<Property> {
    Some(Property {
        name: get_prop_name(prop_name),
        r#type: get_type(schema, required)?,
        visibility: codegen::Visibility::Public,
        decorators: vec![],
        flatten: false,
        required,
    })
}

fn get_type(schema: &SchemaOrRef, required: bool) -> Option<Type> {
    let ty = match schema {
        r @ SchemaOrRef::Ref { .. } => {
            let name = r.get_ref()?;
            let name = normalize(name);
            Type::Named(name)
        }
        SchemaOrRef::Schema(schema) => bind_schema(schema)?.get_type(),
    };
    if required {
        Some(ty)
    } else {
        Some(Type::Option(Box::new(ty)))
    }
}

fn bind_object(name: &str, schemas: &Map<String, SchemaOrRef>) -> Option<Object> {
    match schemas.get(name)? {
        SchemaOrRef::Schema(schema) => {
            let object = bind_schema(schema)?;
            Some(object.with_name(name))
        }
        r @ SchemaOrRef::Ref { .. } => {
            let ty = Type::Named(normalize(r.get_ref()?));
            Some(Object::Struct(Struct {
                name: normalize(name),
                properties: vec![Property::of(ty)],
                ..Default::default()
            }))
        }
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
            r @ SchemaOrRef::Ref { .. } => r.get_ref().map(|n| name != n).unwrap_or_default(),
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
            r @ ErrorOrRef::Ref { .. } => r.get_ref().map(|n| name != n).unwrap_or_default(),
        })
        .collect::<Map<_, _>>();

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
