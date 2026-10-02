use crate::{
  functions::lua_yield::lua_yield, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为可 yield 的协程 `LuaState`：`(*l).base..(*l).top` 区间即待产出结果（nres=top-base，须 ≥0），
/// 该协程须由 `lua_resume`/`auxresume` 驱动、非主线程；`lua_yield` 通过 unwind 挂起当前协程并回到恢复点，
/// 调用方须处于受保护帧以承接跨 VM 边界的展开。
/// cpp VM/src/lcorolib.cpp:348
pub unsafe fn coyield(l: *mut LuaState) -> i32 {
  unsafe {
    // r16-b2 收编：顶-基槽距读数落既有 get_top 门面——其本体 slot_distance(base, top)
    // 即被替代式 `top.offset_from(base) as i32` 的同址同宽镜像（现读位点不变）；
    // isize→i32 折形在现域无截差（协程栈槽距受 LUAI_MAXSTACK 约束、远小于 i32::MAX）
    let nres = (*l).get_top();
    lua_yield(l, nres)
  }
}

lua_lib_fn!(pub fn coyield, coyield_arm);
