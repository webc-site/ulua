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

/// `lua_getmetatable` 核心（cpp `VM/src/lapi.cpp:945`）。调用序契约（正确性，非内存
/// 安全）：`objindex` 为合法（伪）索引，按 `ttype` 分派时该槽须为对应类型
/// （Table/UserData→其 `metatable` 存活、Object→`lclass.instancemetatable` 存活，
/// 其余臂读 `global.mt[ttype]`，索引 < 类型数）；命中元表时压栈需栈顶留 1 空槽、
/// 可触发 GC/写屏障，须处于受保护帧；跨线程经 threadbarrier 同步。越界索引（硬化
/// 的 `index_2_addr` 返回哨兵，tt=LUA_TNIL）与 cpp 越界正索引同落 default 臂读
/// `global.mt[LUA_TNIL]`（通常为 null → 返回 0），逐位一致。
pub(crate) fn lua_getmetatable(l: &mut LuaState, objindex: i32) -> i32 {
  // SAFETY: `l` 存活（引用形保证）；index_2_addr 已对任意索引硬化（越界返回
  // 哨兵，无栈外指针算术）；ensure_stack/threadbarrier/sethvalue/api_incr_top
  // 的指针前提由引用形与 VM 栈不变式成立。
  unsafe {
    let lp = l.as_mut_ptr();
    lua_c_threadbarrier_lapi(lp);
    ensure_stack(lp, 1);

    let obj: StkId = index_2_addr(&*lp, objindex);

    let mt: *mut LuaTable = match ttype!(obj) {
      x if x == LuaType::Table as u32 => (*(*obj).as_table_ptr()).metatable,
      x if x == LuaType::UserData as u32 => (*(*obj).as_userdata_ptr()).metatable,
      x if x == LuaType::Object as u32 => (*(*objectvalue!(obj)).lclass).instancemetatable,
      _ => (*(*lp).global).mt[ttype!(obj) as usize],
    };

    if !mt.is_null() {
      sethvalue!(lp, (*lp).top, mt);
      api_incr_top!(lp);
    }

    (!mt.is_null()) as i32
  }
}
