//! SoftBus IDL编译器
//! 
//! 将IDL定义编译为Rust/C#/C++代码

use clap::Parser;
use std::path::PathBuf;

mod parser;
mod codegen;

/// IDL编译器命令行参数
#[derive(Parser, Debug)]
#[command(name = "softbus-idl-compiler")]
#[command(about = "SoftBus IDL to code compiler", long_about = None)]
struct Args {
    /// IDL文件路径
    #[arg(short, long)]
    input: PathBuf,

    /// 输出目录
    #[arg(short, long)]
    output: PathBuf,

    /// 目标语言 (rust, csharp, cpp)
    #[arg(short, long, default_value = "rust")]
    lang: String,

    /// 详细输出
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("Input file: {:?}", args.input);
        println!("Output directory: {:?}", args.output);
        println!("Target language: {}", args.lang);
    }

    // TODO: 实现IDL编译逻辑
    println!("IDL compiler is not fully implemented yet.");
    println!("This is a placeholder for the IDL compilation functionality.");

    Ok(())
}
