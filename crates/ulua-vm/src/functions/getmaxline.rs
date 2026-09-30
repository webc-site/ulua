use crate::{
  functions::{c_slice, lua_g_getline::lua_g_getline},
  records::proto::Proto,
};

/// # Safety
/// `p` 须为指向存活 `Proto` 的共享引用：`code`/`lineinfo` 与 `sizecode` 自洽，且子原型数组
/// `p.p[0..sizep]` 每个元素须为存活 `Proto`（递归下传）；本函数纯读，不写 Proto。
pub(crate) unsafe fn getmaxline(p: &Proto) -> i32 {
  unsafe {
    // 全部指令行号取最大；空 proto 时为 -1
    let mut result: i32 = (0..p.sizecode)
      .map(|i| lua_g_getline(p, i))
      .max()
      .unwrap_or(-1);

    for &sub in c_slice(p.p, p.sizep as usize) {
      result = result.max(getmaxline(&*sub));
    }

    result
  }
}
