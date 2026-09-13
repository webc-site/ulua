use alloc::vec::Vec;

use crate::type_aliases::type_id::TypeId;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct IntersectionType {
  pub parts: Vec<TypeId>,
}
