use std::collections::HashMap;

use crate::codegen;
use crate::codegen::Type;
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

impl From<Error> for std::fmt::Error {
    fn from(e: Error) -> Self {
        Error(e.to_string()).into()
    }
}

type Result<T> = std::result::Result<T, Error>;

pub fn render_primitive(basic: &codegen::Primitive) -> &str {
    match basic {
        codegen::Primitive::String => "String",
        codegen::Primitive::Integer => "i64",
        codegen::Primitive::Boolean => "bool",
        codegen::Primitive::Null => "Null",
    }
}

pub fn render_type(ty: &codegen::Type) -> String {
    match ty {
        codegen::Type::Named(name) if name.is_empty() => {
            "(/* TODO: empty */)".to_owned()
        }
        codegen::Type::Named(name) => name.to_owned(),
        codegen::Type::Primitive(basic, _) => {
            render_primitive(basic).to_string()
        }
        codegen::Type::Array(ty) => format!("Vec<{}>", render_type(ty)),
        codegen::Type::Option(ty) => format!("Option<{}>", render_type(ty)),
        codegen::Type::Unit => "()".to_string(),
    }
}

pub fn render_object(object: &codegen::Object) -> Result<String> {
    static HEADER: &str = "#[derive(Clone, Debug, Deserialize, Serialize)]";
    static OPTION: &str =
        "    #[serde(skip_serializing_if = \"Option::is_none\")]\n    #[serde(default)]";

    let mut lines: Vec<String> = Vec::new();
    // lines.push(format!("/*\nDEBUG:\n{:#?}\n*/", object));
    match object {
        codegen::Object::Type(_) => (), // noop
        codegen::Object::Alias(name, ty) => {
            lines.push(format!("type {name} = {};\n", render_type(ty)));
        }
        codegen::Object::Struct(s) => {
            // TODO: regex validation for String
            // TODO: range validation for Integer
            lines.push(HEADER.to_owned());
            if s.properties.len() == 1 && s.properties[0].name.is_empty() {
                let ty = render_type(&s.properties[0].r#type);
                lines.push(format!("pub struct {}(pub {});\n", s.name, ty));
            } else {
                lines.push(format!("pub struct {} {{", s.name));
                s.properties.iter().for_each(|p| {
                    let ty = render_type(&p.r#type);
                    let required = matches!(&p.r#type, Type::Option(_));
                    if required {
                        lines.push(OPTION.to_owned());
                    }
                    lines.push(format!("    pub {}: {},", p.name, ty));
                });
                lines.push("}\n".to_owned());
            }
        }
        codegen::Object::Enum(e) => {
            lines.push(HEADER.to_owned());
            lines.push(format!("pub enum {} {{", e.name));
            e.variants.iter().for_each(|v| {
                lines.push(format!("    #[serde(rename = \"{}\")]", v.value));
                lines.push(format!("    {},", v.name));
            });
            lines.push("}\n".to_owned());
        }
    }
    Ok(lines.join("\n"))
}

pub fn render_method(method: &codegen::Method) -> String {
    let mut lines = Vec::new();

    if let Some(doc) = method.doc.as_ref().map(|doc| format!("/// {}", doc)) {
        lines.push(doc);
    }
    lines.push(format!("fn {}(\n    &self,", method.name));

    for (name, ty) in &method.args {
        lines.push(format!("    {}: {},", name, render_type(ty)));
    }

    let ret = render_type(&method.ret);
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

    match rpc.`method_name`(
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
    match rpc.`method_name`() {
        Ok(ret) => match serde_json::to_value(ret) {
            Ok(ret) => jsonrpc::Response::result(ret),
            Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
        },
        Err(e) => jsonrpc::Response::error(e.code, &e.message),
    }
}
"###;

pub fn render_method_handler(method: &codegen::Method) -> String {
    if method.args.is_empty() {
        return METHOD_HANDLER_NO_ARGUMENTS
            .replace("`method_name`", &method.name);
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
        .map(|(_, ty)| format!("{},", render_type(ty)))
        .collect::<Vec<_>>()
        .join("\n");

    let params_names_with_types = method
        .args
        .iter()
        .map(|(name, ty)| format!("{}: {},", name, render_type(ty)))
        .collect::<Vec<_>>()
        .join("\n");

    METHOD_HANDLER_FULL
        .replace("`arg_names`", &params_names_only)
        .replace("`arg_types`", &params_types_only)
        .replace("`arg_names_and_types`", &params_names_with_types)
        .replace("`method_name`", &method.name)
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
fn `method_name`(
    &self,
`arg_names_and_types`
) -> std::result::Result<`result_type`, jsonrpc::Error> {

    let args = (
`arg_names`
    );

    let params: serde_json::Value = serde_json::to_value(args)
        .map_err(|e| jsonrpc::Error::new(
            4001,
            format!("Invalid params: {e}."),
        ))?;
    let req = jsonrpc::Request::new(
            "`method_name`".to_string(), 
            params,
        )
        .with_id(jsonrpc::Id::Number(1));

    log::debug!("{req:#?}");

    let mut res: jsonrpc::Response = self
        .client
        .post(&self.url)
        .json(&req)
        .send()
        .map_err(|e| jsonrpc::Error::new(
            4002,
            format!("Request failed: {e}."),
        ))?
        .json()
        .map_err(|e| jsonrpc::Error::new(
            5001,
            format!("Invalid response JSON: {e}."),
        ))?;

    if let Some(err) = res.error.take() {
        log::error!("{err:#?}");
        return Err(err);
    }

    if let Some(value) = res.result.take() {
        let out: `result_type` =
            serde_json::from_value(value).map_err(|e| {
                jsonrpc::Error::new(
                    5002,
                    format!("Invalid response object: {e}."),
                )
            })?;

        log::debug!("{out:#?}");

        Ok(out)
    } else {
        Err(jsonrpc::Error::new(5003, "Response missing".to_string()))
    }
}
"###;

const CLIENT_METHOD_NO_ARGS_BLOCKING: &str = r###"
fn `method_name`(&self) -> std::result::Result<`result_type`, jsonrpc::Error> {
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
        let out: `result_type` = serde_json::from_value(value)
            .map_err(|e| {
                jsonrpc::Error::new(
                    5002,
                    format!("Invalid response object: {e}.")
                )
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
    let return_type = render_type(&method.ret);

    if method.args.is_empty() {
        return CLIENT_METHOD_NO_ARGS_BLOCKING
            .replace("`method_name`", &method.name)
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
        .map(|(name, ty)| format!("{}: {},", name, render_type(ty)))
        .collect::<Vec<_>>()
        .join("\n");

    CLIENT_METHOD_REQWEST_BLOCKING
        .replace("`arg_names`", &params_names_only)
        .replace("`arg_names_and_types`", &params_names_with_types)
        .replace("`method_name`", &method.name)
        .replace("`result_type`", &return_type)
}
