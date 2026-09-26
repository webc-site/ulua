use crate::{functions::lua_yield::lua_yield, records::lua_state::LuaState};

/// # Safety
/// `l` 须为可 yield 的协程 `LuaState`：`(*l).base..(*l).top` 区间即待产出结果（nres=top-base，须 ≥0），
/// 该协程须由 `lua_resume`/`auxresume` 驱动、非主线程；`lua_yield` 通过 unwind 挂起当前协程并回到恢复点，
/// 调用方须处于受保护帧以承接跨 VM 边界的展开。
/// cpp VM/src/lcorolib.cpp:348
pub unsafe extern "C-unwind" fn coyield(l: *mut LuaState) -> i32 {
  unsafe {
    let nres = (*l).top.offset_from((*l).base) as i32;
    lua_yield(l, nres)
  }
}
