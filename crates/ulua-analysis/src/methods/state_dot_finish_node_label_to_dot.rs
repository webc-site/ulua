use ulua_common::functions::format_append::format_append;

use crate::{
  records::state_dot::StateDot,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl StateDot {
  pub fn finish_node_label_type_id(&mut self, ty: TypeId) {
    if self.opts.show_pointers {
      format_append(&mut self.result, format_args!("\n0x{:p}", ty));
    }
    self.result += "\"";
  }

  pub fn finish_node_label_type_pack_id(&mut self, tp: TypePackId) {
    if self.opts.show_pointers {
      format_append(&mut self.result, format_args!("\n0x{:p}", tp));
    }
    self.result += "\"";
  }
}
