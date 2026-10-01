use core::ffi::c_char;

use crate::{
  functions::{cstr_bytes, lua_l_checklstring::lua_l_checklstring},
  records::lua_state::LuaState,
};

/// # Safety
/// C-ABI 镜像垫片（`def`/`len` 裸指针为 C 约定面，按 review.md §2 保留 unsafe 形；
/// `l` 的存活前提已由 `&mut` 接收者类型承载）：`l` 须为存活 `LuaState` 且第 `narg` 槽可读；`len` 为空或指向可写 usize；
/// 走默认分支且 `def` 非空时 `def` 须为可读 NUL 结尾 C 串（strlen 语义扫描）；返回值指向栈内串数据或 `def`，压栈/回收后即失效。
/// 对应 cpp laux.cpp:184。
pub unsafe fn lua_l_optlstring(
  l: &mut LuaState,
  narg: i32,
  def: *const c_char,
  len: *mut usize,
) -> *const c_char {
  // SAFETY: 契约保证 `l` 存活且 narg 槽可读；非 nil 时返回的串指针与 len 输出落在该 TValue 串数据界内
  unsafe {
    if l.is_none_or_nil(narg) {
      if !len.is_null() {
        // C++ `def ? strlen(def) : 0`：cstr_bytes 零拷贝求长度，与 strlen 语义一致
        *len = if !def.is_null() {
          cstr_bytes(def).len()
        } else {
          0
        };
      }
      def
    } else {
      lua_l_checklstring(l, narg, len)
    }
  }
}
