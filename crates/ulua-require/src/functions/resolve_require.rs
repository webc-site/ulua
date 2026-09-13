use alloc::{ffi::CString, string::String};
use core::ffi::{CStr, c_char, c_void};

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
  functions::{cache_table_keys::REQUIRED_CACHE_TABLE_KEY, is_cached::is_cached},
  records::{
    luarequire_configuration::luarequire_Configuration, navigator::Navigator,
    resolved_require::ResolvedRequire, runtime_error_handler::RuntimeErrorHandler,
    runtime_navigation_context::RuntimeNavigationContext,
  },
};

pub(crate) unsafe fn resolve_require(
  lrc: *mut luarequire_Configuration,
  l: *mut lua_State,
  ctx: *mut c_void,
  requirer_chunkname: *const c_char,
  path: String,
) -> ResolvedRequire {
  unsafe {
    let Some(is_require_allowed) = lrc.as_ref().and_then(|config| config.is_require_allowed) else {
      return ResolvedRequire::from_error_message("require is not supported in this context");
    };

    if !is_require_allowed(l as *mut c_void, ctx, requirer_chunkname) {
      return ResolvedRequire::from_error_message("require is not supported in this context");
    }
  }

  let requirer_chunkname = unsafe {
    if requirer_chunkname.is_null() {
      CString::default()
    } else {
      CStr::from_ptr(requirer_chunkname).to_owned()
    }
  };

  let mut navigation_context =
    unsafe { RuntimeNavigationContext::new(lrc, l as *mut c_void, ctx, requirer_chunkname) };
  let mut error_handler = RuntimeErrorHandler::new(path.clone());
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
    if is_cached(l, cache_key.as_str()) {
      let cache_key_c = CString::new(cache_key.as_str()).unwrap_or_default();
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
