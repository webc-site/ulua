use alloc::vec::Vec;

use crate::type_aliases::def_id_def::DefId;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct FunctionCapture {
  pub capture_defs: Vec<DefId>,
  pub all_versions: Vec<DefId>,
  pub version_offset: usize,
}
