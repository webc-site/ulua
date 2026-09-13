use ulua_require::functions::luaopen_require::luaopen_require;
use ulua_vm::{
  functions::{
    lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
    lua_pushcclosurek::lua_pushcclosurek,
  },
  macros::lua_setglobal::lua_setglobal,
  records::lua_state::lua_State,
};

use crate::functions::{
  create_cli_require_context::create_cli_require_context, lua_collectgarbage::lua_collectgarbage,
  lua_loadstring::lua_loadstring, require_config_init::require_config_init,
};

pub fn setup_state(l: *mut lua_State) {
  unsafe {
    lua_l_openlibs(l);

    // C++ setupState registers a few globals (loadstring/collectgarbage) onto
    // the globals table via luaL_register before opening require. (A CALLGRIND
    // build also registers "callgrind"; the default build registers only these
    // two.)
    lua_pushcclosurek(l, Some(lua_loadstring), c"loadstring".as_ptr(), 0, None);
    lua_setglobal(l, c"loadstring".as_ptr());

    lua_pushcclosurek(
      l,
      Some(lua_collectgarbage),
      c"collectgarbage".as_ptr(),
      0,
      None,
    );
    lua_setglobal(l, c"collectgarbage".as_ptr());

    let ctx = create_cli_require_context(l);
    luaopen_require(l, Some(require_config_init), ctx);

    lua_l_sandbox(l);
  }
}
