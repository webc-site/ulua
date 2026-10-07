//! `state_dot` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::functions::format_append::format_append;

use crate::{
  functions::get_type,
  records::{
    any_type::AnyType, never_type::NeverType, primitive_type::PrimitiveType, state_dot::StateDot,
    unknown_type::UnknownType,
  },
  type_aliases::{bound_type::BoundType, type_id::TypeId, type_pack_id::TypePackId},
};

impl StateDot {
  pub fn can_duplicate_primitive(&self, ty: TypeId) -> bool {
    let bound = get_type::get::<BoundType>(ty);
    if bound.is_some() {
      return false;
    }

    let primitive = get_type::get::<PrimitiveType>(ty);
    if primitive.is_some() {
      return true;
    }

    let any = get_type::get::<AnyType>(ty);
    if any.is_some() {
      return true;
    }

    let unknown = get_type::get::<UnknownType>(ty);
    if unknown.is_some() {
      return true;
    }

    let never = get_type::get::<NeverType>(ty);
    never.is_some()
  }
}

impl StateDot {
  pub fn finish_node(&mut self) {
    format_append(&mut self.result, format_args!("];\n"));
  }
}

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

impl StateDot {
  pub fn start_node(&mut self, index: i32) {
    format_append(&mut self.result, format_args!("n{} [", index));
  }
}

impl StateDot {
  pub fn start_node_label(&mut self) {
    format_append(&mut self.result, format_args!("label=\""));
  }
}
