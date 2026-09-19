use alloc::{rc::Rc, vec::Vec};
use core::{ffi::c_void, ptr::from_ref};

use ulua_vm::{
  functions::{lua_getthreaddata::lua_getthreaddata, lua_setthreaddata::lua_setthreaddata},
  macros::lua_l_error::luaL_error,
  type_aliases::lua_state::lua_State,
};

use crate::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
  },
  records::{
    runtime_luau_config_timer::RuntimeLuauConfigTimer,
    runtime_navigation_context::RuntimeNavigationContext,
  },
};

/// C++ 中 `get_luau_config_timeout` 缺省时的默认超时（毫秒）。
const DEFAULT_LUAU_CONFIG_TIMEOUT_MS: i32 = 2000;

/// 导航上下文接口，由注入方实现（对应 C++ 纯虚基类 `NavigationContext`）。
///
/// 路径 / 别名 / 组件一律是字节串（cpp 为 `std::string`），实现方不得做 UTF-8
/// 校验或替换；`get_alias`/`get_config` 的返回值同样是原始字节。
pub trait NavigationContextTrait {
  fn reset_to_requirer(&mut self) -> NavigateResult;
  fn jump_to_alias(&mut self, path: &[u8]) -> NavigateResult;

  fn to_alias_override(&mut self, _alias_unprefixed: &[u8]) -> NavigateResult {
    NavigateResult::NotFound
  }

  fn to_alias_fallback(&mut self, _alias_unprefixed: &[u8]) -> NavigateResult {
    NavigateResult::NotFound
  }

  fn to_parent(&mut self) -> NavigateResult;
  fn to_child(&mut self, component: &[u8]) -> NavigateResult;

  fn get_config_status(&self) -> ConfigStatus {
    ConfigStatus::Absent
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    ConfigBehavior::GetAlias
  }

  fn get_alias(&self, _alias: &[u8]) -> Option<Vec<u8>> {
    None
  }

  fn get_config(&self) -> Option<Vec<u8>> {
    None
  }

  fn luau_config_init(&self) -> Option<Rc<dyn Fn(*mut lua_State)>> {
    None
  }

  fn luau_config_interrupt(
    &self,
  ) -> Option<unsafe extern "C-unwind" fn(l: *mut lua_State, gc: i32)> {
    None
  }
}

impl NavigationContextTrait for RuntimeNavigationContext<'_> {
  fn reset_to_requirer(&mut self) -> NavigateResult {
    RuntimeNavigationContext::reset_to_requirer(self)
  }

  fn jump_to_alias(&mut self, path: &[u8]) -> NavigateResult {
    RuntimeNavigationContext::jump_to_alias(self, path)
  }

  fn to_alias_override(&mut self, alias_unprefixed: &[u8]) -> NavigateResult {
    RuntimeNavigationContext::to_alias_override(self, alias_unprefixed)
  }

  fn to_alias_fallback(&mut self, alias_unprefixed: &[u8]) -> NavigateResult {
    RuntimeNavigationContext::to_alias_fallback(self, alias_unprefixed)
  }

  fn to_parent(&mut self) -> NavigateResult {
    RuntimeNavigationContext::to_parent(self)
  }

  fn to_child(&mut self, component: &[u8]) -> NavigateResult {
    RuntimeNavigationContext::to_child(self, component)
  }

  fn get_config_status(&self) -> ConfigStatus {
    RuntimeNavigationContext::get_config_status(self)
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    RuntimeNavigationContext::get_config_behavior(self)
  }

  fn get_alias(&self, alias: &[u8]) -> Option<Vec<u8>> {
    RuntimeNavigationContext::get_alias(self, alias)
  }

  fn get_config(&self) -> Option<Vec<u8>> {
    RuntimeNavigationContext::get_config(self)
  }

  fn luau_config_init(&self) -> Option<Rc<dyn Fn(*mut lua_State)>> {
    let config = self.config;
    let ctx = self.ctx;
    // SAFETY: 闭包仅在 extract_luau_config 执行配置期间被调用，此时
    // RuntimeNavigationContext 仍存活（resolve_require 持有它直到导航结束）。
    // timer 是 `&self` 借出的只读引用，其内部可变字段用 Cell 承载，
    // 因此把地址交给 VM 线程数据不构成别名冲突；Luau 状态机单线程使用。
    let timer = from_ref(&self.timer);

    Some(Rc::new(move |l: *mut lua_State| unsafe {
      let timeout = if let Some(get_timeout) = (*config).get_luau_config_timeout {
        get_timeout(l as *mut c_void, ctx)
      } else {
        DEFAULT_LUAU_CONFIG_TIMEOUT_MS
      };

      (*timer).start(timeout);
      // 地址只是经 VM 的线程数据槽转手，写权限由 Cell 提供
      lua_setthreaddata(l, timer.cast::<c_void>().cast_mut());
    }))
  }

  fn luau_config_interrupt(
    &self,
  ) -> Option<unsafe extern "C-unwind" fn(l: *mut lua_State, gc: i32)> {
    Some(runtime_luau_config_interrupt)
  }
}

/// 配置执行超时文案（cpp `luauConfigInterrupt` 的唯一错误消息）。
const CONFIG_TIMEOUT_MSG: &str = "configuration execution timed out";

/// 配置执行中断回调：超时则报错（对应 C++ `luauConfigInterrupt`）。
unsafe extern "C-unwind" fn runtime_luau_config_interrupt(l: *mut lua_State, _gc: i32) {
  unsafe {
    let timer = lua_getthreaddata(l) as *const RuntimeLuauConfigTimer;
    if !timer.is_null() && (*timer).is_finished() {
      luaL_error!(l, "{CONFIG_TIMEOUT_MSG}");
    }
  }
}
