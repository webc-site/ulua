use crate::{
  functions::{lua_g_forerror_l::lua_g_forerror_l, lua_v_tonumber::lua_v_tonumber},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_v_prepare_forn(l: *mut LuaState, plimit: StkId, pstep: StkId, pinit: StkId) {
  unsafe {
    // 数组字面量求值顺序钉死 cpp（lvmutils.cpp luaV_prepareFORN）的
    // init → limit → step 校验次序
    for (p, what) in [
      (pinit, c"initial value"),
      (plimit, c"limit"),
      (pstep, c"step"),
    ] {
      if !(*p).is_number() && lua_v_tonumber(p, &mut *p).is_none() {
        lua_g_forerror_l(l, p, what.as_ptr());
      }
    }
  }
}
