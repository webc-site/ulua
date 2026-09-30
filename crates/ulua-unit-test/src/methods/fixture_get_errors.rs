//! C++ `Fixture::getErrors` 与 `Fixture::dumpErrors(std::ostream&, errors)`
//! （tests/Fixture.cpp:605-633、669-674）：失败断言的错误诊断文本。
use alloc::string::String;
use core::iter::repeat_n;

use ulua_analysis::{
  functions::to_string_error::to_string_type_error, records::check_result::CheckResult,
};

use crate::records::fixture::Fixture;

impl Fixture {
  /// 把每个错误的原文行与列位置指示格式化为诊断文本（测试失败信息）。
  pub fn get_errors(&mut self, cr: &CheckResult) -> String {
    let mut out = String::new();

    for error in &cr.errors {
      out.push_str("\nError: ");
      out.push_str(&to_string_type_error(error));
      out.push('\n');

      let line_index = error.location.begin.line as usize;
      let source = self.file_resolver.source.get(&error.module_name);
      let the_line = source
        .as_deref()
        .and_then(|source| source.split('\n').nth(line_index));

      let Some(the_line) = the_line else {
        out.push_str("\tSource not available?\n");
        continue;
      };

      out.push_str("Line:\t");
      out.push_str(the_line);
      out.push('\n');

      let start_col = error.location.begin.column as usize;
      let end_col = if error.location.end.line == error.location.begin.line {
        error.location.end.column as usize
      } else {
        the_line.len()
      };
      let mark_len = end_col.saturating_sub(start_col).max(1);

      out.push('\t');
      out.extend(repeat_n(' ', start_col));
      out.extend(repeat_n('-', mark_len));
      out.push('\n');
    }

    out
  }
}
