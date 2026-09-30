// 字符串、模式匹配、UTF-8 与 buffer 标准库用例
// 移植自 `cpp/tests/Conformance.test.cpp`。

#[test]
fn conformance_buffers() {
  use crate::common::functions::{
    run_conformance::run_fixture_setup, setup_native_helpers::setup_native_helpers,
  };

  run_fixture_setup("buffers.luau", setup_native_helpers);
}

#[test]
fn conformance_pattern_match() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("pm.luau");
}

#[test]
fn conformance_string_conversion() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("strconv.luau");
}

#[test]
fn conformance_string_interp() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("stringinterp.luau");
}

#[test]
fn conformance_strings() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("strings.luau");
}

#[test]
fn conformance_utf_8() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("utf8.luau");
}
