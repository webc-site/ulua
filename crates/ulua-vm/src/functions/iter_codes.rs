use core::ptr::null;

use crate::{
  functions::iter_aux::iter_aux_arm, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v41 收形后
/// 校验/连压 3 值全经安全门面，仅登记闭包一次为不安全调用而落窄块，故本体降为安全 `fn`）：`l`
/// 须处于可抛错的受保护帧——`check_bytes(1)` 要求索引 1 为字符串否则抛错发散；随后
/// `push_c_function`(C 闭包)/`push_value(1)`/`push_integer` 连压 3 个值，`top` 后须留 ≥3 空槽
/// （由 utf8 库调用点保证）；push 可触发 GC。
/// cpp VM/src/lutf8lib.cpp:267
pub fn iter_codes(l: &mut LuaState) -> i32 {
  l.check_bytes(1);
  // SAFETY: `iter_aux_arm` 是本文件同卫生域生成的合法 `unsafe extern "C-unwind" fn`（遵循 Lua
  // C 函数约定），`null()` 为 `push_c_function` 契约允许的空 debugname。
  unsafe { l.push_c_function(Some(iter_aux_arm), null()) };
  l.push_value(1);
  l.push_integer(0);
  3
}

lua_lib_fn!(pub fn iter_codes @ref, iter_codes_arm);
