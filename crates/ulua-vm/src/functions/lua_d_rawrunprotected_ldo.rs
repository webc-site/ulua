//! Source: `VM/src/ldo.cpp:124-159` (hand-ported; C++-exceptions build
//! flavor — `lua_d_throw` is `panic_any(lua_exception)`, this is the matching
//! `catch_unwind` boundary; see translation/design-cards/lvmexecute.md)
//! Source: `VM/src/ldo.cpp:124`
//!
//! 历史上这里有一份与 `lua_d_rawrunprotected_ldo` 平行的独立 `catch_unwind`
//! 实现（早期移植产物）。C++ 上游只有唯一一个 `lua_d_rawrunprotected`，两条
//! 路径的 payload 分发、panic-hook 安装与 `catch (std::exception&)` 回退完全
//! 同构，故收敛为对 `lua_d_rawrunprotected` 的纯委托；保留函数与导出符号，
//! `lua_d_pcall` / `shrinkstackprotected` 的调用点无需变动。

use alloc::string::String;
use core::{any::Any, ffi::c_void, ptr::eq};
use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_common::{functions::c_str::with_c_str, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    install_lua_exception_panic_hook::install_lua_exception_panic_hook,
    lua_g_pusherror::lua_g_pusherror, resume::resume,
  },
  records::{lua_exception::lua_exception, lua_state::LuaState},
  type_aliases::{pfunc::Pfunc, stk_id::StkId},
};

/// 非 `lua_exception` 载荷的兜底解码（cpp `catch (std::exception&)` 分支）。
///
/// Luau 自身不会抛此类 payload；这里捕获外部实现逃逸的 panic，推入消息让
/// 后续错误处理继续推进。会改栈、可分配。
///
/// # Safety
/// `l` 须为存活 `LuaState`（本函数向其栈推送错误对象）。
unsafe fn decode_error_payload(l: *mut LuaState, payload: Box<dyn Any + Send>) -> i32 {
  unsafe {
    let msg: &str = if let Some(s) = payload.downcast_ref::<&str>() {
      s
    } else if let Some(s) = payload.downcast_ref::<String>() {
      s.as_str()
    } else {
      "unknown error"
    };
    // 载荷含内嵌 NUL 时退到固定文案；否则经 with_c_str 补 NUL 传递指针。
    const FALLBACK_MSG: &[u8] = b"invalid error message\0";
    if memchr::memchr(0, msg.as_bytes()).is_some() {
      lua_g_pusherror(&mut *l, FALLBACK_MSG.as_ptr().cast());
    } else {
      with_c_str(msg.as_bytes(), |cmsg| {
        lua_g_pusherror(&mut *l, cmsg);
      });
    }
    // C++ nests a second try/catch for OOM while pushing; a Rust
    // allocation failure aborts, so the LUA_ERRMEM arm has no analog.
    LuaStatus::ErrRun as i32
  }
}

/// `catch_unwind` 边界的公共壳：装 hook、跑 `f`、解码捕获载荷。
///
/// # Safety
/// `l` 须为存活 `LuaState`；`f` 须满足受保护边界契约（可 `lua_d_throw`，捕获后
/// 按 `lua_exception` 解码）。
#[inline(always)]
unsafe fn rawrunprotected_shell(l: *mut LuaState, f: impl FnOnce()) -> i32 {
  unsafe {
    // Silence the default panic-hook noise for the VM's longjmp-emulation
    // unwinds (a caught `lua_exception` is a normal Lua error, not a crash).
    install_lua_exception_panic_hook();

    let result = catch_unwind(AssertUnwindSafe(f));

    match result {
      Ok(()) => 0,
      Err(payload) => {
        if let Some(e) = payload.downcast_ref::<lua_exception>() {
          // 捕获的异常必须与抛出它的 Luau state 一致（cpp ldo.cpp:134-138）
          LUAU_ASSERT!(eq(e.l, l));
          e.status
        } else {
          decode_error_payload(l, payload)
        }
      }
    }
  }
}

/// # Safety
/// `l` 须为存活 `LuaState`：本函数经 [`rawrunprotected_shell`] 构成 `catch_unwind`
/// 保护边界（与 [`lua_d_rawrunprotected_resume`] 等直连入口共享同一壳），`f` 可为 NULL
/// （Some 时以 `(l,ud)` 调用，允许 `f` 经 `lua_d_throw` 抛 `lua_exception`，其 `e.l`
/// 必须等于 `l`，LUAU_ASSERT 校验）；`ud` 由 `f` 自行解释，须为 `f` 约定的有效载荷
/// 指针或 NULL；非 `lua_exception` 载荷走 `luaG_pusherror` 兜底（会改栈、可分配）。
/// cpp VM/src/ldo.cpp:123
pub unsafe fn lua_d_rawrunprotected(l: *mut LuaState, f: Pfunc, ud: *mut c_void) -> i32 {
  // SAFETY: `l` 存活由调用方契约保证；`f` 的调用与载荷解码满足受保护边界契约
  unsafe {
    rawrunprotected_shell(l, || {
      if let Some(f) = f {
        f(l, ud);
      }
    })
  }
}

/// 粗粒度协程恢复专用受保护驱动（cpp `lua_resume` 里
/// `luaD_rawrunprotected(L, resume, L->top - nargs)` 的直连形）。
///
/// 与 [`lua_d_rawrunprotected`] 传 `Some(resume)` 的捕获语义逐位一致，但回调以
/// 直接调用进入而不经 `Pfunc` 函数指针——省去一次 Option 判空与间接跳转（全 VM
/// 共享单一 `blr` site），并给 LLVM 留出把 `resume` 内联进闭包的自由度。语义
/// 不变量：`resume` 经 `lua_d_throw` 抛出的 `lua_exception`（`e.l == l`）由同一
/// `catch_unwind` 边界承接，与 cpp 的 longjmp 落回 `resume` 发起处同构。
///
/// # Safety
/// `l` 须为存活协程状态；`first_arg` 须为 `lua_resume` 按其调用契约现读的首实参
/// 栈槽（cpp `L->top - nargs`），满足 `resume` 的入参前置。
#[inline(always)]
pub(crate) unsafe fn lua_d_rawrunprotected_resume(l: *mut LuaState, first_arg: StkId) -> i32 {
  // SAFETY: `l` 存活由 `lua_resume` 契约保证；`resume` 即粗粒度恢复回调本体，
  // 其入参前置（首实参栈槽）由 `lua_resume` 的调用契约承担
  unsafe { rawrunprotected_shell(l, || resume(l, first_arg)) }
}

/// # Safety
/// 与 `lua_d_rawrunprotected` 完全同契约（本函数纯委托）：`l` 存活、`f` 可空、`ud` 由 `f` 约定解释。
pub unsafe fn lua_d_rawrunprotected_mut(l: *mut LuaState, f: Pfunc, ud: *mut c_void) -> i32 {
  unsafe { lua_d_rawrunprotected(l, f, ud) }
}
