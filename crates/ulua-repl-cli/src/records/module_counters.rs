use alloc::{string::String, vec::Vec};

use crate::records::function_counters::FunctionCounters;

#[derive(Debug, Clone, Default)]
pub struct ModuleCounters {
  pub(crate) name: String,
  pub(crate) functions: Vec<FunctionCounters>,
}
