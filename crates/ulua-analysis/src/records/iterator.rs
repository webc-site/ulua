use core::ptr::NonNull;

use crate::records::constraint_list::ConstraintList;

#[derive(Debug, Clone)]
pub struct Iterator {
  pub(crate) cl: NonNull<ConstraintList>,
  pub(crate) index: usize,
}
