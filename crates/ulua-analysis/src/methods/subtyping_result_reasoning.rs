use crate::{
  records::subtyping_result::SubtypingResult,
  type_aliases::subtyping_reasonings::SubtypingReasonings,
};

impl SubtypingResult {
  pub fn reasoning(&self) -> &SubtypingReasonings {
    &self.reasoning
  }
}
