use alloc::{string::String, vec::Vec};
use core::{cmp, slice};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{log_2::log2, write_byte::write_byte, write_int::write_int},
  records::bytecode_builder::BytecodeBuilder,
};

/// line-info 分组跨度初值（覆盖全表的足够大跨度）。
const INITIAL_SPAN: usize = 1 << 24;
/// 行号与组内基线之差的最大值（单字节增量编码）。
const MAX_LINE_DELTA: i32 = 255;

impl BytecodeBuilder {
  pub fn write_line_info(&self, ss: &mut String) {
    LUAU_ASSERT!(!self.lines.is_empty());

    let mut span = INITIAL_SPAN;

    let mut offset = 0;
    while offset < self.lines.len() {
      let mut next = offset;
      let mut min = self.lines[offset];
      let mut max = self.lines[offset];

      while next < self.lines.len() && next < offset + span {
        min = cmp::min(min, self.lines[next]);
        max = cmp::max(max, self.lines[next]);

        if max - min > MAX_LINE_DELTA {
          break;
        }
        next += 1;
      }

      if next < self.lines.len() && next - offset < span {
        span = 1 << log2((next - offset) as i32);
      } else {
        offset += span;
      }
    }

    let mut baseline_one = 0;
    let mut baseline_scratch = Vec::new();
    let baseline_size = (self.lines.len() - 1) / span + 1;

    if baseline_size > 1 {
      baseline_scratch.resize(baseline_size, 0);
    }

    let baseline = if baseline_size > 1 {
      &mut baseline_scratch
    } else {
      slice::from_mut(&mut baseline_one)
    };

    for offset in (0..self.lines.len()).step_by(span) {
      let mut next = offset;
      let mut min = self.lines[offset];

      while next < self.lines.len() && next < offset + span {
        min = cmp::min(min, self.lines[next]);
        next += 1;
      }

      baseline[offset / span] = min;
    }

    let logspan = log2(span as i32);
    write_byte(ss, logspan as u8);

    let mut last_offset = 0u8;
    for (i, &line) in self.lines.iter().enumerate() {
      let delta = line - baseline[i >> logspan];
      LUAU_ASSERT!((0..=MAX_LINE_DELTA).contains(&delta));

      write_byte(ss, (delta as u8).wrapping_sub(last_offset));
      last_offset = delta as u8;
    }

    let mut last_line = 0;
    for &line in &baseline[..baseline_size] {
      write_int(ss, line - last_line);
      last_line = line;
    }
  }
}
