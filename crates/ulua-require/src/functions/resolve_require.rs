use alloc::vec::Vec;

use ulua_vm::records::lua_state::LuaState;

use crate::{
  enums::{
    status_require_impl::Status as RequireStatus,
    status_require_navigator::Status as NavigatorStatus,
  },
  functions::is_cached::is_cached,
  records::{
    navigation_context::RequireHost, navigator::Navigator, resolved_require::ResolvedRequire,
    runtime_error_handler::RuntimeErrorHandler,
    runtime_navigation_context::RuntimeNavigationContext,
  },
};

/// 固定错误文案（cpp `ResolvedRequire::fromErrorMessage` 的字面量）。
/// `lua_require` 的 `luaL_error` 同款文案共用此常量（消灭雷同字符串）。
pub(crate) const NOT_ALLOWED_MSG: &str = "require is not supported in this context";
const NOT_ALLOWED: &[u8] = NOT_ALLOWED_MSG.as_bytes();
const NO_MODULE: &[u8] = b"no module present at resolved path";
const NO_CACHE_KEY: &[u8] = b"could not get cache key for module";
const NO_CHUNKNAME: &[u8] = b"could not get chunkname for module";
const NO_LOADNAME: &[u8] = b"could not get loadname for module";

/// 把路径解析为模块（cpp `Require.cpp` 的 `resolveRequire`）：判定 require
/// 许可 → 装配 [`RuntimeNavigationContext`] 驱动 [`Navigator`] → 校验模块
/// 存在并取回 cacheKey/chunkname/loadname（命中缓存则值已留在 `l` 栈顶）。
///
/// `host` 为本次 require 的宿主机（其导航可变性由内部可变性自持，此处只需
/// 共享引用）；`dyn` 保留点在 `HostSlot`（唯一擦除处），本层与下游
/// [`RuntimeNavigationContext`] 只转接该引用，不构成新的分发位点；
/// `requirer_chunkname`/`path` 为字节串。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`：导航期间宿主回调可能执行 VM 代码（Luau
/// 配置的 `extract_luau_config`），is_cached 读写本协程栈与注册表。
pub(crate) unsafe fn resolve_require(
  host: &dyn RequireHost,
  l: *mut LuaState,
  requirer_chunkname: &[u8],
  path: &[u8],
) -> ResolvedRequire {
  if !host.is_require_allowed(requirer_chunkname) {
    return ResolvedRequire::from_error_message(NOT_ALLOWED);
  }

  let navigation_context = RuntimeNavigationContext::new(host, requirer_chunkname);
  let mut error_handler = RuntimeErrorHandler::new(path);
  let mut navigator = Navigator::new(&navigation_context, &mut error_handler);

  if navigator.navigate(path) == NavigatorStatus::ErrorReported {
    return ResolvedRequire::from_error_handler(&error_handler);
  }

  if !navigation_context.is_module_present() {
    return ResolvedRequire::from_error_message(NO_MODULE);
  }

  let Some(cache_key) = navigation_context.get_cache_key() else {
    return ResolvedRequire::from_error_message(NO_CACHE_KEY);
  };

  // Safety: l 为调用方传入且本帧存活的 LuaState（导航上下文只经共享引用
  // 触达宿主，本调用不与之交叠）；is_cached 命中时缓存值留栈顶（缓存表已
  // 移除）供调用方消费，与 cpp 命中路径栈布局一致；未命中时其内部自行配平。
  if unsafe { is_cached(&mut *l, &cache_key) } {
    // cpp 命中缓存路径：值已留在栈顶，结果仅带 Cached 状态（其余字节串成员恒空）
    return ResolvedRequire::with_error(RequireStatus::Cached, Vec::new());
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
