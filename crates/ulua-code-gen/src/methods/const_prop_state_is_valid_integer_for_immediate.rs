use crate::records::const_prop_state::ConstPropState;

impl ConstPropState {
  pub fn is_valid_integer_for_immediate(&mut self, i: i32) -> bool {
    (-4095..=4095).contains(&i)
  }
}
