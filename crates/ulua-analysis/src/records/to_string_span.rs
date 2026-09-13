use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToStringSpan {
  pub(crate) start_pos: usize,
  pub(crate) end_pos: usize,
  pub(crate) r#type: TypeId,
}
