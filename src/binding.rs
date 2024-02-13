use crate::{
    codegen::{
        self, Enum, Object, Primitive, Property, Rule, Struct, Type, Variant,
    },
    openrpc::{self, Content, Error, ErrorOrRef, Schema, SchemaOrRef},
};
use std::collections::HashMap as Map;

pub fn bind_method(
    parent_name: &str,
    method: &openrpc::Method,
    objects: &mut Vec<Object>,
) -> Option<codegen::Method> {
    let name = unprefix(&method.name);
    let object = bind_schema_ref(
        parent_name,
        method.result.as_ref()?.schema.as_ref()?,
        objects,
    )?;
    let ret = match object {
        Object::Type(ty) => ty,
        Object::Alias(name, _) => Type::Named(name),
        object => {
            let name = format!("{}Result", capitalize(&name));
            let object = object.with_name(name.clone());
            // println!("/*\nanonymous object detected (ret): {object:#?}\n*/");
            objects.push(object);
            Type::Named(name)
        }
    };
    Some(codegen::Method {
        name,
        args: method
            .params
            .iter()
            .flat_map(|param| {
                bind_param(parent_name, &method.name, param, objects)
            })
            .collect(),
        ret,
        doc: method.summary.clone(),
    })
}

fn bind_param(
    parent_name: &str,
    method_name: &str,
    param: &Content,
    objects: &mut Vec<Object>,
) -> Option<(String, Type)> {
    let object = bind_schema_ref(parent_name, param.schema.as_ref()?, objects)?;
    let name = get_prop_name(param.name.as_ref()?);
    let ty = match object {
        Object::Type(ty) => ty,
        Object::Alias(name, _) => Type::Named(name),
        object => {
            let name = capitalize(&name);
            let name = format!("{}{name}", capitalize(&unprefix(method_name)));
            let object = object.with_name(name.clone());
            // println!("/*\nanonymous object detected (param): {object:#?}\n*/",);
            objects.push(object);
            Type::Named(name)
        }
    };

    let required = param.required.unwrap_or_default();
    let ty = if !required {
        Type::Option(Box::new(ty))
    } else {
        ty
    };
    Some((name, ty))
}

pub fn get_error<'a>(
    name: &'a str,
    errors: &'a Map<String, ErrorOrRef>,
) -> Option<&'a Error> {
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

fn unprefix(name: &str) -> String {
    name.split('_')
        .nth(1)
        .map(|s| s.to_owned())
        .unwrap_or_default()
}

fn capitalize(name: &str) -> String {
    if name.is_empty() {
        return name.to_owned();
    }
    let head = name.chars().next().unwrap();
    let tail = name.chars().skip(1).collect::<String>();
    format!("{}{}", head.to_ascii_uppercase(), tail)
}

fn normalize(name: &str) -> String {
    name.to_ascii_lowercase()
        .split(|c| c == '_' || c == ' ')
        .map(capitalize)
        .collect::<Vec<_>>()
        .join("")
}

fn bind_schema_ref(
    parent_name: &str,
    schama_or_ref: &SchemaOrRef,
    objects: &mut Vec<Object>,
) -> Option<Object> {
    match schama_or_ref {
        r @ SchemaOrRef::Ref { .. } => {
            let name = r.get_ref()?;
            let name = normalize(name);
            Some(Object::Type(Type::Named(name)))
        }
        SchemaOrRef::Schema(schema) => {
            bind_schema(parent_name, schema, objects)
        }
    }
}

fn bind_schema(
    name: &str,
    schema: &Schema,
    objects: &mut Vec<Object>,
) -> Option<Object> {
    if let Some(variants) = schema.r#enum.as_ref() {
        return bind_enum(variants, schema);
    }
    if let Some(ty) = schema.r#type.as_ref() {
        return bind_type(name, ty, schema, objects);
    }
    if let Some(one_of) = schema.oneOf.as_ref() {
        return Some(bind_one_of(name, one_of, objects));
    }
    if let Some(all_of) = schema.allOf.as_ref() {
        return Some(bind_all_of(name, all_of, objects));
    }
    if let Some(SchemaOrRef::Schema(nested)) = schema.schema.as_deref() {
        return bind_schema(name, nested, objects);
    }
    panic!("schema binding failed: {schema:#?}");
}

fn bind_all_of(
    parent_name: &str,
    all_of: &[SchemaOrRef],
    objects: &mut Vec<Object>,
) -> Object {
    let properties = all_of
        .iter()
        .flat_map(|schema| match schema {
            r @ SchemaOrRef::Ref { .. } => {
                let name = r.get_ref().unwrap();
                let type_name = normalize(name);
                let ty = Type::Named(type_name);
                let prop_name = get_prop_name(name);

                // TODO: mark for flattening with serde?
                eprintln!("marked for flattening prop={prop_name}");

                vec![Property {
                    name: prop_name,
                    r#type: ty,
                }]
            }
            SchemaOrRef::Schema(schema) => {
                match bind_schema(parent_name, schema, objects) {
                    Some(Object::Struct(s)) => s.properties,
                    x => panic!("allOf member not matched to a struct: {x:?}"),
                }
            }
        })
        .collect();
    Object::Struct(Struct {
        name: Default::default(),
        properties,
    })
}

fn bind_one_of(
    parent_name: &str,
    one_of: &[SchemaOrRef],
    objects: &mut Vec<Object>,
) -> Object {
    let variants = one_of
        .iter()
        .map(|schema| match schema {
            r @ SchemaOrRef::Ref { .. } => {
                let name = r.get_ref().unwrap();
                let name = normalize(name);
                let ty = Type::Named(name.clone());
                Variant::Struct(Struct {
                    name,
                    properties: vec![Property {
                        name: Default::default(),
                        r#type: ty,
                    }],
                })
            }
            SchemaOrRef::Schema(schema) => {
                match bind_schema(parent_name, schema, objects) {
                    Some(Object::Struct(mut s)) if !s.properties.is_empty() => {
                        s.name = if let Some(name) = schema.title.as_ref() {
                            normalize(name)
                        } else {
                            normalize(&s.properties[0].name)
                        };
                        Variant::Struct(s)
                    }
                    x => panic!("oneOf variant not matched to a struct: {x:?}"),
                }
            }
        })
        .collect();
    Object::Enum(Enum {
        name: Default::default(),
        variants,
    })
}

fn bind_enum(variants: &[String], schema: &Schema) -> Option<Object> {
    Some(Object::Enum(Enum {
        variants: variants
            .iter()
            .map(|value| {
                let name = normalize(value);
                let name = if name.chars().next().unwrap().is_alphabetic() {
                    name
                } else {
                    let head = schema
                        .title
                        .as_ref()
                        .and_then(|title| title.chars().next())
                        .unwrap_or('V');
                    format!("{head}{name}")
                };
                Variant::Const {
                    name,
                    value: value.to_owned(),
                }
            })
            .collect(),
        ..Default::default()
    }))
}

fn bind_type(
    parent_name: &str,
    ty: &openrpc::Type,
    schema: &Schema,
    objects: &mut Vec<Object>,
) -> Option<Object> {
    match ty {
        openrpc::Type::String => {
            let mut rules = Vec::with_capacity(1);
            if let Some(regex) = schema.pattern.as_ref() {
                rules.push(Rule::Regex(regex.to_owned()));
            };
            if rules.is_empty() {
                return Some(Object::Type(Type::Primitive(
                    Primitive::String,
                    vec![],
                )));
            }
            let object = Struct {
                properties: vec![Property::of(Type::Primitive(
                    Primitive::String,
                    rules,
                ))],
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
                return Some(Object::Type(Type::Primitive(
                    Primitive::Integer,
                    vec![],
                )));
            }
            let object = Struct {
                properties: vec![Property::of(Type::Primitive(
                    Primitive::Integer,
                    rules,
                ))],
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
        openrpc::Type::Object if schema.properties.is_some() => {
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
                            let mut prop = bind_prop(
                                parent_name,
                                prop_name,
                                prop_schema,
                                objects,
                            )?;
                            if !required {
                                let ty = Type::Option(Box::new(prop.r#type));
                                prop.r#type = ty;
                            }
                            Some(prop)
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
        openrpc::Type::Object if schema.allOf.is_some() => {
            let all_of = schema.allOf.as_ref().unwrap();
            Some(bind_all_of(parent_name, all_of, objects))
        }
        openrpc::Type::Object => {
            panic!("object type binding failed: {schema:?}")
        }
        openrpc::Type::Array => {
            let items = schema.items.as_ref()?;
            let ty = match &**items {
                SchemaOrRef::Schema(schema) => {
                    let object = bind_schema(parent_name, schema, objects)?;
                    match object {
                        Object::Type(ty) => ty,
                        Object::Alias(name, _) => Type::Named(name),
                        object => {
                            let name = schema.title.clone().unwrap_or_default();
                            let object = object.with_name(name.clone());
                            // println!("/*\nanonymous object detected (type): {object:#?}\n*/");
                            objects.push(object);
                            Type::Named(name)
                        }
                    }
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
        format!("r#{}", name).to_ascii_lowercase()
    } else {
        name.to_ascii_lowercase()
    }
}

fn bind_prop(
    parent_name: &str,
    prop_name: &str,
    schema: &SchemaOrRef,
    objects: &mut Vec<Object>,
) -> Option<Property> {
    let object = bind_schema_ref(parent_name, schema, objects)?;
    let name = get_prop_name(prop_name);
    let r#type = match object {
        Object::Type(ty) => ty,
        Object::Alias(name, _) => Type::Named(name),
        object => {
            let type_name = normalize(prop_name);
            let type_name = format!("{}{type_name}", normalize(parent_name));
            let object = object.with_name(type_name.clone());
            // println!("/*\nanonymous object detected (prop): {object:#?}\n*/",);
            objects.push(object);
            Type::Named(type_name)
        }
    };
    Some(Property { name, r#type })
}

pub fn bind_object(
    name: &str,
    schemas: &Map<String, SchemaOrRef>,
    objects: &mut Vec<Object>,
) -> Option<Object> {
    match schemas.get(name)? {
        SchemaOrRef::Schema(schema) => {
            let object = bind_schema(name, schema, objects)?;
            Some(object.with_name(normalize(name)))
        }
        r @ SchemaOrRef::Ref { .. } => {
            let ty = Type::Named(normalize(r.get_ref()?));
            Some(Object::Struct(Struct {
                name: normalize(name),
                properties: vec![Property::of(ty)],
            }))
        }
    }
}
