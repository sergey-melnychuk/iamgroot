use serde_json::json;
use std::collections::HashMap;

use crate::{
    bind_object,
    codegen::{Enum, Object, Primitive, Property, Rule, Struct, Type, Variant},
    openrpc::SchemaOrRef,
};

#[test]
fn test_simple_felt() {
    let json = json!({
        "FELT": {
            "type": "string",
            "title": "Field element",
            "description": "A field element. represented by at most 63 hex digits",
            "pattern": "^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$"
        }
    });
    let schemas: HashMap<String, SchemaOrRef> = serde_json::from_value(json).unwrap();

    let expected = Struct {
        name: "Felt".to_owned(),
        properties: vec![Property {
            name: Default::default(),
            r#type: Type::Primitive(
                Primitive::String,
                vec![Rule::Regex(
                    "^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$".to_owned(),
                )],
            ),
            visibility: crate::codegen::Visibility::Public,
            decorators: vec![],
            flatten: false,
        }],
        decorators: vec![],
        visibility: crate::codegen::Visibility::Public,
    };

    let felt = bind_object("FELT", &schemas).unwrap();
    assert_eq!(felt, Object::Struct(expected));
}

#[test]
fn test_simple_int() {
    let json = json!({
        "BLOCK_NUMBER": {
            "title": "Block number",
            "description": "The block's number (its height)",
            "type": "integer",
            "minimum": 0
        }
    });
    let schemas: HashMap<String, SchemaOrRef> = serde_json::from_value(json).unwrap();

    let expected = Struct {
        name: "BlockNumber".to_owned(),
        properties: vec![Property {
            name: Default::default(),
            r#type: Type::Primitive(Primitive::Integer, vec![Rule::Min(0)]),
            visibility: crate::codegen::Visibility::Public,
            decorators: vec![],
            flatten: false,
        }],
        decorators: vec![],
        visibility: crate::codegen::Visibility::Public,
    };

    let block_number = bind_object("BLOCK_NUMBER", &schemas).unwrap();
    assert_eq!(block_number, Object::Struct(expected));
}

#[test]
fn test_wrapped_felt() {
    let json = json!({
        "BLOCK_HASH": {
            "title": "Block hash",
            "$ref": "#/components/schemas/FELT"
        },
        "FELT": {
            "type": "string",
            "title": "Field element",
            "description": "A field element. represented by at most 63 hex digits",
            "pattern": "^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$"
        }
    });
    let schemas: HashMap<String, SchemaOrRef> = serde_json::from_value(json).unwrap();

    let expected = Struct {
        name: "BlockHash".to_owned(),
        properties: vec![Property {
            name: Default::default(),
            r#type: Type::Named("Felt".to_owned()),
            visibility: crate::codegen::Visibility::Public,
            decorators: vec![],
            flatten: false,
        }],
        decorators: vec![],
        visibility: crate::codegen::Visibility::Public,
    };

    let block_hash = bind_object("BLOCK_HASH", &schemas).unwrap();
    assert_eq!(block_hash, Object::Struct(expected));
}

#[test]
fn test_array_felt() {
    let json = json!({
        "LIST": {
            "type": "array",
            "items": {
                "$ref": "#/components/schemas/FELT"
            }
        },
        "FELT": {
            "type": "string",
            "title": "Field element",
            "description": "A field element. represented by at most 63 hex digits",
            "pattern": "^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$"
        }
    });
    let schemas: HashMap<String, SchemaOrRef> = serde_json::from_value(json).unwrap();

    let expected = Object::Alias(
        "List".to_owned(),
        Type::Array(Box::new(Type::Named("Felt".to_owned()))),
    );

    // alternative: value-object wrapper type (see `Object::with_name`).
    /*
    let expected = Object::Struct(Struct {
        name: "List".to_owned(),
        properties: vec![Property {
            name: Default::default(),
            r#type: Type::Array(Box::new(Type::Named("Felt".to_owned()))),
            visibility: crate::codegen::Visibility::Public,
            decorators: vec![],
            flatten: false,
        }],
        decorators: vec![],
        visibility: crate::codegen::Visibility::Public,
    });
    */

    let list = bind_object("LIST", &schemas).unwrap();
    assert_eq!(list, expected);
}

#[test]
fn test_simple_object() {
    let json = json!({
        "EVENT_CONTENT": {
            "title": "Event content",
            "description": "The content of an event",
            "type": "object",
            "properties": {
                "data": {
                    "type": "array",
                    "title": "Data",
                    "items": {
                        "$ref": "#/components/schemas/FELT"
                    }
                },
                "keys": {
                    "type": "array",
                    "title": "Keys",
                    "items": {
                        "$ref": "#/components/schemas/FELT"
                    }
                }
            },
            "required": [
                "keys",
                "data"
            ]
        },
        "FELT": {
            "type": "string",
            "title": "Field element",
            "description": "A field element. represented by at most 63 hex digits",
            "pattern": "^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$"
        }
    });
    let schemas: HashMap<String, SchemaOrRef> = serde_json::from_value(json).unwrap();

    let expected = Struct {
        name: "EventContent".to_owned(),
        properties: vec![
            Property {
                name: "data".to_owned(),
                r#type: Type::Array(Box::new(Type::Named("Felt".to_owned()))),
                visibility: crate::codegen::Visibility::Public,
                decorators: vec![],
                flatten: false,
            },
            Property {
                name: "keys".to_owned(),
                r#type: Type::Array(Box::new(Type::Named("Felt".to_owned()))),
                visibility: crate::codegen::Visibility::Public,
                decorators: vec![],
                flatten: false,
            },
        ],
        decorators: vec![],
        visibility: crate::codegen::Visibility::Public,
    };

    let event_content = bind_object("EVENT_CONTENT", &schemas).unwrap();
    assert_eq!(event_content, Object::Struct(expected));
}

#[test]
fn test_simple_enum() {
    let json = json!({
        "SIMULATION_FLAG": {
            "type": "string",
            "enum": [
                "SKIP_VALIDATE",
                "SKIP_FEE_CHARGE"
            ],
            "description": "lorep ipsum"
        }
    });
    let schemas: HashMap<String, SchemaOrRef> = serde_json::from_value(json).unwrap();

    let object = bind_object("SIMULATION_FLAG", &schemas).unwrap();
    let expected = Object::Enum(Enum {
        name: "SimulationFlag".to_owned(),
        variants: vec![
            Variant {
                name: "SkipValidate".to_owned(),
                value: "SKIP_VALIDATE".to_owned(),
            },
            Variant {
                name: "SkipFeeCharge".to_owned(),
                value: "SKIP_FEE_CHARGE".to_owned(),
            },
        ],
        decorators: vec![],
        visibility: crate::codegen::Visibility::Public,
    });
    assert_eq!(object, expected);
}
