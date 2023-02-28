use std::collections::HashMap;

use crate::codegen;
use crate::openrpc;

const SCHEMA_REF_PREFIX: &str = "#/components/schemas/";
const ERROR_REF_PREFIX: &str = "#/components/errors/";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binding {
    Basic(codegen::Basic),
    Struct(codegen::Struct),
    Enum(codegen::Enum),
    Named(String, codegen::Type),
}

impl Binding {
    pub fn get_name(&self) -> String {
        match self {
            Binding::Basic(basic) => basic.to_string(),
            Binding::Struct(s) => s.name.clone(),
            Binding::Enum(e) => e.name.clone(),
            Binding::Named(name, _) => name.clone(),
        }
    }

    pub fn get_type(&self) -> codegen::Type {
        if !self.get_name().is_empty() {
            match self {
                Binding::Basic(basic) => codegen::Type::Basic(basic.clone()),
                Binding::Struct(r#struct) => codegen::Type::Named(r#struct.name.clone()),
                Binding::Enum(r#enum) => codegen::Type::Named(r#enum.name.clone()),
                Binding::Named(name, _) => codegen::Type::Named(name.clone()),
            }
        } else {
            // anonymous types (not named, thus full structure must be represented)
            match self {
                Binding::Basic(basic) => codegen::Type::Basic(basic.clone()),
                Binding::Struct(s) => {
                    let props = s
                        .properties
                        .iter()
                        .map(|p| (p.name.clone(), p.r#type.clone()))
                        .collect();
                    codegen::Type::Struct(props)
                }
                Binding::Enum(e) => {
                    let vars = e
                        .variants
                        .iter()
                        .map(|p| (p.name.clone().to_ascii_uppercase(), p.r#type.clone()))
                        .collect();
                    codegen::Type::Enum(vars)
                }
                Binding::Named(_, t) => t.clone(),
            }
        }
    }
}

fn deanonimize(binding: &Binding) -> String {
    let mut name = binding.get_name();
    if !name.is_empty() {
        return name;
    }
    if let Binding::Struct(s) = binding {
        if s.properties.len() == 1 {
            name = s.properties.first().unwrap().name.clone();
        }
    }
    name
}

fn all_of(name: String, bindings: Vec<Binding>) -> Binding {
    let properties = bindings
        .into_iter()
        .map(|b| {
            let name = deanonimize(&b).to_ascii_lowercase();
            codegen::Property {
                name,
                r#type: b.get_type(),
                flatten: true,
                ..Default::default()
            }
        })
        .flat_map(unfold_property)
        .collect();
    Binding::Struct(codegen::Struct {
        name,
        properties,
        ..Default::default()
    })
}

fn one_of(name: String, bindings: Vec<Binding>) -> Binding {
    let variants = bindings
        .into_iter()
        .map(|b| {
            let name = deanonimize(&b).to_ascii_uppercase();
            let r#type = match b.get_type() {
                codegen::Type::Struct(fields)
                    if fields.len() == 1 && fields[0].0.to_ascii_uppercase() == name =>
                {
                    // avoid creating unnecessary wrappers (single-property tuples)
                    fields[0].1.clone()
                }
                unchanged => unchanged,
            };
            codegen::Variant { name, r#type }
        })
        .collect();
    Binding::Enum(codegen::Enum {
        name,
        variants,
        ..Default::default()
    })
}

pub fn unfold_property(property: codegen::Property) -> Vec<codegen::Property> {
    if !property.name.is_empty() {
        return match property.r#type {
            codegen::Type::Struct(fields) if fields.len() == 1 => fields
                .into_iter()
                .take(1)
                .map(|(name, r#type)| codegen::Property {
                    name: name.to_ascii_lowercase(),
                    r#type,
                    ..Default::default()
                })
                .collect(),
            _ => vec![property],
        };
    }

    match property.r#type {
        codegen::Type::Struct(fields) => fields
            .into_iter()
            .map(|(name, r#type)| codegen::Property {
                name: name.to_ascii_lowercase(),
                r#type,
                ..Default::default()
            })
            .collect(),
        codegen::Type::Enum(variants) => variants
            .into_iter()
            .map(|(name, r#type)| codegen::Property {
                name: name.to_ascii_lowercase(),
                r#type,
                ..Default::default()
            })
            .collect(),
        _ => vec![property],
    }
}

pub fn extract_property(
    properties: Vec<codegen::Property>,
) -> (Vec<codegen::Property>, Vec<Binding>) {
    let mut props = Vec::new();
    let mut binds = Vec::new();
    for property in properties {
        let name = &property.name;
        match &property.r#type {
            codegen::Type::Struct(struct_fields) => {
                let prop = codegen::Property {
                    name: name.clone(),
                    r#type: codegen::Type::Named(name.clone().to_ascii_uppercase()),
                    ..Default::default()
                };
                props.push(prop);

                let bind = Binding::Struct(codegen::Struct {
                    name: name.clone().to_ascii_uppercase(),
                    properties: struct_fields
                        .iter()
                        .cloned()
                        .map(|(name, r#type)| codegen::Property {
                            name,
                            r#type,
                            ..Default::default()
                        })
                        .collect(),
                    ..Default::default()
                });
                binds.push(bind);
            }
            codegen::Type::Array(boxed) => match &**boxed {
                codegen::Type::Struct(struct_fields) => {
                    let prop = codegen::Property {
                        name: name.clone(),
                        r#type: codegen::Type::Array(Box::new(codegen::Type::Named(
                            name.clone().to_ascii_uppercase() + "_ITEM",
                        ))),
                        ..Default::default()
                    };
                    props.push(prop);

                    let bind = Binding::Struct(codegen::Struct {
                        name: name.clone().to_ascii_uppercase() + "_ITEM",
                        properties: struct_fields
                            .iter()
                            .cloned()
                            .map(|(name, r#type)| codegen::Property {
                                name,
                                r#type,
                                ..Default::default()
                            })
                            .collect(),
                        ..Default::default()
                    });
                    binds.push(bind);
                }
                _ => {
                    props.push(property);
                }
            },
            codegen::Type::Option(boxed) => match &**boxed {
                codegen::Type::Struct(struct_fields) => {
                    let prop = codegen::Property {
                        name: name.clone(),
                        r#type: codegen::Type::Option(Box::new(codegen::Type::Named(
                            name.clone().to_ascii_uppercase() + "_ITEM",
                        ))),
                        ..Default::default()
                    };
                    props.push(prop);

                    let bind = Binding::Struct(codegen::Struct {
                        name: name.clone().to_ascii_uppercase() + "_ITEM",
                        properties: struct_fields
                            .iter()
                            .cloned()
                            .map(|(name, r#type)| codegen::Property {
                                name,
                                r#type,
                                ..Default::default()
                            })
                            .collect(),
                        ..Default::default()
                    });
                    binds.push(bind);
                }
                _ => {
                    props.push(property);
                }
            },
            codegen::Type::Enum(enum_variants) => {
                let prop = codegen::Property {
                    name: name.clone(),
                    r#type: codegen::Type::Named(name.clone().to_ascii_uppercase() + "_ENUM"),
                    ..Default::default()
                };
                props.push(prop);

                let bind = Binding::Enum(codegen::Enum {
                    name: name.clone().to_ascii_uppercase() + "_ENUM",
                    variants: enum_variants
                        .iter()
                        .cloned()
                        .map(|(name, r#type)| codegen::Variant { name, r#type })
                        .collect(),
                    ..Default::default()
                });
                binds.push(bind);
            }
            _ => {
                props.push(property);
            }
        }
    }
    (props, binds)
}

pub fn unfold_binding(binding: Binding) -> Vec<Binding> {
    match binding {
        Binding::Struct(the_struct) => {
            let (props, mut binds) = extract_property(the_struct.properties);
            let binding = Binding::Struct(codegen::Struct {
                properties: props,
                ..the_struct
            });
            binds.push(binding);
            binds
        }
        other => vec![other],
    }
}

pub fn get_schema_binding(
    name: String,
    schema: &openrpc::Schema,
    spec: &openrpc::OpenRpc,
    cache: &mut HashMap<String, Binding>,
) -> Binding {
    if let Some(key) = &schema.r#ref {
        // Allow shared cache lookups for cross-file references
        let key = key.split(SCHEMA_REF_PREFIX).nth(1).unwrap();
        if let Some(binding) = cache.get(key) {
            return binding.clone();
        }

        let schema = spec.get_schema(key).expect("schema");
        let binding = get_schema_binding(key.to_string(), schema, spec, cache);
        cache.insert(key.to_string(), binding.clone());
        return binding;
    }
    if schema.has_type("string") {
        if let Some(values) = schema.r#enum.as_ref() {
            let variants = values
                .iter()
                .cloned()
                .map(|name| codegen::Variant {
                    name,
                    r#type: codegen::Type::Unit,
                })
                .collect();
            return Binding::Enum(codegen::Enum::of(name, variants));
        }
        return Binding::Basic(codegen::Basic::String);
    }
    if schema.has_type("integer") {
        return Binding::Basic(codegen::Basic::Integer);
    }
    if schema.has_type("boolean") {
        return Binding::Basic(codegen::Basic::Boolean);
    }
    if schema.has_type("array") {
        let schema = schema.items.as_ref().expect("schema");
        let binding = get_schema_binding(name.clone(), schema, spec, cache);
        let item_type = Box::new(binding.get_type());
        return Binding::Named(name, codegen::Type::Array(item_type));
    }
    // assuming schema.type="object"
    if schema.properties.is_some() {
        let properties = schema.properties.as_ref().expect("properties");
        let properties = properties
            .iter()
            .map(|(prop_name, prop_schema)| {
                let binding = get_schema_binding(String::default(), prop_schema, spec, cache);
                let is_required = schema
                    .required
                    .as_ref()
                    .map(|required| required.contains(prop_name))
                    .unwrap_or_default();
                let prop_type = if is_required {
                    binding.get_type()
                } else {
                    codegen::Type::Option(Box::new(binding.get_type()))
                };
                codegen::Property::of(prop_name.to_string(), prop_type)
            })
            .flat_map(unfold_property)
            .collect();
        return Binding::Struct(codegen::Struct::of(name, properties));
    }
    if let Some(all) = schema.allOf.as_ref() {
        let bindings = all
            .iter()
            .map(|schema| get_schema_binding(String::default(), schema, spec, cache))
            .collect::<Vec<_>>();
        return all_of(name, bindings);
    }
    if let Some(one) = schema.oneOf.as_ref() {
        let bindings = one
            .iter()
            .map(|schema| get_schema_binding(String::default(), schema, spec, cache))
            .collect::<Vec<_>>();
        return one_of(name, bindings);
    }
    if let Some(schema) = schema.schema.as_ref() {
        // TODO describe exact reasoning & use-case for this clause (might be deprecated corner-case)
        return get_schema_binding(name, schema, spec, cache);
    }
    unreachable!()
}

#[derive(Debug, Clone)]
pub struct Contract {
    pub name: String,
    pub params: Vec<(String, codegen::Type)>,
    pub result: Option<Binding>,
    pub errors: Vec<openrpc::Error>,
    pub summary: String,
    pub description: String,
}

pub fn get_method_contract(
    name: String,
    spec: &openrpc::OpenRpc,
    cache: &mut HashMap<String, Binding>,
) -> Option<Contract> {
    let method = spec.methods.iter().find(|m| m.name == name)?;
    let params = method
        .params
        .iter()
        .map(|param| {
            let schema = param.schema.as_ref().unwrap();
            let name = param.name.clone().unwrap_or_default();
            let binding = get_schema_binding(name.clone(), schema, spec, cache);
            cache.insert(binding.get_name(), binding.clone());
            let is_required = param.required.unwrap_or_default();
            let param_type = if is_required {
                binding.get_type()
            } else {
                codegen::Type::Option(Box::new(binding.get_type()))
            };
            (name, param_type)
        })
        .collect();
    let result = method.result.as_ref().map(|result| {
        let schema = result.schema.as_ref().unwrap();
        let name = result.name.clone().unwrap_or_default();
        let name = format!("{}_{name}", method.name);
        let binding = get_schema_binding(name.clone(), schema, spec, cache);
        cache.insert(name, binding.clone());
        binding
    });
    let errors = method
        .errors
        .clone()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|reference| {
            let key = reference.r#ref?;
            // Enable shared cache lookup for cross-file references
            let key = key.split(ERROR_REF_PREFIX).nth(1).unwrap();
            spec.components
                .as_ref()
                .map(|components| &components.errors)
                .and_then(|errors| errors.get(key))
        })
        .cloned()
        .collect();
    Some(Contract {
        name,
        params,
        result,
        errors,
        summary: method.summary.clone().unwrap_or_default(),
        description: method.description.clone().unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_bindings() {
        // rigorous testing...
    }
}
