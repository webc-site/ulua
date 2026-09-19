use std::{string::String, vec::Vec};

use ulua_common::functions::format_append::format_append;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn dump_source_remarks(&self) -> String {
    let mut result = String::new();

    let mut remarks: Vec<(i32, String)> = self.dump_remarks.clone();
    // C++ `std::sort(remarks)` orders by the WHOLE (line, message) pair, so within a
    // line remarks are ordered lexicographically by text ("builtin ..." before
    // "inlining ...") and the consecutive-duplicate skip can collapse repeated inline
    // remarks. Sorting by line only left them in insertion order.
    remarks.sort();
    let mut remarks = remarks.into_iter().peekable();

    for (i, line) in self.dump_source.iter().enumerate() {
      let line_no = (i + 1) as i32;

      let indent: usize = line
        .bytes()
        .take_while(|&b| b == b' ' || b == b'\t')
        .count();

      // 归并输出归属当前行的 remark
      while let Some((_, msg)) = remarks.next_if(|(no, _)| *no == line_no) {
        format_append(
          &mut result,
          format_args!("{:.*}-- remark: {}\n", indent, line, msg),
        );

        // 跳过重复 remark（内联/展开导致）：cpp `remarks[next] == remarks[next-1]`
        // 是 (line, message) 整对相等，行不同不得吞并
        while remarks
          .next_if(|(no, m)| *no == line_no && *m == msg)
          .is_some()
        {}
      }

      result.push_str(line);
      if i + 1 < self.dump_source.len() {
        result.push('\n');
      }
    }

    result
  }
}
