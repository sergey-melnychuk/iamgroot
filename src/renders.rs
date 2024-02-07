use std::collections::HashMap;

use crate::codegen;
use crate::openrpc;

#[derive(Clone, Debug)]
pub struct Error(String);

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self(message)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self(message.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Code renderer error: {}.", self.0)
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

pub fn render_primitive(basic: &codegen::Primitive) -> &str {
    match basic {
        codegen::Primitive::String => "String",
        codegen::Primitive::Integer => "i64",
        codegen::Primitive::Boolean => "bool",
        codegen::Primitive::Null => "Null",
    }
}

pub fn render_type(ty: &codegen::Type) -> Result<String> {
    match ty {
        codegen::Type::Named(name) => normalize_type_name(name),
        codegen::Type::Primitive(basic, _) => Ok(render_primitive(basic).to_string()),
        codegen::Type::Array(ty) => Ok(format!("Vec<{}>", render_type(ty)?)),
        codegen::Type::Option(ty) => Ok(format!("Option<{}>", render_type(ty)?)),
        codegen::Type::Unit => Ok("()".to_string()),
    }
}

pub fn render_method(method: &codegen::Method) -> String {
    let short_name = method
        .name
        .strip_prefix("starknet_")
        .unwrap_or(&method.name);

    let mut lines = vec![
        format!("/// {}", method.doc.clone().unwrap_or_default()),
        format!("fn {short_name}("),
        format!("    &self,"),
    ];

    for (name, ty) in &method.args {
        lines.push(format!("    {}: {},", name, render_type(ty).expect("type")));
    }

    let ret = render_type(&method.ret).expect("ret");
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
                    message: format!("`type_name` value {value} must be > {MIN}"),
                });
            }
            if value > MAX {
                return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("`type_name` value {value} must be < {MAX}"),
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
            if `type_name_uppercase`_REGEX.is_match(value) {
                Ok(Self(value.to_string()))
            } else {
                Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("`type_name` value does not match regex: {value}"),
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

pub fn render_method_handler(method: &codegen::Method) -> String {
    let short_name = method
        .name
        .strip_prefix("starknet_")
        .unwrap_or(&method.name);

    if method.args.is_empty() {
        return METHOD_HANDLER_NO_ARGUMENTS
            .replace("`method_name`", &method.name)
            .replace("`method_short_name`", short_name);
    }

    let params_names_only = method
        .args
        .iter()
        .map(|(name, _)| format!("{},", name))
        .collect::<Vec<_>>()
        .join("\n");

    let params_types_only = method
        .args
        .iter()
        .map(|(_, ty)| format!("{},", render_type(ty).expect("type")))
        .collect::<Vec<_>>()
        .join("\n");

    let params_names_with_types = method
        .args
        .iter()
        .map(|(name, ty)| format!("{}: {},", name, render_type(ty).expect("type")))
        .collect::<Vec<_>>()
        .join("\n");

    METHOD_HANDLER_FULL
        .replace("`arg_names`", &params_names_only)
        .replace("`arg_types`", &params_types_only)
        .replace("`arg_names_and_types`", &params_names_with_types)
        .replace("`method_name`", &method.name)
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

pub fn render_handle_function(methods: &[codegen::Method]) -> String {
    let lines = methods
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
        log::error!("{err:#?}");
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

pub fn render_client(methods: &[codegen::Method]) -> String {
    let methods = methods
        .iter()
        .map(render_client_method)
        .collect::<Vec<_>>()
        .join("\n");

    CLIENT_MOD_REQWEST_BLOCKING.replace("`client_methods`", &methods)
}

pub fn render_client_method(method: &codegen::Method) -> String {
    let short_name = method
        .name
        .strip_prefix("starknet_")
        .unwrap_or(&method.name);

    let return_type = render_type(&method.ret).expect("ret");

    if method.args.is_empty() {
        return CLIENT_METHOD_NO_ARGS_BLOCKING
            .replace("`method_name`", &method.name)
            .replace("`method_short_name`", short_name)
            .replace("`result_type`", &return_type);
    }

    let params_names_only = method
        .args
        .iter()
        .map(|(name, _)| format!("{},", name))
        .collect::<Vec<_>>()
        .join("\n");

    let params_names_with_types = method
        .args
        .iter()
        .map(|(name, ty)| format!("{}: {},", name, render_type(ty).expect("type")))
        .collect::<Vec<_>>()
        .join("\n");

    CLIENT_METHOD_REQWEST_BLOCKING
        .replace("`arg_names`", &params_names_only)
        .replace("`arg_names_and_types`", &params_names_with_types)
        .replace("`method_name`", &method.name)
        .replace("`method_short_name`", short_name)
        .replace("`result_type`", &return_type)
}
