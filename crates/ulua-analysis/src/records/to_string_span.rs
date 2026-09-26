//! Source: `Analysis/include/Luau/ToString.h:59-64`
//! C++ 为纯聚合体，字段直接公开，不设包装 getter。
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToStringSpan {
  pub start_pos: usize,
  pub end_pos: usize,
  pub r#type: TypeId,
}
