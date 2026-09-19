//! `NavigationContextTrait` impl wiring `FileNavigationContext` to the per-method
//! ports of `CLI/src/AnalyzeRequirer.cpp`. The C++ `: NavigationContext` base
//! relationship is realized here, in lieu of struct embedding.

use alloc::{rc::Rc, vec::Vec};
use core::ffi::{c_int, c_void};

use ulua_require::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
  },
  records::navigation_context::NavigationContextTrait,
};
use ulua_vm::{
  functions::lua_setthreaddata::lua_setthreaddata, type_aliases::lua_state::lua_State,
};

use crate::{
  methods::cli_config_resolver_read_config_rec,
  records::{
    file_navigation_context::FileNavigationContext,
    luau_config_interrupt_info::LuauConfigInterruptInfo,
  },
};
impl NavigationContextTrait for FileNavigationContext {
  fn reset_to_requirer(&mut self) -> NavigateResult {
    FileNavigationContext::reset_to_requirer(self)
  }

  fn jump_to_alias(&mut self, path: &[u8]) -> NavigateResult {
    FileNavigationContext::jump_to_alias(self, path)
  }

  fn to_parent(&mut self) -> NavigateResult {
    FileNavigationContext::to_parent(self)
  }

  fn to_child(&mut self, component: &[u8]) -> NavigateResult {
    FileNavigationContext::to_child(self, component)
  }

  fn get_config_status(&self) -> ConfigStatus {
    FileNavigationContext::get_config_status(self)
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    FileNavigationContext::get_config_behavior(self)
  }

  fn get_alias(&self, alias: &[u8]) -> Option<Vec<u8>> {
    FileNavigationContext::get_alias(self, alias)
  }

  fn get_config(&self) -> Option<Vec<u8>> {
    FileNavigationContext::get_config(self)
  }

  /// C++ `navigationContext.luauConfigInit = [&info](lua_State* l) { lua_setthreaddata(l, &info); };`
  /// (`CLI/src/Analyze.cpp:194-197`).
  fn luau_config_init(&self) -> Option<Rc<dyn Fn(*mut lua_State)>> {
    let info_ptr =
      self.interrupt_info.as_ref()?.as_ref() as *const LuauConfigInterruptInfo as *mut c_void;
    Some(Rc::new(move |l: *mut lua_State| unsafe {
      lua_setthreaddata(l, info_ptr);
    }))
  }

  /// C++ `navigationContext.luauConfigInterrupt = [](lua_State* l, int gc) { ... };`
  /// (`CLI/src/Analyze.cpp:198-205`) — identical body to the config-resolver interrupt.
  fn luau_config_interrupt(
    &self,
  ) -> Option<unsafe extern "C-unwind" fn(l: *mut lua_State, gc: c_int)> {
    self.interrupt_info.as_ref()?;
    Some(cli_config_resolver_read_config_rec::luau_config_interrupt)
  }
}
