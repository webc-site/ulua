use alloc::vec::Vec;

use crate::type_aliases::def_id_def::DefId;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Phi {
  pub operands: Vec<DefId>,
}
