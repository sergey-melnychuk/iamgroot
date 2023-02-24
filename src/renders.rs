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
    let all_caps = s.chars().all(|c| c.is_uppercase());

    let mut chars: Vec<char> = if all_caps {
        s.to_ascii_lowercase().chars().collect()
    } else {
        s.chars().collect()
    };
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
                lines.push("// binding::Basic".to_string());
                lines.push(format!("pub type {name} = {ty};"));
                // TODO wrap into value-object
                // lines.push("#[derive(Debug, Deserialize, Serialize)]".to_string());
                // lines.push(format!("pub struct {name}({ty});"));
            }
        }
        binding::Binding::Named(name, ty) => {
            let ty = render_type(ty)?;
            let name = normalize_type_name(name)?;
            if name != ty {
                lines.push("// binding::Named".to_string());
                lines.push("#[derive(Debug, Deserialize, Serialize)]".to_string());
                lines.push(format!("pub struct {name}({ty});"));
            }
        }
        binding::Binding::Enum(the_enum) => {
            let mut seen = HashSet::new();
            lines.push("#[derive(Debug, Deserialize, Serialize)]".to_string());
            lines.push("#[serde(untagged)]".to_string());
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
            lines.push("#[derive(Debug, Deserialize, Serialize)]".to_string());
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

pub fn render_method(name: &str, contract: &binding::Contract) -> String {
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
            render_type(ty).expect("type")
        ));
    }

    let ret = contract
        .result
        .as_ref()
        .map(|binding| render_type(&binding.get_type()).unwrap_or_default())
        .unwrap_or_else(|| "()".to_string());

    lines.push(format!(") -> std::result::Result<{ret}, jsonrpc::Error>;"));
    lines.join("\n")
}

const METHOD_HANDLER_FULL: &str = r###"
fn handle_`method_name`<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
    #[derive(Deserialize, Serialize)]
    struct ArgByPos(
`arg_types`
    );

    #[derive(Deserialize, Serialize)]
    struct ArgByName {
`arg_names_and_types`
    }

    let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
        serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
            let ArgByPos(
`arg_names`
            ) = args_by_pos;
            ArgByName { 
`arg_names` 
            }
        })
    });

    let args: ArgByName = match args {
        Ok(args) => args,
        Err(e) => return jsonrpc::Response::error(1002, &format!("{e:?}")),
    };

    let ArgByName { 
`arg_names` 
    } = args;

    match rpc.`method_short_name`(
`arg_names`
    ) {
        Ok(ret) => match serde_json::to_value(ret) {
            Ok(ret) => jsonrpc::Response::result(ret),
            Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
        },
        Err(e) => jsonrpc::Response::error(e.code, &e.message),
    }
}
"###;

const METHOD_HANDLER_NO_ARGUMENTS: &str = r###"
fn handle_`method_name`<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
    match rpc.`method_short_name`() {
        Ok(ret) => match serde_json::to_value(ret) {
            Ok(ret) => jsonrpc::Response::result(ret),
            Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
        },
        Err(e) => jsonrpc::Response::error(e.code, &e.message),
    }
}
"###;

pub fn render_method_handler(name: &str, contract: &binding::Contract) -> String {
    // TODO FIXME: starknet-specific processing (strip the common prefix)
    let short_name = name.strip_prefix("starknet_").unwrap_or(name);

    if contract.params.is_empty() {
        return METHOD_HANDLER_NO_ARGUMENTS
            .replace("`method_name`", name)
            .replace("`method_short_name`", short_name);
    }

    let params_names_only = contract
        .params
        .iter()
        .map(|(name, _)| format!("{},", name))
        .collect::<Vec<_>>()
        .join("\n");

    let params_types_only = contract
        .params
        .iter()
        .map(|(_, ty)| format!("{},", render_type(ty).expect("type")))
        .collect::<Vec<_>>()
        .join("\n");

    let params_names_with_types = contract
        .params
        .iter()
        .map(|(name, ty)| format!("{}: {},", name, render_type(ty).expect("type")))
        .collect::<Vec<_>>()
        .join("\n");

    METHOD_HANDLER_FULL
        .replace("`arg_names`", &params_names_only)
        .replace("`arg_types`", &params_types_only)
        .replace("`arg_names_and_types`", &params_names_with_types)
        .replace("`method_name`", name)
        .replace("`method_short_name`", short_name)
}

const HANDLE_FUNCTION: &str = r###"
pub fn handle<RPC: Rpc>(rpc: &RPC, req: jsonrpc::Request) -> jsonrpc::Response {
    let params = if let Some(params) = req.params {
        params
    } else {
        return jsonrpc::Response::error(1001, "Required field is missing: 'params'");
    };

    let response = match req.method.as_str() {
`handlers`
        _ => jsonrpc::Response::error(1004, &format!("No such method: '{}'.", req.method)),
    };

    return if let Some(id) = req.id {
        response.with_id(id)
    } else {
        response
    };
}
"###;

pub fn render_handle_function(contracts: &[binding::Contract]) -> String {
    let lines = contracts
        .iter()
        .map(|contract| {
            format!(
                "        \"{}\" => handle_{}(rpc, &params),",
                contract.name, contract.name
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    HANDLE_FUNCTION.replace("`handlers`", &lines)
}
