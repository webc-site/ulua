use crate::{
  functions::{
    c_slice_mut, lua_createtable::lua_createtable, lua_l_checkinteger::lua_l_checkinteger,
  },
  luaL_argerror,
  macros::lua_isnoneornil::lua_isnoneornil,
  records::lua_state::LuaState,
  setobj2t,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn tcreate(l: *mut LuaState) -> i32 {
  unsafe {
    let size = lua_l_checkinteger(l, 1);
    if size < 0 {
      luaL_argerror!(l, 1, "size out of range");
    }

    if !lua_isnoneornil!(l, 2) {
      lua_createtable(l, size, 0);
      let t = (*(*l).top.offset(-1)).as_table_ptr();

      let v: StkId = (*l).base.add(1);

      for e in c_slice_mut((*t).array, size as usize) {
        setobj2t!(l, e as *mut TValue, v);
      }
    } else {
      lua_createtable(l, size, 0);
    }

    1
  }
}
