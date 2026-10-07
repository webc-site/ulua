use crate::macros::luau_f_arm::luau_f_arm;

luau_f_arm! {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub fn luau_f_missing [] (_res, _arg0, _nresults, _args, _nparams) {
    -1
  }
}
