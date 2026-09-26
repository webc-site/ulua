use crate::{
  functions::{
    auxresume::auxresume, auxwrapfinish::auxwrapfinish, interrupt_thread::interrupt_thread,
    lua_tothread::lua_tothread,
  },
  macros::{co_status_break::CO_STATUS_BREAK, lua_upvalueindex::lua_upvalueindex},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：upvalue 1（`lua_upvalueindex(1)`）须为一个 thread 值，
/// `lua_tothread` 返回的 `co` 非空且存活，供 `auxresume(l,co,narg)` 使用（narg=`(*l).top-(*l).base`，即当前帧
/// 实参数，须 ≥0 且这些槽均在 `(*l).base..(*l).top` 内）；`auxresume`/`interrupt_thread`/`auxwrapfinish` 可抛错/触发 GC。
/// cpp VM/src/lcorolib.cpp:292
pub unsafe extern "C-unwind" fn auxwrapy(l: *mut LuaState) -> i32 {
  unsafe {
    let co =
      lua_tothread(l, lua_upvalueindex(1)).expect("cowrap 建立 upvalue1 为 thread，契约保证非空");
    let narg = (*l).top.offset_from((*l).base) as i32;
    let r = auxresume(l, co, narg);
    if r == CO_STATUS_BREAK {
      interrupt_thread(l, co)
    } else {
      auxwrapfinish(l, r)
    }
  }
}
