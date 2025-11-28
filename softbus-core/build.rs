// softbus-core/build.rs

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 编译Protobuf文件
    let proto_files = &[
        "proto/rpc.proto",
        "proto/service.proto",
    ];
    
    let proto_include = &["proto"];
    
    prost_build::Config::new()
        .out_dir("src/generated")
        .compile_protos(proto_files, proto_include)?;
    
    // 重新编译触发条件
    for file in proto_files {
        println!("cargo:rerun-if-changed={}", file);
    }
    
    Ok(())
}
