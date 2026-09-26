use core::{
  ffi::{c_char, c_void},
  ptr::copy_nonoverlapping,
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_common::{functions::c_slice::c_slice, macros::luau_assert::LUAU_ASSERT};

use crate::{functions::compile::compile, records::compile_options::CompileOptions};

/// C ABI 编译入口，对应 cpp `luau_compile`
/// （cpp/Compiler/include/luacode.h:74）：源码按原始字节编译为 bytecode blob，
/// 返回值由 `malloc` 分配（`malloc_copy`），所有权移交调用方、须以 `free` 释放。
///
/// # Safety
/// 本函数是真正的 C ABI 边界，调用方须保证：
/// - `source` 指向至少 `size` 字节可读内存；`size == 0` 时允许悬垂/空指针
///   （cpp `std::string(source, 0)` 语义）。
/// - `options` 允许 null；非空时须指向与 `#[repr(C)]` 的 `CompileOptions`
///   布局一致的可读结构体（含其内部 C 串/回调字段自身的合法性约定）。
/// - `outsize` 非空（入口 LUAU_ASSERT 断言）且指向可写 `usize`。
/// - 返回的非 null 缓冲区由调用方 `free`；null 表示分配失败。
pub unsafe fn luau_compile(
  source: *const c_char,
  size: usize,
  options: *mut CompileOptions,
  outsize: *mut usize,
) -> *mut c_char {
  LUAU_ASSERT!(!outsize.is_null());

  let opts = if options.is_null() {
    CompileOptions::default()
  } else {
    // Safety: 契约保证 options 非空时指向可读、可按值复制的 C ABI 选项结构。
    unsafe { *options }
  };

  // 同 cpp `std::string(source, size)`：源码按原始字节交给按字节工作的词法器，
  // 不要求 UTF-8（`"\xFF"` 这类字面量是合法源码）。
  // Safety: 契约保证 source 指向 size 字节可读内存；size==0 由 c_slice 归一为 &[]，
  // 且缓冲区在本函数结束前存活。
  let source_bytes = unsafe { c_slice(source.cast::<u8>(), size) };

  let bytecode = compile(source_bytes, &opts, &ParseOptions::default(), NoopEncoder);

  // 对应 cpp 的 `malloc + memcpy + *outsize`：分配失败即返回 null 且不写 outsize。
  // Safety: malloc_copy 的返回即 C 堆指针形态；outsize 写入受函数契约保护。
  unsafe {
    let copy = malloc_copy(&bytecode);
    if !copy.is_null() {
      // Safety: outsize 非空且可写（函数契约，入口已断言）。
      *outsize = bytecode.len();
    }
    copy
  }
}

/// 对应 cpp 的 `malloc(result.size()) + memcpy`，分配失败返回空指针。
///
/// # Safety
/// 返回的非 null 指针指向 `bytecode.len()` 字节的 C 堆内存，所有权移交调用方，
/// 须由 `free` 释放且不得越返回缓冲区边界读写。
unsafe fn malloc_copy(bytecode: &[u8]) -> *mut c_char {
  unsafe extern "C" {
    /// # Safety
    /// C 库 `malloc` 原样声明：任意 `size` 均合法；返回 null 表示分配失败，
    /// 非 null 时保证指向至少 `size` 字节可写内存，所有权移交调用方，须由
    /// `free` 释放且不得越返回缓冲区边界读写。
    unsafe fn malloc(size: usize) -> *mut c_void;
  }
  // Safety: C 库 malloc 原语，任意 size 合法。
  let copy = unsafe { malloc(bytecode.len()) }.cast::<c_char>();
  if copy.is_null() {
    return copy;
  }

  // Safety: copy 指向 malloc 分配的至少 bytecode.len() 字节的内存。
  unsafe { copy_nonoverlapping(bytecode.as_ptr().cast::<c_char>(), copy, bytecode.len()) };
  copy
}
