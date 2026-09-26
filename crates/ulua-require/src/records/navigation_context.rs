use alloc::vec::Vec;
use core::{ffi::c_void, ptr::from_ref};

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
    luarequire_configuration::luarequire_Configuration,
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

  /// Luau 配置执行前的线程数据初始化回调。
  ///
  /// 返回 [`ConfigInitCallback`]（函数指针 + 转手 userdata 的静态分派对，
  /// 零分配零虚分派）：实现方给出的 `callback` 只会在 `extract_luau_config`
  /// 执行配置的同步窗口内被调用一次，`userdata` 指向该窗口内存活的数据即可。
  fn luau_config_init(&self) -> Option<ConfigInitCallback> {
    None
  }

  fn luau_config_interrupt(
    &self,
  ) -> Option<unsafe extern "C-unwind" fn(l: *mut LuaState, gc: i32)> {
    None
  }
}

/// 同名固有方法 → [`NavigationContextTrait`] 的机械转发单源：实现方把方法体
/// 放在类型自己的 impl 块（一文件一函数、对应 cpp 文件），trait impl 只按
/// 「可变性 + 方法名(参数名) -> 返回类型」单行声明，展开体把 `self`/参数原样
/// 交给 `Self::$name` 固有关联函数（固有候选优先于 trait 候选，无递归）。
/// 参数一律 `&[u8]`（路径/别名/组件的字节串约定）；`mut` 选 `&mut self`。
#[macro_export]
macro_rules! forward_nav_trait {
  (mut $name:ident($($arg:ident),*) -> $ret:ty) => {
    fn $name(&mut self, $($arg: &[u8]),*) -> $ret {
      Self::$name(self, $($arg),*)
    }
  };
  ($name:ident($($arg:ident),*) -> $ret:ty) => {
    fn $name(&self, $($arg: &[u8]),*) -> $ret {
      Self::$name(self, $($arg),*)
    }
  };
}

impl NavigationContextTrait for RuntimeNavigationContext<'_> {
  forward_nav_trait!(mut reset_to_requirer() -> NavigateResult);
  forward_nav_trait!(mut jump_to_alias(path) -> NavigateResult);
  forward_nav_trait!(mut to_alias_override(alias_unprefixed) -> NavigateResult);
  forward_nav_trait!(mut to_alias_fallback(alias_unprefixed) -> NavigateResult);
  forward_nav_trait!(mut to_parent() -> NavigateResult);
  forward_nav_trait!(mut to_child(component) -> NavigateResult);
  forward_nav_trait!(get_config_status() -> ConfigStatus);
  forward_nav_trait!(get_config_behavior() -> ConfigBehavior);
  forward_nav_trait!(get_alias(alias) -> Option<Vec<u8>>);
  forward_nav_trait!(get_config() -> Option<Vec<u8>>);

  fn luau_config_init(&self) -> Option<ConfigInitCallback> {
    // 静态分派：回调取具名函数指针，捕获数据改为把 `self`（其 config/timer/ctx
    // 字段即闭包原捕获项）地址作为 userdata 转手。存活论证：callback 仅在
    // navigate_to_and_populate_config 同步调用 extract_luau_config 的窗口内被
    // 触发，该窗口由 resolve_require 调用栈保证本导航上下文存活（与闭包版同义）。
    Some(ConfigInitCallback {
      callback: runtime_luau_config_init,
      // timer 是 `&self` 借出的只读引用，其内部可变字段用 Cell 承载，
      // 因此把地址交给 VM 线程数据不构成别名冲突；Luau 状态机单线程使用。
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
/// [`init_config_thread_data`]（原 `Rc` 闭包体，逐字保留语义）。
///
/// # Safety
/// `l` 必须指向执行配置的 VM 提供的存活 `LuaState`；`userdata` 必须是
/// `RuntimeNavigationContext::luau_config_init` 交出的本上下文地址，且仅在
/// `extract_luau_config` 同步执行配置的窗口内被调用（上下文由
/// `resolve_require` 的调用栈保活）。
unsafe fn runtime_luau_config_init(l: *mut LuaState, userdata: *mut c_void) {
  // Safety: 契约保证 userdata 指向存活的本体，且无人并发可变借用
  // （config/timer 仅共享读取，timer 可变字段由 Cell 承载）。
  let nav = unsafe { &*(userdata as *const RuntimeNavigationContext<'_>) };
  let config = from_ref(nav.config);
  let timer = from_ref(&nav.timer);
  // Safety: 满足 init_config_thread_data 的全部前提（见其 # Safety）。
  unsafe { init_config_thread_data(l, config, timer, nav.ctx) };
}

/// `luau_config_init` 回调的实体：读配置超时、启动计时器、把计时器地址交给
/// VM 线程数据槽（cpp `NavigationContext::luauConfigInit` 回调体）。
///
/// # Safety
/// `l` 必须指向执行配置的 VM 提供的存活 `LuaState`；`config`/`timer` 必须指向
/// 存活对象（导航上下文由 `resolve_require` 的调用栈在配置执行期间保活），
/// timer 可变字段由 Cell 承载且 Luau 状态机单线程串行使用；`ctx` 仅作转手
/// 存储的 lightuserdata，不解引用。
unsafe fn init_config_thread_data(
  l: *mut LuaState,
  config: *const luarequire_Configuration,
  timer: *const RuntimeLuauConfigTimer,
  ctx: *mut c_void,
) {
  // Safety: 契约保证 config/timer 指向存活对象：进入即绑定共享引用，后续只经
  // 引用读取字段，替代链式裸指针解引用；timer 可变性由 Cell 承载，无别名冲突。
  let config = unsafe { &*config };
  let timer = unsafe { &*timer };

  let timeout = if let Some(get_timeout) = config.get_luau_config_timeout {
    // Safety: l 在本次调用窗口内有效，ctx 仅转手给配置回调按其指针契约处理。
    unsafe { get_timeout(l.cast(), ctx) }
  } else {
    DEFAULT_LUAU_CONFIG_TIMEOUT_MS
  };

  timer.start(timeout);
  // Safety: `l` 按契约在本次调用窗口内独占驱动，`&mut *l` 排他引用重建前提成立；
  // 地址只是经 VM 的线程数据槽转手，写权限由 Cell 提供。
  unsafe { lua_setthreaddata(&mut *l, from_ref(timer).cast::<c_void>().cast_mut()) };
}

/// 配置执行超时文案（cpp `luauConfigInterrupt` 的唯一错误消息）。
const CONFIG_TIMEOUT_MSG: &str = "configuration execution timed out";

/// 配置执行中断回调：超时则报错（对应 C++ `luauConfigInterrupt`）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`：本函数只应作为 VM 的中断回调，由 VM 在
/// 配置线程执行期间以当前状态调用；线程数据槽内容只可能为 null，或
/// `luau_config_init` 在本次配置执行期间写入的存活 `RuntimeLuauConfigTimer`
/// 地址（上下文由 `resolve_require` 的调用栈保活）。
unsafe extern "C-unwind" fn runtime_luau_config_interrupt(l: *mut LuaState, _gc: i32) {
  // Safety: 本函数是注册给 VM 的中断回调，l 由 VM 在配置线程仍在执行时以
  // 当前 LuaState* 调用（Lua/C API 中断约定）。lua_getthreaddata 的返回值
  // 要么是 luau_config_init 刚写入的 &RuntimeLuauConfigTimer 地址（配置执行
  // 期间上下文存活），要么是 VM 未初始化时的 null；as_ref() 先判空，仅对
  // 存活 timer 做只读 is_finished()（Cell 承载可变性），不存在悬挂或别名写。
  unsafe {
    let timer = lua_getthreaddata(l).cast::<RuntimeLuauConfigTimer>();
    if timer.as_ref().is_some_and(|timer| timer.is_finished()) {
      luaL_error!(l, "{CONFIG_TIMEOUT_MSG}");
    }
  }
}
