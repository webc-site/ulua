use alloc::ffi::CString;
use core::{
  ffi::{CStr, c_char, c_void},
  ptr::null_mut,
};

use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_error::lua_error, lua_gettop::lua_gettop, lua_pushstring::lua_pushstring,
    lua_settop::lua_settop, lua_tolightuserdata::lua_tolightuserdata, lua_tolstring::lua_tolstring,
    lua_touserdata::lua_touserdata, lua_yield::lua_yield,
  },
  macros::{
    lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error,
    lua_upvalueindex::lua_upvalueindex,
  },
  records::lua_state::lua_State,
};

use crate::{
  enums::status_require_impl::Status,
  functions::{
    check_registered_modules::check_registered_modules,
    lua_requirecont::{K_REQUIRE_STACK_VALUES, lua_requirecont},
    resolve_require::resolve_require,
  },
  records::luarequire_configuration::luarequire_Configuration,
};

/// 去除内部 NUL 后转 `CString`（去 NUL 保证构造不失败）。
fn to_cstring(s: &str) -> CString {
  if s.contains('\0') {
    CString::new(s.replace('\0', "")).unwrap()
  } else {
    CString::new(s).unwrap()
  }
}

pub(crate) unsafe fn lua_requireinternal(
  l: *mut lua_State,
  requirer_chunkname: *const c_char,
) -> i32 {
  unsafe {
    lua_settop(l, 1);

    let lrc = lua_touserdata(l, lua_upvalueindex(1)) as *mut luarequire_Configuration;
    if lrc.is_null() {
      luaL_error!(l, "unable to find require configuration");
      return 0;
    }

    let ctx = lua_tolightuserdata(l, lua_upvalueindex(2));
    let path = luaL_checkstring!(l, 1);
    let path_string = CStr::from_ptr(path).to_string_lossy().into_owned();

    if check_registered_modules(l, &path_string) == 1 {
      return 1;
    }

    let mut resolve_error = false;

    {
      let resolved_require = resolve_require(lrc, l, ctx, requirer_chunkname, path_string);

      if resolved_require.status == Status::Cached {
        return 1;
      }

      if resolved_require.status == Status::ErrorReported {
        let error = to_cstring(&resolved_require.error);
        lua_pushstring(l, error.as_ptr());
        resolve_error = true;
      } else {
        let cache_key = to_cstring(&resolved_require.cache_key);
        let chunkname = to_cstring(&resolved_require.chunkname);
        let loadname = to_cstring(&resolved_require.loadname);
        lua_pushstring(l, cache_key.as_ptr());
        lua_pushstring(l, chunkname.as_ptr());
        lua_pushstring(l, loadname.as_ptr());
      }
    }

    if resolve_error {
      lua_error(l);
    }

    let stack_values = lua_gettop(l);
    ulua_common::LUAU_ASSERT!(stack_values == K_REQUIRE_STACK_VALUES);

    let chunkname = lua_tolstring(l, -2, null_mut());
    let loadname = lua_tolstring(l, -1, null_mut());

    let Some(load) = (*lrc).load else {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: load"
      );
      return 0;
    };

    let num_results = load(l as *mut c_void, ctx, path, chunkname, loadname);
    if num_results == -1 {
      if lua_gettop(l) != stack_values {
        luaL_error!(l, "stack cannot be modified when require yields");
        return 0;
      }

      return lua_yield(l, 0);
    }

    lua_requirecont(l, LuaStatus::Ok as i32)
  }
}
