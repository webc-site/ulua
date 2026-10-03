use crate::{
  functions::lua_rawcheckstack::lua_rawcheckstack,
  macros::{lua_lib_fn::lua_cont_fn, setbvalue::setbvalue, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `status` 为被保护调用返回的 `LUA_OK`/错误码（`l` 的存活与独占
/// 前提已由 `&mut` 接收者类型承载）。
/// cpp `lbaselib.cpp:358`。
pub(crate) unsafe fn lua_b_xpcallcont(l: &mut LuaState, status: i32) -> i32 {
  unsafe {
    if status == 0 {
      let base = l.base;
      // xpcall had an 'errfunc' before the results, so we just replace it with 'true' status
      setbvalue!(base, 1);
      l.top.offset_from(base) as i32
    } else {
      lua_rawcheckstack(l, 1);
      // r12-w7a2 收编（pcallcont 族同形单点）：rawcheckstack 后单次窗读预绑定，
      // 挪位/写 bool 均在已保余量窗内、无场域写，抬顶经 advance_top 原语逐位同值
      let top = l.top;
      // Move error 1 right to make space for the 'false' status
      setobj_2_s!(l, top, top.sub(1));
      setbvalue!(top.sub(1), 0);
      l.advance_top(1);
      2
    }
  }
}

lua_cont_fn!(pub(crate) fn lua_b_xpcallcont @ref, lua_b_xpcallcont_arm);
