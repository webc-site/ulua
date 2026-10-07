use ulua_common::functions::c_str::cstr;
use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  macros::lua_l_error::luaL_error,
  records::{
    lua_debug::{LuaDebug, LuaWhat},
    lua_state::LuaState,
  },
};

use crate::{
  functions::{lua_requireinternal::lua_requireinternal, resolve_require::NOT_ALLOWED_MSG},
  records::navigation_context::RequireHost,
};

/// `lua_getinfo` 的选项串：仅取 `what` 字段（NUL 结尾字节串，经 `cstr` 收口点转
/// C 指针，review.md §10：不散落 `.as_ptr().cast()`）。
const GETINFO_WHAT_OPT: &[u8] = b"s\0";

/// require 闭包体（cpp `requireLikeFunc`）：泛型参数 `C` 为注入时的宿主类型，
/// `luarequire_pushrequire::<C>` 以 `Some(lua_require::<C>)` 具名实例化后 coerce 为
/// `LuaCFunction`——擦除发生在函数指针层（每宿主一个单态化体），槽位读回零 `dyn`。
///
/// # Safety
///
/// `l` must be a valid pointer to a live `LuaState`；upvalue(1) 须为
/// `push_closure::<C>` 以同一 `C` 装箱的宿主 userdata（由注入点与闭包体同源单态化
/// 保证）。
pub(crate) unsafe extern "C-unwind" fn lua_require<C: RequireHost>(l: *mut LuaState) -> i32 {
  // cpp `LuaDebug ar;` 的初值：Rust 原生记录的「未填写」默认态，只作 lua_getinfo
  // 的出参槽，由该 C API 按语义整体填充 owned 字段。
  let mut ar = LuaDebug::default();
  // Safety: 真 FFI 入口，l 为 VM 调 require 闭包时传入的存活 state，入口一次
  // 重建独占借用（不与其他别名冲突），后续均为本帧上的 C API 调用。
  let l: &mut LuaState = unsafe { &mut *l };

  // Safety: lua_getinfo 按其 C API 契约回填 ar：自 1 起沿调用栈上溯停在首个非 C
  // 函数帧（cpp 的手动 level 游标），越界由返回 0 先行报错。回填的 `what` 是
  // `LuaWhat` 枚举、`source` 是 VM 拷成的 owned `Vec<u8>`（未填写为 `None`）。
  // `l.as_mut_ptr()` 由上一句独占借用借出、窗止于当句；尾段 lua_requireinternal::<C>
  // 已收形为安全 fn，其对 upvalue(1) 宿主槽与栈布局的要求即本闭包体的 `# Safety`
  // 契约（同源单态化）。
  let requirer_chunkname = unsafe {
    for level in 1.. {
      if lua_getinfo(l.as_mut_ptr(), level, cstr(GETINFO_WHAT_OPT), &mut ar) == 0 {
        luaL_error!(l, "{NOT_ALLOWED_MSG}");
      }
      if ar.what != LuaWhat::C {
        break;
      }
    }
    ar.source.as_deref().unwrap_or(b"")
  };
  lua_requireinternal::<C>(l, requirer_chunkname)
}
