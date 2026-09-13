use crate::records::const_prop_state::ConstPropState;

impl ConstPropState {
  pub fn invalidate_userdata_data(&mut self) {
    self.useradata_tag_cache.clear();
  }
}
