use crate::records::const_prop_state::ConstPropState;

impl ConstPropState {
  pub fn is_valid_double_for_immediate(&mut self, d: f64) -> bool {
    (-4095.0..=4095.0).contains(&d) && (d as i32) as f64 == d
  }
}
