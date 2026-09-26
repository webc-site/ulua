use core::fmt::{Debug, Formatter, Result};

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
    // union 字段安全初始化：CallTarget 全零与 zeroed 位等价
    Self {
      call_target: FeedbackVectorSlotCallTarget::default(),
    }
  }
}

impl Debug for FeedbackVectorSlotData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("FeedbackVectorSlotData")
      // Safety: call_target 是该 union 唯一变体，任何位模式都是其合法值（Default 亦经它构造）
      .field("call_target", unsafe { &self.call_target })
      .finish()
  }
}
