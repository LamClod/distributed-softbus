//! C#代码生成器

use crate::parser::ServiceDef;
use crate::codegen::Codegen;
use std::path::Path;

/// C#代码生成器
pub struct CSharpCodegen;

impl CSharpCodegen {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CSharpCodegen {
    fn default() -> Self {
        Self::new()
    }
}

impl Codegen for CSharpCodegen {
    fn generate(&self, service: &ServiceDef, output_dir: &Path) -> anyhow::Result<()> {
        // TODO: 实现C#代码生成
        println!("Generating C# code for service: {}", service.name);
        println!("Output directory: {:?}", output_dir);
        Ok(())
    }
}
