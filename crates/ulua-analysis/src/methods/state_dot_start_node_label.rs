use ulua_common::functions::format_append::format_append;

use crate::records::state_dot::StateDot;

impl StateDot {
  pub fn start_node_label(&mut self) {
    format_append(&mut self.result, format_args!("label=\""));
  }
}
