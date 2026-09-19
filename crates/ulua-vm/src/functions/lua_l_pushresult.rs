use crate::{
  functions::{
    lua_pushlstring::lua_pushlstring, lua_s_buffinish::lua_s_buffinish, lua_s_newlstr::luaS_newlstr,
  },
  macros::{lua_c_check_gc::luaC_checkGC, setsvalue::setsvalue},
  records::lua_l_strbuf::LuaLStrbuf,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_l_pushresult(b: *mut LuaLStrbuf) {
  unsafe {
    let l = (*b).l;
    let storage = (*b).storage;

    if !storage.is_null() {
      luaC_checkGC!(l);

      let p = (*b).p;
      let end = (*b).end;

      if p == end {
        setsvalue!(l, (*l).top.offset(-1), lua_s_buffinish(l, storage));
      } else {
        let storage_data = (*storage).data.as_ptr();
        let len = (p as usize).wrapping_sub(storage_data as usize);
        setsvalue!(l, (*l).top.offset(-1), luaS_newlstr(l, storage_data, len));
      }
    } else {
      let p = (*b).p;
      let buffer = (*b).buffer.as_ptr();
      let len = (p as usize).wrapping_sub(buffer as usize);
      lua_pushlstring(l, buffer, len);
    }
  }
}
