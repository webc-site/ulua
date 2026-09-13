use alloc::string::String;
use core::ffi::{CStr, c_char};

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

pub fn get_codegen_assembly(
  name: *const c_char,
  bytecode: &str,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> String {
  let global_state = LuaStateGuard(lua_l_newstate());
  let l = global_state.0;

  if unsafe {
    luau_load(
      l,
      name,
      bytecode.as_ptr() as *const c_char,
      bytecode.len(),
      0,
    )
  } == 0
  {
    unsafe { get_assembly(l, -1, options, stats) }
  } else {
    let name = unsafe { CStr::from_ptr(name).to_string_lossy() };
    eprintln!("Error loading bytecode {}", name);
    String::new()
  }
}
