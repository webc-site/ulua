use ulua_common::functions::format_append::formatAppend;

use crate::{records::state_dot::StateDot, type_aliases::type_id::TypeId};

impl StateDot {
  pub fn finish_node_label_type_id(&mut self, ty: TypeId) {
    if self.opts.show_pointers {
      formatAppend(&mut self.result, format_args!("\n0x{:p}", ty));
    }
    self.result += "\"";
  }
}
