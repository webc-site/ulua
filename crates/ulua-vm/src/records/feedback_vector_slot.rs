use core::{
  fmt::{Debug, Formatter, Result},
  mem::zeroed,
};

use crate::enums::feedback_vector_slot_kind::FeedbackVectorSlotKind;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FeedbackVectorSlot {
  pub kind: FeedbackVectorSlotKind,
  pub data: FeedbackVectorSlotData,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union FeedbackVectorSlotData {
  pub call_target: FeedbackVectorSlotCallTarget,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct FeedbackVectorSlotCallTarget {
  pub pc: u32,
  pub proto: u32,
  pub hits: u32,
}

impl Default for FeedbackVectorSlot {
  fn default() -> Self {
    Self {
      kind: FeedbackVectorSlotKind::CallTarget,
      data: FeedbackVectorSlotData {
        call_target: FeedbackVectorSlotCallTarget::default(),
      },
    }
  }
}

impl Default for FeedbackVectorSlotData {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

impl Debug for FeedbackVectorSlotData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("FeedbackVectorSlotData")
      .field("call_target", unsafe { &self.call_target })
      .finish()
  }
}
