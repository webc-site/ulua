use crate::{
  functions::{
    c_slice, c_slice_mut, lua_createtable::lua_createtable, lua_gettop::lua_gettop,
    lua_h_setstr::lua_h_setstr,
  },
  macros::{lua_s_newliteral::lua_s_newliteral, setnvalue::setnvalue, setobj_2_t::setobj2t},
  records::{lua_state::LuaState, lua_t_value::TValue, lua_table::LuaTable},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn tpack(l: *mut LuaState) -> i32 {
  unsafe {
    let n = lua_gettop(l); // number of elements to pack
    lua_createtable(l, n, 1); // create result table

    let t: *mut LuaTable = (*(*l).top.offset(-1)).as_table_ptr();

    // Safety:t->array 与栈 base 均有 n 个有效 TValue（createtable 预留）。
    for (e, v) in c_slice_mut((*t).array, n as usize)
      .iter_mut()
      .zip(c_slice((*l).base, n as usize))
    {
      setobj2t!(l, e as *mut TValue, v as *const TValue as *mut TValue);
    }

    // t.n = number of elements
    let nv = lua_h_setstr(l, t, lua_s_newliteral(l, b"n"));
    setnvalue!(nv, n as f64);

    1 // return table
  }
}
