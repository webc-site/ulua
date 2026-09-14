use alloc::{ffi::CString, string::String};
use core::ffi::{CStr, c_void};

use ulua_vm::{
  functions::{lua_getfield::lua_getfield, lua_remove::lua_remove},
  macros::lua_registryindex::LUA_REGISTRYINDEX,
  records::lua_state::lua_State,
};

use crate::{
  enums::{
    status_require_impl::Status as RequireStatus,
    status_require_navigator::Status as NavigatorStatus,
  },
  functions::{
    c_str_prefix::c_str_prefix, cache_table_keys::REQUIRED_CACHE_TABLE_KEY, is_cached::is_cached,
  },
  records::{
    luarequire_configuration::luarequire_Configuration, navigator::Navigator,
    resolved_require::ResolvedRequire, runtime_error_handler::RuntimeErrorHandler,
    runtime_navigation_context::RuntimeNavigationContext,
  },
};

/// # Safety
/// `l` 必须指向存活的 `lua_State`；`lrc` 可为空（按不允许 require 处理），
/// 但经 `lua_requireinternal` 调用时应非空。函数会操作 Lua 栈。
pub(crate) unsafe fn resolve_require(
  lrc: *mut luarequire_Configuration,
  l: *mut lua_State,
  ctx: *mut c_void,
  requirer_chunkname: &CStr,
  path: String,
) -> ResolvedRequire {
  unsafe {
    // cpp 无条件解引用 lrc；Rust 对空配置或空回调按"不允许 require"处理
    let allowed = lrc
      .as_ref()
      .and_then(|config| config.is_require_allowed)
      .is_some_and(|allowed| allowed(l as *mut c_void, ctx, requirer_chunkname.as_ptr()));
    if !allowed {
      return ResolvedRequire::from_error_message("require is not supported in this context");
    }
  }

  let mut navigation_context =
    unsafe { RuntimeNavigationContext::new(lrc, l as *mut c_void, ctx, requirer_chunkname) };
  let mut error_handler = RuntimeErrorHandler::new(&path);
  let mut navigator = Navigator::new(&mut navigation_context, &mut error_handler);

  if navigator.navigate(path) == NavigatorStatus::ErrorReported {
    return ResolvedRequire::from_error_handler(&error_handler);
  }

  if !navigation_context.is_module_present() {
    return ResolvedRequire::from_error_message("no module present at resolved path");
  }

  let Some(cache_key) = navigation_context.get_cache_key() else {
    return ResolvedRequire::from_error_message("could not get cache key for module");
  };

  unsafe {
    if is_cached(l, &cache_key) {
      // 与 cpp `cacheKey->c_str()` 一致：按首个 NUL 截断后查注册表
      let cache_key_c = CString::new(c_str_prefix(&cache_key)).unwrap();
      lua_getfield(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY.as_ptr());
      lua_getfield(l, -1, cache_key_c.as_ptr());
      lua_remove(l, -2);

      return ResolvedRequire {
        status: RequireStatus::Cached,
        chunkname: String::new(),
        loadname: String::new(),
        cache_key: String::new(),
        error: String::new(),
      };
    }
  }

  let Some(chunkname) = navigation_context.get_chunkname() else {
    return ResolvedRequire::from_error_message("could not get chunkname for module");
  };

  let Some(loadname) = navigation_context.get_loadname() else {
    return ResolvedRequire::from_error_message("could not get loadname for module");
  };

  ResolvedRequire {
    status: RequireStatus::ModuleRead,
    chunkname,
    loadname,
    cache_key,
    error: String::new(),
  }
}
