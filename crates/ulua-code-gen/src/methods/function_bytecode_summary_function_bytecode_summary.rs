use alloc::{string::String, vec::Vec};

use crate::records::function_bytecode_summary::FunctionBytecodeSummary;

impl FunctionBytecodeSummary {
  pub fn new(source: String, name: String, line: i32, nesting_limit: u32) -> Self {
    let mut summary = Self {
      source,
      name,
      line,
      nesting_limit,
      counts: Vec::new(),
    };

    let op_limit = summary.get_op_limit() as usize;
    let mut counts: Vec<Vec<u32>> = Vec::with_capacity((1 + nesting_limit) as usize);
    for _ in 0..(1 + nesting_limit) {
      counts.push(vec![0u32; op_limit]);
    }

    summary.counts = counts;
    summary
  }
}
