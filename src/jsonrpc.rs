use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
}

impl Request {
    pub fn new(method: String, params: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method,
            params: Some(params),
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

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

impl Error {
    pub fn new(code: i64, message: String) -> Self {
        Self { code, message }
    }
}

#[cfg(feature = "anyhow")]
impl From<Error> for anyhow::Error {
    fn from(value: Error) -> Self {
        anyhow::anyhow!(value)
    }
}

#[cfg(feature = "anyhow")]
impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self {
            code: 500,
            message: value.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Id {
    Number(i64),
    String(String),
}
