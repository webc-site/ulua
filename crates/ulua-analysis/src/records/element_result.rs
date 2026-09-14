use alloc::{string::String, vec::Vec};

use crate::records::to_string_span::ToStringSpan;
#[derive(Debug, Clone, Default)]
pub struct ElementResult {
  pub str: String,
  pub spans: Vec<ToStringSpan>,
}

unsafe impl Send for ElementResult {}
unsafe impl Sync for ElementResult {}
