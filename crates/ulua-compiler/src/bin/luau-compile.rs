//! Differential-oracle compiler driver: read a `.lua` source file, compile it
//! to Luau bytecode with the *Rust* compiler (luau_compile, null options →
//! optimization_level=1/debug_level=1, same as the C++ `luau-compile --binary`),
//! and write the raw bytecode bytes to stdout. Lets us run the FULL Rust stack
//! (Rust compile → Rust VM) and diff against the C++ compile→run oracle, which
//! exercises the parser/compiler — including Luau type-annotation syntax.

use core::{ffi::c_char, ptr::null_mut, slice::from_raw_parts};
use std::{
  env::args,
  fs::File,
  io::{self, Read, Write},
};

use ulua_common::set_all_flags;
use ulua_compiler::functions::luau_compile::luau_compile;

// 所有失败路径（缺参数、文件不可读/不可写、编译返回空指针、stdout 写入失败）
// 都以 `Err` 传播，由 `main` 返回的 `Result` 统一转成退出码，不再 panic。
fn main() -> io::Result<()> {
  let Some(path) = args().nth(1) else {
    return Err(io::Error::other("usage: luau_compile <lua-file>"));
  };

  let mut src = Vec::new();
  File::open(&path)?.read_to_end(&mut src)?;

  // mirror the CLI's flag state so compilation decisions match the oracle
  set_all_flags(true);

  // SAFETY: src 是合法 UTF-8 源码缓冲区；outsize 指向可写 usize。
  let (bc, outsize) = unsafe {
    let mut outsize: usize = 0;
    let bc = luau_compile(
      src.as_ptr() as *const c_char,
      src.len(),
      null_mut(),
      &mut outsize,
    );
    (bc, outsize)
  };
  if bc.is_null() {
    return Err(io::Error::other("luau_compile returned null"));
  }
  // SAFETY: luau_compile 返回 outsize 字节的 malloc 缓冲区。
  let bytes = unsafe { from_raw_parts(bc as *const u8, outsize) };
  io::stdout().write_all(bytes)
}
