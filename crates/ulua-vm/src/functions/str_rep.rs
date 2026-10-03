//! Source: `VM/src/lstrlib.cpp:85`
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

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：`l` 须处于可
/// 抛错受保护帧；栈槽 #1 为串实参（非串经 `lua_l_checklstring_ref` 抛 "string expected" 发散，
/// 严格先于 #2 的 `check_integer`）、#2 为重复次数；`n <= 0` 压空串、`n == 1` 或空串直投 #1，
/// 越 MAXSSIZE 乘积则 `luaL_error` 抛 "resulting string too large" 发散；结果压栈需 `top` 后
/// ≥1 空槽，分配可触发 GC。
///
/// `unsafe fn` 屏障按 r16-v21 判例保留：目标区仍由 `lua_l_buffinitsize`（裸形被调，经
/// `l.as_mut_ptr()` 一次就地转手）返回的 `ptr` 经 `from_raw_parts_mut` 成窗，指数填充走
/// `split_at_mut` 区间复制。
pub(crate) unsafe fn str_rep(l: &mut LuaState) -> i32 {
  // SAFETY: `l` 由 `&mut` 保证有效且独占，转手不外于当句；重复次数/串长经溢出检查，
  // 块内经缓冲写入的总量受扩展协议保护
  unsafe {
    // 锚定形（p28 A 类「二次派窗」判例）：首次派窗止于取长，兼作 cpp `luaL_checklstring` 的
    // 先位抛错件，其后 `l` 复原可借；落定 #2 与溢出闸门后二次派窗直达首段拷贝
    let len = lua_l_checklstring_ref(l, 1).len();
    let n = l.check_integer(2);

    if n <= 0 {
      lua_pushlstring_bytes(l, &[]);
      return 1;
    }

    if n == 1 || len == 0 {
      l.push_value(1);
      return 1;
    }

    if len > (MAXSSIZE as usize) / (n as usize) {
      luaL_error!(l.as_mut_ptr(), "resulting string too large");
    }

    let total = len * (n as usize);

    let mut b = LuaLStrbuf::new();
    let ptr = lua_l_buffinitsize(l.as_mut_ptr(), &mut b, total);
    // 目标缓冲一次成切片：指数填充走 split_at_mut 区间复制，消除 add/copy_nonoverlapping 指针算术
    let buf = from_raw_parts_mut(ptr, total);

    // 二次派窗取回同一栈槽串体（同槽同值，观测等价）；切片源于裸 `ptr` 不借 `l`，故窗口
    // 借用止于本次拷贝
    let s = lua_l_checklstring_ref(l, 1);
    debug_assert_eq!(s.len(), len);
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

lua_lib_fn!(pub(crate) fn str_rep @ref, str_rep_arm);
