use ulua_common::fflag::LuauCyclicRequireShortCircuit;
use ulua_vm::{
  macros::{lua_l_error::luaL_error, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::LuaState,
};

use crate::functions::{
  cache_table_keys::{CYCLIC_PLACEHOLDER_PROVIDED_KEY, REQUIRED_CACHE_TABLE_KEY},
  cyclic_placeholder::populate_placeholder,
  registry_table::{
    cache_stack_value, get_table_field, push_registry_table, set_table_field, take_registry_mark,
  },
};

/// require 挂起前的固定栈槽数（cacheKey/chunkname/loadname + 路径）。
pub(crate) const REQUIRE_STACK_VALUES: i32 = 4;

/// 把结果值按 cacheKey 写进已在栈顶的 `_MODULES` 表（cpp `lua_requirecont` 非
/// 短路缓存路径的收尾三连）：进入时栈顶为缓存表、其下为结果值，结束后缓存表
/// 弹掉、结果回到栈顶。
fn cache_result_under_key(l: &mut LuaState, result_idx: i32, cache_key: &[u8]) {
  l.push_value(result_idx);
  set_table_field(l, -2, cache_key);
  l.pop(1);
}

/// cpp `lua_requirecont` provided 分支：复核结果为表后取回缓存的占位表、把模块
/// 结果灌进占位表，并用灌满后的占位表替换栈上结果——让首个调用方与后续缓存命中
/// 拿到的是循环各方持有的同一张表。
fn adopt_provided_placeholder(l: &mut LuaState, cache_key: &[u8], result_idx: i32) {
  ulua_common::LUAU_ASSERT!(l.is_table(result_idx));
  push_registry_table(l, REQUIRED_CACHE_TABLE_KEY);
  get_table_field(l, -1, cache_key);
  populate_placeholder(l, -1, result_idx);
  l.replace(result_idx);
  l.pop(1);
}

/// cpp `lua_requirecont` 的单结果缓存收尾：恰有一个结果时把模块结果写进 `_MODULES`
/// 缓存表（开启循环短路时优先复用/灌入占位表），结束后结果回到栈顶。
fn cache_required_result(l: &mut LuaState, cache_key: &[u8]) {
  // 结果在 4 个固定槽之上（缓存表压栈后即 -2，与常规路径同一形态）
  let result_idx = REQUIRE_STACK_VALUES + 1;

  if LuauCyclicRequireShortCircuit.get() {
    // 短路开且占位表已交给循环方：灌占位表（adopt 内含结果类型复核）
    if take_registry_mark(l, CYCLIC_PLACEHOLDER_PROVIDED_KEY, cache_key) {
      adopt_provided_placeholder(l, cache_key, result_idx);
      return;
    }
    // 未发生循环：常规路径缓存结果（cpp findtable 建表后写入，门面自配平）
    cache_stack_value(l, REQUIRED_CACHE_TABLE_KEY, cache_key, result_idx);
  } else {
    // 短路关时 cpp 用 lua_getfield 取缓存表（不建表），压缓存表后栈形如
    // (-2) result, (-1) cache table，收尾后结果回到栈顶。
    get_table_field(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY);
    cache_result_under_key(l, -2, cache_key);
  }
}

/// # Safety
///
/// `l` must be a valid pointer to a live `LuaState` (VM 交给 continuation 的协程
/// 状态)；本函数只操作本协程帧栈与注册表，不触发宿主回调，故各步独占借用安全。
pub(crate) unsafe extern "C-unwind" fn lua_requirecont(l: *mut LuaState, _status: i32) -> i32 {
  // Safety: 契约保证 l 为 VM continuation 交给的存活协程 state，入口一次重建
  // 独占借用（不与其他别名冲突）；后续均为本协程帧上的栈/注册表操作。
  let l: &mut LuaState = unsafe { &mut *l };

  ulua_common::LUAU_ASSERT!(l.get_top() >= REQUIRE_STACK_VALUES);
  let num_results = l.get_top() - REQUIRE_STACK_VALUES;
  // cacheKey 视图取栈 2 固定槽（Lua 串非 GC 搬迁对象，指针调用窗内恒有效，
  // cpp 亦直接使用同一 VM 串指针）。
  let cache_key = l.check_bytes(2);

  if num_results > 1 {
    // Safety: l 存活，luaL_error! 抛错发散。
    unsafe { luaL_error!(l, "module must return a single value") };
  }

  if num_results == 1 {
    // 栈为固定槽 + 唯一结果的挂起布局，cache_required_result 只操作本协程栈
    // 与注册表（无宿主回调），为纯安全调用。
    cache_required_result(l, cache_key);
  }

  num_results
}
