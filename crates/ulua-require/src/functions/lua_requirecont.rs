use core::ffi::c_int;

use ulua_common::{fflag::LuauCyclicRequireShortCircuit, functions::c_str::cstr_bytes};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_gettop::lua_gettop, lua_pushvalue::lua_pushvalue, lua_replace::lua_replace,
    lua_type::lua_type,
  },
  macros::{
    lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error, lua_pop::lua_pop,
    lua_registryindex::LUA_REGISTRYINDEX,
  },
  records::lua_state::LuaState,
};

use crate::functions::{
  cache_table_keys::REQUIRED_CACHE_TABLE_KEY,
  cyclic_placeholder::{
    consume_placeholder_provided, populate_placeholder, push_cached_placeholder,
  },
  registry_table::{get_table_field, push_registry_table, set_table_field},
};

pub(crate) const K_REQUIRE_STACK_VALUES: i32 = 4;

/// 把结果值按 cacheKey 写进已在栈顶的 `_MODULES` 表（cpp `lua_requirecont` 两条
/// 缓存路径共用的收尾三连）：进入时栈顶为缓存表、其下为结果值，结束后缓存表弹掉、
/// 结果回到栈顶。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`，栈顶必须是缓存表、其下必须是 `result_idx` 处的结果。
unsafe fn cache_result_under_key(l: *mut LuaState, result_idx: c_int, cache_key: &[u8]) {
  // Safety: l/result_idx 由契约保证；pushvalue 复制结果、set_table_field 门面把键按
  // ptr+len 形态即时压栈并消费，pop 弹掉缓存表，栈净变化 -1。
  unsafe {
    lua_pushvalue(l, result_idx);
    set_table_field(l, -2, cache_key);
    lua_pop(l, 1);
  }
}

/// cpp `lua_requirecont` provided 分支：复核结果为表后取回缓存的占位表、把模块
/// 结果灌进占位表，并用灌满后的占位表替换栈上结果——让首个调用方与后续缓存命中
/// 拿到的是循环各方持有的同一张表。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState` 且栈为 require 挂起后的布局（固定槽 + 唯一
/// 结果，`result_idx` 为该结果槽）；`cache_key` 为同帧字节串借用，仅转手给各门面。
unsafe fn adopt_provided_placeholder(l: *mut LuaState, cache_key: &[u8], result_idx: c_int) {
  // Safety: consume_placeholder_provided 已确认 provided 标记，LUAU_ASSERT 复核结果
  // 为表；push_cached_placeholder 净增两槽、populate_placeholder 自配平、
  // lua_replace/lua_pop 仅栈操作。
  unsafe {
    ulua_common::LUAU_ASSERT!(lua_type(l, result_idx) == LuaType::Table as i32);
    push_cached_placeholder(l, cache_key);
    populate_placeholder(l, -1, result_idx);
    lua_replace(l, result_idx);
    lua_pop(l, 1);
  }
}

/// cpp `lua_requirecont` 的单结果缓存收尾：恰有一个结果时把模块结果写进 `_MODULES`
/// 缓存表（开启循环短路时优先复用/灌入占位表），结束后结果回到栈顶。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState` 且栈为 require 挂起后的布局（`K_REQUIRE_STACK_VALUES`
/// 固定槽 + 唯一结果）；`cache_key` 为栈 2 处 cacheKey 的字节串借用，仅传给各门面即时消费。
unsafe fn cache_required_result(l: *mut LuaState, cache_key: &[u8]) {
  // 结果在 4 个固定槽之上（缓存表压栈后即 -2，与常规路径同一形态）——纯算术，safe 侧
  let result_idx = K_REQUIRE_STACK_VALUES + 1;

  if !LuauCyclicRequireShortCircuit.get() {
    // Safety: 栈布局由 lua_requireinternal 建立、调用方保证恰有一个结果；
    // registry_table 门面与 cache_result_under_key 各按契约操作栈且配平，
    // 收尾后结果回到栈顶。
    unsafe {
      // 压缓存表后栈形如 (-2) result, (-1) cache table
      get_table_field(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY);
      cache_result_under_key(l, -2, cache_key);
    }
  } else if unsafe { consume_placeholder_provided(l, cache_key) } {
    // Safety: l/result_idx/cache_key 满足 adopt_provided_placeholder 的契约
    unsafe { adopt_provided_placeholder(l, cache_key, result_idx) };
  } else {
    // Safety: 未发生循环，按常规路径缓存结果；门面栈配平同上一分支
    unsafe {
      push_registry_table(l, REQUIRED_CACHE_TABLE_KEY);
      cache_result_under_key(l, result_idx, cache_key);
    }
  }
}

/// # Safety
///
/// `l` must be a valid pointer to a live `LuaState`.
pub(crate) unsafe extern "C-unwind" fn lua_requirecont(l: *mut LuaState, _status: i32) -> i32 {
  // Safety: l 是 VM 交给 continuation 的协程状态，require 挂起前的栈布局由
  // lua_requireinternal 建立，LUAU_ASSERT 先行复核；lua_gettop 是纯栈读。
  let num_results = unsafe {
    ulua_common::LUAU_ASSERT!(lua_gettop(l) >= K_REQUIRE_STACK_VALUES);
    lua_gettop(l) - K_REQUIRE_STACK_VALUES
  };

  // Safety: 真 FFI 入口：`luaL_checkstring` 保证返回 NUL 结尾有效指针，经 cstr_bytes
  // 门面一次性转字节串，内部链与出参边界（registry_table 门面）都只见字节
  let cache_key = unsafe { cstr_bytes(luaL_checkstring!(l, 2)) };

  if num_results > 1 {
    // Safety: l 存活，luaL_error! 抛错发散
    unsafe { luaL_error!(l, "module must return a single value") };
  }

  if num_results == 1 {
    // Safety: 栈为固定槽 + 唯一结果的挂起布局，满足 cache_required_result 契约
    unsafe { cache_required_result(l, cache_key) };
  }

  num_results
}
