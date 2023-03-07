#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Basic {
    String,
    Integer,
    Boolean,
}

impl std::fmt::Display for Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Rules {
    pub pattern: Option<String>,
    pub min: Option<i64>,
    pub max: Option<i64>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Type {
    #[default]
    Unit,
    Basic(Basic, Rules),
    Array(Box<Type>),
    Option(Box<Type>),
    Struct(Vec<(String, Type)>),
    Enum(Vec<(String, Type)>),
    Named(String),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Visibility {
    #[default]
    Public,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
            ..Default::default()
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub out: Type,
}

#[derive(Debug, Clone)]
pub struct Trait {
    pub name: String,
    pub visibility: Visibility,
    pub methods: Vec<Method>,
}
