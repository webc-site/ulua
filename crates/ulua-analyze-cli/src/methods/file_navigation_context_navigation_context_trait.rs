//! `NavigationContextTrait` impl wiring `FileNavigationContext` to the per-method
//! ports of `CLI/src/AnalyzeRequirer.cpp`. The C++ `: NavigationContext` base
//! relationship is realized here, in lieu of struct embedding.

use alloc::vec::Vec;
use core::ffi::{c_int, c_void};

use ulua_config::records::interrupt_callbacks::{ConfigInitCallback, attach_threaddata_init};
use ulua_require::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
  },
  records::navigation_context::NavigationContextTrait,
};
use ulua_vm::records::lua_state::LuaState;

use crate::{
  methods::cli_config_resolver_read_config_rec,
  records::{
    file_navigation_context::FileNavigationContext,
    luau_config_interrupt_info::LuauConfigInterruptInfo,
  },
};
impl NavigationContextTrait for FileNavigationContext {
  // 八枚纯转发臂经 forward_nav_trait! 单源生成（同名固有方法在各
  // file_navigation_context_*.rs 文件）；alias override/fallback 走 trait 缺省。
  ulua_require::forward_nav_trait!(mut reset_to_requirer() -> NavigateResult);
  ulua_require::forward_nav_trait!(mut jump_to_alias(path) -> NavigateResult);
  ulua_require::forward_nav_trait!(mut to_parent() -> NavigateResult);
  ulua_require::forward_nav_trait!(mut to_child(component) -> NavigateResult);
  ulua_require::forward_nav_trait!(get_config_status() -> ConfigStatus);
  ulua_require::forward_nav_trait!(get_config_behavior() -> ConfigBehavior);
  ulua_require::forward_nav_trait!(get_alias(alias) -> Option<Vec<u8>>);
  ulua_require::forward_nav_trait!(get_config() -> Option<Vec<u8>>);

  /// C++ `navigationContext.luauConfigInit = [&info](LuaState* l) { lua_setthreaddata(l, &info); };`
  /// (`CLI/src/Analyze.cpp:194-197`) — 以 `attach_threaddata_init` 具名函数指针
  /// + info 地址的静态分派对实现（原 `Rc<dyn Fn>` 闭包捕获即此一枚指针）。
  fn luau_config_init(&self) -> Option<ConfigInitCallback> {
    let info_ptr = self.interrupt_info.as_ref()?.as_ref() as *const LuauConfigInterruptInfo;
    Some(ConfigInitCallback {
      callback: attach_threaddata_init,
      // Safety 契约随 [`attach_threaddata_init`]：info 指针只作转手存储进 VM
      // 线程数据槽、不解引用；该槽仅在 `resolve_module` 的 navigate 窗口内被
      // 读写，此窗口由同一 `FileNavigationContext` 对 Box 的借用维持存活。
      userdata: info_ptr.cast::<c_void>().cast_mut(),
    })
  }

  /// C++ `navigationContext.luauConfigInterrupt = [](LuaState* l, int gc) { ... };`
  /// (`CLI/src/Analyze.cpp:198-205`) — identical body to the config-resolver interrupt.
  fn luau_config_interrupt(
    &self,
  ) -> Option<unsafe extern "C-unwind" fn(l: *mut LuaState, gc: c_int)> {
    self.interrupt_info.as_ref()?;
    Some(cli_config_resolver_read_config_rec::luau_config_interrupt)
  }
}
