use core::slice;

use crate::{
  functions::{
    lua_l_buffinitsize::lua_l_buffinitsize, lua_l_checklstring::lua_l_checklstring_ref,
    lua_l_pushresultsize::lua_l_pushresultsize,
  },
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// 字符串单目变换骨架（lower/upper/reverse 共用）：
/// 取 1 号实参字符串 `s` 及其长度 `len`，申请等长 `LuaLStrbuf` 缓冲，
/// 传给闭包 `transform(dst, src)` 完成逐字节写入，最后压回结果并返回 1。
///
/// # Safety
///
/// `l` 必须指向本次 strlib 调用的存活 `LuaState`，所需实参按索引可读且栈顶有结果余量。
pub(crate) unsafe fn str_transform1(
  l: *mut LuaState,
  transform: impl FnOnce(&mut [u8], &[u8]),
) -> i32 {
  // SAFETY: 契约保证 `l` 存活且 checklstring 取回有效源切片，buffinitsize 分配等长目标缓冲
  unsafe {
    let src = lua_l_checklstring_ref(l, 1);
    let len = src.len();

    let mut b = LuaLStrbuf::new();
    let ptr = lua_l_buffinitsize(l, &mut b, len);

    // src 指向 1 号实参串体内数据（GC 串不因栈搬移移动），dst 为等长新缓冲
    let dst = slice::from_raw_parts_mut(ptr, len);
    transform(dst, src);

    lua_l_pushresultsize(&mut b, len);
    1
  }
}

// 留证：骨架的三个驱动入口 str_lower/str_upper/str_reverse 均为 pub(crate)
// （cpp lstrlib.cpp:47/59/71 的同名函数是 static，经字符串库表注册后才可达，
// 无独立导出面），本测试直调三入口以覆盖 str_transform1 的「取参 / 等长缓冲 /
// 压回结果」路径；迁 tests/ 需把入口提升为 pub，属 §8 禁止的泄漏式迁移。
#[cfg(test)]
mod tests {
  use crate::functions::{
    lua_close::lua_close, lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs,
    lua_pushlstring::lua_pushlstring_bytes, lua_tolstring::lua_tolstring_ref, str_byte::str_byte,
    str_lower::str_lower, str_reverse::str_reverse, str_sub::str_sub, str_upper::str_upper,
  };

  #[test]
  fn str_transforms_and_ops() {
    unsafe {
      let l = lua_l_newstate();
      assert!(!l.is_null());
      lua_l_openlibs(l);

      // test lower
      lua_pushlstring_bytes(l, b"Hello, WORLD 123!");
      str_lower(l);
      let res = lua_tolstring_ref(l, -1);
      assert_eq!(res, Some(b"hello, world 123!".as_slice()));
      (*l).set_top(0);

      // test upper
      lua_pushlstring_bytes(l, b"Hello, world 123!");
      str_upper(l);
      let res = lua_tolstring_ref(l, -1);
      assert_eq!(res, Some(b"HELLO, WORLD 123!".as_slice()));
      (*l).set_top(0);

      // test reverse
      lua_pushlstring_bytes(l, b"Hello!");
      str_reverse(l);
      let res = lua_tolstring_ref(l, -1);
      assert_eq!(res, Some(b"!olleH".as_slice()));
      (*l).set_top(0);

      // test str_sub
      lua_pushlstring_bytes(l, b"abcdef");
      (*l).push_integer(2);
      (*l).push_integer(4);
      str_sub(l);
      let res = lua_tolstring_ref(l, -1);
      assert_eq!(res, Some(b"bcd".as_slice()));
      (*l).set_top(0);

      // test str_byte
      lua_pushlstring_bytes(l, b"a");
      let n = str_byte(l);
      assert_eq!(n, 1);
      assert_eq!((*l).to_integer(-1).unwrap_or(0), b'a' as i32);
      (*l).set_top(0);

      lua_close(l);
    }
  }
}
