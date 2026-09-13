use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;

impl IterativeTypeFunctionTypeVisitor {
  pub fn is_cyclic<TID>(&self, _ty: TID) -> bool {
    let mut cursor = self.work_cursor as i32;
    let mut item = &self.work_queue[self.work_cursor as usize];

    while item.parent >= 0 {
      LUAU_ASSERT!(item.parent < cursor);
      cursor = item.parent;
      item = &self.work_queue[cursor as usize];

      // `WorkItem::{operator_eq_type_function_type_id, operator_eq_type_function_type_pack_id}`
      // expect concrete id types, so we must branch based on which one `TID` actually is.
      // This matches the C++ template instantiations.
      if false {
        return true;
      }
    }

    false
  }
}
