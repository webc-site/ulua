//! Source: `VM/src/lstrlib.cpp:47`
//!
//! `string.reverse` — copy the argument bytes into a fresh buffer back-to-front.

use crate::{functions::str_shared::str_transform1, records::lua_state::LuaState};

/// # Safety
///
/// `l` 必须指向本次 strlib 调用的存活 `LuaState`，所需实参按索引可读且栈顶有结果余量。
pub(crate) unsafe extern "C-unwind" fn str_reverse(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 存活，str_transform1 内部按 len 界读写等长缓冲
  unsafe {
    // 源逆序 zip 目标切片，单次遍历消除索引与越界检查
    str_transform1(l, |dst, src| {
      for (d, &s) in dst.iter_mut().zip(src.iter().rev()) {
        *d = s;
      }
    })
  }
}
