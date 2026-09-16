use alloc::vec::Vec;

use crate::records::mapped_generic_frame::MappedGenericFrame;
#[derive(Debug, Clone)]
pub struct MappedGenericEnvironment {
  pub(crate) frames: Vec<MappedGenericFrame>,
  pub(crate) current_scope_index: Option<usize>,
}
