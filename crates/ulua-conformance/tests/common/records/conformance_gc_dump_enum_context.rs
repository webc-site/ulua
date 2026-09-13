use std::collections::HashMap;

use crate::common::records::conformance_gc_dump_node::ConformanceGcDumpNode;

#[derive(Default)]
pub struct ConformanceGcDumpEnumContext {
  pub nodes: HashMap<usize, ConformanceGcDumpNode>,
  pub edges: HashMap<usize, usize>,
  pub seen_target_string: bool,
  pub errors: Vec<String>,
}
