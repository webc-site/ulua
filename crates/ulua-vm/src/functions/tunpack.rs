use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    c_slice, c_slice_mut, lua_checkstack::lua_checkstack, lua_l_checktype::lua_l_checktype,
    lua_l_error_l::lua_l_error_l, lua_l_optinteger::lua_l_optinteger, lua_objlen::lua_objlen,
    lua_rawgeti::lua_rawgeti,
  },
  macros::{hvalue::hvalue, setobj_2_s::setobj2s},
  records::lua_t_value::TValue,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_tunpack")]
pub(crate) unsafe extern "C-unwind" fn tunpack(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    let t = hvalue!((*l).base);

    let i = lua_l_optinteger(l, 2, 1);
    let e = lua_objlen(l, 1);
    let e = lua_l_optinteger(l, 3, e);

    if i > e {
      return 0; // empty range
    }

    // `n` here is the element count MINUS ONE. C++ guards on this value
    // (`n >= INT_MAX`) BEFORE adding one, so a full-range request
    // (i = INT_MIN, e = INT_MAX -> n = 0xFFFF_FFFF) is rejected. Adding one first
    // (as the previous port did) wrapped n to 0, passed the guard, and let the
    // push loop overrun the stack into an api_incr_top assert (SIGTRAP).
    let n = (e as u32).wrapping_sub(i as u32); // number of elements minus 1 (avoid overflows)
    if n >= c_int::MAX as u32 || lua_checkstack(l, n.wrapping_add(1) as c_int) == 0 {
      lua_l_error_l(
        l,
        c"too many results to unpack".as_ptr(),
        core::format_args!("too many results to unpack"),
      );
    }
    let n = n + 1; // safe: guard above guarantees n (minus one) < INT_MAX

    // fast-path: direct array-to-stack copy
    if i == 1 && (n as c_int) <= (*t).sizearray {
      // SAFETY：快路径已断言 n <= sizearray；栈上 n 个槽位随后由 top 前移提交。
      for (dst, src) in c_slice_mut((*l).top, n as usize)
        .iter_mut()
        .zip(c_slice((*t).array, n as usize))
      {
        setobj2s!(l, dst as *mut TValue, src as *const TValue as *mut TValue);
      }
      (*l).top = (*l).top.offset(n as isize);
    } else {
      // push arg[i..e - 1] (to avoid overflows)
      let mut current_i = i;
      while current_i < e {
        lua_rawgeti(l, 1, current_i);
        current_i += 1;
      }
      lua_rawgeti(l, 1, e); // push last element
    }

    n as c_int
  }
}
