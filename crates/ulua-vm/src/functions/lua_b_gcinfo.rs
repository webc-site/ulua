use crate::{
  enums::lua_gc_op::LuaGcOp,
  functions::{lua_gc::lua_gc, lua_pushinteger::lua_pushinteger},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`（`lua_gc(l,LUA_GCCOUNT,..)` 读其 `(*l).global` 的 GC 统计，可触发一次 GC 步进）：
/// 结果经 `lua_pushinteger` 写回需 `(*l).top` 后 ≥1 空槽；调用点应处于受保护帧。
/// cpp VM/src/lbaselib.cpp:194
pub unsafe extern "C-unwind" fn lua_b_gcinfo(l: *mut LuaState) -> i32 {
  unsafe {
    lua_pushinteger(l, lua_gc(l, LuaGcOp::Count as i32, 0));
    1
  }
}
