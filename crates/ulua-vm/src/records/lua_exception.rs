use alloc::borrow::Cow;

use crate::{
  enums::lua_status::LuaStatus, functions::lua_tolstring::lua_tolstring_ref,
  records::lua_state::LuaState,
};

use memchr::memchr;

// review.md §10 收形：what 消息模板为无终止 NUL 的原生字节窗。旧 `*const c_char`
// 消费面（`cstr_cow`/`cstr_bytes`）按首 NUL 截读，与这里的全窗等值（模板无内部
// NUL）；本结构非 C ABI 镜像（panic 载荷、无布局消费者），`#[repr(C)]` 随之消亡。
const WHAT_RUNTIME_ERROR: &[u8] = b"lua_exception: runtime error";
const WHAT_SYNTAX_ERROR: &[u8] = b"lua_exception: syntax error";
const WHAT_MEM_BLOCK_TOO_BIG: &[u8] = b"lua_exception: memory allocation error: block too big";
const WHAT_ERROR_IN_ERROR_HANDLING: &[u8] = b"lua_exception: error in error handling";
const WHAT_UNEXPECTED_STATUS: &[u8] = b"lua_exception: unexpected exception status";

/// cpp 镜像类名（原 `#[repr(C)]` 曾豁免命名 lint；§10 收形后按 cpp 名保留，局部豁免）
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct lua_exception {
  pub(crate) l: *mut LuaState,
  pub(crate) status: i32,
}

impl lua_exception {
  pub fn new(l: *mut LuaState, status: i32) -> Self {
    Self { l, status }
  }

  /// 错误消息字节窗（旧 `what() -> *const c_char` 的原生收形）：
  /// `Cow::Borrowed` 恒为不含终止 NUL 的完整观察值——
  /// - `ErrRun` 且栈顶错误对象为串：解引用 `self.l` 取栈顶串（旧消费面 NUL
  ///   扫描读，此处同点截断于首个 NUL，逐字节等值）；
  /// - 其余：静态状态模板。
  ///
  /// # Safety
  /// `ErrRun` 路径解引用 `self.l`：抛出与捕获同线程、`self.l` 指向抛出点存活
  /// `LuaState` 且 `-1` 为错误对象槽（`lua_d_throw`/`lua_d_rawrunprotected` 协议）。
  /// 返回借用的 `'a` 窗随该状态串缓冲存续（既有指针寿命契约的类型表达）。
  pub unsafe fn what<'a>(&self) -> Cow<'a, [u8]> {
    // LUA_ERRRUN 的错误对象在栈顶
    if self.status == LuaStatus::ErrRun as i32 {
      // SAFETY: 契约保证 self.l 为抛出点存活 LuaState 且 ERRRUN 时错误对象在其栈顶；
      // 旧观察面对返回指针按首 NUL 截读，此处同点截断，两路字节全等
      if let Some(s) = unsafe { lua_tolstring_ref(self.l, -1) } {
        let end = memchr(0, s).unwrap_or(s.len());
        return Cow::Borrowed(&s[..end]);
      }
    }

    // 状态到 what 消息的映射
    Cow::Borrowed(match self.status {
      s if s == LuaStatus::ErrRun as i32 => WHAT_RUNTIME_ERROR,
      s if s == LuaStatus::ErrSyntax as i32 => WHAT_SYNTAX_ERROR,
      s if s == LuaStatus::ErrMem as i32 => WHAT_MEM_BLOCK_TOO_BIG,
      s if s == LuaStatus::ErrErr as i32 => WHAT_ERROR_IN_ERROR_HANDLING,
      _ => WHAT_UNEXPECTED_STATUS,
    })
  }
}

/// # Safety
///
/// `lua_exception` 只作为 panic 载荷镜像 C++ 的 throw/catch：抛出与捕获（`lua_pcall`）
/// 都发生在同一线程，`*mut LuaState` 在跨线程边界前即被消费、绝不会被另一线程解引用。
/// 实现 `Send` 仅为满足 `panic_any` 对载荷的 trait 约束，不代表指针可安全跨线程使用。
unsafe impl Send for lua_exception {}
