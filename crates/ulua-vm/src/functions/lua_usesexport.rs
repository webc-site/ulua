//! cpp `VM/src/lapi.cpp` 的 `lua_usesexport`：查询栈上闭包的主函数 proto
//! 是否携带 `LPF_USES_EXPORT` 旗标（编译期由 `export` 语法置位）。
//! CLI require 链路据此为循环依赖创建占位表。

use ulua_common::enums::luau_proto_flag::LuauProtoFlag;

use crate::{
  functions::index_2_addr::index_2_addr, macros::iscfunction::iscfunction,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 必须指向存活的 `LuaState`，`idx` 为合法（伪）索引。
pub unsafe fn lua_usesexport(l: *mut LuaState, idx: i32) -> i32 {
  // Safety: C API 契约由调用方保证 `l`/`idx` 合法，index_2_addr 返回有效栈槽。
  let o: StkId = unsafe { index_2_addr(l, idx) };
  // cpp `isLfunction(o)`：是闭包且非 C 函数
  if !unsafe { (*o).is_function() } || unsafe { iscfunction!(o) } {
    return 0;
  }
  // Safety: ttisfunction 成立时 value.gc 指向存活的 Closure（LDF 快照期不外扩）。
  let cl = unsafe { (*o).as_closure_ptr() };
  // Safety: Lua 闭包的 `inner.l.p` 恒指向存活 Proto（GC 根经 closure 维持）。
  let flags = unsafe { (*(*cl).inner.l.p).flags };
  i32::from((flags & LuauProtoFlag::LPF_USES_EXPORT as u8) != 0)
}
