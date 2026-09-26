use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    auxresumecont::auxresumecont, auxwrapfinish::auxwrapfinish, interrupt_thread::interrupt_thread,
    lua_tothread::lua_tothread,
  },
  macros::lua_upvalueindex::lua_upvalueindex,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 且处于 wrap 续体（yieldable C 帧）中，其 upvalue 1 号位必须是 coroutine 线程
/// （`lua_tothread` 读该槽并解引用返回的 `*co`），`co` 非空由建立续体的 `auxwrapy` 保证。转调
/// `interrupt_thread`/`auxresumecont`/`auxwrapfinish` 可抛错/可 GC，须在受保护帧。
/// cpp/VM/src/lcorolib.cpp:312 auxwrapcont。
pub(crate) unsafe extern "C-unwind" fn auxwrapcont(l: *mut LuaState, _status: i32) -> i32 {
  unsafe {
    let co = lua_tothread(l, lua_upvalueindex(1))
      .expect("auxwrapy 建立续体时 upvalue1 为 thread，契约保证非空");

    if (*co).status == LuaStatus::Break as u8 {
      return interrupt_thread(l, co);
    }

    let r = auxresumecont(l, co);
    auxwrapfinish(l, r)
  }
}
