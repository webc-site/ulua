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
