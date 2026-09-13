use ulua_common::functions::format_append::formatAppend;

use crate::{records::state_dot::StateDot, type_aliases::type_pack_id::TypePackId};

impl StateDot {
  pub fn finish_node_label_type_pack_id(&mut self, tp: TypePackId) {
    if self.opts.show_pointers {
      formatAppend(&mut self.result, format_args!("\n0x{:p}", tp));
    }
    self.result += "\"";
  }
}
