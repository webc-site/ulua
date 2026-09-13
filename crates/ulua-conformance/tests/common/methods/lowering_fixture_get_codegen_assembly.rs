use alloc::string::String;
use core::{
  ffi::c_char,
  ptr::{null, null_mut},
};
use std::ffi::CStr;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_code_gen::{
  enums::{
    include_cfg_info::IncludeCfgInfo, include_ir_prefix::IncludeIrPrefix,
    include_reg_flow_info::IncludeRegFlowInfo, include_use_info::IncludeUseInfo, target::Target,
  },
  functions::{get_assembly::get_assembly, luau_codegen_supported::luau_codegen_supported},
  records::{assembly_options::AssemblyOptions, compilation_options::CompilationOptions},
};
use ulua_compiler::{
  functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};
use ulua_vm::functions::{lua_l_newstate::lua_l_newstate, luau_load::luau_load};

use crate::common::{
  methods::lowering_fixture_initialize_codegen::{
    luau_library_constant_lookup_callback, luau_library_type_lookup_callback,
    userdata_access_bytecode_type_callback, userdata_access_callback,
    userdata_metamethod_bytecode_type_callback, userdata_metamethod_callback,
    userdata_namecall_bytecode_type_callback, userdata_namecall_callback,
    vector_access_bytecode_type_callback, vector_access_callback,
    vector_namecall_bytecode_type_callback, vector_namecall_callback,
  },
  records::lowering_fixture::LoweringFixture,
  type_aliases::state_ref::StateRef,
};
fn configure_codegen_options(userdata_types: *const *const c_char) -> CompilationOptions {
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

fn configure_compile_options(
  options: &mut CompileOptions,
  userdata_types: *const *const c_char,
  libraries: *const *const c_char,
  debug_level: i32,
  optimization_level: i32,
) {
  options.optimization_level = optimization_level;
  options.debug_level = debug_level;
  options.vector_ctor = c"vector".as_ptr();
  options.vector_type = c"vector".as_ptr();
  options.userdata_types = userdata_types;
  options.libraries_with_known_members = libraries;
  options.library_member_type_cb = Some(luau_library_type_lookup_callback);
  options.library_member_constant_cb = Some(luau_library_constant_lookup_callback);
}

fn make_assembly_options(
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

fn clip_assembly_to_first_return(mut assembly: String) -> String {
  if let Some(pos) = assembly.find("RETURN")
    && let Some(newline) = assembly[pos..].find('\n')
  {
    assembly.truncate(pos + newline + 1);
  }

  assembly
}

impl LoweringFixture {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn get_codegen_assembly(
    &mut self,
    source: *const c_char,
    include_ir_types: bool,
    debug_level: i32,
    optimization_level: i32,
    clip_to_first_return: bool,
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

    configure_compile_options(
      &mut self.compilation_options,
      userdata_compile_types.as_ptr(),
      libraries_with_constants.as_ptr(),
      debug_level,
      optimization_level,
    );

    let source = unsafe { CStr::from_ptr(source) }
      .to_string_lossy()
      .into_owned();
    let mut bcb = BytecodeBuilder::new(None);
    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
      &mut bcb,
      &source,
      &self.compilation_options,
      &ParseOptions::default(),
    );

    let bytecode = bcb.get_bytecode();
    let state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = state.as_ptr();

    self.initialize_codegen(l);

    let load_result = unsafe {
      luau_load(
        l,
        c"name".as_ptr(),
        bytecode.as_ptr() as *const c_char,
        bytecode.len(),
        0,
      )
    };
    assert_eq!(load_result, 0, "Failed to load bytecode");

    let codegen_options = configure_codegen_options(userdata_run_types.as_ptr());
    let mut assembly_options = make_assembly_options(codegen_options, include_ir_types);
    assembly_options.compilation_options.flags = self.assembly_options.compilation_options.flags;
    assembly_options.include_outlined_code = self.assembly_options.include_outlined_code;
    assembly_options.include_reg_flow_info = self.assembly_options.include_reg_flow_info;

    let result = unsafe { get_assembly(l, -1, assembly_options.clone(), null_mut()) };

    if luau_codegen_supported() != 0 {
      assembly_options.target = Target::A64;
      unsafe {
        get_assembly(l, -1, assembly_options, null_mut());
      }
    }

    if clip_to_first_return {
      clip_assembly_to_first_return(result)
    } else {
      result
    }
  }
}
