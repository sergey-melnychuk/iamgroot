use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenRpc {
    pub openrpc: String,
    pub info: Value,
    pub servers: Vec<Server>,
    pub methods: Vec<Method>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
}

impl OpenRpc {
    pub fn get_schema(&self, id: &str) -> Option<&Schema> {
        let components = self.components.as_ref()?;
        components.schemas.get(id)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    pub name: String,
    pub url: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub variables: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reference {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Method {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paramStructure: Option<String>,
    pub params: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Reference>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Schema {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    // Supported types: string, boolean, integer, array, object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oneOf: Option<Vec<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allOf: Option<Vec<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Box<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "$comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _comment: Option<String>,
}

impl Schema {
    pub fn has_type(&self, expected: &str) -> bool {
        self.r#type
            .as_ref()
            .map(|s| s.as_str() == expected)
            .unwrap_or_default()
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Components {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentDescriptors: Option<Value>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub schemas: HashMap<String, Schema>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub errors: HashMap<String, Error>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Error {
    pub code: i64,
    pub message: String,
}
