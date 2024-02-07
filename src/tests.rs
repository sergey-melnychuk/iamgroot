use serde_json::json;
use std::collections::HashMap;

use crate::{
    bind_schema,
    codegen::{Object, Primitive, Property, Rule, Struct, Type},
    openrpc::SchemaOrRef,
};

#[cfg(test)]
mod tests {
    use super::*;

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

        let felt = bind_schema("FELT", &schemas).unwrap();
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

        let block_number = bind_schema("BLOCK_NUMBER", &schemas).unwrap();
        assert_eq!(block_number, Object::Struct(expected));
    }
}
