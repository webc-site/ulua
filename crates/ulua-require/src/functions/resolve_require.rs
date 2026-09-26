use alloc::vec::Vec;
use core::ffi::c_void;

use ulua_vm::records::lua_state::LuaState;

use crate::{
  enums::{
    status_require_impl::Status as RequireStatus,
    status_require_navigator::Status as NavigatorStatus,
  },
  functions::{c_str_prefix::with_c_str, is_cached::is_cached},
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
/// `l` 必须指向存活的 `LuaState`；函数会操作 Lua 栈。
/// `ctx` 为与 `l` 同帧存活、仅原样转交配置回调的宿主 lightuserdata 指针
/// （本函数不解引用）。
/// `requirer_chunkname`/`path` 为字节串，仅在真 FFI 边界补 NUL。
pub(crate) unsafe fn resolve_require(
  config: &luarequire_Configuration,
  l: *mut LuaState,
  ctx: *mut c_void,
  requirer_chunkname: &[u8],
  path: &[u8],
) -> ResolvedRequire {
  // cpp 无条件调用 `is_require_allowed`；回调缺失时按"不允许 require"处理
  let allowed = config.is_require_allowed.is_some_and(|allowed| {
    // 此处 with_c_str 是真 C-ABI 边界：`PredicateWithInputFn` 即宿主按 cpp
    // `Require.h` 实现的 `unsafe extern "C-unwind" fn(.., input: *const c_char)`，
    // 形参类型跨语言定格，不能收 `&[u8]`。
    // Safety: l/ctx 是 lua_requireinternal 从闭包 upvalue 取得并在本次导航内保持存活的 state 与 lightuserdata；allowed 为 validate_config 确认存在的配置回调，签名即 (state*, ctx, NUL 结尾串)；chunkname 指针由 with_c_str 补 NUL、仅闭包调用期内存活。
    with_c_str(requirer_chunkname, |chunkname| unsafe {
      allowed(l.cast(), ctx, chunkname)
    })
  });
  if !allowed {
    return ResolvedRequire::from_error_message(NOT_ALLOWED);
  }

  let mut navigation_context =
    RuntimeNavigationContext::new(config, l.cast(), ctx, requirer_chunkname);
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

  // Safety: l 为调用方传入且本帧存活的 LuaState；cache_key 为本地字节串，with_c_str 构造的临时指针在调用内消费；is_cached 命中时缓存值留栈顶（缓存表已移除）供调用方消费，与 cpp 命中路径（getfield 后值在栈顶）栈布局一致；未命中时其内部自行配平。
  unsafe {
    if is_cached(l, &cache_key) {
      // cpp 命中缓存路径：值已留在栈顶，结果仅带 Cached 状态（其余字节串成员恒空）
      return ResolvedRequire::with_error(RequireStatus::Cached, Vec::new());
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
