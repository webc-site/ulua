use core::{
  ffi::{c_char, c_void},
  ptr::{copy_nonoverlapping, null_mut},
  slice::from_raw_parts,
  str::from_utf8_unchecked,
};
use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
  records::{compile_options::CompileOptions, lua_compile_options::LuaCompileOptions},
};

/// # Safety
/// `source` 必须指向至少 `size` 字节的合法内存；`options`/`outsize` 若非空必须可写。
pub unsafe fn luau_compile(
  source: *const c_char,
  size: usize,
  options: *mut LuaCompileOptions,
  outsize: *mut usize,
) -> *mut c_char {
  LUAU_ASSERT!(!outsize.is_null());

  let mut opts = CompileOptions::default();

  if !options.is_null() {
    // SAFETY: options 与 CompileOptions 布局一致（lua_CompileOptions 的
    // #[repr(C)] 移植），调用方保证非空指针指向可读的合法结构体。
    unsafe { copy_nonoverlapping(options as *const CompileOptions, &mut opts, 1) };
  }

  // SAFETY: source 指向 size 字节合法内存（函数契约），且内容为 UTF-8。
  let source_str = unsafe { from_raw_parts(source as *const u8, size) };
  let source_rust = unsafe { from_utf8_unchecked(source_str) }.to_string();

  let mut allocator = Allocator::new();
  let mut names = AstNameTable::new(&mut allocator);
  let parse_options = ParseOptions::default();
  let result = Parser::parse(
    source_rust.as_str(),
    source_rust.len(),
    &mut names,
    &mut allocator,
    parse_options,
  );

  let bytecode = if result.errors.is_empty() {
    match catch_unwind(AssertUnwindSafe(|| {
      let mut bcb = BytecodeBuilder::new(None);
      compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
        &mut bcb, &result, &mut names, &opts,
      );
      bcb.get_bytecode().clone()
    })) {
      Ok(bc) => bc,
      Err(_) => {
        let error = ":0: compilation failed";
        BytecodeBuilder::get_error(error)
      }
    }
  } else {
    let parse_error = &result.errors[0];
    let error = alloc::format!(
      ":{}: {}",
      parse_error.get_location().begin.line + 1,
      parse_error.what()
    );
    BytecodeBuilder::get_error(&error)
  };

  unsafe extern "C" {
    /// # Safety
    /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
    unsafe fn malloc(size: usize) -> *mut c_void;
  }
  // SAFETY: malloc 返回的字节缓冲区由调用方负责释放，与 C API 约定一致。
  let copy = unsafe { malloc(bytecode.len()) } as *mut c_char;
  if copy.is_null() {
    return null_mut();
  }

  // SAFETY: copy 指向 malloc 分配的至少 bytecode.len() 字节的内存。
  unsafe { copy_nonoverlapping(bytecode.as_ptr() as *const c_char, copy, bytecode.len()) };
  // SAFETY: outsize 非空（函数契约），指向可写的 usize。
  unsafe { *outsize = bytecode.len() };
  copy
}
