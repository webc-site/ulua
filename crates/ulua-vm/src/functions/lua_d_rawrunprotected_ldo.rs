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
use core::{ffi::c_void, ptr::eq};
use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_common::{functions::c_str::with_c_str, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    install_lua_exception_panic_hook::install_lua_exception_panic_hook,
    lua_g_pusherror::lua_g_pusherror,
  },
  records::{lua_exception::lua_exception, lua_state::LuaState},
  type_aliases::pfunc::Pfunc,
};

/// # Safety
/// `l` 须为存活 `LuaState`：本函数是其唯一的 `catch_unwind` 保护边界，`f` 可为 NULL（Some 时以 `(l,ud)` 调用，
/// 允许 `f` 经 `lua_d_throw` 抛 `lua_exception`，其 `e.l` 必须等于 `l`，LUAU_ASSERT 校验）；`ud` 由 `f` 自行解释，
/// 须为 `f` 约定的有效载荷指针或 NULL；非 `lua_exception` 载荷走 `luaG_pusherror` 兜底（会改栈、可分配）。
/// cpp VM/src/ldo.cpp:123
pub unsafe fn lua_d_rawrunprotected(l: *mut LuaState, f: Pfunc, ud: *mut c_void) -> i32 {
  unsafe {
    let mut status: i32 = 0;

    // Silence the default panic-hook noise for the VM's longjmp-emulation
    // unwinds (a caught `lua_exception` is a normal Lua error, not a crash).
    install_lua_exception_panic_hook();

    let result = catch_unwind(AssertUnwindSafe(|| {
      if let Some(f) = f {
        f(l, ud);
      }
    }));

    if let Err(payload) = result {
      if let Some(e) = payload.downcast_ref::<lua_exception>() {
        // 捕获的异常必须与抛出它的 Luau state 一致（cpp ldo.cpp:134-138）
        LUAU_ASSERT!(eq(e.l, l));
        status = e.status;
      } else {
        // Luau 自身不会抛此类 payload；兜底捕获外部实现逃逸的 panic
        // （对应 C++ `catch (std::exception&)` 分支），推入消息让
        // 后续错误处理继续推进
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
          lua_g_pusherror(l, FALLBACK_MSG.as_ptr().cast());
        } else {
          with_c_str(msg.as_bytes(), |cmsg| {
            lua_g_pusherror(l, cmsg);
          });
        }
        // C++ nests a second try/catch for OOM while pushing; a Rust
        // allocation failure aborts, so the LUA_ERRMEM arm has no analog.
        status = LuaStatus::ErrRun as i32;
      }
    }

    status
  }
}

/// # Safety
/// 与 `lua_d_rawrunprotected` 完全同契约（本函数纯委托）：`l` 存活、`f` 可空、`ud` 由 `f` 约定解释。
pub unsafe fn lua_d_rawrunprotected_mut(l: *mut LuaState, f: Pfunc, ud: *mut c_void) -> i32 {
  unsafe { lua_d_rawrunprotected(l, f, ud) }
}
