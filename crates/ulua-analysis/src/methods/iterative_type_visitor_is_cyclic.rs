use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::iterative_type_visitor::IterativeTypeVisitor;

impl IterativeTypeVisitor {
  pub fn iterative_type_visitor_is_cyclic(&mut self) -> bool {
    let current_item = &self.work_queue[self.work_cursor as usize];
    let mut cursor = self.work_cursor as i32;
    let mut parent = current_item.parent;

    while parent >= 0 {
      LUAU_ASSERT!(parent < cursor);
      cursor = parent;
      let ancestor_item = &self.work_queue[cursor as usize];

      if let Some(ty) = current_item.type_id() {
        if ancestor_item.operator_eq_type_id(ty) {
          return true;
        }
      } else if let Some(tp) = current_item.type_pack_id()
        && ancestor_item.operator_eq_type_pack_id(tp)
      {
        return true;
      }

      parent = ancestor_item.parent;
    }

    false
  }
}
