//! Source: `VM/src/ldebug.cpp:286-296` (hand-ported)

use core::ptr::null;

use crate::{
  functions::{cstr_cow, lua_t_objtypename::lua_t_objtypename},
  macros::{getstr::getstr, lua_g_runerror::lua_g_runerror},
  records::{lua_state::LuaState, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_g_indexerror(l: *mut LuaState, p1: *const TValue, p2: *const TValue) -> ! {
  // SAFETY: 契约保证 `l`/操作数存活可读；`lua_t_objtypename`/`getstr` 返回的 C 串指针
  // 立即经 `cstr_cow` 收口为 `Cow<str>`（unsafe 关在门面内），本函数不再出现宿主 C 串
  // 裸指针（review.md §10）。
  unsafe {
    let t1 = cstr_cow(lua_t_objtypename(&*l, &*p1));
    let t2 = cstr_cow(lua_t_objtypename(&*l, &*p2));
    let key: *const tstring = if (*p2).is_string() {
      (*p2).as_string_ptr()
    } else {
      null()
    };

    // limit length to make sure we don't generate very long error messages for very long keys
    if !key.is_null() && (*key).len <= 64 {
      lua_g_runerror!(
        l,
        "attempt to index {} with '{}'",
        t1,
        cstr_cow(getstr(key))
      )
    } else {
      lua_g_runerror!(l, "attempt to index {} with {}", t1, t2)
    }
  }
}
