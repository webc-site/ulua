use core::ffi::{CStr, c_char};

use ulua_common::functions::format_append::formatAppend;

use crate::{records::state_dot::StateDot, type_aliases::type_pack_id::TypePackId};
impl StateDot {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_child_type_pack_id_i32_c_char(
    &mut self,
    tp: TypePackId,
    parent_index: i32,
    link_name: *const c_char,
  ) {
    if !self.tp_to_index.contains_key(&tp) {
      self.tp_to_index.try_insert(tp, self.next_index);
      self.next_index += 1;
    }

    let tp_index = *self.tp_to_index.find(&tp).unwrap();

    if parent_index != 0 {
      if !link_name.is_null() {
        let name = unsafe { CStr::from_ptr(link_name).to_string_lossy() };
        formatAppend(
          &mut self.result,
          format_args!("n{} -> n{} [label=\"{}\"];\n", parent_index, tp_index, name),
        );
      } else {
        formatAppend(
          &mut self.result,
          format_args!("n{} -> n{};\n", parent_index, tp_index),
        );
      }
    }

    self.visit_children_type_pack_id_i32(tp, tp_index);
  }
}
