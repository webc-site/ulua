//! Source: `VM/src/ldebug.cpp:264-275` (hand-ported; `luaT_eventname[op]`
//! is read from `g->tmname[op]`, built from the same string table)

use core::ffi::c_char;

use crate::{
  enums::tms::TMS,
  functions::{cstr_cow, lua_t_objtypename::lua_t_objtypename},
  macros::{getstr::getstr, lua_g_runerror::lua_g_runerror},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 必须指向存活 `LuaState`（错误经其全局错误路径抛出，且 `tmname[op]` 从其 global 读取）；`p1`/`p2` 须为
/// 可读、对齐的 TValue——`lua_t_objtypename` 按其头部 tag 游走对象布局；`op` 须为 `TMS` 枚举合法序值（直接
/// 作 tmname 下标）。对应 cpp ldebug.cpp:298。
pub unsafe fn lua_g_aritherror(
  l: *mut LuaState,
  p1: *const TValue,
  p2: *const TValue,
  op: TMS,
) -> ! {
  // Safety: 契约保证 `l` 为存活调用帧、操作数 TValue 可读；错误串格式化后经 luaG 路径抛出、不返回
  unsafe {
    let t1: *const c_char = lua_t_objtypename(l, p1);
    let t2: *const c_char = lua_t_objtypename(l, p2);
    // skip __ from metamethod name
    let opname = getstr((*(*l).global).tmname[op as usize]).add(2);

    if t1 == t2 {
      // C++ compares interned typename pointers
      lua_g_runerror!(
        l,
        "attempt to perform arithmetic ({}) on {}",
        cstr_cow(opname),
        cstr_cow(t1)
      )
    } else {
      lua_g_runerror!(
        l,
        "attempt to perform arithmetic ({}) on {} and {}",
        cstr_cow(opname),
        cstr_cow(t1),
        cstr_cow(t2)
      )
    }
  }
}
