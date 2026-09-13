use core::ptr::null;

use crate::{
  functions::compile_code_gen_context::compile_module_id_lua_state_i32_compilation_options_compilation_stats,
  records::{
    compilation_options::CompilationOptions, compilation_result::CompilationResult,
    compilation_stats::CompilationStats, host_ir_hooks::HostIrHooks,
  },
  type_aliases::{lua_state::lua_State, module_id::ModuleId},
};

pub fn compile_lua_state_i32_i32_compilation_stats(
  l: *mut lua_State,
  idx: i32,
  flags: u32,
  stats: *mut CompilationStats,
) -> CompilationResult {
  let options = CompilationOptions {
    flags,
    hooks: HostIrHooks::default(),
    userdata_types: null(),
    record_counters: false,
    nop_padding: false,
  };

  compile_module_id_lua_state_i32_compilation_options_compilation_stats(
    &ModuleId::default(),
    l,
    idx,
    &options,
    stats,
  )
}
