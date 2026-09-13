use alloc::{rc::Rc, string::String};
use core::ffi::c_void;

use ulua_vm::{
  functions::{
    lua_getthreaddata::lua_getthreaddata, lua_l_error_l::lua_l_error_l,
    lua_setthreaddata::lua_setthreaddata,
  },
  type_aliases::lua_state::lua_State,
};

pub use crate::enums::{config_behavior::ConfigBehavior, config_status::ConfigStatus};
use crate::{
  enums::navigate_result::NavigateResult,
  records::{
    runtime_luau_config_timer::RuntimeLuauConfigTimer,
    runtime_navigation_context::RuntimeNavigationContext,
  },
};

/// C++ 中 `get_luau_config_timeout` 缺省时的默认超时（毫秒）。
const DEFAULT_LUAU_CONFIG_TIMEOUT_MS: i32 = 2000;

/// 导航上下文接口，由注入方实现（对应 C++ 纯虚基类 `NavigationContext`）。
pub trait NavigationContextTrait {
  fn reset_to_requirer(&mut self) -> NavigateResult;
  fn jump_to_alias(&mut self, path: &str) -> NavigateResult;

  fn to_alias_override(&mut self, _alias_unprefixed: &str) -> NavigateResult {
    NavigateResult::NotFound
  }

  fn to_alias_fallback(&mut self, _alias_unprefixed: &str) -> NavigateResult {
    NavigateResult::NotFound
  }

  fn to_parent(&mut self) -> NavigateResult;
  fn to_child(&mut self, component: &str) -> NavigateResult;

  fn get_config_status(&self) -> ConfigStatus {
    ConfigStatus::Absent
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    ConfigBehavior::GetAlias
  }

  fn get_alias(&self, _alias: &str) -> Option<String> {
    None
  }

  fn get_config(&self) -> Option<String> {
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

impl NavigationContextTrait for RuntimeNavigationContext {
  fn reset_to_requirer(&mut self) -> NavigateResult {
    RuntimeNavigationContext::reset_to_requirer(self)
  }

  fn jump_to_alias(&mut self, path: &str) -> NavigateResult {
    RuntimeNavigationContext::jump_to_alias(self, path)
  }

  fn to_alias_override(&mut self, alias_unprefixed: &str) -> NavigateResult {
    RuntimeNavigationContext::to_alias_override(self, alias_unprefixed)
  }

  fn to_alias_fallback(&mut self, alias_unprefixed: &str) -> NavigateResult {
    RuntimeNavigationContext::to_alias_fallback(self, alias_unprefixed)
  }

  fn to_parent(&mut self) -> NavigateResult {
    RuntimeNavigationContext::to_parent(self)
  }

  fn to_child(&mut self, component: &str) -> NavigateResult {
    RuntimeNavigationContext::to_child(self, component)
  }

  fn get_config_status(&self) -> ConfigStatus {
    RuntimeNavigationContext::get_config_status(self)
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    RuntimeNavigationContext::get_config_behavior(self)
  }

  fn get_alias(&self, alias: &str) -> Option<String> {
    RuntimeNavigationContext::get_alias(self, alias)
  }

  fn get_config(&self) -> Option<String> {
    RuntimeNavigationContext::get_config(self)
  }

  fn luau_config_init(&self) -> Option<Rc<dyn Fn(*mut lua_State)>> {
    let config = self.config;
    let ctx = self.ctx;
    let timer = &self.timer as *const RuntimeLuauConfigTimer as *mut RuntimeLuauConfigTimer;

    Some(Rc::new(move |l: *mut lua_State| unsafe {
      let timeout = if let Some(get_timeout) = (*config).get_luau_config_timeout {
        get_timeout(l as *mut c_void, ctx)
      } else {
        DEFAULT_LUAU_CONFIG_TIMEOUT_MS
      };

      (*timer).start(timeout);
      lua_setthreaddata(l, timer as *mut c_void);
    }))
  }

  fn luau_config_interrupt(
    &self,
  ) -> Option<unsafe extern "C-unwind" fn(l: *mut lua_State, gc: i32)> {
    Some(runtime_luau_config_interrupt)
  }
}

/// 配置执行中断回调：超时则报错（对应 C++ `luauConfigInterrupt`）。
unsafe extern "C-unwind" fn runtime_luau_config_interrupt(l: *mut lua_State, _gc: i32) {
  unsafe {
    let timer = lua_getthreaddata(l) as *const RuntimeLuauConfigTimer;
    if !timer.is_null() && (*timer).is_finished() {
      lua_l_error_l(
        l,
        c"configuration execution timed out".as_ptr(),
        format_args!("configuration execution timed out"),
      );
    }
  }
}
