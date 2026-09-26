//! Test fixture: faithful port of `compileFunction` + `luauLibraryConstantLookup`
//! (tests/Compiler.test.cpp).
// Port of the test's `luauLibraryConstantLookup` callback. The compiler passes
// `&mut Constant as *mut CompileConstant` (CompileConstant = *mut ()), so the
// incoming `constant` pointer's bits ARE the `*mut Constant`; `as CompileConstant`
// recovers it for the `set_compile_constant_*` helpers.
use alloc::string::String;
use core::{ffi::c_char, ptr::null};

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::functions::c_str::cstr_cow;
use ulua_compiler::{
  functions::set_compile_constant::{
    set_compile_constant_boolean, set_compile_constant_nil, set_compile_constant_number,
    set_compile_constant_string, set_compile_constant_vector,
  },
  records::compile_options::CompileOptions,
  type_aliases::compile_constant::CompileConstant,
};

use crate::{
  functions::compile_dumped_bcb::compile_dumped_bcb,
  macros::cstr_names::{NAME_NEW, NAME_TEST, NAME_TEST_STR, NAME_VECTOR, NAME_VECTOR3},
};
/// `library_member_constant_cb` 的 C ABI 回调（cpp `luauLibraryConstantLookup` 同形）。
///
/// # Safety
/// 回调契约：`library`/`member` 指向编译器交付的 NUL 结尾合法 C 字符串（帧内
/// 有效非空），`constant` 为可写的 `&mut Constant as *mut _` 位形指针
/// （CompileConstant = *mut ()，bits 即 Constant 地址，见模块头注）。
unsafe extern "C-unwind" fn luau_library_constant_lookup(
  library: *const u8,
  member: *const u8,
  constant: *mut CompileConstant,
) {
  // Safety: 契约见函数级 `# Safety`；两个缓冲均帧内有效，读取收口到 cstr_cow
  // 门面，借用不出帧。
  let (lib, mem) = unsafe { (cstr_cow(library.cast()), cstr_cow(member.cast())) };
  let cc = constant as CompileConstant;

  match (lib.as_ref(), mem.as_ref()) {
    ("vector", "zero") => set_compile_constant_vector(cc, 0.0, 0.0, 0.0, 0.0),
    ("vector", "one") | ("Vector3", "one") => set_compile_constant_vector(cc, 1.0, 1.0, 1.0, 0.0),
    ("Vector3", "xAxis") => set_compile_constant_vector(cc, 1.0, 0.0, 0.0, 0.0),
    ("test", "some_nil") => set_compile_constant_nil(cc),
    ("test", "some_boolean") => set_compile_constant_boolean(cc, true),
    ("test", "some_number") => set_compile_constant_number(cc, 4.75),
    ("test", "some_string") => {
      set_compile_constant_string(cc, NAME_TEST_STR.as_ptr(), NAME_TEST_STR.len())
    }
    _ => {}
  }
}

pub fn compile_function(
  source: &str,
  id: u32,
  optimization_level: i32,
  type_info_level: i32,
) -> String {
  let mut options = CompileOptions {
    optimization_level,
    type_info_level,
    vector_lib: NAME_VECTOR3.as_ptr().cast(),
    vector_ctor: NAME_NEW.as_ptr().cast(),
    ..Default::default()
  };
  let libraries_with_constants: [*const c_char; 4] = [
    NAME_VECTOR.as_ptr().cast(),
    NAME_VECTOR3.as_ptr().cast(),
    NAME_TEST.as_ptr().cast(),
    null(),
  ];
  options.libraries_with_known_members = libraries_with_constants.as_ptr();
  options.library_member_constant_cb = Some(luau_library_constant_lookup);

  compile_dumped_bcb(source, &options, BytecodeBuilder::DUMP_CODE, None).dump_function(id)
}
