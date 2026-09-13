use crate::records::{path::Path, subtyping_reasoning::SubtypingReasoning};

impl SubtypingReasoning {
  pub fn sub_path(&self) -> &Path {
    &self.sub_path
  }
}
