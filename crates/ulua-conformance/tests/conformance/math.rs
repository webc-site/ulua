// 数值与日期标准库 fixture 用例
// 移植自 `cpp/tests/Conformance.test.cpp`。

#[test]
fn conformance_bitwise() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("bitwise.luau");
}

#[test]
fn conformance_date_time() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("datetime.luau");
}

#[test]
fn conformance_integers() {
  use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
  use ulua_common::fflag;

  use crate::common::{
    functions::{
      run_conformance::{codegen, run_fixture_setup},
      setup_native_helpers::setup_native_helpers,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _ncg_buffer_integer = ScopedFastFlag::new(&fflag::LuauCodegenBufferInteger, true);

  if fflag::LuauIntegerType2.get() && fflag::LuauIntegerLibrary.get() {
    run_fixture_setup("integers.luau", setup_native_helpers);
    if codegen() && luau_codegen_supported() != 0 {
      run_fixture_setup("integers_regspill.luau", setup_native_helpers);
    }
  }
}

/// cpp `Conformance.test.cpp:1281-1286` 用
/// `ScopedFastFlag luauMathRoundNegZero{FFlag::LuauMathRoundNegZero, true}` 把
/// `math.round` 固定在「保留 -0.0 符号」的分支上（flag 关闭时 `-0.0` 会经
/// `a1 < 0 ? -offset : offset` 变成 `+0.0`，`math.luau:357` 的
/// `1 / math.round(-0.0) == -math.huge` 就会失败）。本端口没有该 FFlag：
/// `ulua-vm/src/functions/math_round.rs` 无条件走 `f64::round()`（与 C99 `round`
/// 一样保留零的符号），语义恒等于上游 flag-on，故无需作用域 flag。
#[test]
fn conformance_math() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("math.luau");
}
