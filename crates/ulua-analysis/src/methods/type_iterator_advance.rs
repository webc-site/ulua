//! Source: `Analysis/include/Luau/Type.h:1209-1222` (hand-ported)

use crate::{
  functions::follow_type::follow,
  records::type_iterator::{TypeIterator, TypeIteratorMember},
  type_aliases::type_id::TypeId,
};

impl<T: TypeIteratorMember> TypeIterator<T> {
  pub(crate) fn advance_cursor(&mut self) {
    unsafe {
      while !self.stack.empty() {
        let (t, current_index) = self.stack.front_mut();
        *current_index += 1;

        let types = (**t).get_types();
        if *current_index >= types.len() {
          self.stack.pop_front();
        } else {
          break;
        }
      }
    }
  }

  /// 获取当前指向的类型 ID。
  pub fn current(&mut self) -> TypeId {
    self.descend();
    unsafe {
      let (t, current_index) = *self.stack.front();
      let types = (*t).get_types();
      follow(types[current_index])
    }
  }

  /// 向前推进迭代器游标（先前进当前位置，再降落至有效叶子类型）。
  pub fn advance(&mut self) {
    self.advance_cursor();
    self.descend();
  }

  /// 后置自增（相当于 C++ `operator++(int)`）。
  pub fn advance_and_get_prev(&mut self) -> Self {
    let copy = self.clone();
    self.advance();
    copy
  }
}
