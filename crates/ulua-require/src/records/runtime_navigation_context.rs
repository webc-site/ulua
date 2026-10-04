use alloc::vec::Vec;
use core::{cell::Cell, ffi::c_void, ptr::from_ref};

use coarsetime::Instant;
use ulua_config::records::interrupt_callbacks::ConfigInitCallback;
use ulua_vm::{
  functions::{lua_getthreaddata::lua_getthreaddata, lua_setthreaddata::lua_setthreaddata},
  macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

use crate::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
  },
  records::{
    navigation_context::{NavigationContext, RequireHost},
    runtime_luau_config_timer::RuntimeLuauConfigTimer,
  },
};

/// 运行时导航上下文：对应 cpp `Require/src/Navigation.h` 的
/// `RuntimeNavigationContext`——把 require 宿主机（[`RequireHost`]）适配成
/// [`NavigationContext`] 供 [`crate::records::navigator::Navigator`] 驱动，
/// 并持有 Luau 配置执行计时器与本次 require 的 requirer chunkname。
///
/// cpp 里该适配层的全部 NUL 串/缓冲 out 参样板都源自 `luarequire_Configuration`
/// 的 C ABI；Rust 侧接口直接收字节串、按值返回，本类型只剩薄转接与计时器装配。
///
/// 宿主以泛型参数 `H` 静态注入（`resolve_require::<H>` 由 `lua_requireinternal::<C>`
/// 的单态化链传入 `C`），转接即直接调用，零 `dyn`、零虚分派。
pub(crate) struct RuntimeNavigationContext<'ctx, H: RequireHost> {
  /// 借自 `HostSlot<H>`（按宿主类型单态化的装箱槽位）；本类型只转接该共享引用。
  host: &'ctx H,
  /// 本次 require 的发起方 chunkname 字节串（Lua 字符串非 UTF-8），
  /// 由调用方借用、导航期间存活（cpp `requirerChunkname` 成员）。
  requirer_chunkname: &'ctx [u8],
  timer: RuntimeLuauConfigTimer,
}

/// 配置执行超时文案（cpp `luauConfigInterrupt` 的唯一错误消息）。
const CONFIG_TIMEOUT_MSG: &str = "configuration execution timed out";

impl<'ctx, H: RequireHost> RuntimeNavigationContext<'ctx, H> {
  /// `host` 为 `borrowed_host::<H>` 从 `HostSlot<H>` 借出的宿主共享引用。
  pub(crate) fn new(host: &'ctx H, requirer_chunkname: &'ctx [u8]) -> Self {
    Self {
      host,
      requirer_chunkname,
      timer: RuntimeLuauConfigTimer {
        start_time: Cell::new(Instant::now()),
        timeout_duration: Cell::new(None),
      },
    }
  }

  /// 当前层级是否指向模块（cpp `isModulePresent`，供 resolve 收尾直用）。
  pub(crate) fn is_module_present(&self) -> bool {
    self.host.is_module_present()
  }

  /// 当前模块的 chunkname 字节串（cpp `getChunkname`）。
  pub(crate) fn get_chunkname(&self) -> Option<Vec<u8>> {
    self.host.get_chunkname()
  }

  /// 当前模块的 loadname 字节串（cpp `getLoadname`）：可能含非 UTF-8 字节。
  pub(crate) fn get_loadname(&self) -> Option<Vec<u8>> {
    self.host.get_loadname()
  }

  /// 当前模块的缓存键字节串（cpp `getCacheKey`）：可能含非 UTF-8 字节。
  pub(crate) fn get_cache_key(&self) -> Option<Vec<u8>> {
    self.host.get_cache_key()
  }

  /// 读配置超时、启动计时器、把计时器地址交给 VM 线程数据槽
  /// （cpp `RuntimeNavigationContext` 的 `luauConfigInit` 回调体）。
  ///
  /// 别名论证（原 `# Safety` 的可变字段部分）：timer 可变字段由 `Cell` 承载且
  /// Luau 状态机单线程串行使用，`l` 在本次调用窗口内由调用方以 `&mut` 独占
  /// 驱动；`lua_setthreaddata` 只是把 timer 地址转手给 VM 数据槽（地址经
  /// `from_ref` 只读借出，写权限由 Cell 提供）。
  fn start_config_timer(&self, l: &mut LuaState) {
    let timeout = self.host.get_luau_config_timeout();
    self.timer.start(timeout);
    lua_setthreaddata(l, from_ref(&self.timer).cast::<c_void>().cast_mut());
  }
}

impl<H: RequireHost> NavigationContext for RuntimeNavigationContext<'_, H> {
  fn reset_to_requirer(&self) -> NavigateResult {
    self.host.reset_to_requirer(self.requirer_chunkname)
  }

  fn jump_to_alias(&self, alias_path: &[u8]) -> NavigateResult {
    self.host.jump_to_alias(alias_path)
  }

  fn to_alias_override(&self, alias_unprefixed: &[u8]) -> NavigateResult {
    self.host.to_alias_override(alias_unprefixed)
  }

  fn to_alias_fallback(&self, alias_unprefixed: &[u8]) -> NavigateResult {
    self.host.to_alias_fallback(alias_unprefixed)
  }

  fn to_parent(&self) -> NavigateResult {
    self.host.to_parent()
  }

  fn to_child(&self, component: &[u8]) -> NavigateResult {
    self.host.to_child(component)
  }

  fn get_config_status(&self) -> ConfigStatus {
    self.host.get_config_status()
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    self.host.get_config_behavior()
  }

  fn get_alias(&self, alias: &[u8]) -> Option<Vec<u8>> {
    self.host.get_alias(alias)
  }

  fn get_config(&self) -> Option<Vec<u8>> {
    self.host.get_config()
  }

  fn luau_config_init(&self) -> Option<ConfigInitCallback> {
    // 静态分派：回调取按宿主类型 `H` 单态化的具名函数指针（`runtime_luau_config_init
    // ::<H>` coerce 为 `ConfigInitFn`），捕获数据为 `self` 地址（timer/host/
    // requirer_chunkname 即原闭包捕获项）。存活论证：callback 仅在
    // navigate_to_and_populate_config 同步调用 extract_luau_config 的窗口内被
    // 触发，该窗口由 resolve_require 调用栈保证本导航上下文存活。
    Some(ConfigInitCallback {
      callback: runtime_luau_config_init::<H>,
      // timer 是 `&self` 借出的只读引用，其内部可变字段用 Cell 承载，
      // 因此把地址交给 VM 线程数据槽不构成别名冲突；Luau 状态机单线程使用。
      userdata: from_ref(self).cast::<c_void>().cast_mut(),
    })
  }

  fn luau_config_interrupt(
    &self,
  ) -> Option<unsafe extern "C-unwind" fn(l: *mut LuaState, gc: i32)> {
    Some(runtime_luau_config_interrupt)
  }
}

/// `luau_config_init` 的具名回调：从 userdata 取回导航上下文本体，转交
/// 计时器启动（原 `Rc<dyn Fn>` 闭包体，逐字保留语义；`H` 由交出回调的
/// `luau_config_init` 与本体同一单态化实例，读回类型必然一致）。
///
/// # Safety
/// `l` 必须指向执行配置的 VM 提供的存活 `LuaState`；`userdata` 必须是
/// [`RuntimeNavigationContext`] 的 `luau_config_init` 交出的本上下文地址（即
/// `RuntimeNavigationContext<'_, H>` 本体，`H` 与本实例化一致），且仅在配置执行的
/// 同步窗口内被调用（上下文由 `resolve_require` 的调用栈保活）。
unsafe fn runtime_luau_config_init<H: RequireHost>(l: *mut LuaState, userdata: *mut c_void) {
  // Safety: 契约保证 userdata 指向存活的本体，且无人并发可变借用
  // （host/timer 仅共享读取，timer 可变字段由 Cell 承载）。
  let nav = unsafe { &*userdata.cast::<RuntimeNavigationContext<'_, H>>() };
  // Safety: l 与本次配置执行窗口同存活（fn 契约），重建独占借用。
  let l = unsafe { &mut *l };
  nav.start_config_timer(l);
}

/// 配置执行中断回调：超时则报错（对应 C++ `luauConfigInterrupt`）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`：本函数只应作为 VM 的中断回调，由 VM 在
/// 配置线程执行期间以当前状态调用；线程数据槽内容只可能为 null，或
/// `luau_config_init` 在本次配置执行期间写入的存活 `RuntimeLuauConfigTimer`
/// 地址（上下文由 `resolve_require` 的调用栈保活）。
unsafe extern "C-unwind" fn runtime_luau_config_interrupt(l: *mut LuaState, _gc: i32) {
  // Safety: 中断回调入口，l 由 VM 在配置线程仍在执行时以当前 LuaState* 调用
  // （Lua/C API 中断约定），重建只读驱动的借用。
  let l = unsafe { &mut *l };
  // 线程数据槽内容只能为 null，或 luau_config_init 在本次配置执行期间写入的
  // 存活 timer 地址（上下文由 resolve_require 调用栈保活）；as_ref() 先判空，
  // 仅对存活 timer 做只读 is_finished()（Cell 承载可变性），无悬挂或别名写。
  let timer = lua_getthreaddata(l).cast::<RuntimeLuauConfigTimer>();
  // Safety: 见函数 # Safety：槽内容为 null 或本次配置执行期写入的存活 timer
  // 地址，as_ref 判空后仅做只读 is_finished（Cell 承载可变性）。
  if unsafe { timer.as_ref() }.is_some_and(|timer| timer.is_finished()) {
    luaL_error!(l, "{CONFIG_TIMEOUT_MSG}");
  }
}
