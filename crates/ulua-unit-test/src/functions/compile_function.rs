//! Test fixture: faithful port of `compileFunction` + `luauLibraryConstantLookup`
//! (tests/Compiler.test.cpp).
// Port of the test's `luauLibraryConstantLookup` callback. The compiler passes
// `&mut Constant as *mut CompileConstant` (CompileConstant = *mut c_void), so the
// incoming `constant` pointer's bits ARE the `*mut Constant`; `as CompileConstant`
// recovers it for the `set_compile_constant_*` helpers.
use alloc::string::String;
use core::{
  ffi::{CStr, c_char},
  ptr::null,
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::{
    compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
    set_compile_constant_boolean::set_compile_constant_boolean,
    set_compile_constant_nil::set_compile_constant_nil,
    set_compile_constant_number::set_compile_constant_number,
    set_compile_constant_string::set_compile_constant_string,
    set_compile_constant_vector::set_compile_constant_vector,
  },
  records::compile_options::CompileOptions,
  type_aliases::compile_constant::CompileConstant,
};
unsafe extern "C-unwind" fn luau_library_constant_lookup(
  library: *const c_char,
  member: *const c_char,
  constant: *mut CompileConstant,
) {
  let lib = unsafe { CStr::from_ptr(library).to_str().unwrap_or("") };
  let mem = unsafe { CStr::from_ptr(member).to_str().unwrap_or("") };
  let cc = constant as CompileConstant;

  match (lib, mem) {
    ("vector", "zero") => set_compile_constant_vector(cc, 0.0, 0.0, 0.0, 0.0),
    ("vector", "one") | ("Vector3", "one") => set_compile_constant_vector(cc, 1.0, 1.0, 1.0, 0.0),
    ("Vector3", "xAxis") => set_compile_constant_vector(cc, 1.0, 0.0, 0.0, 0.0),
    ("test", "some_nil") => set_compile_constant_nil(cc),
    ("test", "some_boolean") => set_compile_constant_boolean(cc, true),
    ("test", "some_number") => set_compile_constant_number(cc, 4.75),
    ("test", "some_string") => set_compile_constant_string(cc, c"test".as_ptr(), 4),
    _ => {}
  }
}

pub fn compile_function(
  source: &str,
  id: u32,
  optimization_level: i32,
  type_info_level: i32,
) -> String {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

  let mut options = CompileOptions {
    optimization_level,
    type_info_level,
    vector_lib: c"Vector3".as_ptr(),
    vector_ctor: c"new".as_ptr(),
    ..Default::default()
  };
  let libraries_with_constants: [*const c_char; 4] = [
    c"vector".as_ptr(),
    c"Vector3".as_ptr(),
    c"test".as_ptr(),
    null(),
  ];
  options.libraries_with_known_members = libraries_with_constants.as_ptr();
  options.library_member_constant_cb = Some(luau_library_constant_lookup);

  let source = String::from(source);
  let parse_options = ParseOptions::default();
  compile_or_throw_bytecode_builder_string_compile_options_parse_options(
    &mut bcb,
    &source,
    &options,
    &parse_options,
  );

  bcb.dump_function(id)
}
