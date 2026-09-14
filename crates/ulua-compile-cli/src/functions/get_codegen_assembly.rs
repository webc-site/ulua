use std::ffi::CString;

use ulua_code_gen::{
  functions::get_assembly::get_assembly,
  records::{assembly_options::AssemblyOptions, lowering_stats::LoweringStats},
};
use ulua_vm::{
  functions::{lua_close::lua_close, lua_l_newstate::lua_l_newstate, luau_load::luau_load},
  type_aliases::lua_state::lua_State,
};

struct LuaStateGuard(*mut lua_State);

impl Drop for LuaStateGuard {
  fn drop(&mut self) {
    if !self.0.is_null() {
      unsafe {
        lua_close(self.0);
      }
    }
  }
}

/// cpp `getCodegenAssembly` (CLI/src/Compile.cpp:114-129)
pub fn get_codegen_assembly(
  name: &str,
  bytecode: &str,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> String {
  // luau_load 的 chunkname 需要 NUL 结尾 (命令行参数不含 NUL, 必成功)
  let name_c = CString::new(name).unwrap_or_default();

  let global_state = LuaStateGuard(lua_l_newstate());
  let l = global_state.0;

  if unsafe {
    luau_load(
      l,
      name_c.as_ptr(),
      bytecode.as_ptr() as *const _,
      bytecode.len(),
      0,
    )
  } == 0
  {
    unsafe { get_assembly(l, -1, options, stats) }
  } else {
    eprintln!("Error loading bytecode {name}");
    String::new()
  }
}
