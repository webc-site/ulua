//! Source: `VM/src/ldebug.cpp:277-284` (hand-ported)

use core::ffi::c_char;

use crate::{
  enums::tms::TMS,
  functions::{cstr_cow, lua_t_objtypename::lua_t_objtypename},
  macros::lua_g_runerror::lua_g_runerror,
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 必须指向存活 `LuaState`（错误经其 lua_g_runerror 路径抛出不返回）；`p1`/`p2` 须为可读、对齐的 TValue
/// （类型名经 lua_t_objtypename 按对象头游走）；`op` 仅按 TmLt/TmLe 判别、其余值一律输出 `==` 文案，须由比较
/// 事件调用点保证语义正确。对应 cpp ldebug.cpp:310。
pub unsafe fn lua_g_ordererror(
  l: *mut LuaState,
  p1: *const TValue,
  p2: *const TValue,
  op: TMS,
) -> ! {
  // Safety: 契约保证 `l` 为存活调用帧、操作数 TValue 可读；错误串格式化后经 luaG 路径抛出、不返回
  unsafe {
    let t1: *const c_char = lua_t_objtypename(l, p1);
    let t2: *const c_char = lua_t_objtypename(l, p2);
    let opname: &str = if op == TMS::TmLt {
      "<"
    } else if op == TMS::TmLe {
      "<="
    } else {
      "=="
    };

    lua_g_runerror!(
      l,
      "attempt to compare {} {} {}",
      cstr_cow(t1),
      opname,
      cstr_cow(t2)
    )
  }
}
