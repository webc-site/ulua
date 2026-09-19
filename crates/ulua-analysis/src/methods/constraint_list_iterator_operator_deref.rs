use crate::{
  records::iterator::Iterator, type_aliases::blocked_constraint_id::BlockedConstraintId,
};

impl Iterator {
  #[inline]
  pub fn operator_deref(&self) -> BlockedConstraintId {
    let cl = unsafe { self.cl.as_ref() };
    cl.order[self.index].clone()
  }
}
