use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_checkstack::lua_checkstack, lua_rawcheckstack::lua_rawcheckstack, lua_xmove::lua_xmove,
  },
  macros::{co_status_error::CO_STATUS_ERROR, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn auxresumecont(l: *mut LuaState, co: *mut LuaState) -> i32 {
  unsafe {
    if (*co).status == LuaStatus::Ok as u8 || (*co).status == LuaStatus::Yield as u8 {
      // r16-b2 收编：顶-基槽距读数落既有 get_top 门面——其本体 slot_distance(base, top)
      // 即被替代式 `top.offset_from(base) as i32` 的同址同宽镜像（现读位点不变）；
      // isize→i32 折形在现域无截差（协程栈槽距受 LUAI_MAXSTACK 约束、远小于 i32::MAX）
      let nres = (*co).get_top();
      if lua_checkstack(&mut *l, nres + 1) == 0 {
        luaL_error!(l, "too many results to resume");
      }
      lua_xmove(&mut *co, &mut *l, nres);
      nres
    } else {
      lua_rawcheckstack(&mut *l, 2);
      lua_xmove(&mut *co, &mut *l, 1);
      CO_STATUS_ERROR
    }
  }
}
