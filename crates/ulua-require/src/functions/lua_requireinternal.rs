use core::{
  ffi::{CStr, c_void},
  ptr::null_mut,
};

use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_error::lua_error, lua_gettop::lua_gettop, lua_settop::lua_settop,
    lua_tolightuserdata::lua_tolightuserdata, lua_tolstring::lua_tolstring,
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
    c_str_prefix::push_c_str,
    check_registered_modules::check_registered_modules,
    lua_requirecont::{K_REQUIRE_STACK_VALUES, lua_requirecont},
    resolve_require::resolve_require,
  },
  records::luarequire_configuration::luarequire_Configuration,
};

/// # Safety
/// `l` 必须指向存活的 `lua_State`；upvalue(1)/(2) 须分别为配置指针与 ctx，
/// 栈顶为 require 路径参数（由 C 闭包调用约定保证）。
/// `requirer_chunkname` 为 requirer chunkname 字节串，由真 FFI 入口一次性 `.to_bytes()` 取得。
pub(crate) unsafe fn lua_requireinternal(l: *mut lua_State, requirer_chunkname: &[u8]) -> i32 {
  unsafe {
    lua_settop(l, 1);

    let lrc = lua_touserdata(l, lua_upvalueindex(1)) as *mut luarequire_Configuration;
    if lrc.is_null() {
      luaL_error!(l, "unable to find require configuration");
    }

    let ctx = lua_tolightuserdata(l, lua_upvalueindex(2));
    let path = luaL_checkstring!(l, 1);
    // 对应 cpp `std::string path(luaL_checkstring(L, 1))`：取首个 NUL 前的纯字节，
    // 零拷贝且不校验 UTF-8（旧实现 `to_string_lossy` 会把非 UTF-8 字节换成 U+FFFD）
    let path_bytes = CStr::from_ptr(path).to_bytes();

    if check_registered_modules(l, path_bytes) {
      return 1;
    }

    let mut resolve_error = false;

    {
      let resolved_require = resolve_require(lrc, l, ctx, requirer_chunkname, path_bytes);

      if resolved_require.status == Status::Cached {
        return 1;
      }

      if resolved_require.status == Status::ErrorReported {
        push_c_str(l, &resolved_require.error);
        resolve_error = true;
      } else {
        push_c_str(l, &resolved_require.cache_key);
        push_c_str(l, &resolved_require.chunkname);
        push_c_str(l, &resolved_require.loadname);
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
    };

    let num_results = load(l as *mut c_void, ctx, path, chunkname, loadname);
    if num_results == -1 {
      if lua_gettop(l) != stack_values {
        luaL_error!(l, "stack cannot be modified when require yields");
      }

      return lua_yield(l, 0);
    }

    lua_requirecont(l, LuaStatus::Ok as i32)
  }
}
