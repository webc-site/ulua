//! Source: `VM/src/lapi.cpp:1055-1090` (hand-ported)

use core::ptr::{eq, null_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::{index_2_addr::index_2_addr, lua_g_readonlyerror::check_writable},
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_c_objbarrier::lua_c_objbarrier,
    lua_o_nilobject::LUA_O_NILOBJECT, ttype::ttype,
  },
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::stk_id::StkId,
};

/// `lua_setmetatable` 核心（cpp `VM/src/lapi.cpp:1067`）。调用序契约（正确性，非
/// 内存安全）：栈顶留有 1 个元表或 nil 值（`api_checknelems 1`；非 nil 时须为
/// table，debug 断言校验）；`objindex` 经硬化的 `index_2_addr` 须解析为非哨兵槽，
/// 且按 `ttype` 分派时该槽须为对应类型（Table→只读表走 `luaG_readonlyerror` 抛
/// 错、UserData→存活 udata、default→`global.mt[ttype]` 索引 < 类型数）；写
/// `metatable` 字段并 `luaC_objbarrier`（可 GC），须处于受保护帧。
pub(crate) fn lua_setmetatable(l: &mut LuaState, objindex: i32) -> i32 {
  // SAFETY: `l` 存活（引用形保证）；index_2_addr 已对任意索引硬化（越界返回
  // 哨兵）；check_writable/objbarrier 的指针前提由引用形、调用序契约与 VM 栈
  // 不变式成立。
  unsafe {
    let lp = l.as_mut_ptr();
    api_checknelems!(lp, 1);

    let obj: StkId = index_2_addr(&*lp, objindex);
    api_check!(lp, !eq(obj, LUA_O_NILOBJECT));

    // 既有约定（review.md §2）：VM c-API 边界局部哨兵——栈顶为 nil 时 `mt` 保持空表示清除元表，null 为合法实参，边界体内保留裸指针
    let mut mt: *mut LuaTable = null_mut();
    if !(*(*lp).top.sub(1)).is_nil() {
      api_check!(lp, (*(*lp).top.sub(1)).is_table());
      mt = (*(*lp).top.sub(1)).as_table_ptr();
    }

    match ttype!(obj) {
      x if x == LuaType::Table as u32 => {
        let h = (*obj).as_table_ptr();
        check_writable(lp, h);
        (*h).metatable = mt;
        if !mt.is_null() {
          lua_c_objbarrier!(lp, h, mt);
        }
      }
      x if x == LuaType::UserData as u32 => {
        let u = (*obj).as_userdata_ptr().cast_mut();
        (*u).metatable = mt;
        if !mt.is_null() {
          lua_c_objbarrier!(lp, u, mt);
        }
      }
      _ => {
        (*(*lp).global).mt[ttype!(obj) as usize] = mt;
      }
    }

    (*lp).top = (*lp).top.sub(1);
    1
  }
}
