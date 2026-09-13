use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_absindex::lua_absindex, lua_g_readonlyerror::lua_g_readonlyerror, lua_pushnil::lua_pushnil,
    lua_rawgeti::lua_rawgeti, lua_rawiter::lua_rawiter, lua_rawseti::lua_rawseti,
    lua_type::lua_type,
  },
  macros::{
    hvalue::hvalue, lua_c_barrierfast::lua_c_barrierfast, lua_newtable::lua_newtable,
    lua_pop::lua_pop, lua_tointeger::lua_tointeger, lua_tonumber::lua_tonumber,
    setobj_2_t::setobj2t,
  },
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

unsafe fn tovalidintkey(l: *mut lua_State, idx: i32, f: i32, e: i32, result: &mut i32) -> bool {
  unsafe {
    if lua_type(l, idx) == LuaType::Number as i32 {
      let nkey = lua_tonumber!(l, idx);
      if nkey >= f as f64 && nkey <= e as f64 {
        *result = nkey as i32;
        return (*result as f64) == nkey;
      }
    }
    false
  }
}

pub(crate) unsafe fn moveelements(
  l: *mut lua_State,
  srct: i32,
  dstt: i32,
  f: i32,
  e: i32,
  t: i32,
  sparsemove: bool,
) {
  unsafe {
    let src = hvalue!((*l).base.offset((srct - 1) as isize));
    let dst = hvalue!((*l).base.offset((dstt - 1) as isize));

    if (*dst).readonly != 0 {
      lua_g_readonlyerror(l);
    }

    let n = e - f + 1;
    let f_index = (f as u32).wrapping_sub(1);
    let t_index = (t as u32).wrapping_sub(1);
    let n_unsigned = n as u32;

    if f_index < (*src).sizearray as u32
      && t_index < (*dst).sizearray as u32
      && f_index.wrapping_add(n_unsigned) <= (*src).sizearray as u32
      && t_index.wrapping_add(n_unsigned) <= (*dst).sizearray as u32
    {
      let srcarray = (*src).array;
      let dstarray = (*dst).array;

      if t > e || t <= f || (dstt != srct && dst != src) {
        for i in 0..n {
          let s: *mut TValue = srcarray.offset((f + i - 1) as isize);
          let d: *mut TValue = dstarray.offset((t + i - 1) as isize);
          setobj2t!(l, d, s);
        }
      } else {
        for i in (0..n).rev() {
          let s: *mut TValue = srcarray.offset((f + i - 1) as isize);
          let d: *mut TValue = dstarray.offset((t + i - 1) as isize);
          setobj2t!(l, d, s);
        }
      }

      lua_c_barrierfast!(l, dst);
    } else if sparsemove {
      let srcta = lua_absindex(l, srct);
      let dstta = lua_absindex(l, dstt);
      let te = t + (n - 1);

      lua_newtable(l);

      let mut iter = 0;
      loop {
        iter = lua_rawiter(l, srcta, iter);
        if iter == -1 {
          break;
        }
        let mut ikey = 0;
        if tovalidintkey(l, -2, f, e, &mut ikey) {
          lua_rawseti(l, -3, ikey);
        } else {
          lua_pop(l, 1);
        }
        lua_pop(l, 1);
      }

      iter = 0;
      loop {
        iter = lua_rawiter(l, dstta, iter);
        if iter == -1 {
          break;
        }
        let mut ikey = 0;
        if tovalidintkey(l, -2, t, te, &mut ikey) {
          lua_pushnil(l);
          lua_rawseti(l, dstta, ikey);
        }
        lua_pop(l, 2);
      }

      iter = 0;
      loop {
        iter = lua_rawiter(l, -1, iter);
        if iter == -1 {
          break;
        }
        let ikey = lua_tointeger!(l, -2);
        lua_rawseti(l, dstta, ikey - f + t);
        lua_pop(l, 1);
      }

      lua_pop(l, 1);
    } else {
      if t > e || t <= f || dst != src {
        for i in 0..n {
          lua_rawgeti(l, srct, f + i);
          lua_rawseti(l, dstt, t + i);
        }
      } else {
        for i in (0..n).rev() {
          lua_rawgeti(l, srct, f + i);
          lua_rawseti(l, dstt, t + i);
        }
      }
    }
  }
}
