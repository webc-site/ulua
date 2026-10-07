use alloc::vec::Vec;
use core::{cell::Cell, ffi::c_void, ptr::NonNull};

use coarsetime::Instant;
use ulua_vm::{
  functions::lua_getthreaddata::lua_getthreaddata, macros::lua_l_error::luaL_error,
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

  /// 读配置超时、启动计时器，交出计时器地址作为 VM 线程数据槽的转手载荷
  /// （cpp `RuntimeNavigationContext` 的 `luauConfigInit` 回调体的 Rust 形态）。
  ///
  /// 别名论证：timer 可变字段由 `Cell` 承载且 Luau 状态机单线程串行使用；
  /// 地址经 `NonNull::from` 只读取址，写权限由 `Cell` 提供，交出的指针只会被
  /// `extract_config` 原样挂进线程数据槽、本模块内不解引用。
  ///
  /// DELIBERATE DEVIATION（review.md §0）：cpp 的 init 回调在
  /// `executeAndExtractConfig` 内被调用时才启动计时器；此处函数指针抽象
  /// 收敛为载荷转手后，启动点前移到交出载荷（构造 `InterruptCallbacks`）
  /// 之时，超时窗口起点略早于 cpp（多出沙箱建机/装载耗时，判定更保守），
  /// 配置执行全程仍被覆盖。
  fn start_config_timer(&self) -> NonNull<c_void> {
    let timeout = self.host.get_luau_config_timeout();
    self.timer.start(timeout);
    // 只读取址、非空由 `NonNull` 类型承载（review.md §2：不再交 `*mut c_void` 让
    // 消费侧自行判空）
    NonNull::from(&self.timer).cast::<c_void>()
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

  fn luau_config_thread_data(&self) -> Option<NonNull<c_void>> {
    // 存活论证：载荷指向本上下文的 timer，仅在
    // navigate_to_and_populate_config 同步调用 extract_luau_config 的窗口内
    // 被挂接与读取，该窗口由 resolve_require 调用栈保证本导航上下文存活。
    Some(self.start_config_timer())
  }

  fn luau_config_interrupt(
    &self,
  ) -> Option<unsafe extern "C-unwind" fn(l: *mut LuaState, gc: i32)> {
    Some(runtime_luau_config_interrupt)
  }
}

/// 配置执行中断回调：超时则报错（对应 C++ `luauConfigInterrupt`）。
///
/// 保留 `unsafe extern "C-unwind"`：本函数是 VM 中断钩子的 Lua/C 回调形态（签名由
/// ulua-vm 的 `lua_CInterrupt` 约定钉死），`l` 由 VM 交回且被解引用（判定 1）。
///
/// # Safety
/// - `l`：必须指向存活的 `LuaState`，本函数只应作为 VM 的中断回调，由 VM 在配置
///   线程执行期间以当前状态调用；
/// - 线程数据槽：内容只可能为 null，或 `luau_config_thread_data` 交出、
///   `extract_config` 在本次配置执行窗口内挂接的存活
///   `RuntimeLuauConfigTimer` 地址（上下文由 `resolve_require` 的调用栈保活）；
/// - `_gc`：仅按 Lua/C 中断约定占位，不被读。
unsafe extern "C-unwind" fn runtime_luau_config_interrupt(l: *mut LuaState, _gc: i32) {
  // Safety: 中断回调入口，l 由 VM 在配置线程仍在执行时以当前 LuaState* 调用
  // （Lua/C API 中断约定），重建只读驱动的借用。
  let l = unsafe { &mut *l };
  // 线程数据槽的可空载荷收编为 `Option<NonNull<_>>`（review.md §2）：null 即「本次
  // 配置执行未挂计时器」，缺席态由类型表达而非裸指针判空。
  let Some(timer) = NonNull::new(lua_getthreaddata(l).cast::<RuntimeLuauConfigTimer>()) else {
    return;
  };
  // Safety: 见函数 # Safety——槽内容只可能是 `luau_config_thread_data` 交出、
  // extract_config 在本次配置执行期写入的存活 timer 地址；as_ref 之后仅做只读
  // is_finished()（可变性由 Cell 承载），无悬挂、无别名写。
  if unsafe { timer.as_ref() }.is_finished() {
    luaL_error!(l, "{CONFIG_TIMEOUT_MSG}");
  }
}
