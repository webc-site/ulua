//! 激活记录（activation record）：`lua_getinfo` 按选项掩码回填的一次栈帧快照。
//!
//! 出处 cpp `VM/include/lua.h:488-502`。Rust 侧不复刻 C 的 `char*` 指针字段与
//! 内嵌 `char[256]` 缓冲（review.md §0/§10）：字符串字段在 `auxgetinfo` 填写时
//! 即从 GC 管理的 TString / VM 静态字面量拷成拥有的 `Vec<u8>`，「未填写」以
//! `Option::None` 表达（替代原裸指针 null 哨兵），`what` 收敛为枚举，
//! `isvararg` 收敛为 `bool`。仅 `userdata` 仍是跨回调透传的不透明指针，保留
//! 裸形并附 `# Safety` 契约。

use alloc::vec::Vec;
use core::ffi::c_void;

/// `what` 字段：本记录所指函数的类别（cpp `lua_Debug.what`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LuaWhat {
  /// 未经 `s` 选项填写。
  #[default]
  Unknown,
  /// Lua 闭包。
  Lua,
  /// C / Rust（原生）函数。
  C,
  /// 主 chunk。
  Main,
  /// 被 tail call 替换的帧。
  Tail,
}

impl LuaWhat {
  /// cpp 中该字段写入的字符串字面量。
  pub fn as_str(self) -> &'static str {
    match self {
      LuaWhat::Unknown => "",
      LuaWhat::Lua => "Lua",
      LuaWhat::C => "C",
      LuaWhat::Main => "main",
      LuaWhat::Tail => "tail",
    }
  }
}

/// 激活记录快照，字段按 `lua_getinfo` 的选项掩码填写，未填写者保持默认值。
#[derive(Clone, Debug, Default)]
pub struct LuaDebug {
  /// `(n)` 函数名；无名字或未请求 `n` 时为 `None`。
  pub name: Option<Vec<u8>>,
  /// `(s)` 函数类别。
  pub what: LuaWhat,
  /// `(s)` chunk 源名；未请求 `s` 时为 `None`。
  pub source: Option<Vec<u8>>,
  /// `(s)` 人类可读的短源名；未请求 `s` 时为 `None`。
  pub short_src: Option<Vec<u8>>,
  /// `(s)` 函数定义所在行。
  pub linedefined: i32,
  /// `(l)` 当前执行行。
  pub currentline: i32,
  /// `(p)` VM 内全局唯一的 proto id；C 函数为 0。
  pub protoid: i32,
  /// `(p)` proto 在自身字节码模块内的下标；C 函数为 -1。
  pub bytecodeid: i32,
  /// `(u)` 上值数量。
  pub nupvals: u8,
  /// `(a)` 参数数量。
  pub nparams: u8,
  /// `(a)` 是否为可变参数函数。
  pub isvararg: bool,
  /// 仅 `luau_callhook` 有效：向 hook 回调透传的不透明用户数据。
  ///
  /// # Safety（字段契约）
  /// 该指针由 [`luau_callhook`](crate::functions::luau_callhook::luau_callhook)
  /// 的调用方传入、仅在本次 hook 回调期间有效，从不被 VM 解引用；`None` 对应
  /// cpp 的 `nullptr`。消费方（`extern "C-unwind"` hook）须视其为跨回调句柄，
  /// 不得在回调返回后继续持有或解引用。
  pub userdata: Option<*mut c_void>,
}
