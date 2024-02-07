#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    String,
    Integer,
    Boolean,
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rules {
    Regex(String),
    Min(i64),
    Max(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    Primitive(Primitive, Vec<Rules>),
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
    pub fn of(name: String, r#type: Type) -> Self {
        Self {
            name,
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

impl Struct {
    pub fn of(name: String, properties: Vec<Property>) -> Self {
        Self {
            name,
            properties,
            visibility: Default::default(),
            decorators: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<Variant>,
    pub decorators: Vec<String>,
    pub visibility: Visibility,
}

impl Enum {
    pub fn of(name: String, variants: Vec<Variant>) -> Self {
        Self {
            name,
            variants,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Struct(Struct),
    Enum(Enum),
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
