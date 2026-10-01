//! Source: `Analysis/include/Luau/ToString.h:66-77`
//! C++ 为纯聚合体，字段直接公开，不设包装 getter。
use alloc::{string::String, vec::Vec};

use crate::records::to_string_span::ToStringSpan;

#[derive(Debug, Clone, Default)]
pub struct ToStringResult {
  pub name: String,
  /// Records which TypeId produced each substring of the output. Only recorded for named types
  pub type_spans: Vec<ToStringSpan>,
  pub invalid: bool,
  pub error: bool,
  pub cycle: bool,
  pub truncated: bool,
}
