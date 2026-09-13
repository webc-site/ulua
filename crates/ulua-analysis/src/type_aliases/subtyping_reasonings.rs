use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{
  subtyping_reasoning::SubtypingReasoning, subtyping_reasoning_hash::SubtypingReasoningHash,
};

pub type SubtypingReasonings = DenseHashSet<SubtypingReasoning, SubtypingReasoningHash>;
