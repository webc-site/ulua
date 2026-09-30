use alloc::string::String;
use core::ptr::NonNull;

use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_callbacks::lua_callbacks, lua_close::lua_close, lua_l_newstate::lua_l_newstate,
    lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
  },
  macros::lua_memerrmsg::LUA_MEMERRMSG_STR,
  records::lua_state::LuaState,
};

use crate::{
  error::ConfigError,
  functions::{load::load, lua_string::lua_string, serialize_table::serialize_table},
  records::{config_table::ConfigTable, interrupt_callbacks::InterruptCallbacks},
};

// `lua_resume` 返回码（对应 C++ switch 分支）
const RESUME_OK: i32 = LuaStatus::Ok as i32;
const RESUME_BREAK: i32 = LuaStatus::Break as i32;
const RESUME_YIELD: i32 = LuaStatus::Yield as i32;

/// 沙箱 VM 状态守卫，Drop 时自动 `lua_close`。
///
/// 不变量：仅持有 `lua_l_newstate` 成功返回的非空有效状态
/// （`NonNull` 钉死，内存耗尽路径不构造本守卫）。
pub(crate) struct StateGuard(pub(crate) NonNull<LuaState>);

impl StateGuard {
  /// 守卫持有的唯一可变借用：本 crate 内的装载/执行窗口一律经此
  /// `&mut LuaState` 走安全栈 API，裸指针不外泄。
  pub(crate) fn state(&mut self) -> &mut LuaState {
    // Safety: 按不变量，指针必为 lua_l_newstate 创建且未被 close 的有效 VM 状态，
    // 且 `&mut self` 保证本窗口内无其他别名借用。
    unsafe { self.0.as_mut() }
  }
}

impl Drop for StateGuard {
  fn drop(&mut self) {
    // Safety: 按不变量，指针必为 lua_l_newstate 创建且未被 close 的有效 VM 状态
    unsafe { lua_close(self.0.as_ptr()) };
  }
}

/// 创建沙箱 VM（对应 C++ `luaL_newstate + luaL_openlibs + luaL_sandbox`）。
/// `luaL_newstate` 内存耗尽返回 null 时经 `Option` 回报失败，不解引用。
fn new_sandbox() -> Option<StateGuard> {
  let state = NonNull::new(lua_l_newstate())?;

  // 先入守卫再开库：openlibs/sandbox 自身可抛错（VVM 错误通道即 OOM），
  // 守卫保证展开路径 lua_close——对齐 cpp LuauConfig.cpp:145 的 closing
  // unique_ptr 先于 openlibs 构造的顺序
  let guard = StateGuard(state);

  // Safety: null 已被 NonNull::new 上界拒绝，state 为 lua_l_newstate
  // 刚创建的有效 VM 状态，可安全开库与沙箱化
  unsafe {
    lua_l_openlibs(state.as_ptr());
    lua_l_sandbox(state.as_ptr());
  }

  Some(guard)
}

/// 对应 C++ `executeAndExtractConfig`：跑中断回调、resume 并校验返回表，
/// 错误经 `Result` 传播（`?`）。
///
/// `l` 契约：须是 [`StateGuard`] 持有的有效 VM 状态且已加载配置脚本；
/// `unsafe` 仅存于回调对/中断槽两处 FFI 形态调用点，栈操作全部走安全 API。
///
/// 唯一调用方是本模块的 [`run_in_sandbox`]（沙箱生命周期由它持有）。
fn execute_and_extract(
  l: &mut LuaState,
  callbacks: &InterruptCallbacks,
) -> Result<ConfigTable, ConfigError> {
  if let Some(init) = callbacks.init_callback {
    // Safety: `ConfigInitCallback` 契约——`userdata` 指向配置执行同步窗口内
    // 存活的数据，VM 状态是本函数调用方（run_in_sandbox）守卫持有的有效状态；
    // 回调仅在本调用点使用指针，返回后不再保留。
    unsafe { (init.callback)(l.as_mut_ptr(), init.userdata) };
  }

  // Safety: l 为 new_sandbox 返回的有效 VM 状态；
  // lua_callbacks 对有效状态恒返回非空回调表指针；interrupt 槽签名为
  // `extern "C-unwind"` fn 指针，与 InterruptCallbacks::interrupt_callback 类型一致
  unsafe { (*lua_callbacks(l.as_mut_ptr())).interrupt = callbacks.interrupt_callback };

  // 沙箱 VM 主协程首启无父调用方，走 `resume_main`（C 契约 `from == NULL` 的
  // 专用安全入口，见 ulua-vm `LuaState::resume_main`）。
  match l.resume_main(0) {
    RESUME_OK => {}
    // 暂不支持调试中断
    RESUME_BREAK | RESUME_YIELD => {
      return Err(ConfigError::CannotYield);
    }
    // resume 出错时栈顶为错误字符串
    _ => return Err(ConfigError::Message(lua_string(l, -1))),
  }

  if l.get_top() != 1 {
    return Err(ConfigError::NotExactlyOneValue);
  }

  if !l.is_table(-1) {
    return Err(ConfigError::NotATable);
  }

  // 栈顶（-1）为已校验的返回表
  serialize_table(l)
}

/// 沙箱一条链的公共骨架（cpp `extractConfig` / `extractLuauConfigFromBytecode`
/// 共用的八行样板收口一处）：建沙箱 VM → 由 `load` 装载 → 执行并提取配置表。
///
/// DELIBERATE DEVIATION（吸收裁定）：cpp `extractConfig` 尾段由
/// `FFlag::LuauRbsConfigAliasResolution`（LuauConfig.cpp:23）门控走
/// `executeAndExtractConfig`，flag 关分支为旧执行序列；Rust 侧按 flag-ON
/// 路径固化、不经 fflag 注册表（同族已登记的有 LuauCyclicRequireShortCircuit/
/// LuauSelfIsSelfAndAlwaysSelf）——cpp 两分支行为恒等（纯重构门控，
/// LuauConfig.cpp:153/341），见 review.md §0 偏差锚点纪律。
///
/// `load` 收守卫持有的 `&mut LuaState`，返回 `Some(错误串)` 表示装载失败；
/// 沙箱由 [`StateGuard`] 在本函数返回时关闭，裸指针不出本模块。
pub(crate) fn run_in_sandbox(
  load: impl FnOnce(&mut LuaState) -> Option<String>,
  callbacks: &InterruptCallbacks,
) -> Result<ConfigTable, ConfigError> {
  let Some(mut state) = new_sandbox() else {
    return Err(ConfigError::Message(
      // r7-tlossy1 收口：常量臂恒合法 UTF-8，直取 STR 门面（免 lossy 扫描；
      // lua_memerrmsg.rs 文件头即规定 BYTES 形只面向 `*const c_char` 指针契约，
      // Rust 侧展示用 STR——旧形误吞尾部 NUL，现与 `run_loaded_chunk.rs:82` 同款）。
      LUA_MEMERRMSG_STR.to_owned(),
    ));
  };

  if let Some(load_error) = load(state.state()) {
    return Err(ConfigError::Message(load_error));
  }

  execute_and_extract(state.state(), callbacks)
}

/// 对应 C++ `extractConfig`：在沙箱 VM 中执行 `source` 并提取返回的配置表。
/// 错误（含内存耗尽）以 [`ConfigError`] 携带，与内部
/// [`execute_and_extract`] 同一条传播路径。
pub fn extract_config(
  source: &str,
  callbacks: &InterruptCallbacks,
) -> Result<ConfigTable, ConfigError> {
  run_in_sandbox(|l| load(l, source), callbacks)
}
