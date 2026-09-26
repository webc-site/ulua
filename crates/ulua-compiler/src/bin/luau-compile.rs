//! 差分 oracle 编译器驱动：读取 `.lua` 源文件，用 *Rust* 编译器编译为
//! Luau 字节码（luau_compile，null options → optimization_level=1/debug_level=1，
//! 与 C++ `luau-compile --binary` 相同），并把原始字节码写到 stdout。
//! 这样可运行完整 Rust 栈（Rust 编译 → Rust VM），与 C++ 编译→运行的
//! oracle 做差分，覆盖 parser/compiler——含 Luau 类型标注语法。

use std::{
  env::args,
  fs::File,
  io::{self, Read, Write},
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_common::set_luau_bool_flags;
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};

// 所有失败路径（缺参数、文件不可读/不可写、stdout 写入失败）
// 都以 `Err` 传播，由 `main` 返回的 `Result` 统一转成退出码，不再 panic。
fn main() -> io::Result<()> {
  let Some(path) = args().nth(1) else {
    return Err(io::Error::other("usage: luau_compile <lua-file>"));
  };

  let mut src = Vec::new();
  File::open(&path)?.read_to_end(&mut src)?;

  // 对齐 CLI 的 flag 状态，使编译决策与 oracle 一致
  set_luau_bool_flags(true);

  let bytecode = compile(
    &src,
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );
  io::stdout().write_all(&bytecode)
}
