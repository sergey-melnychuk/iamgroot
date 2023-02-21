use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<Id>,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    pub jsonrpc: String,
    pub result: Option<Value>,
    pub error: Option<Error>,
    pub id: Option<Id>,
}

#[derive(Deserialize, Serialize)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Id {
    Number(i64),
    String(String),
}

pub trait JsonRpc {
    fn serve(&self, req: Request) -> Response;
}
