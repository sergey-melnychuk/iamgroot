use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Response {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
}

impl Response {
    pub fn result(value: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(value),
            error: None,
            id: None,
        }
    }

    pub fn error(code: i64, message: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(Error {
                code,
                message: message.to_string(),
            }),
            id: None,
        }
    }

    pub fn with_id(self, id: Id) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Id {
    Number(i64),
    String(String),
}

// EXAMPLE BELOW

#[derive(Deserialize, Serialize)]
struct Ret(i32);

trait Rpc {
    fn wtf(&self, flag: bool, x: i32, y: f32) -> Result<Ret, Error>;
}

fn handle<RPC: Rpc>(rpc: &RPC, req: Request) -> Response {
    let params = if let Some(params) = req.params {
        params
    } else {
        return Response::error(1001, "Required field is missing: 'params'");
    };

    let response = match req.method.as_str() {
        "starknet_wtf" => handle_starknet_wtf(rpc, &params),
        _ => Response::error(1004, &format!("No such method: '{}'.", req.method)),
    };

    return if let Some(id) = req.id {
        response.with_id(id)
    } else {
        response
    };
}

fn handle_starknet_wtf<RPC: Rpc>(rpc: &RPC, params: &Value) -> Response {
    #[derive(Deserialize, Serialize)]
    struct ArgByPos(bool, i32, f32);

    #[derive(Deserialize, Serialize)]
    struct ArgByName {
        flag: bool,
        x: i32,
        y: f32,
    }

    let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
        serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
            let ArgByPos(flag, x, y) = args_by_pos;
            ArgByName { flag, x, y }
        })
    });

    let args: ArgByName = match args {
        Ok(args) => args,
        Err(e) => return Response::error(1002, &format!("{e:?}")),
    };

    match rpc.wtf(args.flag, args.x, args.y) {
        Ok(ret) => match serde_json::to_value(ret) {
            Ok(ret) => Response::result(ret),
            Err(e) => Response::error(1003, &format!("{e:?}")),
        },
        Err(e) => Response::error(e.code, &e.message),
    }
}
