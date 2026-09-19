//! `extractStringTable` 移植与字节码 fixture 的二进制字符串保真测试
//! （原位于 src/functions/extract_string_table.rs 的 #[cfg(test)] 模块，移入 tests/）。

use ulua_unit_test::{
  functions::extract_string_table::extract_string_table,
  records::{
    bytecode_compiler_fixture::BytecodeCompilerFixture,
    bytecode_inliner_fixture::BytecodeInlinerFixture,
  },
};

#[test]
fn string_table_preserves_bytes() {
  let data = [6, 3, 3, 0, 3, b'a', 0, 255, 2, 195, 169];
  assert_eq!(
    extract_string_table(&data),
    [vec![], vec![b'a', 0, 255], vec![195, 169]]
  );
}

#[test]
fn compiler_fixture_preserves_binary_strings() {
  let mut fixture = BytecodeCompilerFixture::new();
  let function = fixture.build_bytecode(r#"return "\000\255\254""#, 0);
  assert!(function.is_some());
  assert!(fixture.strings.iter().any(|s| s == &[0, 255, 254]));
}

#[test]
fn inliner_fixture_preserves_binary_strings() {
  let mut fixture = BytecodeInlinerFixture::new();
  let functions = fixture.build_bytecode(
    r#"
      local function inlinee() return "\000\255\254" end
      local function caller() return inlinee() end
    "#,
    0,
  );
  assert!(functions.is_some());
  assert!(fixture.strings.iter().any(|s| s == &[0, 255, 254]));
}
