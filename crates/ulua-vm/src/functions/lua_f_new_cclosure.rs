use core::ptr::null_mut;

use crate::{
  enums::lua_type::LuaType,
  functions::lua_m_newgco::lua_m_newgco,
  macros::{lua_c_init::luaC_init, lua_minstack::LUA_MINSTACK, size_cclosure::size_cclosure},
  records::{closure::Closure, lua_state::LuaState, lua_table::LuaTable},
};

/// # Safety
/// 调用方须保证 `e` 为存活 `LuaTable` 指针（本函数仅按指针存入 `env`、不解引用；
/// 交付前置于受保护帧，`l` 存活与独占由 `&mut LuaState` 承载）。cpp lfunc.cpp:83。
///
/// r19-w4 收形：首参 `*mut LuaState` → `&mut LuaState`（存活与独占交由类型承载）；`e` 为
/// 调用方传入的裸表指针、体内只做「按位存入 cl.env」的指针搬移，从不解引用该入参，依 §2
/// 假合规防线判例（`SubtypingEnvironment::get_mapped_type_bounds` 降 safe）由 `unsafe fn`
/// 降为 `fn`；体内 `lua_m_newgco`/`luaC_init!`/`&mut *c` 的分配器返回裸指针解引用包于单一
/// `unsafe` 块并附契约——`c` 源自分配器而非调用方入参，非假合规。对仍收裸形的
/// `lua_m_newgco` 转调经一次借出裸指针就地重建（借用窗止于当句）。
pub(crate) fn lua_f_new_cclosure(l: &mut LuaState, nelems: i32, e: *mut LuaTable) -> *mut Closure {
  // SAFETY: `l` 存活可分配，`c` 为 `lua_m_newgco` 按 size_cclosure(nelems) 分配的存活 GCO 内存，
  // `luaC_init!`/`&mut *c` 只写本闭包自有字段。`e` 仅按指针存入 `env`、不解引用。
  unsafe {
    let c = lua_m_newgco(l.as_mut_ptr(), size_cclosure(nelems), l.activememcat) as *mut Closure;

    luaC_init!(l, c, LuaType::Function as i32);
    let cl = &mut *c;
    cl.is_c = 1;
    cl.env = e;
    cl.nupvalues = nelems as u8;
    cl.stacksize = LUA_MINSTACK as u8;
    cl.preload = 0;
    cl.gclist = null_mut();

    let cc = &mut cl.inner.c;
    cc.f = None;
    cc.cont = None;
    cc.debugname = None;

    c
  }
}
