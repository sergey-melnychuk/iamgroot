#[derive(Debug, Clone)]
pub enum Basic {
    String,
    Integer,
    Boolean,
}

impl std::fmt::Display for Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Clone)]
pub enum Type {
    Basic(Basic),
    Array(Box<Type>),
    Struct(Vec<(String, Type)>),
    Enum(Vec<(String, Type)>),
    Named(String),
    #[default]
    Unit,
}

#[derive(Debug, Default, Clone)]
pub enum Visibility {
    #[default]
    Public,
    //Package,
    //Private,
}

#[derive(Debug, Default, Clone)]
pub struct Property {
    pub name: String,
    pub _type: Type,
    pub visibility: Visibility,
    pub decorators: Vec<String>,
}

impl Property {
    pub fn of(name: String, _type: Type) -> Self {
        Self {
            name, 
            _type,
            visibility: Default::default(),
            decorators: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
pub struct Variant {
    pub name: String,
    pub _type: Type,
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub _type: Type,
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
