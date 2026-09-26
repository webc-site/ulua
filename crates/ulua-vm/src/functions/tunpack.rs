use crate::{
  enums::lua_type::LuaType,
  functions::{
    c_slice, c_slice_mut, lua_checkstack::lua_checkstack, lua_l_checktype::lua_l_checktype,
    lua_l_optinteger::lua_l_optinteger, lua_objlen::lua_objlen, lua_rawgeti::lua_rawgeti,
  },
  macros::{lua_l_error::luaL_error, setobj_2_s::setobj_2_s},
  records::{lua_state::LuaState, lua_t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn tunpack(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    let t = (*(*l).base).as_table_ptr();

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
    if n >= i32::MAX as u32 || lua_checkstack(l, n.wrapping_add(1) as i32) == 0 {
      luaL_error!(l, "too many results to unpack");
    }
    let n = n + 1; // safe: guard above guarantees n (minus one) < INT_MAX

    // fast-path: direct array-to-stack copy
    if i == 1 && (n as i32) <= (*t).sizearray {
      // Safety:快路径已断言 n <= sizearray；栈上 n 个槽位随后由 top 前移提交。
      for (dst, src) in c_slice_mut((*l).top, n as usize)
        .iter_mut()
        .zip(c_slice((*t).array, n as usize))
      {
        setobj_2_s!(l, dst as *mut TValue, src as *const TValue as *mut TValue);
      }
      (*l).top = (*l).top.offset(n as isize);
    } else {
      // push arg[i..e - 1] (to avoid overflows)：cpp `while current_i < e` 游走
      // 收为区间迭代，末元素单独压栈（i <= e 此前已由空区间早退保证）
      for current_i in i..e {
        lua_rawgeti(l, 1, current_i);
      }
      lua_rawgeti(l, 1, e); // push last element
    }

    n as i32
  }
}
