use crate::{
  functions::compile_code_gen_context::compile_module_id_lua_state_i32_compilation_options_compilation_stats,
  records::{
    compilation_options::CompilationOptions, compilation_result::CompilationResult,
    compilation_stats::CompilationStats,
  },
  type_aliases::{lua_state::lua_State, module_id::ModuleId},
};

pub fn compile_module_id_lua_state_i32_i32_compilation_stats(
  module_id: &ModuleId,
  l: *mut lua_State,
  idx: i32,
  flags: i32,
  stats: *mut CompilationStats,
) -> CompilationResult {
  let options = CompilationOptions {
    flags: flags as u32,
    ..CompilationOptions::default()
  };

  compile_module_id_lua_state_i32_compilation_options_compilation_stats(
    module_id, l, idx, &options, stats,
  )
}
