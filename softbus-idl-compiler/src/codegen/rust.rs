//! Rust代码生成器

use crate::parser::ServiceDef;
use crate::codegen::Codegen;
use std::path::Path;

/// Rust代码生成器
pub struct RustCodegen;

impl RustCodegen {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RustCodegen {
    fn default() -> Self {
        Self::new()
    }
}

impl Codegen for RustCodegen {
    fn generate(&self, service: &ServiceDef, output_dir: &Path) -> anyhow::Result<()> {
        // TODO: 实现Rust代码生成
        println!("Generating Rust code for service: {}", service.name);
        println!("Output directory: {:?}", output_dir);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_codegen_creation() {
        let _codegen = RustCodegen::new();
    }
}
