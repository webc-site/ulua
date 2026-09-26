//! Source: `VM/src/lstrlib.cpp:66`
//!
//! `string.lower` — lowercase each byte of the argument into a fresh buffer.

use crate::{
  functions::str_shared::str_transform1, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 strlib 调用的存活 `LuaState`，所需实参按索引可读且栈顶有结果余量。
pub(crate) unsafe fn str_lower(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 存活，str_transform1 内部按 len 界读写等长缓冲
  unsafe {
    str_transform1(l, |dst, src| {
      // u8 的 ASCII 小写映射对非 ASCII 字节恒等，与 cpp 逐字节 tolower 语义一致
      for (d, &s) in dst.iter_mut().zip(src) {
        *d = s.to_ascii_lowercase();
      }
    })
  }
}

lua_lib_fn!(pub(crate) fn str_lower, str_lower_arm);
