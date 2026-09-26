use core::ffi::c_char;

use crate::{
  enums::lua_status::LuaStatus, functions::lua_tolstring::lua_tolstring_ref,
  records::lua_state::LuaState,
};

/// NUL 结尾字节串（`*const c_char` 契约调用点 `.as_ptr().cast()`；§10 不引入 `CStr`/`c"…"`）。
const WHAT_RUNTIME_ERROR: &[u8] = b"lua_exception: runtime error\0";
const WHAT_SYNTAX_ERROR: &[u8] = b"lua_exception: syntax error\0";
const WHAT_MEM_BLOCK_TOO_BIG: &[u8] = b"lua_exception: memory allocation error: block too big\0";
const WHAT_ERROR_IN_ERROR_HANDLING: &[u8] = b"lua_exception: error in error handling\0";
const WHAT_UNEXPECTED_STATUS: &[u8] = b"lua_exception: unexpected exception status\0";

#[repr(C)]
#[derive(Debug)]
pub struct lua_exception {
  pub(crate) l: *mut LuaState,
  pub(crate) status: i32,
}

impl lua_exception {
  pub fn new(l: *mut LuaState, status: i32) -> Self {
    Self { l, status }
  }

  pub fn what(&self) -> *const c_char {
    // LUA_ERRRUN 的错误对象在栈顶
    if self.status == LuaStatus::ErrRun as i32 {
      // Safety: self.l 为抛出点存活的 LuaState 且 ERRRUN 时错误对象在其栈顶（抛出与捕获同线程）；
      // 返回指针指向串首字节（Lua 串缓冲恒有终止 NUL，C 串语义与旧出参形态一致）
      if let Some(s) = unsafe { lua_tolstring_ref(self.l, -1) } {
        return s.as_ptr().cast::<c_char>();
      }
    }

    // 状态到 what 消息的映射（与 cpp 消息文本一致；`b"..\0"` NUL 结尾静态字节串）
    match self.status {
      s if s == LuaStatus::ErrRun as i32 => WHAT_RUNTIME_ERROR.as_ptr().cast(),
      s if s == LuaStatus::ErrSyntax as i32 => WHAT_SYNTAX_ERROR.as_ptr().cast(),
      s if s == LuaStatus::ErrMem as i32 => WHAT_MEM_BLOCK_TOO_BIG.as_ptr().cast(),
      s if s == LuaStatus::ErrErr as i32 => WHAT_ERROR_IN_ERROR_HANDLING.as_ptr().cast(),
      _ => WHAT_UNEXPECTED_STATUS.as_ptr().cast(),
    }
  }
}

/// # Safety
///
/// `lua_exception` 只作为 panic 载荷镜像 C++ 的 throw/catch：抛出与捕获（`lua_pcall`）
/// 都发生在同一线程，`*mut LuaState` 在跨线程边界前即被消费、绝不会被另一线程解引用。
/// 实现 `Send` 仅为满足 `panic_any` 对载荷的 trait 约束，不代表指针可安全跨线程使用。
unsafe impl Send for lua_exception {}
