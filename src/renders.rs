use std::collections::HashMap;
use std::collections::HashSet;

use crate::binding;
use crate::codegen;
use crate::openrpc;

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
    let name = name.replace([' ', '-'], "_");
    Ok(name.to_ascii_lowercase())
}

pub fn normalize_type_name(name: &str) -> Result<String> {
    if name == "i64" {
        return Ok(name.to_owned());
    }
    let name = name.replace([' ', '-'], "_");
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
        codegen::Basic::Null => "Null",
    };
    ty.to_string()
}

pub fn render_type(ty: &codegen::Type) -> Result<String> {
    match ty {
        codegen::Type::Named(name) => normalize_type_name(name),
        codegen::Type::Basic(basic, _) => Ok(render_basic(basic)),
        codegen::Type::Array(ty) => Ok(format!("Vec<{}>", render_type(ty)?)),
        codegen::Type::Option(ty) => Ok(format!("Option<{}>", render_type(ty)?)),
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
            if matches!(basic, codegen::Basic::Null) {
                lines.push("#[derive(Debug, Deserialize, Serialize)]".to_string());
                lines.push("pub struct Null;".to_string());
            } else if ty != name {
                // Keep type aliases just for the reference
                lines.push(format!("// pub type {name} = {ty};"));
            }
        }
        binding::Binding::Named(binding_name, ty) => {
            let name = normalize_type_name(name)?;
            let binding_name = normalize_type_name(binding_name)?;
            let rules = match ty {
                codegen::Type::Basic(_, rules) => rules.clone(),
                _ => Default::default(),
            };
            let ty_is_basic = matches!(ty, codegen::Type::Basic(_, _));
            let ty = normalize_type_name(&render_type(ty)?)?;
            lines.push("#[derive(Debug, Deserialize, Serialize)]".to_string());
            if name == binding_name {
                lines.push(format!(
                    "pub struct {name}(pub {ty}); // name == binding_name"
                ));
            } else {
                lines.push(format!(
                    "pub struct {name}(pub {binding_name}); // name != binding_name"
                ));
            }

            // Add regex-based validation impl enclosed in a module
            if name == binding_name && ty_is_basic && rules.pattern.is_some() {
                *lines.last_mut().unwrap() = format!("// {}", lines.last().unwrap());

                lines.push("#[serde(try_from = \"String\")]".to_string());
                lines.push(format!("pub struct {name}({ty});"));

                let pattern = rules.pattern.as_ref().unwrap().replace("\\", "\\\\");

                let code = PATTERN_VALIDATION_IMPL
                    .replace("`type_name`", &name)
                    .replace("`type_name_uppercase`", &name.to_ascii_uppercase())
                    .replace("`type_name_lowercase`", &name.to_ascii_lowercase())
                    .replace("`pattern`", &pattern);
                lines.push(code);
            } else if name == binding_name
                && ty_is_basic
                && (rules.min.is_some() || rules.max.is_some())
            {
                *lines.last_mut().unwrap() = format!("// {}", lines.last().unwrap());

                lines.push("#[serde(try_from = \"i64\")]".to_string());
                lines.push(format!("pub struct {name}({ty});"));

                let code = RANGE_VALIDATION_IMPL
                    .replace("`type_name`", &name)
                    .replace("`type_name_lowercase`", &name.to_ascii_lowercase())
                    .replace("`min`", &format!("{}", rules.min.unwrap_or(i64::MIN)))
                    .replace("`max`", &format!("{}", rules.max.unwrap_or(i64::MAX)));
                lines.push(code);
            }
        }
        binding::Binding::Enum(the_enum) => {
            let mut seen = HashSet::new();
            lines.push("#[derive(Debug, Deserialize, Serialize)]".to_string());
            let all_units = the_enum
                .variants
                .iter()
                .all(|variant| variant.r#type == codegen::Type::Unit);
            if !all_units {
                lines.push("#[serde(untagged)]".to_string());
            }
            lines.push(format!("pub enum {} {{", normalize_type_name(name)?));

            let mut ordered = the_enum.variants.iter().collect::<Vec<_>>();
            ordered.sort_by_key(|v| v.name.to_ascii_lowercase());

            for variant in ordered {
                let name = normalize_type_name(&variant.name)?;
                if seen.contains(&name) {
                    continue;
                }
                seen.insert(name.clone());
                if all_units {
                    lines.push(format!("#[serde(rename = \"{}\")]", variant.name));
                }
                match &variant.r#type {
                    codegen::Type::Struct(props) => {
                        lines.push(format!("  {name}{{"));
                        for (prop_name, prop_type) in props {
                            let prop_type = render_type(prop_type)?;
                            lines.push(format!("  {prop_name}: {prop_type},"));
                        }
                        lines.push("  },".to_string());
                    }
                    _ => {
                        let ty = render_type(&variant.r#type)?;
                        let suffix = if ty.is_empty() {
                            "".to_string()
                        } else {
                            format!("({ty})")
                        };
                        lines.push(format!("  {name}{suffix},"));
                    }
                }
            }
            lines.push("}".to_string());
        }
        binding::Binding::Struct(the_struct) => {
            let mut seen = HashSet::new();
            lines.push("#[derive(Debug, Deserialize, Serialize)]".to_string());
            lines.push(format!("pub struct {} {{", normalize_type_name(name)?));

            let mut ordered = the_struct.properties.iter().collect::<Vec<_>>();
            ordered.sort_by_key(|v| v.name.to_ascii_lowercase());

            for property in ordered {
                let name = normalize_prop_name(&property.name)?;
                if seen.contains(&name) {
                    continue;
                }
                seen.insert(name.clone());

                if matches!(property.r#type, codegen::Type::Option(_)) {
                    lines.push("  #[serde(default)]".to_string());
                    lines.push("  #[serde(skip_serializing_if = \"Option::is_none\")]".to_string());
                }
                if property.flatten {
                    lines.push("  #[serde(flatten)]".to_string());
                }
                if property.name != name && !property.flatten {
                    lines.push(format!("  #[serde(rename = \"{}\")]", property.name));
                }
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
        format!("fn {short_name}("),
        format!("    &self,"),
    ];

    for (name, ty) in &contract.params {
        lines.push(format!("    {}: {},", name, render_type(ty).expect("type")));
    }

    let ret = contract
        .result
        .as_ref()
        .map(|binding| render_type(&binding.get_type()).unwrap_or_default())
        .unwrap_or_else(|| "()".to_string());

    lines.push(format!(") -> std::result::Result<{ret}, jsonrpc::Error>;"));
    lines.join("\n")
}

const RANGE_VALIDATION_IMPL: &str = r###"
mod `type_name_lowercase` {
    use super::jsonrpc;
    use super::`type_name`;
    
    static MIN: i64 = `min`;
    static MAX: i64 = `max`;

    impl `type_name` {
        pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
            if value < MIN {
                return Err(jsonrpc::Error {
                    code: 1001,
                    message: "`type_name` value is < min".to_string(),
                });
            }
            if value > MAX {
                return Err(jsonrpc::Error {
                    code: 1001,
                    message: "`type_name` value is > max".to_string(),
                });
            }
            Ok(Self(value))
        }
    }

    impl TryFrom<i64> for `type_name` {
        type Error = String;
        fn try_from(value: i64) -> Result<Self, Self::Error> {
            Self::try_new(value).map_err(|e| e.message)
        }
    }

    impl AsRef<i64> for `type_name` {
        fn as_ref(&self) -> &i64 {
            &self.0
        }
    }
}
"###;

const PATTERN_VALIDATION_IMPL: &str = r###"
mod `type_name_lowercase` {
    use super::jsonrpc;
    use super::`type_name`;
    use once_cell::sync::Lazy;
    use regex::Regex;

    static `type_name_uppercase`_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new("`pattern`").unwrap());

    impl `type_name` {
        pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
            if `type_name_uppercase`_REGEX.is_match(&value) {
                Ok(Self(value.to_string()))
            } else {
                Err(jsonrpc::Error {
                    code: 1001,
                    message: "`type_name` value does not match regex".to_string(),
                })
            }
        }
    }

    impl TryFrom<String> for `type_name` {
        type Error = String;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            Self::try_new(&value).map_err(|e| e.message)
        }
    }

    impl AsRef<String> for `type_name` {
        fn as_ref(&self) -> &String {
            &self.0
        }
    }
}
"###;

const IMPL_INTO_ERROR: &str = r###"
pub struct Error(i64, &'static str);

impl From<Error> for super::jsonrpc::Error {
    fn from(Error(code, message): Error) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}
"###;

fn render_error(name: &str, error: &openrpc::Error) -> String {
    format!(
        "pub const {}: Error = Error({}, \"{}\");",
        name, error.code, error.message
    )
}

pub fn render_errors(errors: HashMap<String, openrpc::Error>) -> String {
    let mut target = String::new();
    use std::fmt::Write;

    writeln!(target, "pub mod error {{").unwrap();

    let mut ordered = errors.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|e| e.0);

    for (name, error) in ordered {
        writeln!(target, "{}", render_error(name, error)).unwrap();
    }
    writeln!(target, "{IMPL_INTO_ERROR}").unwrap();
    writeln!(target, "}}").unwrap();
    target
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
        Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
    };

    let ArgByName { 
`arg_names` 
    } = args;

    match rpc.`method_short_name`(
`arg_names`
    ) {
        Ok(ret) => match serde_json::to_value(ret) {
            Ok(ret) => jsonrpc::Response::result(ret),
            Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
        },
        Err(e) => jsonrpc::Response::error(e.code, &e.message),
    }
}
"###;

const METHOD_HANDLER_NO_ARGUMENTS: &str = r###"
fn handle_`method_name`<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
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
pub fn handle<RPC: Rpc>(rpc: &RPC, req: &jsonrpc::Request) -> jsonrpc::Response {
    let params = &req.params.clone().unwrap_or_default();

    let response = match req.method.as_str() {
`handlers`
        _ => jsonrpc::Response::error(-32601, "Method not found"),
    };

    return if let Some(id) = req.id.as_ref() {
        response.with_id(id.clone())
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
                "        \"{}\" => handle_{}(rpc, params),",
                contract.name, contract.name
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    HANDLE_FUNCTION.replace("`handlers`", &lines)
}

const CLIENT_MOD_REQWEST_BLOCKING: &str = r###"
pub mod client {
    use super::*;

    pub struct Client {
        client: reqwest::blocking::Client,
        url: String,
    }

    impl Client {
        pub fn new(url: &str) -> Self {
            Self {
                url: url.to_string(),
                client: reqwest::blocking::Client::new(),
            }
        }
    }

    impl super::Rpc for Client {
`client_methods`
    }
}
"###;

const CLIENT_METHOD_REQWEST_BLOCKING: &str = r###"
fn `method_short_name`(
    &self,
`arg_names_and_types`
) -> std::result::Result<`result_type`, jsonrpc::Error> {

    let args = (
`arg_names`
    );

    let params: serde_json::Value = serde_json::to_value(args)
        .map_err(|e| jsonrpc::Error::new(4001, format!("Invalid params: {e}.")))?;
    let req = jsonrpc::Request::new("`method_name`".to_string(), params)
        .with_id(jsonrpc::Id::Number(1));

    log::debug!("{req:#?}");

    let mut res: jsonrpc::Response = self
        .client
        .post(&self.url)
        .json(&req)
        .send()
        .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
        .json()
        .map_err(|e| jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}.")))?;

    if let Some(err) = res.error.take() {
        return Err(err);
    }

    if let Some(value) = res.result.take() {
        let out: `result_type` =
            serde_json::from_value(value).map_err(|e| {
                jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
            })?;

        log::debug!("{out:#?}");

        Ok(out)
    } else {
        Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
    }
}
"###;

const CLIENT_METHOD_NO_ARGS_BLOCKING: &str = r###"
fn `method_short_name`(&self) -> std::result::Result<`result_type`, jsonrpc::Error> {
    let req = jsonrpc::Request::new(
        "`method_name`".to_string(),
        serde_json::Value::Array(vec![]),
    )
    .with_id(jsonrpc::Id::Number(1));

    log::debug!("{req:#?}");

    let mut res: jsonrpc::Response = self
        .client
        .post(&self.url)
        .json(&req)
        .send()
        .map_err(|e| jsonrpc::Error::new(4002, format!("Request failed: {e}.")))?
        .json()
        .map_err(|e| jsonrpc::Error::new(5001, format!("Invalid response JSON: {e}.")))?;

    if let Some(err) = res.error.take() {
        return Err(err);
    }

    if let Some(value) = res.result.take() {
        let out: `result_type` = serde_json::from_value(value).map_err(|e| {
            jsonrpc::Error::new(5002, format!("Invalid response object: {e}."))
        })?;

        log::debug!("{out:#?}");

        Ok(out)
    } else {
        Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
    }
}
"###;

pub fn render_client(contracts: &[binding::Contract]) -> String {
    let methods = contracts
        .iter()
        .map(render_client_method)
        .collect::<Vec<_>>()
        .join("\n");

    CLIENT_MOD_REQWEST_BLOCKING.replace("`client_methods`", &methods)
}

pub fn render_client_method(contract: &binding::Contract) -> String {
    // TODO FIXME: starknet-specific processing (strip the common prefix)
    let short_name = contract
        .name
        .strip_prefix("starknet_")
        .unwrap_or(&contract.name);

    let return_type = contract
        .result
        .as_ref()
        .map(|b| b.get_type())
        .unwrap_or_default();
    let return_type = render_type(&return_type).expect("return type");

    if contract.params.is_empty() {
        return CLIENT_METHOD_NO_ARGS_BLOCKING
            .replace("`method_name`", &contract.name)
            .replace("`method_short_name`", short_name)
            .replace("`result_type`", &return_type);
    }

    let params_names_only = contract
        .params
        .iter()
        .map(|(name, _)| format!("{},", name))
        .collect::<Vec<_>>()
        .join("\n");

    let params_names_with_types = contract
        .params
        .iter()
        .map(|(name, ty)| format!("{}: {},", name, render_type(ty).expect("type")))
        .collect::<Vec<_>>()
        .join("\n");

    CLIENT_METHOD_REQWEST_BLOCKING
        .replace("`arg_names`", &params_names_only)
        .replace("`arg_names_and_types`", &params_names_with_types)
        .replace("`method_name`", &contract.name)
        .replace("`method_short_name`", short_name)
        .replace("`result_type`", &return_type)
}
