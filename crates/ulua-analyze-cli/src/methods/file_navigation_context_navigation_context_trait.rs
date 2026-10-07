//! `NavigationContext` impl wiring `FileNavigationContext` to the per-method
//! ports of `CLI/src/AnalyzeRequirer.cpp`. The C++ `: NavigationContext` base
//! relationship is realized here, in lieu of struct embedding.

use alloc::vec::Vec;
use core::ffi::c_void;

use ulua_require::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
  },
  records::navigation_context::NavigationContext,
};
use ulua_vm::records::lua_state::LuaState;

use crate::{
  methods::cli_config_resolver_read_config_rec,
  records::{
    file_navigation_context::FileNavigationContext,
    luau_config_interrupt_info::LuauConfigInterruptInfo,
  },
};
impl NavigationContext for FileNavigationContext {
  // 八枚纯转发臂经 forward_nav_trait! 单源生成（同名固有方法在各
  // file_navigation_context_*.rs 文件）；alias override/fallback 走 trait 缺省。
  ulua_require::forward_nav_trait!(reset_to_requirer() -> NavigateResult);
  ulua_require::forward_nav_trait!(jump_to_alias(path) -> NavigateResult);
  ulua_require::forward_nav_trait!(to_parent() -> NavigateResult);
  ulua_require::forward_nav_trait!(to_child(component) -> NavigateResult);
  ulua_require::forward_nav_trait!(get_config_status() -> ConfigStatus);
  ulua_require::forward_nav_trait!(get_config_behavior() -> ConfigBehavior);
  ulua_require::forward_nav_trait!(get_alias(alias) -> Option<Vec<u8>>);
  ulua_require::forward_nav_trait!(get_config() -> Option<Vec<u8>>);

  /// C++ `navigationContext.luauConfigInit = [&info](LuaState* l) { lua_setthreaddata(l, &info); };`
  /// (`CLI/src/Analyze.cpp:194-197`) — 原闭包捕获即此一枚 info 指针；Rust 侧
  /// 直接交出该地址，由 `extract_config` 在配置执行窗口前单点挂接。
  fn luau_config_thread_data(&self) -> Option<*mut c_void> {
    // 真边界：返回的 `*mut c_void` 是 VM lightuserdata 线程数据槽
    // （`InterruptCallbacks::thread_data`）的转手地址形态，只挂接、不解引用；
    // 该槽仅在 `resolve_module` 的 navigate 窗口内被读写，此窗口由同一
    // `FileNavigationContext` 对 Box 的借用维持存活。
    let info_ptr = self.interrupt_info.as_ref()?.as_ref() as *const LuauConfigInterruptInfo;
    Some(info_ptr.cast_mut().cast())
  }

  /// C++ `navigationContext.luauConfigInterrupt = [](LuaState* l, int gc) { ... };`
  /// (`CLI/src/Analyze.cpp:198-205`) — identical body to the config-resolver interrupt.
  fn luau_config_interrupt(
    &self,
  ) -> Option<unsafe extern "C-unwind" fn(l: *mut LuaState, gc: i32)> {
    // 真边界：返回的指针最终写入 VM `LuaCallbacks::interrupt` 槽（`extern
    // "C-unwind"`，safepoint 处按 C ABI 调用），ABI 与形参形态随槽位契约保留；
    // `gc` 直接用槽位声明的原生 `i32`（同 trait 声明，`c_int` 即其别名）。
    self.interrupt_info.as_ref()?;
    Some(cli_config_resolver_read_config_rec::luau_config_interrupt)
  }
}
