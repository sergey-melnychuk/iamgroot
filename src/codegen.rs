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
    pub flatten: bool,
}

impl Property {
    pub fn of(r#type: Type) -> Self {
        Self {
            name: Default::default(),
            r#type,
            flatten: false,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Struct {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variant {
    Const { name: String, value: String },
    Struct(Struct),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Struct(Struct),
    Enum(Enum),
    Type(Type),
    Alias(String, Type),
}

impl Object {
    pub fn with_name(self, name: String) -> Self {
        match self {
            Self::Struct(this) => Self::Struct(Struct { name, ..this }),
            Self::Enum(this) => Self::Enum(Enum { name, ..this }),
            Self::Type(ty) => Self::Alias(name, ty),
            Self::Alias(_, ty) => Self::Alias(name, ty),
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Self::Struct(s) => &s.name,
            Self::Enum(e) => &e.name,
            Self::Type(_) => "",
            Self::Alias(name, _) => name,
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
