use crate::{
  functions::{
    auxresume::auxresume, auxwrapfinish::auxwrapfinish, interrupt_thread::interrupt_thread,
    lua_tothread::lua_tothread,
  },
  macros::{
    co_status_break::CO_STATUS_BREAK, lua_lib_fn::lua_lib_fn, lua_upvalueindex::lua_upvalueindex,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：upvalue 1（`lua_upvalueindex(1)`）须为一个 thread 值，
/// `lua_tothread` 返回的 `co` 非空且存活，供 `auxresume(l,co,narg)` 使用（narg=`(*l).top-(*l).base`，即当前帧
/// 实参数，须 ≥0 且这些槽均在 `(*l).base..(*l).top` 内）；`auxresume`/`interrupt_thread`/`auxwrapfinish` 可抛错/触发 GC。
/// cpp VM/src/lcorolib.cpp:292
pub unsafe fn auxwrapy(l: *mut LuaState) -> i32 {
  unsafe {
    let co = lua_tothread(&*l, lua_upvalueindex(1))
      .expect("cowrap 建立 upvalue1 为 thread，契约保证非空");
    // r16-b2 收编：顶-基槽距读数落既有 get_top 门面——其本体 slot_distance(base, top)
    // 即被替代式 `top.offset_from(base) as i32` 的同址同宽镜像（现读位点不变）；
    // isize→i32 折形在现域无截差（协程栈槽距受 LUAI_MAXSTACK 约束、远小于 i32::MAX）
    let narg = (*l).get_top();
    let r = auxresume(l, co, narg);
    if r == CO_STATUS_BREAK {
      interrupt_thread(l, co)
    } else {
      auxwrapfinish(l, r)
    }
  }
}

lua_lib_fn!(pub fn auxwrapy, auxwrapy_arm);
