#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Dec,
    Txt,
    Bool,
    Array(Box<Type>),
    Struct(String),
    Void,
}

impl Type {
    pub fn from_keyword(s: &str) -> Option<Self> {
        match s {
            "int" => Some(Type::Int),
            "dec" => Some(Type::Dec),
            "txt" => Some(Type::Txt),
            "bool" => Some(Type::Bool),
            "void" => Some(Type::Void),
            _ => None,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Dec => write!(f, "dec"),
            Type::Txt => write!(f, "txt"),
            Type::Bool => write!(f, "bool"),
            Type::Array(inner) => write!(f, "array<{}>", inner),
            Type::Struct(name) => write!(f, "{}", name),
            Type::Void => write!(f, "void"),
        }
    }
}
