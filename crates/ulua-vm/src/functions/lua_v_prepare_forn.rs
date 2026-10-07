use crate::{
  functions::{lua_g_forerror_l::lua_g_forerror_str, lua_v_tonumber::lua_v_tonumber},
  records::{lua_state::LuaState, slot::Slot},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 须为存活 `LuaState`；`plimit`/`pstep`/`pinit` 各须指向其栈数组内一个可读写、对齐的
/// `TValue` 槽（FORN 建立期就地改写数值），且调用期间栈不重分配（`lua_g_forerror_str` 抛错
/// 止于本调用，句柄不跨借用存续）。cpp `lvmutils.cpp` luaV_prepareFORN。
pub unsafe fn lua_v_prepare_forn(l: *mut LuaState, plimit: StkId, pstep: StkId, pinit: StkId) {
  // SAFETY: 契约保证三槽指针均指向存活可读写 TValue，据此派生的句柄读写面成立；
  // 循环逐槽串行使用（同帧句柄无并发别名），解引用窗口止于各次读/写。
  unsafe {
    // 数组字面量求值顺序钉死 cpp（lvmutils.cpp luaV_prepareFORN）的
    // init → limit → step 校验次序
    for (p, what) in [
      (Slot::from_raw(pinit), "initial value"),
      (Slot::from_raw(plimit), "limit"),
      (Slot::from_raw(pstep), "step"),
    ] {
      if !p.get().is_number() {
        // cpp 以同槽裸指针 `luaV_tonumber(p, p)` 就地改写；`lua_v_tonumber` 收口
        // 引用面（B2-1）后共享/可变借用不可同址并存，改经本地暂存转换、成功写回槽，
        // 值语义逐位一致（FORN 建立期冷路径）
        let mut temp = TValue::default();
        if lua_v_tonumber(p.get(), &mut temp).is_some() {
          p.set(&temp);
        } else {
          lua_g_forerror_str(l, p.as_const_ptr(), what);
        }
      }
    }
  }
}
