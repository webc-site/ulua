use crate::{
  functions::index_2_addr::index_2_addr,
  macros::{api_check::api_check, registry::registry},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`objindex` 为合法（伪）索引，`index_2_addr` 所得 `o` 须为 table（`api_check!` is_table），
/// `hvalue(o)` 后存活；且该表不得为 registry（`api_check!` 排除 `registry!(l)`，冻结 registry 会破坏 GC/引用记账）；
/// 仅置 `(*t).readonly` 标志位，不分配、不抛错。
/// cpp VM/src/lapi.cpp:907
pub unsafe fn lua_setreadonly(l: *mut LuaState, objindex: i32, enabled: i32) {
  unsafe {
    let o: *const TValue = index_2_addr(l, objindex);
    api_check!(l, (*o).is_table());

    let t: *mut LuaTable = (*o).as_table_ptr();

    // The registry macro returns a reference &TValue.
    // hvalue! 接收 *mut TValue 指针
    // We use addr_of! to get a pointer from the reference safely before casting.
    api_check!(l, t != (*registry!(l)).as_table_ptr());

    (*t).readonly = if enabled != 0 { 1 } else { 0 };
  }
}
