use crate::{records::iterator::Iterator, type_aliases::constraint_vertex::ConstraintVertex};

impl Iterator {
  #[inline]
  pub fn operator_deref(&self) -> ConstraintVertex {
    let cl = unsafe { self.cl.as_ref() };
    cl.order[self.index].clone()
  }
}
