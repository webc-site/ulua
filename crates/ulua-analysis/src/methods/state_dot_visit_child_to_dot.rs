use core::ffi::{CStr, c_char};

use ulua_common::functions::format_append::formatAppend;

use crate::{
  functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
  records::{
    any_type::AnyType, never_type::NeverType, primitive_type::PrimitiveType, state_dot::StateDot,
    unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};
impl StateDot {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_child_type_id_i32_c_char(
    &mut self,
    ty: TypeId,
    parent_index: i32,
    link_name: *const c_char,
  ) {
    if !self.ty_to_index.contains_key(&ty)
      || (self.opts.duplicate_primitives && self.can_duplicate_primitive(ty))
    {
      *self.ty_to_index.get_or_insert(ty) = self.next_index;
      self.next_index += 1;
    }

    let index = *self.ty_to_index.find(&ty).unwrap();

    if parent_index != 0 {
      if !link_name.is_null() {
        let name = unsafe { CStr::from_ptr(link_name).to_string_lossy() };
        formatAppend(
          &mut self.result,
          format_args!("n{} -> n{} [label=\"{}\"];\n", parent_index, index, name),
        );
      } else {
        formatAppend(
          &mut self.result,
          format_args!("n{} -> n{};\n", parent_index, index),
        );
      }
    }

    if self.opts.duplicate_primitives && self.can_duplicate_primitive(ty) {
      if !get_type_id::<PrimitiveType>(ty).is_none() {
        let s = to_string_type_id(ty);
        formatAppend(
          &mut self.result,
          format_args!("n{} [label=\"{}\"];\n", index, s),
        );
      } else if !get_type_id::<AnyType>(ty).is_none() {
        formatAppend(
          &mut self.result,
          format_args!("n{} [label=\"any\"];\n", index),
        );
      } else if !get_type_id::<UnknownType>(ty).is_none() {
        formatAppend(
          &mut self.result,
          format_args!("n{} [label=\"unknown\"];\n", index),
        );
      } else if !get_type_id::<NeverType>(ty).is_none() {
        formatAppend(
          &mut self.result,
          format_args!("n{} [label=\"never\"];\n", index),
        );
      }
    } else {
      self.visit_children_type_id_i32(ty, index);
    }
  }
}
