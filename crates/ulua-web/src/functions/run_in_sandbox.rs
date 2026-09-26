//! 「建沙箱状态 → 跑码」骨架：cpp `CLI/src/Web.cpp:184-208`（`executeScript`）的
//! 调用序列。native C 入口与 wasm `#[wasm_bindgen] run` 两条入口在这段完全同构，
//! 唯一差别是「沙箱冻结全局表之前要不要装捕获版 `print`」——故收进一个带钩子的
//! 泛型函数，而不是两份逐行复制。

use std::string::String;

use ulua_vm::{
  functions::{
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
    lua_l_sandboxthread::lua_l_sandboxthread,
  },
  records::{lua_state::LuaState, lua_state_guard::LuaStateGuard},
};

use crate::{
  functions::run_code::run_code,
  util::{NOT_ENOUGH_MEMORY, init_default_flags},
};

/// 建一个新状态、开标准库、跑 `source`，全程镜像 cpp `executeScript`：
/// `setLuauFlagsDefault` → `luaL_newstate`（`lua_close` 守卫）→ `luaL_openlibs` →
/// `before_sandbox` → `luaL_sandbox` → `luaL_sandboxthread` → [`run_code`]。
///
/// 返回 [`run_code`] 的文本（成功即空串）；`luaL_newstate` 内存耗尽时返回
/// [`NOT_ENOUGH_MEMORY`]——两条入口对宿主的可观察输出本就在同一个「结果文本」
/// 通道上（`Web.cpp:207` 的 `result`），故不必额外造错误类型。
///
/// `before_sandbox` 是 openlibs 之后、`luaL_sandbox` 冻结全局表之前的安装钩子
/// （cpp 无此步，`Web.cpp:64-69` 的 `setupState` 只做 openlibs + sandbox）：wasm
/// 侧的捕获版 `print` 必须落在这个窗口，否则沙箱代理表建好后写不进全局。
pub(crate) fn run_in_sandbox(source: &str, before_sandbox: impl FnOnce(*mut LuaState)) -> String {
  // 沿用 ulua-common 的 CLI 语义（仅 `Luau*` 前缀且非实验性 bool FastFlag，见
  // `is_default_enabled_flag`）。已知偏差：本 crate 的 oracle `Web.cpp:185-189`
  // 实为内联全量置位循环（含 `LuauSolverV2` 等实验性旗标，每次调用重置）——
  // DELIBERATE DEVIATION：原生宿主「先 executeScript 后 checkScript」时分析旗标与
  // oracle 可观察不同；上游写在 per-call 位置是因为 C++ 宿主单线程，本入口是
  // `extern "C-unwind"` 可并发，故经 [`init_default_flags`] 一次性完成。
  init_default_flags();

  // std::unique_ptr<LuaState, void (*)(LuaState*)> globalState(luaL_newstate(), lua_close)
  let global_state = LuaStateGuard(lua_l_newstate());
  let l: *mut LuaState = global_state.0;

  // cpp 直接把可能为 null 的状态交给 setupState，这里改为显式回报错误，
  // 不解引用空指针（wasm 侧分配器在内存耗尽时确实返回 NULL）。
  if l.is_null() {
    return NOT_ENOUGH_MEMORY.to_owned();
  }

  // Safety: `l` 已由上方判空保证是本函数独占的活跃状态机，且直到本函数返回才由
  // `global_state` 守卫 `lua_close`；`before_sandbox` 只在 openlibs 之后、沙箱冻结
  // 全局表之前取得该指针，不跨调用持有（钩子安装的全局由状态自身拥有）。
  // 调用序与 cpp `executeScript` 一致：openlibs →（安装钩子）→ sandbox →
  // sandboxthread，三者正是 [`run_code`] 的全部前置条件。
  unsafe { lua_l_openlibs(l) };
  before_sandbox(l);
  unsafe {
    lua_l_sandbox(l);
    lua_l_sandboxthread(l);
  }

  // Safety: 前置（openlibs / 钩子 / sandbox / sandboxthread）恰由上三步成立；
  // `source` 借用的宿主缓冲区在本次调用内持续有效，`run_code` 不保留该借用。
  unsafe { run_code(l, source) }
}
