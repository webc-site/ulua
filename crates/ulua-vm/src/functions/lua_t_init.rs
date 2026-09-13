use crate::{
  enums::tms::TMS,
  macros::{lua_s_fix::luaS_fix, lua_s_new::luaS_new},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_t_init(l: *mut lua_State) {
  unsafe {
    let typenames = [
      c"nil".as_ptr(),
      c"boolean".as_ptr(),
      c"userdata".as_ptr(),
      c"number".as_ptr(),
      c"integer".as_ptr(),
      c"vector".as_ptr(),
      c"string".as_ptr(),
      c"table".as_ptr(),
      c"function".as_ptr(),
      c"userdata".as_ptr(),
      c"thread".as_ptr(),
      c"buffer".as_ptr(),
      c"class".as_ptr(),
      c"object".as_ptr(),
    ];

    let eventnames = [
      c"__index".as_ptr(),
      c"__newindex".as_ptr(),
      c"__mode".as_ptr(),
      c"__namecall".as_ptr(),
      c"__call".as_ptr(),
      c"__iter".as_ptr(),
      c"__len".as_ptr(),
      c"__eq".as_ptr(),
      c"__add".as_ptr(),
      c"__sub".as_ptr(),
      c"__mul".as_ptr(),
      c"__div".as_ptr(),
      c"__idiv".as_ptr(),
      c"__mod".as_ptr(),
      c"__pow".as_ptr(),
      c"__unm".as_ptr(),
      c"__lt".as_ptr(),
      c"__le".as_ptr(),
      c"__concat".as_ptr(),
      c"__type".as_ptr(),
      c"__metatable".as_ptr(),
    ];

    let mut i = 0;
    while i < typenames.len() {
      (*(*l).global).ttname[i] = luaS_new(l, typenames[i]);
      luaS_fix!((*(*l).global).ttname[i]);
      i += 1;
    }

    i = 0;
    while i < TMS::TmN as usize {
      (*(*l).global).tmname[i] = luaS_new(l, eventnames[i]);
      luaS_fix!((*(*l).global).tmname[i]);
      i += 1;
    }
  }
}

pub use lua_t_init as luaT_init;
