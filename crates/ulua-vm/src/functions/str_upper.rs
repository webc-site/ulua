//! Source: `VM/src/lstrlib.cpp:73`
//!
//! `string.upper` — uppercase each byte of the argument into a fresh buffer.

use crate::{
  functions::str_shared::str_transform1, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v38 收形后
/// 体内唯一点名转手、无裸操作，降为安全 `fn`）：`l` 须处于可抛错受保护帧，栈槽 #1 为串实参
/// （非串经 `str_transform1` 内的 `lua_l_checklstring_ref` 抛 "string expected" 发散），
/// `str_transform1` 内部按 len 界读写等长缓冲，结果压栈需 `top` 后 ≥1 空槽；分配可触发 GC。
pub(crate) fn str_upper(l: &mut LuaState) -> i32 {
  // SAFETY: `l` 为借用形式的存活调用帧（&mut 保证有效且独占），as_mut_ptr 由该借用重取裸指针
  // 交仍收裸形的 `str_transform1`，借用窗止于本调用；变换闭包仅在等长缓冲界内逐字节读写。
  unsafe {
    str_transform1(l.as_mut_ptr(), |dst, src| {
      // u8 的 ASCII 大写映射对非 ASCII 字节恒等，与 cpp 逐字节 toupper 语义一致
      for (d, &s) in dst.iter_mut().zip(src) {
        *d = s.to_ascii_uppercase();
      }
    })
  }
}

lua_lib_fn!(pub(crate) fn str_upper @ref, str_upper_arm);
