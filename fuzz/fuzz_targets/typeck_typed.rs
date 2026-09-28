// Type-checker fuzzing with TYPE-RICH programs: type annotations, aliases,
// generics, unions/intersections/optionals, function & table types, type
// assertions, and partially-typed code. The untyped `typeck` target barely
// exercises the type system (ulua's unique surface) — this generator drives
// inference + the annotation/unification/cycle machinery where every serious bug
// found in this repo lived (#6, the DFG aliasing failures, the shared_seen leak).
//
// Oracle: never panic/abort/hang — only Ok or a structured diagnostic.

fn exercise_input(data: &[u8]) {
  let src = ulua_fuzz::generate_typed(data);
  // 复用线程本地 Checker（见 `ulua_fuzz::check_reuse`），避免每输入重建前端。
  let _ = ulua_fuzz::check_reuse(&src);
}

ulua_fuzz::fuzz_main!(exercise_input);
