use core::ptr::null;

use ulua_code_gen::functions::luau_codegen_create::luau_codegen_create;
use ulua_require::functions::luaopen_require::luaopen_require;
use ulua_vm::{
  functions::{
    lua_l_openlibs::lua_l_openlibs, lua_l_register::lua_l_register, lua_l_sandbox::lua_l_sandbox,
    lua_pushvalue::lua_pushvalue,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_pop::lua_pop},
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

use crate::functions::{
  create_cli_require_context::create_cli_require_context, lua_collectgarbage::lua_collectgarbage,
  lua_loadstring::lua_loadstring, repl_main::repl_codegen_enabled,
  require_config_init::require_config_init,
};

/// # Safety
///
/// `l` must be a valid, active pointer to a newly created `lua_State`.
pub unsafe fn setup_state(l: *mut lua_State) {
  unsafe {
    if repl_codegen_enabled() {
      luau_codegen_create(l);
    }

    lua_l_openlibs(l);

    // Note: a CALLGRIND build also registers {"callgrind", lua_callgrind}; the
    // upstream default (non-CALLGRIND) build registers only these two.
    let funcs: [LuaLReg; 3] = [
      LuaLReg {
        name: c"loadstring".as_ptr(),
        func: Some(lua_loadstring),
      },
      LuaLReg {
        name: c"collectgarbage".as_ptr(),
        func: Some(lua_collectgarbage),
      },
      LuaLReg {
        name: null(),
        func: None,
      },
    ];

    lua_pushvalue(l, LUA_GLOBALSINDEX);
    lua_l_register(l, null(), funcs.as_ptr());
    lua_pop(l, 1);

    let ctx = create_cli_require_context(l);
    luaopen_require(l, Some(require_config_init), ctx);

    lua_l_sandbox(l);
  }
}
