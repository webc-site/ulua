use crate::records::constraint::Constraint;

#[derive(Debug, Clone)]
pub struct BlockedTypePack {
  pub(crate) index: usize,
  pub(crate) owner: *mut Constraint,
}
