use crate::cache::Cache;
use crate::codegen;
use crate::openrpc;
use crate::renders;

pub const SCHEMA_REF_PREFIX: &str = "#/components/schemas/";
pub const ERROR_REF_PREFIX: &str = "#/components/errors/";

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
                Binding::Basic(basic) => {
                    codegen::Type::Basic(basic.clone(), codegen::Rules::default())
                }
                Binding::Struct(r#struct) => codegen::Type::Named(r#struct.name.clone()),
                Binding::Enum(r#enum) => codegen::Type::Named(r#enum.name.clone()),
                Binding::Named(name, _) => codegen::Type::Named(name.clone()),
            }
        } else {
            // anonymous types (not named, thus full structure must be represented)
            match self {
                Binding::Basic(basic) => {
                    codegen::Type::Basic(basic.clone(), codegen::Rules::default())
                }
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
            let name = deanonimize(&b);
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
            let name = deanonimize(&b);
            let r#type = b.get_type();
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
                    name,
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
                name,
                r#type,
                ..Default::default()
            })
            .collect(),
        codegen::Type::Enum(variants) => variants
            .into_iter()
            .map(|(name, r#type)| codegen::Property {
                name,
                r#type,
                ..Default::default()
            })
            .collect(),
        _ => vec![property],
    }
}

pub fn extract_properties(
    binding_name: String,
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
            codegen::Type::Enum(enum_variants) => {
                let enum_name =
                    binding_name.to_ascii_uppercase() + "_" + &name.clone().to_ascii_lowercase();
                let prop = codegen::Property {
                    name: name.clone(),
                    r#type: codegen::Type::Named(enum_name.clone()),
                    ..Default::default()
                };
                props.push(prop);

                let bind = Binding::Enum(codegen::Enum {
                    name: enum_name,
                    variants: enum_variants
                        .iter()
                        .cloned()
                        .map(|(name, r#type)| codegen::Variant { name, r#type })
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
                codegen::Type::Enum(enum_variants) => {
                    let enum_name = binding_name.to_ascii_uppercase()
                        + "_"
                        + &name.clone().to_ascii_lowercase();
                    let prop = codegen::Property {
                        name: name.clone(),
                        r#type: codegen::Type::Option(Box::new(codegen::Type::Named(
                            enum_name.clone(),
                        ))),
                        ..Default::default()
                    };
                    props.push(prop);

                    let bind = Binding::Enum(codegen::Enum {
                        name: enum_name,
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
            },
            _ => {
                props.push(property);
            }
        }
    }
    (props, binds)
}

pub fn unfold_binding(binding: Binding) -> Vec<Binding> {
    let binding_name = binding.get_name();
    match binding {
        Binding::Struct(the_struct) => {
            let (props, mut binds) = extract_properties(binding_name, the_struct.properties);
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
    cache: &mut Cache,
    trace: &mut Vec<String>,
) -> Binding {
    //println!("\n// name={name}\n// schema={schema:?}\n// trace={trace:?}");
    if let Some(binding) = cache.get(&name) {
        return binding.clone();
    }

    // Avoid stack overflow for cyclic definitions (starknet_simulateTransaction)
    // TODO: Provide more elegan solution for tracing (RAII-style guard with custom dtor)
    let has_cycle = trace.iter().filter(|item| item == &&name).count() > 2;
    if has_cycle && !name.is_empty() {
        return Binding::Named(name.clone(), codegen::Type::Named(name.clone()));
    }

    trace.push(name.clone());
    if let Some(key) = &schema.r#ref {
        // Allow shared cache lookups for cross-file references
        let key = key.split(SCHEMA_REF_PREFIX).nth(1).unwrap();
        if let Some(binding) = cache.get(key) {
            trace.pop().unwrap_or_default();
            return binding.clone();
        }

        let schema = spec.get_schema(key).expect("schema");
        let binding = get_schema_binding(key.to_string(), schema, spec, cache, trace);
        cache.insert(key.to_string(), binding.clone());
        trace.pop().unwrap_or_default();
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
            trace.pop().unwrap_or_default();
            return Binding::Enum(codegen::Enum::of(name, variants));
        }
        if !name.is_empty() {
            let rules = schema
                .pattern
                .as_ref()
                .cloned()
                .map(|pattern| codegen::Rules {
                    pattern: Some(pattern),
                    ..Default::default()
                })
                .unwrap_or_default();

            let binding = Binding::Named(
                name.clone(),
                codegen::Type::Basic(codegen::Basic::String, rules),
            );
            cache.insert(name, binding.clone());
            trace.pop().unwrap_or_default();
            return binding;
        }
        trace.pop().unwrap_or_default();
        return Binding::Basic(codegen::Basic::String);
    }
    if schema.has_type("integer") || schema.has_type("number") {
        if !name.is_empty() {
            let rules = codegen::Rules {
                min: schema.minimum.clone(),
                max: schema.maximum.clone(),
                ..Default::default()
            };

            let binding = Binding::Named(
                name.clone(),
                codegen::Type::Basic(codegen::Basic::Integer, rules),
            );
            cache.insert(name, binding.clone());
            trace.pop().unwrap_or_default();
            return binding;
        }
        trace.pop().unwrap_or_default();
        return Binding::Basic(codegen::Basic::Integer);
    }
    if schema.has_type("boolean") {
        trace.pop().unwrap_or_default();
        return Binding::Basic(codegen::Basic::Boolean);
    }
    if schema.has_type("array") || schema.items.is_some() {
        let schema = schema.items.as_ref().expect("schema");
        let binding = get_schema_binding(name.clone(), schema, spec, cache, trace);
        let item_type = Box::new(binding.get_type());
        trace.pop().unwrap_or_default();
        return Binding::Named(name, codegen::Type::Array(item_type));
    }
    if schema.properties.is_some() {
        let properties = schema.properties.as_ref().expect("properties");
        let properties = properties
            .iter()
            .map(|(prop_name, prop_schema)| {
                let type_name = prop_schema
                    .r#ref
                    .as_ref()
                    .and_then(|key| key.split(SCHEMA_REF_PREFIX).nth(1))
                    .unwrap_or_default()
                    .to_string();
                let binding =
                    get_schema_binding(type_name.clone(), prop_schema, spec, cache, trace);
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
                let prop_type = if !type_name.is_empty() {
                    if type_name == binding.get_name() || matches!(binding, Binding::Basic(_)) {
                        prop_type
                    } else if is_required {
                        codegen::Type::Named(type_name)
                    } else {
                        codegen::Type::Option(Box::new(codegen::Type::Named(type_name)))
                    }
                } else {
                    prop_type
                };
                codegen::Property::of(prop_name.to_string(), prop_type)
            })
            .flat_map(unfold_property)
            .collect();
        trace.pop().unwrap_or_default();
        let binding = Binding::Struct(codegen::Struct::of(name.clone(), properties));
        if !name.is_empty() {
            cache.insert(name, binding.clone());
        }
        return binding;
    }
    if let Some(all) = schema.allOf.as_ref() {
        let bindings = all
            .iter()
            .map(|schema| get_schema_binding(String::default(), schema, spec, cache, trace))
            .collect::<Vec<_>>();
        let binding = all_of(name.clone(), bindings);
        cache.insert(name, binding.clone());
        trace.pop().unwrap_or_default();
        return binding;
    }
    if let Some(one) = schema.oneOf.as_ref() {
        let bindings = one
            .iter()
            .map(|schema| {
                let name = schema.title.clone().unwrap_or_default();
                get_schema_binding(name.clone(), schema, spec, cache, trace)
            })
            .collect::<Vec<_>>();
        let name = if !name.is_empty() {
            name
        } else {
            schema.title.clone().unwrap_or_default()
        };
        if name.is_empty() {
            panic!("Anonymous enum detected (schema.title missing):\n{one:#?}");
        }
        let binding = one_of(name.clone(), bindings);
        cache.insert(name, binding.clone());
        trace.pop().unwrap_or_default();
        return binding;
    }
    if let Some(schema) = schema.schema.as_ref() {
        // TODO describe exact reasoning & use-case for this clause (might be deprecated corner-case)
        trace.pop().unwrap_or_default();
        return get_schema_binding(name, schema, spec, cache, trace);
    }
    if schema.has_type("null") {
        cache.insert("Null".to_string(), Binding::Basic(codegen::Basic::Null));
        trace.pop().unwrap_or_default();
        return Binding::Basic(codegen::Basic::Null);
    }
    eprintln!("unreachable: name={name} schema={schema:#?}");
    trace.pop().unwrap_or_default();
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
    cache: &mut Cache,
    trace: &mut Vec<String>,
) -> Option<Contract> {
    let method = spec.methods.iter().find(|m| m.name == name)?;
    let params = method
        .params
        .iter()
        .map(|param| {
            let param = if let Some(key) = param._ref.as_ref() {
                let id = key.split('/').last().unwrap();
                spec.get_content(id).unwrap()
            } else {
                param
            };
            let schema = param.schema.as_ref().unwrap();
            let name = param.name.clone().unwrap_or_default();
            let name = renders::normalize_prop_name(&name).expect("name");
            let type_name = schema
                .r#ref
                .as_ref()
                .and_then(|key| key.split(SCHEMA_REF_PREFIX).nth(1))
                .unwrap_or(&name)
                .to_string();
            let binding = get_schema_binding(type_name.clone(), schema, spec, cache, trace);
            cache.insert(binding.get_name(), binding.clone());
            let is_required = param.required.unwrap_or_default();
            let param_type = if is_required {
                binding.get_type()
            } else {
                codegen::Type::Option(Box::new(binding.get_type()))
            };
            let param_type =
                if type_name == binding.get_name() || matches!(binding, Binding::Basic(_)) {
                    param_type
                } else {
                    codegen::Type::Named(type_name)
                };
            (name, param_type)
        })
        .collect();
    let result = method.result.as_ref().map(|result| {
        let result = if let Some(key) = result._ref.as_ref() {
            let id = key.split('/').last().unwrap();
            spec.get_content(id).unwrap()
        } else {
            &result
        };
        let schema = result.schema.as_ref().unwrap();
        let name = result.name.clone().unwrap_or_default();
        let name = format!("{}_{name}", method.name);
        // TODO FIXME: starknet-specific processing (strip the common prefix)
        let name = name.strip_prefix("starknet_").unwrap_or(&name).to_string();
        let binding = get_schema_binding(name.clone(), schema, spec, cache, trace);
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
                .map(|error| match error {
                    openrpc::ErrorOrRef::Err(error) => {
                        cache.errors.insert(key.to_owned(), error.to_owned());
                        error.to_owned()
                    }
                    openrpc::ErrorOrRef::Ref { key } => {
                        let key = key.split(ERROR_REF_PREFIX).nth(1).unwrap();
                        cache.errors.get(key).unwrap().to_owned()
                    }
                })
        })
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

pub fn extract_contracts(
    spec: &openrpc::OpenRpc,
    cache: &mut Cache,
    trace: &mut Vec<String>,
) -> Vec<Contract> {
    let bindings = spec
        .components
        .as_ref()
        .expect("components")
        .schemas
        .iter()
        .map(|(name, schema)| {
            let binding = get_schema_binding(name.to_string(), schema, spec, cache, trace);
            cache.insert(name.clone(), binding.clone());
            binding
        })
        .flat_map(unfold_binding)
        // Make a second pass (extra unfolding might be necessary)
        .flat_map(unfold_binding)
        .collect::<Vec<_>>();

    for b in bindings {
        cache.overwrite(b.get_name(), b);
    }

    let contracts = spec
        .methods
        .iter()
        .filter_map(|method| {
            let name = method.name.clone();
            get_method_contract(name, spec, cache, trace)
        })
        .collect::<Vec<_>>();

    contracts
}
