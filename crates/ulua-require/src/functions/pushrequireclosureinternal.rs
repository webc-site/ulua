use core::{
  ffi::{c_char, c_void},
  mem::{size_of, zeroed},
  ptr::write,
};

use ulua_vm::{
  functions::lua_pushcclosurek::lua_pushcclosurek,
  macros::{
    LUA_PUSHLIGHTUSERDATA::LUA_PUSHLIGHTUSERDATA, lua_l_error::luaL_error,
    lua_newuserdata::lua_newuserdata,
  },
  records::lua_state::lua_State,
  type_aliases::lua_c_function::LuaCfunction,
};

use crate::{
  functions::{lua_requirecont::lua_requirecont, validate_config::validate_config},
  records::luarequire_configuration::{LuarequireConfigurationInit, luarequire_Configuration},
};

pub(crate) unsafe fn pushrequireclosureinternal(
  l: *mut lua_State,
  config_init: LuarequireConfigurationInit,
  ctx: *mut c_void,
  requirelikefunc: LuaCfunction,
  debugname: *const c_char,
) -> i32 {
  unsafe {
    let ud = lua_newuserdata(l, size_of::<luarequire_Configuration>());
    if ud.is_null() {
      luaL_error!(l, "failed to allocate memory for require configuration");
      return 0;
    }

    let config = ud as *mut luarequire_Configuration;
    write(config, zeroed());

    let Some(config_init) = config_init else {
      luaL_error!(
        l,
        "require configuration is missing required initializer function"
      );
      return 0;
    };

    config_init(config);
    validate_config(l, &*config);

    LUA_PUSHLIGHTUSERDATA(l as *mut c_void, ctx);
    lua_pushcclosurek(l, requirelikefunc, debugname, 2, Some(lua_requirecont));
    1
  }
}
