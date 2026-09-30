// Port of Luau's `fuzz/typeck.cpp`: run arbitrary source through the static type
// checker (ulua-analysis, via `ulua_rt::Checker`). The checker must never
// panic/crash — only return `Ok(())` or `Err(Vec<TypeDiagnostic>)`. (Several of
// the bugs hardened in this repo lived in the analysis layer, so this target is
// especially valuable.)

use std::str::from_utf8;

fn exercise_input(data: &[u8]) {
  if let Ok(src) = from_utf8(data) {
    // 复用线程本地 Checker（见 `ulua_fuzz::check_reuse`），避免每输入重建前端。
    let _ = ulua_fuzz::check_reuse(src);
  }
}

ulua_fuzz::fuzz_main!(exercise_input);
