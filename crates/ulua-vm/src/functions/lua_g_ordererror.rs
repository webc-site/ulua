//! Source: `VM/src/ldebug.cpp:277-284` (hand-ported)

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
pub(crate) unsafe fn lua_g_ordererror(
  l: *mut LuaState,
  p1: *const TValue,
  p2: *const TValue,
  op: TMS,
) -> ! {
  // SAFETY: 契约保证 `l` 为存活调用帧、操作数 TValue 可读；错误串格式化后经 luaG 路径抛出、不返回。
  // lua_t_objtypename 返回的 C 串指针立即经 `cstr_cow` 收口为 `Cow<str>`（借用门面，unsafe 收拢于门
  // 面内），本函数不再出现宿主 C 串裸指针（review.md §10）。
  unsafe {
    let t1 = cstr_cow(lua_t_objtypename(&*l, &*p1));
    let t2 = cstr_cow(lua_t_objtypename(&*l, &*p2));
    let opname: &str = if op == TMS::TmLt {
      "<"
    } else if op == TMS::TmLe {
      "<="
    } else {
      "=="
    };

    lua_g_runerror!(l, "attempt to compare {} {} {}", t1, opname, t2)
  }
}
