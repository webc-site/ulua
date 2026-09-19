use core::ptr::copy;

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
  },
  type_aliases::{lua_state::lua_State, t_value::TValue},
};
/// cpp `ltablib.cpp:tovalidintkey`：`idx` 处为落在 `[f, e]` 内的整数键时返回该键。
///
/// cpp 用 `int* result` 出参 + bool 返回值，Rust 版折叠为 `Option<i32>`。
unsafe fn tovalidintkey(l: *mut lua_State, idx: i32, f: i32, e: i32) -> Option<i32> {
  unsafe {
    if lua_type(l, idx) == LuaType::Number as i32 {
      let nkey = lua_tonumber!(l, idx);
      if nkey >= f as f64 && nkey <= e as f64 {
        let result = nkey as i32;
        if (result as f64) == nkey {
          return Some(result);
        }
      }
    }
    None
  }
}

/// 数组区间拷贝：源元素均经 setobj2t 写入（构造时已验证 liveness），
/// 正/逆序分支仅为手写 memmove，此处直接用 ptr::copy 语义等价且单次向量化拷贝。
///
/// # Safety
/// `[srcarray + f - 1, srcarray + f - 1 + n)` 与 `[dstarray + t - 1, dstarray + t - 1 + n)`
/// 必须落在各自表的 sizearray 内，调用方已保证。
#[inline]
unsafe fn copy_array_range(srcarray: *mut TValue, dstarray: *mut TValue, f: i32, t: i32, n: i32) {
  unsafe {
    copy(
      srcarray.offset((f - 1) as isize),
      dstarray.offset((t - 1) as isize),
      n as usize,
    );
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// 栈槽区间搬移：经 lua_rawgeti/lua_rawseti 逐元素搬运，方向语义同数组拷贝
#[inline]
unsafe fn move_stack_range(
  l: *mut lua_State,
  srct: i32,
  dstt: i32,
  f: i32,
  t: i32,
  n: i32,
  reverse: bool,
) {
  unsafe {
    if reverse {
      for i in (0..n).rev() {
        lua_rawgeti(l, srct, f + i);
        lua_rawseti(l, dstt, t + i);
      }
    } else {
      for i in 0..n {
        lua_rawgeti(l, srct, f + i);
        lua_rawseti(l, dstt, t + i);
      }
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
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

      // ptr::copy 自身处理区间重叠（memmove 语义），无需再按方向分支
      copy_array_range(srcarray, dstarray, f, t, n);

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
        match tovalidintkey(l, -2, f, e) {
          // 命中：lua_rawseti 自行弹出值；未命中：手动弹出值
          Some(ikey) => lua_rawseti(l, -3, ikey),
          None => lua_pop(l, 1),
        }
        lua_pop(l, 1); // 弹出键
      }

      iter = 0;
      loop {
        iter = lua_rawiter(l, dstta, iter);
        if iter == -1 {
          break;
        }
        if let Some(ikey) = tovalidintkey(l, -2, t, te) {
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
        move_stack_range(l, srct, dstt, f, t, n, false);
      } else {
        move_stack_range(l, srct, dstt, f, t, n, true);
      }
    }
  }
}
