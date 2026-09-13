//! `NavigationContextTrait` impl wiring `FileNavigationContext` to the per-method
//! ports of `CLI/src/AnalyzeRequirer.cpp`. The C++ `: NavigationContext` base
//! relationship is realized here, in lieu of struct embedding.

use alloc::{rc::Rc, string::String};
use core::ffi::{c_int, c_void};

use ulua_require::{
  enums::{config_behavior::ConfigBehavior, navigate_result::NavigateResult},
  records::navigation_context::{ConfigStatus, NavigationContextTrait},
};
use ulua_vm::{
  functions::lua_setthreaddata::lua_setthreaddata, type_aliases::lua_state::lua_State,
};

use crate::{
  methods::{
    cli_config_resolver_read_config_rec,
    file_navigation_context_get_alias::file_navigation_context_get_alias,
    file_navigation_context_get_config::file_navigation_context_get_config,
    file_navigation_context_get_config_behavior::file_navigation_context_get_config_behavior,
    file_navigation_context_get_config_status::file_navigation_context_get_config_status,
    file_navigation_context_jump_to_alias::file_navigation_context_jump_to_alias,
    file_navigation_context_reset_to_requirer::file_navigation_context_reset_to_requirer,
    file_navigation_context_to_child::file_navigation_context_to_child,
    file_navigation_context_to_parent::file_navigation_context_to_parent,
  },
  records::{
    file_navigation_context::FileNavigationContext,
    luau_config_interrupt_info::LuauConfigInterruptInfo,
  },
};
impl NavigationContextTrait for FileNavigationContext {
  fn reset_to_requirer(&mut self) -> NavigateResult {
    unsafe { file_navigation_context_reset_to_requirer(self) }
  }

  fn jump_to_alias(&mut self, path: &str) -> NavigateResult {
    unsafe { file_navigation_context_jump_to_alias(self, path) }
  }

  fn to_parent(&mut self) -> NavigateResult {
    unsafe { file_navigation_context_to_parent(self) }
  }

  fn to_child(&mut self, component: &str) -> NavigateResult {
    unsafe { file_navigation_context_to_child(self, component) }
  }

  fn get_config_status(&self) -> ConfigStatus {
    unsafe { file_navigation_context_get_config_status(self) }
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    unsafe { file_navigation_context_get_config_behavior(self) }
  }

  fn get_alias(&self, alias: &str) -> Option<String> {
    unsafe { file_navigation_context_get_alias(self, &String::from(alias)) }
  }

  fn get_config(&self) -> Option<String> {
    unsafe { file_navigation_context_get_config(self) }
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
