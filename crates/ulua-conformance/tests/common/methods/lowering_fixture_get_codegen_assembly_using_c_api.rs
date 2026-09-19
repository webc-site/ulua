use alloc::string::String;
use core::{
  ffi::{c_char, c_void},
  ptr::{null, null_mut},
};
use std::ffi::CStr;

use ulua_code_gen::functions::get_assembly::{assembly_text, get_assembly};
use ulua_compiler::{
  functions::luau_compile::luau_compile, records::lua_compile_options::LuaCompileOptions,
  type_aliases::compile_constant::CompileConstant,
};
use ulua_vm::functions::{lua_l_newstate::lua_l_newstate, luau_load::luau_load};

use crate::common::{
  functions::luau_library_constant_lookup::luau_library_constant_lookup,
  methods::lowering_fixture_get_codegen_assembly::{
    configure_codegen_options, cstr_to_str, luau_library_type_lookup_callback,
    make_assembly_options,
  },
  records::lowering_fixture::LoweringFixture,
  type_aliases::state_ref::StateRef,
};

// 与 `luau_compile` 内部的 malloc 配对：ir_lowering 树不含 c_alloc（realloc
// 仅 conformance 二进制使用），此处保留单点的 free extern。
unsafe extern "C" {
  fn free(ptr: *mut c_void);
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
unsafe extern "C-unwind" fn luau_library_constant_lookup_c_callback(
  library: *const c_char,
  member: *const c_char,
  constant: *mut CompileConstant,
) {
  let lib = unsafe { cstr_to_str(library) };
  let mem = unsafe { cstr_to_str(member) };
  unsafe { luau_library_constant_lookup(lib, mem, constant as *mut CompileConstant) };
}

impl LoweringFixture {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn get_codegen_assembly_using_c_api(
    &mut self,
    source: *const c_char,
    include_ir_types: bool,
    debug_level: i32,
  ) -> String {
    let userdata_compile_types = [
      c"vec2".as_ptr(),
      c"color".as_ptr(),
      c"mat3".as_ptr(),
      c"vertex".as_ptr(),
      null(),
    ];
    let userdata_run_types = [
      c"extra".as_ptr(),
      c"color".as_ptr(),
      c"vec2".as_ptr(),
      c"mat3".as_ptr(),
      c"vertex".as_ptr(),
      null(),
    ];
    let libraries_with_constants = [
      c"vector".as_ptr(),
      c"Vector3".as_ptr(),
      c"test".as_ptr(),
      null(),
    ];

    self.compilation_options_c.optimization_level = 2;
    self.compilation_options_c.debug_level = debug_level;
    self.compilation_options_c.type_info_level = 1;

    let mut compile_options = LuaCompileOptions {
      optimization_level: 2,
      debug_level,
      type_info_level: 1,
      coverage_level: 0,
      vector_lib: null(),
      vector_ctor: c"vector".as_ptr(),
      vector_type: c"vector".as_ptr(),
      mutable_globals: null(),
      userdata_types: userdata_compile_types.as_ptr(),
      libraries_with_known_members: libraries_with_constants.as_ptr(),
      library_member_type_cb: Some(luau_library_type_lookup_callback),
      library_member_constant_cb: Some(luau_library_constant_lookup_c_callback),
      disabled_builtins: null(),
    };

    let source_bytes = unsafe { CStr::from_ptr(source) }.to_bytes();
    let mut bytecode_size = 0usize;
    let bytecode = unsafe {
      luau_compile(
        source,
        source_bytes.len(),
        &mut compile_options,
        &mut bytecode_size,
      )
    };
    assert!(!bytecode.is_null());

    let state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = state.as_ptr();

    self.initialize_codegen(l);

    let load_result = unsafe {
      let lr = luau_load(l, c"name".as_ptr(), bytecode, bytecode_size, 0);
      free(bytecode.cast());
      lr
    };

    assert_eq!(load_result, 0, "Failed to load bytecode");

    let codegen_options = configure_codegen_options(userdata_run_types.as_ptr());
    let assembly_options = make_assembly_options(codegen_options, include_ir_types);

    assembly_text(unsafe { get_assembly(l, -1, assembly_options, null_mut()) })
  }
}
