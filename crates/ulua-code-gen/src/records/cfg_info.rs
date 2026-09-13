use alloc::vec::Vec;

use crate::records::{block_ordering::BlockOrdering, register_set::RegisterSet};

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CfgInfo {
  pub predecessors: Vec<u32>,
  pub predecessors_offsets: Vec<u32>,

  pub successors: Vec<u32>,
  pub successors_offsets: Vec<u32>,

  pub idoms: Vec<u32>,

  pub dom_children: Vec<u32>,
  pub dom_children_offsets: Vec<u32>,

  pub dom_ordering: Vec<BlockOrdering>,

  pub r#in: Vec<RegisterSet>,
  pub def: Vec<RegisterSet>,
  pub out: Vec<RegisterSet>,

  pub captured: RegisterSet,
  pub written: RegisterSet,
}
