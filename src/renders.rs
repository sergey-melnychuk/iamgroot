use std::collections::HashMap;
use std::collections::HashSet;

use crate::binding;
use crate::codegen;

#[derive(Clone, Debug)]
pub enum Error {
    Message(String),
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::Message(message)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self::Message(message.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(message) => write!(f, "Code renderer error: {message}."),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

pub fn normalize_prop_name(name: &str) -> Result<String> {
    if "type" == name || "enum" == name || "struct" == name {
        return Ok(format!("r#{name}"));
    }
    Ok(name.to_ascii_lowercase())
}

pub fn normalize_type_name(name: &str) -> Result<String> {
    let chunks = name
        .split(|c| c == '_')
        .flat_map(capitalize)
        .collect::<Vec<_>>();

    Ok(chunks.join(""))
}

pub fn capitalize(s: &str) -> Result<String> {
    if s.is_empty() {
        return Err("cannot capitalize empty string".into());
    }
    if !s.is_ascii() {
        return Err("cannot capitalize non-ASCII string".into());
    }
    let mut chars: Vec<char> = s.to_ascii_lowercase().chars().collect();
    let head = chars[0].to_ascii_uppercase();
    *chars.get_mut(0).unwrap() = head;

    Ok(chars.into_iter().collect())
}

pub fn render_basic(basic: &codegen::Basic) -> String {
    let ty = match basic {
        codegen::Basic::String => "String",
        codegen::Basic::Integer => "i64",
        codegen::Basic::Boolean => "bool",
    };
    ty.to_string()
}

pub fn render_type(ty: &codegen::Type) -> Result<String> {
    match ty {
        codegen::Type::Named(name) => normalize_type_name(name),
        codegen::Type::Basic(basic) => Ok(render_basic(basic)),
        codegen::Type::Array(boxed) => Ok(format!("Vec<{}>", render_type(boxed)?)),
        codegen::Type::Unit => Ok(String::default()),
        unexpected => Err(format!("Unexpected enum variant type: {unexpected:?}").into()),
    }
}

pub fn render_object(name: &str, binding: &binding::Binding) -> Result<String> {
    let mut lines: Vec<String> = Vec::new();
    match binding {
        binding::Binding::Basic(basic) => {
            let ty = render_basic(basic);
            let name = normalize_type_name(name)?;
            if ty != name {
                lines.push(format!("pub type {name} = {ty};"));
            }
        }
        binding::Binding::Named(name, ty) => {
            let ty = render_type(ty)?;
            let name = normalize_type_name(name)?;
            if ty != name {
                lines.push(format!("pub type {name} = {ty};"));
            }
        }
        binding::Binding::Enum(the_enum) => {
            let mut seen = HashSet::new();
            lines.push(format!("pub enum {} {{", normalize_type_name(name)?));
            for variant in &the_enum.variants {
                let name = normalize_type_name(&variant.name)?;
                if seen.contains(&name) {
                    continue;
                }
                seen.insert(name.clone());
                let ty = render_type(&variant.r#type)?;
                let suffix = if ty.is_empty() {
                    "".to_string()
                } else {
                    format!("({ty})")
                };
                lines.push(format!("  {name}{suffix},"));
            }
            lines.push("}".to_string());
        }
        binding::Binding::Struct(the_struct) => {
            let mut seen = HashSet::new();
            lines.push(format!("pub struct {} {{", normalize_type_name(name)?));
            for property in &the_struct.properties {
                let name = normalize_prop_name(&property.name)?;
                if seen.contains(&name) {
                    continue;
                }
                seen.insert(name.clone());
                lines.push(format!(
                    "  pub {}: {},",
                    name,
                    render_type(&property.r#type)?
                ));
            }
            lines.push("}".to_string());
        }
    }
    Ok(lines.join("\n"))
}

pub fn render_method(
    name: &str,
    contract: &binding::Contract,
    cache: &HashMap<String, binding::Binding>,
) -> String {
    // TODO FIXME: starknet-specific processing (strip the common prefix)
    let short_name = name.strip_prefix("starknet_").unwrap_or(name);

    let mut lines = vec![
        format!("/// Method: '{name}'"),
        format!("/// Summary: {}", &contract.summary),
        format!("/// Description: {}", &contract.description),
        format!("///"),
        format!("fn {short_name} ("),
        format!("    &self,"),
    ];

    for (name, ty) in &contract.params {
        lines.push(format!(
            "    {}: {},",
            name,
            render_type(ty).expect("render type")
        ));
    }

    let mut extra_objects = Vec::new();

    let ret = if let Some(binding) = &contract.result {
        match binding {
            binding::Binding::Struct(st) if !cache.contains_key(&st.name) => {
                let object =
                    render_object(&binding.get_name(), binding).expect("render result object");
                extra_objects.push(object);
                st.name.clone()
            }
            binding::Binding::Enum(en) if !cache.contains_key(&en.name) => {
                let object =
                    render_object(&binding.get_name(), binding).expect("render result object");
                extra_objects.push(object);
                en.name.clone()
            }
            other => render_type(&other.get_type()).expect("render result type"),
        }
    } else {
        "()".to_string()
    };
    lines.push(format!(
        ") -> std::result::Result<{ret}, jsonrpc::Error> {{"
    ));
    lines.push("    todo!()".to_string());
    lines.push("}".to_string());

    vec![extra_objects.join("\n"), lines.join("\n")].join("")
}

// TODO: serde for DTOs
// Add #[serde] annotations on enum variants
// #[serde(untagged)]: https://serde.rs/enum-representations.html

// TODO: errors
// Wrap parameters into <method_name>Input struct
// Generate error subset enums per-method
// Generate return types as Result<Output, ErrorSubset>
