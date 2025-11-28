//! 代码生成模块

pub mod rust;
pub mod csharp;
pub mod cpp;

pub use rust::RustCodegen;
pub use csharp::CSharpCodegen;
pub use cpp::CppCodegen;

use crate::parser::ServiceDef;
use std::path::Path;

/// 代码生成器trait
pub trait Codegen {
    /// 生成代码
    fn generate(&self, service: &ServiceDef, output_dir: &Path) -> anyhow::Result<()>;
}
