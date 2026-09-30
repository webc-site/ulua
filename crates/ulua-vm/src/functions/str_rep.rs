//! Source: `VM/src/lstrlib.cpp:83`
//!
//! `string.rep` — repeat the argument `n` times into a single buffer, doubling
//! the already-written prefix each step so the fill is O(result) with log(n)
//! memcpys (the classic exponential-pattern trick), with an overflow guard.

use core::slice::from_raw_parts_mut;

use crate::{
  functions::{
    lua_l_buffinitsize::lua_l_buffinitsize, lua_l_checklstring::lua_l_checklstring_ref,
    lua_l_pushresultsize::lua_l_pushresultsize, lua_pushlstring::lua_pushlstring_bytes,
  },
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn, maxssize::MAXSSIZE},
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// # Safety
///
/// `l` 必须指向本次 strlib 调用的存活 `LuaState`，所需实参按索引可读且栈顶有结果余量。
pub(crate) unsafe fn str_rep(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 存活且重复次数/串长经溢出检查，块内经缓冲写入的总量受扩展协议保护
  unsafe {
    let s = lua_l_checklstring_ref(l, 1);
    let len = s.len();
    let n = (*l).check_integer(2);

    if n <= 0 {
      lua_pushlstring_bytes(l, &[]);
      return 1;
    }

    if n == 1 || len == 0 {
      (*l).push_value(1);
      return 1;
    }

    if len > (MAXSSIZE as usize) / (n as usize) {
      luaL_error!(l, "resulting string too large");
    }

    let total = len * (n as usize);

    let mut b = LuaLStrbuf::new();
    let ptr = lua_l_buffinitsize(l, &mut b, total);
    // 目标缓冲一次成切片：指数填充走 split_at_mut 区间复制，消除 add/copy_nonoverlapping 指针算术
    let buf = from_raw_parts_mut(ptr, total);
    buf[..len].copy_from_slice(s);

    // use the increasing 'pattern' inside our target buffer to fill the next part
    // 保留 while 游走：step 倍增与已写量 written 双游标协同（指数填充），无等差区间可迭代
    let mut written = len;
    let mut step = len;
    while step < total - written {
      let (prefix, dest) = buf.split_at_mut(written);
      dest[..step].copy_from_slice(&prefix[..step]);
      written += step;
      step <<= 1;
    }

    // fill tail
    let (prefix, dest) = buf.split_at_mut(written);
    dest.copy_from_slice(&prefix[..total - written]);

    lua_l_pushresultsize(&mut b, total);

    1
  }
}

lua_lib_fn!(pub(crate) fn str_rep, str_rep_arm);
