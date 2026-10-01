use crate::{
  functions::{c_slice, lua_g_getline::lua_g_getline},
  records::proto::Proto,
};

/// 全部指令行号取最大；空 proto 时为 -1（cpp `ldebug.cpp`）。
///
/// 契约：`p` 须为指向存活 `Proto` 的共享引用：`code`/`lineinfo` 与 `sizecode` 自洽，且子原型数组
/// `p.p[0..sizep]` 每个非空元素须为存活 `Proto`（递归下传，空槽跳过）。本函数纯读，不写 Proto。
pub(crate) fn getmaxline(p: &Proto) -> i32 {
  // 全部指令行号取最大；空 proto 时为 -1
  let mut result: i32 = (0..p.sizecode)
    .map(|i| lua_g_getline(p, i))
    .max()
    .unwrap_or(-1);

  // SAFETY: 契约保证 `p.p[0..sizep]` 各元素为存活 `Proto` 的非拥有共享句柄，空槽经 `as_ref` 折叠为
  // None 跳过；递归仅以只读借用下传，不写、不回收，别名只读不冲突。借用半径只覆盖本循环。
  let subs = unsafe { c_slice(p.p, p.sizep as usize) };
  for sub in subs.iter().filter_map(|&sub| unsafe { sub.as_ref() }) {
    result = result.max(getmaxline(sub));
  }

  result
}
