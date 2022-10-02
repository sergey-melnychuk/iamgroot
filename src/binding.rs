use std::collections::HashMap;

use crate::codegen;
use crate::openrpc;

const REF_ID_PREFIX: &str = "#/components/schemas/";

#[derive(Debug, Clone)]
pub enum Binding {
    Basic(codegen::Basic),
    Struct(codegen::Struct),
    // Tuple(codegen::Tuple),
    Enum(codegen::Enum),
    Named(String, codegen::Type),
    //Empty,
}

impl Binding {
    pub fn get_name(&self) -> String {
        match self {
            Binding::Basic(basic) => basic.to_string(),
            Binding::Struct(s) => s.name.clone(),
            // Binding::Tuple(t) => t.name.clone(),
            Binding::Enum(e) => e.name.clone(),
            Binding::Named(name, _) => name.clone(),
            //Binding::Empty => "<empty>".to_string(),
        }
    }

    pub fn get_type(&self) -> codegen::Type {
        if !self.get_name().is_empty() {
            match self {
                Binding::Basic(basic) => codegen::Type::Basic(basic.clone()),
                Binding::Struct(s) => codegen::Type::Named(s.name.clone()),
                //Binding::Tuple(t) => codegen::Type::Named(t.name.clone()),
                Binding::Enum(e) => codegen::Type::Named(e.name.clone()),
                Binding::Named(_, t) => t.clone(),
                //Binding::Empty => codegen::Type::Unit,
            }
        } else {
            // anonymous types (not named, thus full structure must be represented)
            match self {
                Binding::Basic(basic) => codegen::Type::Basic(basic.clone()),
                Binding::Struct(s) => {
                    let props = s.properties
                        .iter()
                        .map(|p| (p.name.clone(), p._type.clone()))
                        .collect();
                    codegen::Type::Struct(props)
                }
                Binding::Enum(e) => {
                    let vars = e.variants
                        .iter()
                        .map(|p| (p.name.clone().to_ascii_uppercase(), p._type.clone()))
                        .collect();
                    codegen::Type::Enum(vars)
                },
                //Binding::Tuple(t) => codegen::Type::Tuple(t.types.clone()),
                Binding::Named(_, t) => t.clone(),
                //Binding::Empty => codegen::Type::Unit,
            }
        }
    }
}

fn deanonimize(binding: &Binding) -> String {
    let mut name = binding.get_name();
    if !name.is_empty() {
        return name;
    }
    match binding {
        Binding::Struct(s) => {
            if s.properties.len() == 1 {
                name = s.properties.first().unwrap().name.clone();
            }
        },
        _ => ()
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
                _type: b.get_type(),
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
    let variants = bindings.clone()
        .into_iter()
        .map(|b| {
            //eprintln!("one_of: name={} var_name={} var_type={:?}", name, b.get_name(), b.get_type());
            let name = deanonimize(&b).to_ascii_uppercase();
            let _type = match b.get_type() {
                codegen::Type::Struct(fields) 
                    if fields.len() == 1 && fields[0].0.to_ascii_uppercase() == name => 
                        // avoid creating unnecessary wrappers (single-property tuples)
                        fields[0].1.clone(),
                unchanged => unchanged
            };
            codegen::Variant {
                name,
                _type,
                ..Default::default()
            }
        })
        .collect();
    eprintln!("one_of={} bindings={:#?}\nvariants={:#?}", name, bindings, variants);
    Binding::Enum(codegen::Enum {
        name,
        variants,
        ..Default::default()
    })
}

pub fn unfold_property(property: codegen::Property) -> Vec<codegen::Property> {
    if !property.name.is_empty() {
        return match property._type {
            codegen::Type::Struct(fields) if fields.len() == 1 => {
                fields.into_iter()
                    .take(1)
                    .map(|(name, _type)| codegen::Property {
                        name: name.to_ascii_lowercase(),
                        _type,
                        ..Default::default()
                    })
                    .collect()
            }
            _ => vec![property]
        }    
    }

    match property._type {
        codegen::Type::Struct(fields) =>  {
            fields.into_iter()
                .map(|(name, _type)| codegen::Property {
                    name: name.to_ascii_lowercase(),
                    _type,
                    ..Default::default()
                })
                .collect()
        }
        codegen::Type::Enum(variants) => {
            variants.into_iter()
                .map(|(name, _type)| codegen::Property {
                    name: name.to_ascii_lowercase(),
                    _type,
                    ..Default::default()
                })
                .collect()
        }
        _ => vec![property]
    }
}

pub fn get_schema_binding(name: String, schema: &openrpc::Schema, spec: &openrpc::OpenRpc, cache: &mut HashMap<String, Binding>) -> Binding {
    eprintln!("\nname={}\nschema={:#?}", name, schema);
    if let Some(id) = &schema._ref {
        let id = id.strip_prefix(REF_ID_PREFIX).expect("ID prefix");
        if cache.contains_key(id) {
            return cache.get(id).cloned().expect("Cache hit");
        }
        let schema = spec.get_schema(id).expect("Schema lookup");
        let binding = get_schema_binding(id.to_string(), schema, spec, cache);
        cache.insert(id.to_string(), binding.clone());
        return binding;
    }
    if schema.has_type("string") {
        if let Some(values) = schema._enum.as_ref() {
            let variants = values.into_iter()
                .cloned()
                .map(|name| codegen::Variant {
                    name, 
                    _type: codegen::Type::Unit,
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
        let schema = schema.items.as_ref().expect("Item schema");
        let binding = get_schema_binding(name.clone(), schema, spec, cache);
        let item_type = Box::new(binding.get_type());
        return Binding::Named(name, codegen::Type::Array(item_type));
    }
    if schema.properties.is_some() /* assuming schema.type="object" */ {
        let properties = schema.properties.as_ref().expect("Object properties");
        let properties = properties.into_iter()
            .map(|(prop_name, prop_schema)| {
                let binding = get_schema_binding(String::default(), prop_schema, spec, cache);
                let prop_type = binding.get_type();
                eprintln!("name={} prop_name={} prop_type={:?} schema={:#?}", name, prop_name, prop_type, prop_schema);
                codegen::Property::of(prop_name.to_string(), prop_type)
            })
            .flat_map(unfold_property)
            .collect();
        return Binding::Struct(codegen::Struct::of(name, properties));
    }
    if let Some(all) = schema.allOf.as_ref() {
        let bindings = all.into_iter()
            .map(|schema| get_schema_binding(String::default(), schema, spec, cache))
            .collect::<Vec<_>>();
        return all_of(name, bindings);
    }
    if let Some(one) = schema.oneOf.as_ref() {
        let bindings = one.into_iter()
            .map(|schema| get_schema_binding(String::default(), schema, spec, cache))
            .collect::<Vec<_>>();
        return one_of(name, bindings);
    }
    if let Some(schema) = schema.schema.as_ref() {
        return get_schema_binding(name, schema, spec, cache);
    }
    unreachable!()
}

// TODO Extract anonymous structs ('\s+Struct\(' in ast.txt):
// - CONTRACT_CLASS.entry_points_by_type
// - STATE_UPDATE.state_diff
// - STATE_UPDATE."state_diff".nonces
// - CONTRACT_STORAGE_DIFF_ITEM.storage_entries

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_bindings() {
        // rigorous testing...
    }
}
