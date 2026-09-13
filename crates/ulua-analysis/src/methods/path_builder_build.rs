use core::mem::take;

use crate::records::{path::Path, path_builder::PathBuilder};
pub trait PathBuilderBuild {
  fn build(&mut self) -> Path;
}

impl PathBuilderBuild for PathBuilder {
  fn build(&mut self) -> Path {
    Path::from_components(take(&mut self.components))
  }
}
