use ulua_common::functions::format_append::formatAppend;

use crate::records::state_dot::StateDot;

impl StateDot {
  pub fn start_node(&mut self, index: i32) {
    formatAppend(&mut self.result, format_args!("n{} [", index));
  }
}
