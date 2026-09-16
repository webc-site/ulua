use alloc::borrow::Cow;
use core::{
  ffi::{CStr, c_char, c_int},
  slice::from_raw_parts,
};
use std::io::Write;

/// cpp `coverageCallback` (`CLI/src/Coverage.cpp`): 输出 FN/FNDA/DA 行。
///
/// 仅由 `coverage_callback_cb` (lua_getcoverage 回调 ABI) 调用; `function`
/// 指针来自 VM, CStr 转换收敛于此外部入口。cpp 忽略 fprintf 返回值, 此处一致。
pub unsafe fn coverage_callback<W: Write>(
  out: &mut W,
  function: *const c_char,
  linedefined: c_int,
  depth: c_int,
  hits: *const c_int,
  size: usize,
) {
  let name = if depth == 0 {
    Cow::Borrowed("<main>")
  } else if !function.is_null() {
    // SAFETY: VM 契约保证 function 指向 NUL 结尾字符串
    let func = unsafe { CStr::from_ptr(function) }.to_string_lossy();
    Cow::Owned(format!("{func}:{linedefined}"))
  } else {
    Cow::Owned(format!("<anonymous>:{linedefined}"))
  };

  let _ = writeln!(out, "FN:{linedefined},{name}");

  // SAFETY: VM 契约保证 hits 指向 size 个元素
  let hits = unsafe { from_raw_parts(hits, size) };

  // 首个非 -1 计数即函数整体命中 (cpp 提前 break)
  for &hit in hits {
    if hit != -1 {
      let _ = writeln!(out, "FNDA:{hit},{name}");
      break;
    }
  }

  for (i, &hit) in hits.iter().enumerate() {
    if hit != -1 {
      let _ = writeln!(out, "DA:{i},{hit}");
    }
  }
}
