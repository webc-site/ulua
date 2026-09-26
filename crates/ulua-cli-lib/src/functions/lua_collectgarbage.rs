//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：对应 C++
//! `CLI/src/Repl.cpp:118` 的 `lua_collectgarbage`。两侧实现逻辑逐行相同。

use core::{ffi::c_int, ptr::null_mut};

use ulua_common::functions::c_str::cstr_bytes;
use ulua_vm::{
  enums::lua_gc_op::LuaGcOp,
  functions::{lua_gc::lua_gc, lua_l_optlstring::lua_l_optlstring, lua_pushnumber::lua_pushnumber},
  macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

/// `collectgarbage` 缺省参数与比较值（cpp 双用途：栈参默认值 + 内容比较）。
/// 双形态常量：比较用无 NUL 版，`luaL_optlstring` 收口点用 NUL 结尾版。
const GC_OPT_COLLECT: &[u8] = b"collect";
const GC_OPT_COLLECT_C: &[u8] = b"collect\0";

/// # Safety
///
/// `l` 必须是有效、活跃的 `LuaState` 指针。
pub unsafe extern "C-unwind" fn lua_collectgarbage(l: *mut LuaState) -> c_int {
  // Safety: `# Safety` 契约保证 `l` 为活跃状态机；默认值是静态 NUL 结尾字节串。
  // 保留空指针：C 接口 luaL_optlstring(L, arg, def, szfl) 契约
  // 允许长度出参 szfl 传 NULL，本函数按内容比较，不需要长度。
  let option = unsafe { lua_l_optlstring(l, 1, GC_OPT_COLLECT_C.as_ptr().cast(), null_mut()) };
  // Safety: `luaL_optlstring` 恒返回 NUL 结尾字符串（栈上参数或默认值）。
  let option = unsafe { cstr_bytes(option) };

  if option == GC_OPT_COLLECT {
    // Safety: 同上，`l` 为活跃状态。
    unsafe { lua_gc(l, LuaGcOp::Collect as c_int, 0) };
    return 0;
  }

  if option == b"count" {
    // Safety: 同上，`l` 为活跃状态。
    let c = unsafe { lua_gc(l, LuaGcOp::Count as c_int, 0) };
    // Safety: 同上；压回的数取自上一步 `lua_gc` 的返回值。
    unsafe { lua_pushnumber(l, c as f64) };
    return 1;
  }

  // luaL_error! 恒发散（longjmp），无需回退值
  // Safety: 同上；宏内 `lua_pushstring`/`lua_error` 均以 `l` 为活跃状态为前提。
  unsafe { luaL_error!(l, "collectgarbage must be called with 'count' or 'collect'") }
}
