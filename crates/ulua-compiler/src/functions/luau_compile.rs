use core::{
  ffi::{c_char, c_void},
  ptr::{copy_nonoverlapping, null_mut},
  slice::from_raw_parts,
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::compile::compile,
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

  // 同 cpp `std::string(source, size)`：源码按原始字节交给按字节工作的词法器，
  // 不要求 UTF-8（`"\xFF"` 这类字面量是合法源码）。
  // SAFETY: 契约保证 source 指向 size 字节可读内存；缓冲区在本函数结束前存活。
  let source_bytes = unsafe { get_source(source, size) };

  // SAFETY: outsize 非空（函数契约）。
  unsafe {
    copy_out(
      &compile(source_bytes, &opts, &ParseOptions::default(), NoopEncoder),
      outsize,
    )
  }
}

/// 把 C 缓冲区收成 `&[u8]`；空缓冲区允许悬垂指针（cpp 语义）。
///
/// # Safety
/// `source` 非空时必须指向至少 `size` 字节可读内存。
unsafe fn get_source<'a>(source: *const c_char, size: usize) -> &'a [u8] {
  if size == 0 {
    return &[];
  }
  // SAFETY: 调用方保证 source 非空（size>0 时）且可读 size 字节。
  unsafe { from_raw_parts(source as *const u8, size) }
}

/// 对应 cpp 的 `malloc(result.size()) + memcpy + *outsize`，失败返回空指针。
///
/// # Safety
/// `outsize` 必须非空且可写。
unsafe fn copy_out(bytecode: &[u8], outsize: *mut usize) -> *mut c_char {
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
