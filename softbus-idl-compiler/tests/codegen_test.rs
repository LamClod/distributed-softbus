//! 代码生成测试

use softbus_idl_compiler::parser::*;
use softbus_idl_compiler::codegen::*;
use tempfile::TempDir;

#[test]
fn test_rust_codegen() {
    let service = ServiceDef {
        name: "TestService".to_string(),
        methods: vec![],
    };

    let temp_dir = TempDir::new().unwrap();
    let codegen = RustCodegen::new();
    
    let result = codegen.generate(&service, temp_dir.path());
    assert!(result.is_ok());
}
