//! 核心保名 + `_arm` 一行边界壳骨架宏（r7 vm 内部 ABI B0 基建）。
//!
//! review §2/§3：`unsafe extern "C-unwind"` 真边界只准留在 FFI/注册表对接面。纯内部库函数
//! 采用两段式形态——Rust ABI 核心（保名、保文件、Rust 侧仍可直呼）+ 本宏生成的一行边界臂
//! （该函数唯一的 `extern "C-unwind"` fn，供 `LuaLReg` const 表 / `Some(..)` 存
//! `CClosure.f` 等指针位点）。复刻仓内两先例的组合：`lua_lib_arm!`（臂自写体，签名宏承包）
//! 与 `X`/`X_export` 成对（核心 + 一行转发壳）。本宏把签名、`# Safety` 契约与转发体全部
//! 单源进宏定义 ⇒ 函数文件只余核心与一行宏调用，注册站点只换一个 token。
//!
//! 用法（写在核心函数所在文件，核心与壳同卫生域）：
//! - `lua_lib_fn!(pub(crate) fn b_and, b_and_arm);` — 库 C 函数臂 `(l) -> i32`，
//!   与 `LuaCFunction`/`LuaLReg::new` 契约同形；
//! - `lua_cont_fn!(pub(crate) fn x_cont, x_cont_arm);` — 续延臂 `(l, status) -> i32`，
//!   与 `LuaContinuation` 契约同形（pcall/co 族 B4 批备妥，本票先行落地）。
//!
//! 不变量：核心绝不被 `_arm` 之外的方式存进任何 extern fn-ptr 槽位；panic 穿 Rust 核心帧
//! 后由 `_arm` 的 `extern "C-unwind"` 帧原样上传，与改造前满血 extern 体重合，行为零变。

#[macro_export]
macro_rules! lua_lib_fn {
  ($vis:vis fn $core:ident, $arm:ident) => {
    /// `extern "C-unwind"` 边界臂：一行转发本文件 Rust 核心，签名与契约由 `lua_lib_fn!` 单源。
    ///
    /// # Safety
    ///
    /// `l` 须满足核心的全部前提：本次受保护帧内存活 `LuaState`、实参栈槽按 API 索引约定
    /// 可读、栈顶预留结果空间；解引用/抛错/GC 义务见核心自身 `# Safety` 文档（契约单源）。
    $vis unsafe extern "C-unwind" fn $arm(
      l: *mut $crate::records::lua_state::LuaState,
    ) -> i32 {
      unsafe { $core(l) }
    }
  };
}

#[macro_export]
macro_rules! lua_cont_fn {
  ($vis:vis fn $core:ident, $arm:ident) => {
    /// `extern "C-unwind"` 续延臂：一行转发本文件 Rust 核心，签名与契约由 `lua_cont_fn!` 单源。
    ///
    /// # Safety
    ///
    /// `l`/`status` 须满足核心的全部前提（`LuaContinuation` 调用约定：resume 结束后受保护
    /// 帧内存活 `LuaState` 与协程终态码）；解引用/抛错/GC 义务见核心自身 `# Safety` 文档
    /// （契约单源）。
    $vis unsafe extern "C-unwind" fn $arm(
      l: *mut $crate::records::lua_state::LuaState,
      status: i32,
    ) -> i32 {
      unsafe { $core(l, status) }
    }
  };
}

pub use {lua_cont_fn, lua_lib_fn};
