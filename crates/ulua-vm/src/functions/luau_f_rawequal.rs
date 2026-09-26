use crate::{
  functions::lua_o_rawequal_obj::lua_o_rawequal_obj,
  macros::{luau_f_arm::luau_f_arm, luau_fastmath_end::LUAU_FASTMATH_END, setbvalue::setbvalue},
  type_aliases::t_value::TValue,
};

luau_f_arm! {
  /// C++ `luauF_rawequal`（`lbuiltins.cpp:971`）：`rawequal` 的快速调用实现。
  ///
  /// # Safety
  /// `l` 为当前快速调用的存活 `lua_State`；`arg0` 与 `args` 指向的实参、`res` 结果槽均有效可读/可写，实参数与 `nparams`、结果数与 `nresults` 相符；`args` 为 `None` 表示 FASTCALL1 的单实参派发（无第二参数槽）。
  pub fn luau_f_rawequal [] (res, arg0, nresults, args, nparams) => args {
    // Safety: 契约保证 `arg0`/`args` 指向存活实参 TValue（比较仅读标签与值），`res` 为可写结果槽
    unsafe {
      LUAU_FASTMATH_END!();

      if nparams >= 2 && nresults <= 1 {
        setbvalue!(
          res,
          lua_o_rawequal_obj(arg0 as *const TValue, args as *const TValue)
        );
        return 1;
      }

      -1
    }
  }
}
