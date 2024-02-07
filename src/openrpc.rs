use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Spec {
    pub openrpc: String,
    pub info: Value,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    pub methods: Vec<Method>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
}

impl Spec {
    pub fn get_schema(&self, id: &str) -> Option<&Schema> {
        self.components
            .as_ref()?
            .schemas
            .get(id)
            .and_then(|schema_or_ref| match schema_or_ref {
                SchemaOrRef::Schema(schema) => Some(schema),
                _ => None,
            })
    }

    pub fn get_content(&self, id: &str) -> Option<&Content> {
        self.components.as_ref()?.contentDescriptors.get(id)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    pub url: String,
    pub name: String,
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
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SchemaOrRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    String,
    Boolean,
    Integer,
    Array,
    Object,
    Null,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SchemaOrRef {
    Ref {
        #[serde(rename = "$ref")]
        r#ref: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    Schema(Schema),
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schema {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oneOf: Option<Vec<SchemaOrRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allOf: Option<Vec<SchemaOrRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<SchemaOrRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, SchemaOrRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Box<SchemaOrRef>>,
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
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Value>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Components {
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub contentDescriptors: HashMap<String, Content>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub schemas: HashMap<String, SchemaOrRef>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub errors: HashMap<String, ErrorOrRef>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ErrorOrRef {
    Ref {
        #[serde(rename = "$ref")]
        r#ref: String,
    },
    Err(Error),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Error {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Schema>,
}
