use alloc::borrow::Cow;
use std::io::Write;

/// cpp `coverageCallback` (`CLI/src/Coverage.cpp`): 输出 FN/FNDA/DA 行。
///
/// 仅由 `coverage_dump.rs` 的 `coverage_callback_cb` (lua_getcoverage 回调 ABI)
/// 调用；VM 裸指针（`function`/`hits`）在该 C-ABI 外壳单点转成 Rust 借用后传入，
/// 本函数为纯安全逻辑。cpp 忽略 fprintf 返回值, 此处一致。
pub(crate) fn coverage_callback<W: Write>(
  out: &mut W,
  function: Option<&str>,
  linedefined: i32,
  depth: i32,
  hits: &[i32],
) {
  // cpp（Coverage.cpp `else if (function)`）对 null 输出独立的 `<anonymous>`
  // 标记，与空串译法不同，故 `function` 以 `Option` 区分 null 哨兵。
  let name = if depth == 0 {
    Cow::Borrowed("<main>")
  } else if let Some(func) = function {
    Cow::Owned(format!("{func}:{linedefined}"))
  } else {
    Cow::Owned(format!("<anonymous>:{linedefined}"))
  };

  let _ = writeln!(out, "FN:{linedefined},{name}");

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
