use core::mem::zeroed;

use ulua_common::functions::c_str::cstr_bytes;
use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  macros::lua_l_error::luaL_error,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

use crate::functions::lua_requireinternal::lua_requireinternal;

/// `lua_getinfo` 的选项串：仅取 `what` 字段（NUL 结尾字节串，收口点转 C 指针）。
const GETINFO_WHAT_OPT: &[u8] = b"s\0";

/// # Safety
///
/// `l` must be a valid pointer to a live `LuaState`.
pub(crate) unsafe extern "C-unwind" fn lua_require(l: *mut LuaState) -> i32 {
  // Safety: zeroed 的 ar 只作 lua_getinfo 的出参槽，由该 C API 按语义整体填充。
  let mut ar: LuaDebug = unsafe { zeroed() };

  // Safety: 真 FFI 入口，l 为 VM 调 require 闭包时传入的存活 state；自 1 起沿
  // 调用栈上溯停在首个非 C 函数帧（cpp 的手动 level 游标），越界由 lua_getinfo
  // 返回 0 先行报错；`what` 是 VM 回填的 C 串（或 null），判空/解引用统一收口在
  // cstr_bytes 门面。
  unsafe {
    for level in 1.. {
      if lua_getinfo(l, level, GETINFO_WHAT_OPT.as_ptr().cast(), &mut ar) == 0 {
        luaL_error!(l, "require is not supported in this context");
      }
      // `what` 单字符判定经 cstr_bytes 门面收口（null 译空串→false）。
      // Safety: 外层 C-ABI 入口契约：ar.what 为本次 lua_getinfo 回填的 null 或
      // 调用期内存活的静态串。
      let is_c_function = cstr_bytes(ar.what).first() == Some(&b'C');
      if !is_c_function {
        break;
      }
    }
  }

  // 真 FFI 入口：C 串 → 字节经 cstr_bytes 门面单点收口，内部链全程走字节串
  // （ar.source 为 null 时译成空 chunkname，cpp 直接传指针，Rust 保底避免 UB）。
  // Safety: ar.source 保持 null 或指向本次调用期内存活的 VM/调用帧字符串。
  let requirer_chunkname = unsafe { cstr_bytes(ar.source) };

  // Safety: l 存活，lua_requireinternal 按其自身契约操作本帧栈与 upvalue。
  unsafe { lua_requireinternal(l, requirer_chunkname) }
}
