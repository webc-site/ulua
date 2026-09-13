use alloc::string::String;
use core::{
  ffi::{c_char, c_void},
  ptr::{null, null_mut},
};
use std::ffi::CStr;

use ulua_code_gen::{
  enums::{
    include_cfg_info::IncludeCfgInfo, include_ir_prefix::IncludeIrPrefix,
    include_reg_flow_info::IncludeRegFlowInfo, include_use_info::IncludeUseInfo, target::Target,
  },
  functions::get_assembly::get_assembly,
  records::{assembly_options::AssemblyOptions, compilation_options::CompilationOptions},
};
use ulua_compiler::{
  functions::luau_compile::luau_compile, records::lua_compile_options::LuaCompileOptions,
};
use ulua_vm::functions::{lua_l_newstate::lua_l_newstate, luau_load::luau_load};

use crate::common::{
  methods::lowering_fixture_initialize_codegen::{
    luau_library_constant_lookup_c_callback, luau_library_type_lookup_callback,
    userdata_access_bytecode_type_callback, userdata_access_callback,
    userdata_metamethod_bytecode_type_callback, userdata_metamethod_callback,
    userdata_namecall_bytecode_type_callback, userdata_namecall_callback,
    vector_access_bytecode_type_callback, vector_access_callback,
    vector_namecall_bytecode_type_callback, vector_namecall_callback,
  },
  records::lowering_fixture::LoweringFixture,
  type_aliases::state_ref::StateRef,
};
unsafe extern "C" {
  fn free(ptr: *mut c_void);
}

pub fn configure_codegen_options(userdata_types: *const *const c_char) -> CompilationOptions {
  let mut options = CompilationOptions::default();
  options.hooks.vector_access_bytecode_type = Some(vector_access_bytecode_type_callback);
  options.hooks.vector_namecall_bytecode_type = Some(vector_namecall_bytecode_type_callback);
  options.hooks.vector_access = Some(vector_access_callback);
  options.hooks.vector_namecall = Some(vector_namecall_callback);
  options.hooks.userdata_access_bytecode_type = Some(userdata_access_bytecode_type_callback);
  options.hooks.userdata_metamethod_bytecode_type =
    Some(userdata_metamethod_bytecode_type_callback);
  options.hooks.userdata_namecall_bytecode_type = Some(userdata_namecall_bytecode_type_callback);
  options.hooks.userdata_access = Some(userdata_access_callback);
  options.hooks.userdata_metamethod = Some(userdata_metamethod_callback);
  options.hooks.userdata_namecall = Some(userdata_namecall_callback);
  options.userdata_types = userdata_types;
  options
}

pub fn make_assembly_options(
  compilation_options: CompilationOptions,
  include_ir_types: bool,
) -> AssemblyOptions {
  AssemblyOptions {
    target: Target::X64SystemV,
    compilation_options,
    output_binary: false,
    include_assembly: false,
    include_ir: true,
    include_outlined_code: false,
    include_ir_types,
    include_ir_prefix: IncludeIrPrefix::No,
    include_use_info: IncludeUseInfo::No,
    include_cfg_info: IncludeCfgInfo::No,
    include_reg_flow_info: IncludeRegFlowInfo::No,
    annotator: None,
    annotator_context: null_mut(),
  }
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
      free(bytecode as *mut c_void);
      lr
    };

    assert_eq!(load_result, 0, "Failed to load bytecode");

    let codegen_options = configure_codegen_options(userdata_run_types.as_ptr());
    let assembly_options = make_assembly_options(codegen_options, include_ir_types);

    unsafe { get_assembly(l, -1, assembly_options, null_mut()) }
  }
}
