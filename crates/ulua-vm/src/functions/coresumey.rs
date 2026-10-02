use crate::{
  functions::{
    auxresume::auxresume, coresumefinish::coresumefinish, interrupt_thread::interrupt_thread,
    lua_tothread::lua_tothread,
  },
  macros::{co_status_break::CO_STATUS_BREAK, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`lua_tothread(l,1)` 要求索引 1 存在（`luaL_argexpected` 对
/// NULL 抛错回退），返回的协程 `co` 非空且存活；narg=`(*l).top-(*l).base`-1（须 ≥0，索引 2 起为传入恢复实参）；
/// `auxresume`/`interrupt_thread`/`coresumefinish` 可再入 Lua、抛错、扩栈与触发 GC。
/// cpp VM/src/lcorolib.cpp:218
pub(crate) unsafe fn coresumey(l: *mut LuaState) -> i32 {
  unsafe {
    let co = lua_tothread(l, 1);
    (*l).arg_expected(co.is_some(), 1, "thread");
    let co = co.expect("arg_expected 已证 co 非空");
    // r16-b2 收编：顶-基槽距读数落既有 get_top 门面——其本体 slot_distance(base, top)
    // 即被替代式 `(top.offset_from(base) as i32) - 1` 的同址同宽镜像（现读位点不变、
    // `- 1` 次序保持）；isize→i32 折形在现域无截差（栈槽距受 LUAI_MAXSTACK 约束）
    let narg = (*l).get_top() - 1;
    let r = auxresume(l, co, narg);

    if r == CO_STATUS_BREAK {
      return interrupt_thread(l, co);
    }

    coresumefinish(l, r)
  }
}

lua_lib_fn!(pub(crate) fn coresumey, coresumey_arm);
