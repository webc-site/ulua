use ulua_common::functions::format_append::formatAppend;

use crate::records::state_dot::StateDot;

impl StateDot {
  pub fn start_node_label(&mut self) {
    formatAppend(&mut self.result, format_args!("label=\""));
  }
}
