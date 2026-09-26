use crate::{
  functions::math_modf::modf,
  macros::{luau_f_arm::luau_f_arm, setnvalue::setnvalue},
};

luau_f_arm! {
  /// C++ `luauF_modf`（`lbuiltins.cpp:330`）：`math.modf` 的快速调用实现。
  ///
  /// 上游写为 `double ip; double fp = modf(a1, &ip);`，两个结果按
  /// `res = ip`、`res + 1 = fp` 的次序落栈；`modf` 的 C 语义（±inf 给 ±0 分数、
  /// NaN 双 NaN、整数值保留 ±0 符号、`ip` 是截断值而非 `a1 - fp`）由
  /// [`modf`] 统一实现，`math.modf` 与这里共用一份。
  ///
  /// # Safety
  /// `l` 为当前快速调用的存活 `lua_State`；`arg0` 指向的实参、`res` 结果槽均有效可读/可写，实参数与 `nparams`、结果数与 `nresults` 相符；`args` 的 `None` 表示 FASTCALL1 单实参派发，本实现不读取。
  pub fn luau_f_modf [] (res, arg0, nresults, _args, nparams) {
    // Safety: 契约保证 `arg0`/`args` 指向存活实参 TValue（双 f64 payload 可读），结果按 nresults 协议写回
    unsafe {
      if nparams >= 1 && nresults <= 2 && (*arg0).is_number() {
        // C 语义（±inf 给 ±0 分数、NaN 双 NaN、整数值保留 ±0 符号）统一由
        // `math_modf::modf` 实现，与 `math.modf` 共用一份
        let (fp, ip) = modf((*arg0).as_number());

        setnvalue!(res, ip);
        setnvalue!(res.add(1), fp);
        2
      } else {
        -1
      }
    }
  }
}
