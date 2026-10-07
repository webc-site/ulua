use crate::macros::{luau_f_arm::luau_f_arm, setnvalue::setnvalue};

luau_f_arm! {
  /// C++ `luauF_sqrt`（`lbuiltins.cpp`）：`math.sqrt` 的快速调用实现。
  ///
  /// # Safety
  /// `l` 为当前快速调用的存活 `lua_State`；`arg0` 指向存活实参，`res` 结果槽有效可写。
  pub fn luau_f_sqrt [] (res, arg0, nresults, _args, nparams) {
    // SAFETY: 契约保证 arg0 指向存活实参 TValue，结果按 nresults 协议写回
    unsafe {
      if nparams >= 1 && nresults <= 1 && (*arg0).is_number() {
        setnvalue!(res, (*arg0).as_number().sqrt());
        1
      } else {
        -1
      }
    }
  }
}
