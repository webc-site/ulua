use core::ffi::{c_char, c_int};

use crate::{
  functions::{
    c_slice, c_slice_mut, lua_createtable::lua_createtable, lua_gettop::lua_gettop,
    lua_h_setstr::lua_h_setstr,
  },
  macros::{
    hvalue::hvalue, lua_s_newliteral::lua_s_newliteral, setnvalue::setnvalue, setobj_2_t::setobj2t,
  },
  records::{lua_state::lua_State, lua_t_value::TValue, lua_table::LuaTable},
};

#[unsafe(export_name = "ulua_tpack")]
pub(crate) unsafe extern "C-unwind" fn tpack(l: *mut lua_State) -> c_int {
  unsafe {
    let n = lua_gettop(l); // number of elements to pack
    lua_createtable(l, n, 1); // create result table

    let t: *mut LuaTable = hvalue!((*l).top.offset(-1));

    // SAFETY：t->array 与栈 base 均有 n 个有效 TValue（createtable 预留）。
    for (e, v) in c_slice_mut((*t).array, n as usize)
      .iter_mut()
      .zip(c_slice((*l).base, n as usize))
    {
      setobj2t!(l, e as *mut TValue, v as *const TValue as *mut TValue);
    }

    // t.n = number of elements
    let nv = lua_h_setstr(
      l,
      t,
      lua_s_newliteral(l, b"n\0" as *const _ as *const c_char),
    );
    setnvalue!(nv, n as f64);

    1 // return table
  }
}
