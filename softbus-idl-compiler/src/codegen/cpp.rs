//! C++代码生成器

use crate::parser::ServiceDef;
use crate::codegen::Codegen;
use std::path::Path;

/// C++代码生成器
pub struct CppCodegen;

impl CppCodegen {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CppCodegen {
    fn default() -> Self {
        Self::new()
    }
}

impl Codegen for CppCodegen {
    fn generate(&self, service: &ServiceDef, output_dir: &Path) -> anyhow::Result<()> {
        // TODO: 实现C++代码生成
        println!("Generating C++ code for service: {}", service.name);
        println!("Output directory: {:?}", output_dir);
        Ok(())
    }
}
