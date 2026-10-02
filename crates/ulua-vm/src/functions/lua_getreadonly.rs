use crate::{
  functions::index_2_addr::index_2_addr,
  macros::api_check::api_check,
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// `lua_getreadonly`：读 `objindex` 处 table 的只读标志（cpp `lua_getreadonly`）。
/// 调用序契约（正确性，非内存安全）：`l` 须为存活 `LuaState`，`objindex` 须解析到
/// table 槽（`api_check!` 断言 `(*o).is_table()` 兜底）；`index_2_addr` 所得 `o` 与
/// 其 `as_table_ptr()` 所得 `t` 均为栈内合法槽/活表指针，属本实现内部裸指针读数，
/// unsafe 收进实现、不再外包给调用方（r16-v3 引用形前移）。
pub fn lua_getreadonly(l: &LuaState, objindex: i32) -> i32 {
  // SAFETY: 契约保证 `l` 存活且 `objindex` 为合法栈索引；`o`/`t` 只读 is_table/readonly，
  // 不写场域、不触 GC。
  unsafe {
    let o: *const TValue = index_2_addr(l, objindex);

    api_check!(l, (*o).is_table());

    let t: *mut LuaTable = (*o).as_table_ptr();

    (*t).readonly as i32
  }
}
