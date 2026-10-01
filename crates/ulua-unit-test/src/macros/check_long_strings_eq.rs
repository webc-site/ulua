#[macro_export]
macro_rules! CHECK_LONG_STRINGS_EQ {
  ($a:expr, $b:expr) => {
    let aa = $a;
    let bb = $b;
    let a_lines = ulua_common::functions::split::split(&aa, '\n');
    let b_lines = ulua_common::functions::split::split(&bb, '\n');

    assert_eq!(
      a_lines.len(),
      b_lines.len(),
      "Line counts don't match: {} != {}",
      a_lines.len(),
      b_lines.len()
    );

    // 行数相等已由上方断言保证，zip 按行配对逐行比对
    for (i, (a_raw, b_raw)) in a_lines.iter().zip(b_lines.iter()).enumerate() {
      let a_line = ulua_common::functions::strip::strip(a_raw);
      let b_line = ulua_common::functions::strip::strip(b_raw);

      assert_eq!(
        a_line, b_line,
        "Mismatch on line {} between:\n\t«{}»\nand\t«{}»\n",
        i, a_line, b_line
      );
    }
  };
}

pub use CHECK_LONG_STRINGS_EQ;
