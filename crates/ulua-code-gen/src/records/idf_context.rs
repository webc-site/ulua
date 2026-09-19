use alloc::{collections::BinaryHeap, vec::Vec};

use crate::records::{block_and_ordering::BlockAndOrdering, idf_visit_marks::IdfVisitMarks};

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct IdfContext {
  pub queue: BinaryHeap<BlockAndOrdering>,
  pub worklist: Vec<u32>,
  pub visits: Vec<IdfVisitMarks>,
  pub idf: Vec<u32>,
}
