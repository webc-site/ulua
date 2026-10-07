//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：对应 C++
//! `CLI/src/Repl.cpp:118` 的 `lua_collectgarbage`。两侧实现逻辑逐行相同。

use core::ffi::c_int;

use ulua_vm::{
  enums::lua_gc_op::LuaGcOp,
  functions::{lua_gc::lua_gc, lua_l_checklstring::lua_l_checklstring_ref},
  macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

/// `collectgarbage` 缺省参数与比较值（cpp 双用途：栈参默认值 + 内容比较）。
/// Rust 形取参后单形态即可——不再需要 `luaL_optlstring` 收口点的 NUL 结尾副本。
const GC_OPT_COLLECT: &[u8] = b"collect";

/// # Safety
///
/// `l` 必须是有效、活跃的 `LuaState` 指针。
// DELIBERATE DEVIATION（review.md §9.3）：以 `extern "C-unwind"` `lua_CFunction`
// 形态注册进 VM 并被 Lua 调用点按 C ABI 回调，裸 `*mut LuaState`/`c_int` 系槽位
// 契约；内部 `lua_gc`/`checklstring`/`luaL_error` 均为 ulua-vm c-API 边界。
pub unsafe extern "C-unwind" fn lua_collectgarbage(l: *mut LuaState) -> c_int {
  // Safety: 本 fn 是装入 VM 的 `lua_CFunction`（真 C ABI 边界），VM 在调用点交出的 `l`
  // 非空且在本次调用期内活跃；REPL/VM 单线程驱动，该借用存活期内无并存可变别名。
  // 边界一次物化为借用后，下游全走 ulua-vm 的安全引用形方法。
  let l = unsafe { &mut *l };
  // cpp `luaL_optlstring(L, 1, "collect", NULL)` 的 Rust 化取形：none/nil 槽
  // 落默认串，其余槽走 checklstring 切片形态（数字自动转换、非转换类型按 cpp
  // 抛 "string expected" 发散），长度出参本就弃用，C 形指针与 null_mut 哨兵随之消失。
  let option: &[u8] = if l.is_none_or_nil(1) {
    GC_OPT_COLLECT
  } else {
    // Safety: `l` 为活跃状态机；返回切片在本次调用期内有效（实参被栈槽持有）。
    lua_l_checklstring_ref(l, 1)
  };

  if option == GC_OPT_COLLECT {
    lua_gc(l, LuaGcOp::Collect as c_int, 0);
    return 0;
  }

  if option == b"count" {
    let c = lua_gc(l, LuaGcOp::Count as c_int, 0);
    l.push_number(c as f64);
    return 1;
  }

  // luaL_error! 恒发散（longjmp），无需回退值
  luaL_error!(l, "collectgarbage must be called with 'count' or 'collect'")
}
