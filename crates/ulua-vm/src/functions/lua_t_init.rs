use core::{ffi::CStr, ptr::addr_of_mut};

use crate::{
  enums::tms::TMS,
  functions::lua_typename::TYPENAMES_C,
  macros::{lua_s_fix::luaS_fix, lua_s_new::luaS_new},
  type_aliases::lua_state::lua_State,
};

/// 元表方法名，索引即 `TMS` 变体序号（cpp `luaT_init` 的 eventnames）。
const EVENTNAMES: [&CStr; 21] = [
  c"__index",
  c"__newindex",
  c"__mode",
  c"__namecall",
  c"__call",
  c"__iter",
  c"__len",
  c"__eq",
  c"__add",
  c"__sub",
  c"__mul",
  c"__div",
  c"__idiv",
  c"__mod",
  c"__pow",
  c"__unm",
  c"__lt",
  c"__le",
  c"__concat",
  c"__type",
  c"__metatable",
];

// 名字表与枚举必须同步，否则 tmname 会漏填或多填
const _: () = assert!(EVENTNAMES.len() == TMS::TmN as usize);

/// cpp `luaT_init`：把类型名与元表方法名 intern 成固定字符串（不参与 GC）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_t_init(l: *mut lua_State) {
  unsafe {
    // 类型名表与 `lua_typename` 共用同一常量，索引即 LUA_T*
    for (i, name) in TYPENAMES_C.iter().enumerate() {
      let slot = addr_of_mut!((*(*l).global).ttname[i]);
      *slot = luaS_new(l, name.as_ptr());
      luaS_fix!(*slot);
    }

    for (i, name) in EVENTNAMES.iter().enumerate() {
      let slot = addr_of_mut!((*(*l).global).tmname[i]);
      *slot = luaS_new(l, name.as_ptr());
      luaS_fix!(*slot);
    }
  }
}
