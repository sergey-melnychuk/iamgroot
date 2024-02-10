use crate::normalize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    String,
    Integer,
    Boolean,
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    Regex(String),
    Min(i64),
    Max(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    Primitive(Primitive, Vec<Rule>),
    Array(Box<Type>),
    Option(Box<Type>),
    Named(String),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Visibility {
    #[default]
    Public,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    pub name: String,
    pub r#type: Type,
    pub visibility: Visibility,
    pub decorators: Vec<String>,
    pub flatten: bool,
}

impl Property {
    pub fn of(r#type: Type) -> Self {
        Self {
            name: Default::default(),
            r#type,
            visibility: Visibility::Public,
            decorators: vec![],
            flatten: false,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Struct {
    pub name: String,
    pub properties: Vec<Property>,
    pub decorators: Vec<String>,
    pub visibility: Visibility,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<Variant>,
    pub decorators: Vec<String>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Struct(Struct),
    Enum(Enum),
    Type(Type),
    Alias(String, Type),
}

impl Object {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Struct(s) => Type::Named(s.name.to_owned()),
            Self::Enum(e) => Type::Named(e.name.to_owned()),
            Self::Type(ty) => ty.clone(),
            Self::Alias(_, ty) => ty.clone(),
        }
    }

    pub fn with_name(self, name: &str) -> Self {
        let name = normalize(name);
        match self {
            Self::Struct(this) => Self::Struct(Struct { name, ..this }),
            Self::Enum(this) => Self::Enum(Enum { name, ..this }),

            // alternative: extract value-object type wrapper
            Self::Type(ty) => Self::Alias(name, ty),

            // alternative: extract value-object type wrapper
            Self::Alias(_, ty) => Self::Alias(name, ty),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub doc: Option<String>,
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub ret: Type,
}

#[derive(Debug, Clone)]
pub struct Trait {
    pub name: String,
    pub visibility: Visibility,
    pub methods: Vec<Method>,
}
