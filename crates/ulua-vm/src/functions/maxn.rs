use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{c_slice, lua_l_checktype::lua_l_checktype, lua_pushnumber::lua_pushnumber},
  macros::{
    gkey::{gkey, gval},
    gnode::gnode,
    hvalue::hvalue,
    nvalue::nvalue,
    sizenode::sizenode,
    ttisnil::ttisnil,
    ttisnumber::ttisnumber,
  },
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn maxn(l: *mut lua_State) -> c_int {
  let mut max: f64 = 0.0;
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);

    let t = hvalue!((*l).base);

    // 数组尾部：最后一个非 nil 元素决定 max，rposition 从后往前提前终止
    let arr = c_slice((*t).array, (*t).sizearray as usize);
    if let Some(i) = arr.iter().rposition(|v| !ttisnil!(v)) {
      max = (i + 1) as f64;
    }

    let node_size = sizenode!(t);
    for i in 0..node_size {
      let n = gnode!(t, i);

      if !ttisnil!(gval!(n)) && ttisnumber!(gkey!(n)) {
        let v = nvalue!(gkey!(n));

        if v > max {
          max = v;
        }
      }
    }

    lua_pushnumber(l, max);
  }
  1
}
