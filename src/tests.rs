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
    let schemas: HashMap<String, SchemaOrRef> =
        serde_json::from_value(json).unwrap();

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
        }],
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
    let schemas: HashMap<String, SchemaOrRef> =
        serde_json::from_value(json).unwrap();

    let expected = Struct {
        name: "BlockNumber".to_owned(),
        properties: vec![Property {
            name: Default::default(),
            r#type: Type::Primitive(Primitive::Integer, vec![Rule::Min(0)]),
        }],
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
    let schemas: HashMap<String, SchemaOrRef> =
        serde_json::from_value(json).unwrap();

    let expected = Struct {
        name: "BlockHash".to_owned(),
        properties: vec![Property {
            name: Default::default(),
            r#type: Type::Named("Felt".to_owned()),
        }],
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
    let schemas: HashMap<String, SchemaOrRef> =
        serde_json::from_value(json).unwrap();

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
        }],
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
    let schemas: HashMap<String, SchemaOrRef> =
        serde_json::from_value(json).unwrap();

    let expected = Struct {
        name: "EventContent".to_owned(),
        properties: vec![
            Property {
                name: "data".to_owned(),
                r#type: Type::Array(Box::new(Type::Named("Felt".to_owned()))),
            },
            Property {
                name: "keys".to_owned(),
                r#type: Type::Array(Box::new(Type::Named("Felt".to_owned()))),
            },
        ],
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
    let schemas: HashMap<String, SchemaOrRef> =
        serde_json::from_value(json).unwrap();

    let object = bind_object("SIMULATION_FLAG", &schemas).unwrap();
    let expected = Object::Enum(Enum {
        name: "SimulationFlag".to_owned(),
        variants: vec![
            Variant::Const {
                name: "SkipValidate".to_owned(),
                value: "SKIP_VALIDATE".to_owned(),
            },
            Variant::Const {
                name: "SkipFeeCharge".to_owned(),
                value: "SKIP_FEE_CHARGE".to_owned(),
            },
        ],
    });
    assert_eq!(object, expected);
}

mod event_filter {
    pub fn json() -> serde_json::Value {
        serde_json::json!({
            "EVENT_FILTER": {
                "title": "Event filter",
                "description": "An event filter/query",
                "type": "object",
                "properties": {
                    "from_block": {
                        "title": "from block",
                        "$ref": "#/components/schemas/BLOCK_ID"
                    },
                    "to_block": {
                        "title": "to block",
                        "$ref": "#/components/schemas/BLOCK_ID"
                    },
                    "address": {
                        "title": "from contract",
                        "$ref": "#/components/schemas/ADDRESS"
                    },
                    "keys": {
                        "title": "Keys",
                        "description": "The values used to filter the events",
                        "type": "array",
                        "items": {
                            "title": "Keys",
                            "description": "Per key (by position), designate the possible values to be matched for events to be returned. Empty array designates 'any' value",
                            "type": "array",
                            "items": {
                                "$ref": "#/components/schemas/FELT"
                            }
                        }
                    }
                },
                "required": []
            },
            "BLOCK_ID": {
                "title": "Block id",
                "description": "Block hash, number or tag",
                "oneOf": [
                    {
                        "title": "Block hash",
                        "type": "object",
                        "properties": {
                            "block_hash": {
                                "title": "Block hash",
                                "$ref": "#/components/schemas/BLOCK_HASH"
                            }
                        },
                        "required": [
                            "block_hash"
                        ]
                    },
                    {
                        "title": "Block number",
                        "type": "object",
                        "properties": {
                            "block_number": {
                                "title": "Block number",
                                "$ref": "#/components/schemas/BLOCK_NUMBER"
                            }
                        },
                        "required": [
                            "block_number"
                        ]
                    },
                    {
                        "title": "Block tag",
                        "$ref": "#/components/schemas/BLOCK_TAG"
                    }
                ]
            },
            "BLOCK_TAG": {
                "title": "Block tag",
                "type": "string",
                "description": "A tag specifying a dynamic reference to a block",
                "enum": [
                    "latest",
                    "pending"
                ]
            },
            "FELT": {
                "type": "string",
                "title": "Field element",
                "description": "A field element. represented by at most 63 hex digits",
                "pattern": "^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$"
            }
        })
    }
}

#[test]
fn test_one_of() {
    let schemas: HashMap<String, SchemaOrRef> =
        serde_json::from_value(event_filter::json()).unwrap();

    let expected = Object::Enum(Enum {
        name: "BlockId".to_owned(),
        variants: vec![
            Variant::Struct(Struct {
                name: "BlockNumber".to_owned(),
                properties: vec![Property {
                    name: "block_number".to_owned(),
                    r#type: Type::Named("BlockNumber".to_owned()),
                }],
            }),
            Variant::Struct(Struct {
                name: "BlockHash".to_owned(),
                properties: vec![Property {
                    name: "block_hash".to_owned(),
                    r#type: Type::Named("BlockHash".to_owned()),
                }],
            }),
            Variant::Struct(Struct {
                name: "BlockTag".to_owned(),
                properties: vec![Property {
                    name: "".to_owned(),
                    r#type: Type::Named("BlockTag".to_owned()),
                }],
            }),
        ],
    });

    assert_eq!(bind_object("BLOCK_ID", &schemas), Some(expected),);
}

#[test]
fn test_nested_array() {
    let schemas: HashMap<String, SchemaOrRef> =
        serde_json::from_value(event_filter::json()).unwrap();

    let expected = Object::Struct(Struct {
        name: "EventFilter".to_owned(),
        properties: vec![
            Property {
                name: "address".to_owned(),
                r#type: Type::Option(Box::new(Type::Named(
                    "Address".to_owned(),
                ))),
            },
            Property {
                name: "from_block".to_owned(),
                r#type: Type::Option(Box::new(Type::Named(
                    "BlockId".to_owned(),
                ))),
            },
            Property {
                name: "keys".to_owned(),
                r#type: Type::Option(Box::new(Type::Array(Box::new(
                    Type::Array(Box::new(Type::Named("Felt".to_owned()))),
                )))),
            },
            Property {
                name: "to_block".to_owned(),
                r#type: Type::Option(Box::new(Type::Named(
                    "BlockId".to_owned(),
                ))),
            },
        ],
    });

    assert_eq!(bind_object("EVENT_FILTER", &schemas), Some(expected),);
}
