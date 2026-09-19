use alloc::vec::Vec;
use core::ffi::{c_char, c_void};

use ulua_vm::{
  functions::{
    lua_getfield::lua_getfield, lua_gettable::lua_gettable, lua_pushlstring::lua_pushlstring,
    lua_remove::lua_remove,
  },
  macros::lua_registryindex::LUA_REGISTRYINDEX,
  records::lua_state::lua_State,
};

use crate::{
  enums::{
    status_require_impl::Status as RequireStatus,
    status_require_navigator::Status as NavigatorStatus,
  },
  functions::{
    c_str_prefix::{c_str_prefix, with_c_str},
    cache_table_keys::REQUIRED_CACHE_TABLE_KEY,
    is_cached::is_cached,
  },
  records::{
    luarequire_configuration::luarequire_Configuration, navigator::Navigator,
    resolved_require::ResolvedRequire, runtime_error_handler::RuntimeErrorHandler,
    runtime_navigation_context::RuntimeNavigationContext,
  },
};

/// 固定错误文案（cpp `ResolvedRequire::fromErrorMessage` 的字面量）。
const NOT_ALLOWED: &[u8] = b"require is not supported in this context";
const NO_MODULE: &[u8] = b"no module present at resolved path";
const NO_CACHE_KEY: &[u8] = b"could not get cache key for module";
const NO_CHUNKNAME: &[u8] = b"could not get chunkname for module";
const NO_LOADNAME: &[u8] = b"could not get loadname for module";

/// # Safety
/// `l` 必须指向存活的 `lua_State`；`lrc` 可为空（按不允许 require 处理），
/// 但经 `lua_requireinternal` 调用时应非空。函数会操作 Lua 栈。
/// `requirer_chunkname`/`path` 为字节串，仅在真 FFI 边界补 NUL。
pub(crate) unsafe fn resolve_require(
  lrc: *mut luarequire_Configuration,
  l: *mut lua_State,
  ctx: *mut c_void,
  requirer_chunkname: &[u8],
  path: &[u8],
) -> ResolvedRequire {
  unsafe {
    // cpp 无条件解引用 lrc；Rust 对空配置或空回调按"不允许 require"处理
    let allowed = lrc
      .as_ref()
      .and_then(|config| config.is_require_allowed)
      .is_some_and(|allowed| {
        with_c_str(requirer_chunkname, |chunkname| {
          allowed(l as *mut c_void, ctx, chunkname)
        })
      });
    if !allowed {
      return ResolvedRequire::from_error_message(NOT_ALLOWED);
    }
  }

  let mut navigation_context =
    unsafe { RuntimeNavigationContext::new(lrc, l as *mut c_void, ctx, requirer_chunkname) };
  let mut error_handler = RuntimeErrorHandler::new(path);
  let mut navigator = Navigator::new(&mut navigation_context, &mut error_handler);

  if navigator.navigate(path) == NavigatorStatus::ErrorReported {
    return ResolvedRequire::from_error_handler(&error_handler);
  }

  if !navigation_context.is_module_present() {
    return ResolvedRequire::from_error_message(NO_MODULE);
  }

  let Some(cache_key) = navigation_context.get_cache_key() else {
    return ResolvedRequire::from_error_message(NO_CACHE_KEY);
  };

  unsafe {
    if is_cached(l, &cache_key) {
      // 与 cpp `lua_getfield(L, -1, cacheKey->c_str())` 一致：按首个 NUL 截断，
      // 经 pushlstring + gettable 零拷贝等价实现（免 NUL 补齐）
      let cache_key = c_str_prefix(&cache_key);
      with_c_str(REQUIRED_CACHE_TABLE_KEY, |table_key| {
        lua_getfield(l, LUA_REGISTRYINDEX, table_key);
      });
      lua_pushlstring(l, cache_key.as_ptr().cast::<c_char>(), cache_key.len());
      lua_gettable(l, -2);
      lua_remove(l, -2);

      return ResolvedRequire {
        status: RequireStatus::Cached,
        chunkname: Vec::new(),
        loadname: Vec::new(),
        cache_key: Vec::new(),
        error: Vec::new(),
      };
    }
  }

  let Some(chunkname) = navigation_context.get_chunkname() else {
    return ResolvedRequire::from_error_message(NO_CHUNKNAME);
  };

  let Some(loadname) = navigation_context.get_loadname() else {
    return ResolvedRequire::from_error_message(NO_LOADNAME);
  };

  ResolvedRequire {
    status: RequireStatus::ModuleRead,
    chunkname,
    loadname,
    cache_key,
    error: Vec::new(),
  }
}
