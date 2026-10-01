use crate::{
  functions::{lua_g_forerror_l::lua_g_forerror_str, lua_v_tonumber::lua_v_tonumber},
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_v_prepare_forn(l: *mut LuaState, plimit: StkId, pstep: StkId, pinit: StkId) {
  unsafe {
    // 数组字面量求值顺序钉死 cpp（lvmutils.cpp luaV_prepareFORN）的
    // init → limit → step 校验次序
    for (p, what) in [(pinit, "initial value"), (plimit, "limit"), (pstep, "step")] {
      if !(*p).is_number() {
        // cpp 以同槽裸指针 `luaV_tonumber(p, p)` 就地改写；`lua_v_tonumber` 收口
        // 引用面（B2-1）后共享/可变借用不可同址并存，改经本地暂存转换、成功写回槽，
        // 值语义逐位一致（FORN 建立期冷路径）
        let mut temp = TValue::default();
        if lua_v_tonumber(&*p, &mut temp).is_some() {
          *p = temp;
        } else {
          lua_g_forerror_str(l, p, what);
        }
      }
    }
  }
}
