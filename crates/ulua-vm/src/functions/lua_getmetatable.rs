use crate::{
  enums::lua_type::LuaType,
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
  },
  macros::{
    api_incr_top::api_incr_top, objectvalue::objectvalue, sethvalue::sethvalue, ttype::ttype,
  },
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`objindex` 为合法（伪）索引，`index_2_addr` 所得槽存活；按 `ttype` 分派时
/// 该槽须为对应类型（Table→`hvalue`、UserData→`uvalue`、Object→`objectvalue.lclass`，其 instancemetatable 存活），
/// 否则回退 `(*g).mt[ttype]`（`(*l).global` 须存活、索引 < 类型数）。命中时 `ensure_stack(l,1)` 后 `sethvalue` 直写
/// `(*l).top` 再 `api_incr_top`（前须留 ≥1 槽）。跨线程经 threadbarrier 同步。
/// cpp VM/src/lapi.cpp:945
pub unsafe fn lua_getmetatable(l: *mut LuaState, objindex: i32) -> i32 {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

    let obj: StkId = index_2_addr(l, objindex);

    let mt: *mut LuaTable = match ttype!(obj) {
      x if x == LuaType::Table as u32 => (*(*obj).as_table_ptr()).metatable,
      x if x == LuaType::UserData as u32 => (*(*obj).as_userdata_ptr()).metatable,
      x if x == LuaType::Object as u32 => (*(*objectvalue!(obj)).lclass).instancemetatable,
      _ => (*(*l).global).mt[ttype!(obj) as usize],
    };

    if !mt.is_null() {
      sethvalue!(l, (*l).top, mt);
      api_incr_top!(l);
    }

    (!mt.is_null()) as i32
  }
}
