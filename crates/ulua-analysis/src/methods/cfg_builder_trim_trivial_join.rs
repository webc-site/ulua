use crate::records::{cfg_builder::CfgBuilder, join::Join};

impl CfgBuilder {
  pub fn trim_trivial_join(&mut self, _j: *mut Join) {}
}
