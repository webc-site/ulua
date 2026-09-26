use crate::{
  functions::lua_rawcheckstack::lua_rawcheckstack,
  macros::{setbvalue::setbvalue, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `status` 为被保护调用返回的 `LUA_OK`/错误码。
/// cpp `lbaselib.cpp:358`。
pub(crate) unsafe extern "C-unwind" fn lua_b_xpcallcont(l: *mut LuaState, status: i32) -> i32 {
  unsafe {
    if status == 0 {
      let base = (*l).base;
      // xpcall had an 'errfunc' before the results, so we just replace it with 'true' status
      setbvalue!(base, 1);
      (*l).top.offset_from(base) as i32
    } else {
      lua_rawcheckstack(l, 1);
      let top = (*l).top;
      // Move error 1 right to make space for the 'false' status
      setobj_2_s!(l, top, top.sub(1));
      setbvalue!(top.sub(1), 0);
      (*l).top = top.add(1);
      2
    }
  }
}
