//! `type_iterator` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::{
  functions::follow_type::follow,
  records::{
    arena_handle::alias_ref,
    type_iterator::{TypeIterator, TypeIteratorMember},
  },
  type_aliases::type_id::TypeId,
};

// Source: `Analysis/include/Luau/Type.h:1209-1222` (hand-ported)

impl<T: TypeIteratorMember> TypeIterator<T> {
  pub(crate) fn advance_cursor(&mut self) {
    while !self.stack.empty() {
      let (t, current_index) = self.stack.front_mut();
      *current_index += 1;

      let types = alias_ref(*t).get_types();
      if *current_index >= types.len() {
        self.stack.pop_front();
      } else {
        break;
      }
    }
  }

  /// 获取当前指向的类型 ID。
  pub fn current(&mut self) -> TypeId {
    self.descend();
    let (t, current_index) = *self.stack.front();
    let types = alias_ref(t).get_types();
    follow(types[current_index])
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

// Source: `Analysis/include/Luau/Type.h:1224-1246` (hand-ported)

impl<T: TypeIteratorMember> TypeIterator<T> {
  pub(crate) fn descend(&mut self) {
    {
      while !self.stack.empty() {
        let (current, current_index) = *self.stack.front();
        let types = alias_ref(current).get_types();
        let inner_ref = T::get_if(&alias_ref(follow(types[current_index])).ty);
        if let Some(inner) = inner_ref {
          let inner = inner as *const T;
          // If we are about to descend into a cyclic type, we should skip over this.
          // Ideally this should never happen, but alas it does from time to time. :(
          if self.seen.contains(&inner) {
            self.advance();
          } else {
            self.seen.insert(inner);
            self.stack.push_front((inner, 0));
          }

          continue;
        }

        break;
      }
    }
  }
}

// Source: `Analysis/include/Luau/Type.h:1138-1148` (hand-ported)

impl<T: TypeIteratorMember> TypeIterator<T> {
  /// C++ `explicit TypeIterator(const T* t)`.
  /// 接收安全引用 `&T`，消除裸指针入参与外部 unsafe。
  pub fn type_iterator_type(t: &T) -> Self {
    let mut it = Self::type_iterator_default();

    let types = t.get_types();
    let ptr = t as *const T;
    if !types.is_empty() {
      it.stack.push_front((ptr, 0));
    }

    it.seen.insert(ptr);
    it.descend();

    it
  }
}
// Source: `Analysis/include/Luau/Type.h:1201` (hand-ported)
impl<T: TypeIteratorMember> TypeIterator<T> {
  /// C++ private `TypeIterator() = default;` — the `end()` sentinel.
  pub fn type_iterator_default() -> Self {
    Self {
      stack: VecDeque::new(),
      seen: DenseHashSet::default(),
    }
  }
}
