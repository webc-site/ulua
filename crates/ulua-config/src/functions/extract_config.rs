use alloc::string::String;
use core::ptr::{NonNull, null_mut};

use ulua_vm::{
  enums::{lua_status::LuaStatus, lua_type::LuaType},
  functions::{
    lua_callbacks::lua_callbacks, lua_close::lua_close, lua_gettop::lua_gettop,
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
    lua_resume::lua_resume, lua_type::lua_type,
  },
  macros::lua_memerrmsg::LUA_MEMERRMSG,
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

// 结果表类型码
const T_TABLE: i32 = LuaType::Table as i32;

/// 沙箱 VM 状态守卫，Drop 时自动 `lua_close`。
///
/// 不变量：仅持有 `lua_l_newstate` 成功返回的非空有效状态
/// （`NonNull` 钉死，内存耗尽路径不构造本守卫）。
pub(crate) struct StateGuard(pub(crate) NonNull<LuaState>);

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
/// ulua-vm C-API 绑定边界：`l` 调用契约为须是 [`StateGuard`] 持有的有效
/// VM 状态且已加载配置脚本；裸指针仅在本函数的栈操作处出现，不外泄到返回值。
///
/// 唯一调用方是本模块的 [`run_in_sandbox`]（沙箱生命周期由它持有）。
fn execute_and_extract(
  l: *mut LuaState,
  callbacks: &InterruptCallbacks,
) -> Result<ConfigTable, ConfigError> {
  if let Some(init) = callbacks.init_callback {
    // Safety: `ConfigInitCallback` 契约——`userdata` 指向配置执行同步窗口内
    // 存活的数据，`l` 是本函数调用方（run_in_sandbox）持有的有效 VM 状态。
    unsafe { (init.callback)(l, init.userdata) };
  }

  // Safety: l 为 new_sandbox 返回的有效 VM 状态；
  // lua_callbacks 对有效状态恒返回非空回调表指针；interrupt 槽签名为
  // `extern "C-unwind"` fn 指针，与 InterruptCallbacks::interrupt_callback 类型一致
  unsafe { (*lua_callbacks(l)).interrupt = callbacks.interrupt_callback };

  // Safety: l 为有效 VM 状态且已加载配置脚本
  // 保留 null 的裁定（复评 b25）：`from` 是「父协程」入参而非出参，ulua-vm
  // `lua_resume` 的签名即 C-API 边界形态，其自身 # Safety 契约明文「from 可为
  // NULL（主状态恢复）」——本处沙箱 VM 主协程首启无父调用方，null 是唯一合法
  // 表达，非可改 Option 的 Rust 侧哨兵（与 cpp executeAndExtractConfig 一致）。
  match unsafe { lua_resume(l, null_mut(), 0) } {
    RESUME_OK => {}
    // 暂不支持调试中断
    RESUME_BREAK | RESUME_YIELD => {
      return Err(ConfigError::CannotYield);
    }
    // Safety: resume 出错时栈顶为错误字符串
    _ => return Err(ConfigError::Message(unsafe { lua_string(l, -1) })),
  }

  // Safety: l 为有效 VM 状态
  if unsafe { lua_gettop(l) } != 1 {
    return Err(ConfigError::NotExactlyOneValue);
  }

  // Safety: l 为有效 VM 状态
  if unsafe { lua_type(l, -1) } != T_TABLE {
    return Err(ConfigError::NotATable);
  }

  // Safety: 栈顶（-1）为已校验的返回表
  unsafe { serialize_table(l) }
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
/// `load` 收 VM 状态、返回 `Some(错误串)` 表示装载失败；沙箱由 [`StateGuard`]
/// 在本函数返回时关闭，裸指针不外泄。
pub(crate) fn run_in_sandbox(
  load: impl FnOnce(*mut LuaState) -> Option<String>,
  callbacks: &InterruptCallbacks,
) -> Result<ConfigTable, ConfigError> {
  let Some(state) = new_sandbox() else {
    return Err(ConfigError::Message(
      String::from_utf8_lossy(LUA_MEMERRMSG).into_owned(),
    ));
  };
  let l = state.0.as_ptr();

  if let Some(load_error) = load(l) {
    return Err(ConfigError::Message(load_error));
  }

  execute_and_extract(l, callbacks)
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
