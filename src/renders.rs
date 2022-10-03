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
            Self::Message(message) => write!(f, "Code renderer error: {}.", message),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
pub fn normalize_prop_name(name: &str) -> Result<String> {
    // TODO Add field name normalization (snake_case) and respective #[serde] annotations
    Ok(name.to_string())
}

pub fn normalize_type_name(name: &str) -> Result<String> {
    // TODO Add type name normalization (CamelCase)
    Ok(name.to_string())
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
        codegen::Type::Named(name) => Ok(name.clone()),
        codegen::Type::Basic(basic) => Ok(render_basic(basic)),
        codegen::Type::Array(boxed) => Ok(format!("Vec<{}>", render_type(boxed)?)),
        codegen::Type::Unit => Ok(String::default()),
        unexpected => Err(format!("Unexpected enum variant type: {:?}", unexpected).into()),
    }
}

pub fn render_object(name: &str, binding: &binding::Binding) -> Result<String> {
    let mut lines: Vec<String> = Vec::new();
    match binding {
        binding::Binding::Basic(basic) => {
            let ty = render_basic(basic);
            let name = normalize_type_name(name)?;
            if ty != name {
                lines.push(format!("pub type {} = {};", name, ty));
            }
        }
        binding::Binding::Named(name, ty) => {
            let ty = render_type(ty)?;
            let name = normalize_type_name(name)?;
            if ty != name {
                lines.push(format!("pub type {} = {};", name, ty));
            }
        }
        binding::Binding::Enum(the_enum) => {
            let mut seen = HashSet::new();
            lines.push(format!("pub enum {} {{", normalize_type_name(name)?));
            for variant in &the_enum.variants {
                let name = variant.name.to_uppercase();
                if seen.contains(&name) {
                    continue;
                }
                seen.insert(name.clone());
                let ty = render_type(&variant._type)?;
                let suffix = if ty.is_empty() {
                    "".to_string()
                } else {
                    format!("({})", ty)
                };
                lines.push(format!("  {}{},", name, suffix));
            }
            lines.push("}".to_string());
        }
        binding::Binding::Struct(the_struct) => {
            let mut seen = HashSet::new();
            lines.push(format!("pub struct {} {{", normalize_type_name(name)?));
            for property in &the_struct.properties {
                let name = property.name.to_uppercase();
                if seen.contains(&name) {
                    continue;
                }
                seen.insert(name.clone());
                lines.push(format!(
                    "  pub {}: {},",
                    name,
                    render_type(&property._type)?
                ));
            }
            lines.push("}".to_string());
        }
    }
    Ok(lines.join("\n"))
}

#[allow(dead_code)]
pub fn render_method(
    name: &str,
    _contract: &binding::Contract,
    _cache: &HashMap<String, binding::Binding>,
) -> String {
    // TODO impl
    // TODO Add random filled DTO generation (for further testing)?
    format!("// TODO: Method '{}' definition", name)
}

// TODO
// Add stop-words substitution for field names ("type", "struct", "enum" etc)
// Consider raw identifiers: https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html

// TODO
// Add #[serde] annotations on enum variants
// #[serde(untagged)]: https://serde.rs/enum-representations.html
