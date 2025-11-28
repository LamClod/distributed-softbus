//! 抽象语法树定义

use serde::{Serialize, Deserialize};

/// 服务定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDef {
    pub name: String,
    pub methods: Vec<MethodDef>,
}

/// 方法定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodDef {
    pub name: String,
    pub params: Vec<ParamDef>,
    pub return_type: TypeDef,
}

/// 参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDef {
    pub name: String,
    pub param_type: TypeDef,
}

/// 类型定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeDef {
    Void,
    Bool,
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
    String,
    Bytes,
    Custom(String),
    Array(Box<TypeDef>),
    Map(Box<TypeDef>, Box<TypeDef>),
}

impl TypeDef {
    /// 转换为Rust类型字符串
    pub fn to_rust_type(&self) -> String {
        match self {
            TypeDef::Void => "()".to_string(),
            TypeDef::Bool => "bool".to_string(),
            TypeDef::I32 => "i32".to_string(),
            TypeDef::I64 => "i64".to_string(),
            TypeDef::U32 => "u32".to_string(),
            TypeDef::U64 => "u64".to_string(),
            TypeDef::F32 => "f32".to_string(),
            TypeDef::F64 => "f64".to_string(),
            TypeDef::String => "String".to_string(),
            TypeDef::Bytes => "Vec<u8>".to_string(),
            TypeDef::Custom(name) => name.clone(),
            TypeDef::Array(inner) => format!("Vec<{}>", inner.to_rust_type()),
            TypeDef::Map(k, v) => format!("HashMap<{}, {}>", k.to_rust_type(), v.to_rust_type()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_def_to_rust() {
        assert_eq!(TypeDef::I32.to_rust_type(), "i32");
        assert_eq!(TypeDef::String.to_rust_type(), "String");
        assert_eq!(TypeDef::Array(Box::new(TypeDef::I32)).to_rust_type(), "Vec<i32>");
    }
}
