use crate::{
  functions::{c_slice, lua_g_getline::lua_g_getline},
  records::proto::Proto,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn getmaxline(p: *mut Proto) -> i32 {
  unsafe {
    // 全部指令行号取最大；空 proto 时为 -1
    let mut result: i32 = (0..(*p).sizecode)
      .map(|i| lua_g_getline(p, i))
      .max()
      .unwrap_or(-1);

    for &sub in c_slice((*p).p, (*p).sizep as usize) {
      result = result.max(getmaxline(sub));
    }

    result
  }
}
