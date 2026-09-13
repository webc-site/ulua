use alloc::string::String;
use core::{
  ffi::{c_char, c_int},
  mem::zeroed,
  ptr::null,
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_code_gen::{
  functions::summarize_bytecode::summarize_bytecode,
  records::function_bytecode_summary::FunctionBytecodeSummary,
};
use ulua_compiler::{
  functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};
use ulua_vm::{
  functions::{lua_close::lua_close, lua_l_newstate::lua_l_newstate, luau_load::luau_load},
  type_aliases::lua_state::lua_State,
};
pub fn analyze_file(
  source: &str,
  nesting_limit: u32,
  opt_level: u32,
) -> Vec<FunctionBytecodeSummary> {
  let mut bytecode_builder = BytecodeBuilder::new(None);

  let options = CompileOptions {
    optimization_level: opt_level as c_int,
    debug_level: 1,
    type_info_level: 1,
    coverage_level: 0,
    vector_lib: null(),
    vector_ctor: null(),
    vector_type: null(),
    mutable_globals: null(),
    userdata_types: null(),
    libraries_with_known_members: null(),
    library_member_type_cb: unsafe { zeroed() },
    library_member_constant_cb: unsafe { zeroed() },
    disabled_builtins: null(),
  };

  compile_or_throw_bytecode_builder_string_compile_options_parse_options(
    &mut bytecode_builder,
    &String::from(source),
    &options,
    &ParseOptions::default(),
  );

  let bytecode = bytecode_builder.get_bytecode();
  let bytecode_cstr = bytecode.as_str();

  let global_state = lua_l_newstate();
  let l: *mut lua_State = global_state;

  let result = unsafe {
    luau_load(
      l,
      c"source".as_ptr(),
      bytecode_cstr.as_ptr() as *const c_char,
      bytecode_cstr.len(),
      0,
    )
  };
  assert!(result == 0);

  let out = unsafe { summarize_bytecode(l, -1, nesting_limit) };

  unsafe {
    lua_close(l);
  }

  out
}
