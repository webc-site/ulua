use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  macros::lua_l_error::luaL_error,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

use crate::{
  functions::{lua_requireinternal::lua_requireinternal, resolve_require::NOT_ALLOWED_MSG},
  records::navigation_context::RequireHost,
};

/// `lua_getinfo` 的选项串：仅取 `what` 字段（review.md §10：`what` 形参已是原生
/// 选项字节窗，`b"C"`/`b"Lua"` 一类模板按整窗比较，不含终止 NUL）。
const GETINFO_WHAT_OPT: &[u8] = b"s";

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
  // cpp `LuaDebug ar;` 的零初值：§10 后该记录为原生 Rust 形态（串体窗 + 定长
  // `ShortSrc` 载体），`Default` 即全零/空窗初值，`mem::zeroed()` 与「POD 全零
  // 合法」的论证一并消解。只作 lua_getinfo 的出参槽，由该 C API 按语义整体填充。
  let mut ar = LuaDebug::default();
  // Safety: 真 FFI 入口，l 为 VM 调 require 闭包时传入的存活 state，入口一次
  // 重建独占借用（不与其他别名冲突），后续均为本帧上的 C API 调用。
  let l: &mut LuaState = unsafe { &mut *l };

  // Safety: lua_getinfo 按其 C API 契约回填 ar：自 1 起沿调用栈上溯停在首个非 C
  // 函数帧（cpp 的手动 level 游标），越界由返回 0 先行报错；回填的 `what`/`source`
  // 为 `Option<&'static [u8]>` 原生窗（`None` 即原 null 哨兵，判空由 Option 承载），
  // 读取窗内函数值存活故串体不失效。尾段 lua_requireinternal::<C> 按其自身契约
  // 操作本帧栈与 upvalue（`C` 即本闭包体的单态化宿主类型）。
  unsafe {
    for level in 1.. {
      if lua_getinfo(l, level, GETINFO_WHAT_OPT, &mut ar) == 0 {
        luaL_error!(l, "{NOT_ALLOWED_MSG}");
      }
      // `what` 模板字节窗首字节判定：`None`（原 null）与空窗同为 false。
      let is_c_function = ar.what.is_some_and(|what| what.first() == Some(&b'C'));
      if !is_c_function {
        break;
      }
    }
    // `source` 为原样串体窗（含 `=`/`@` sigil）；`None` 译空 chunkname，与旧
    // `cstr_bytes(null)` 的空串保底同果。
    let requirer_chunkname = ar.source.unwrap_or_default();
    lua_requireinternal::<C>(l, requirer_chunkname)
  }
}
