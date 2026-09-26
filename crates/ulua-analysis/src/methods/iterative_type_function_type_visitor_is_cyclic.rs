use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
  },
};

/// C++ 模板 `IterativeTypeFunctionTypeVisitor::isCyclic<TID>`
/// （IterativeTypeFunctionTypeVisitor.cpp:345-362）：沿 `parent` 链回溯，
/// 逐项 `*item == ty` 比对。`WorkItem::t` 存的就是 id 裸指针，按指针值比较。
pub(crate) trait IdRaw {
  fn raw(self) -> *const ();
}
impl IdRaw for TypeFunctionTypeId {
  fn raw(self) -> *const () {
    self as *const ()
  }
}
impl IdRaw for TypeFunctionTypePackId {
  fn raw(self) -> *const () {
    self as *const ()
  }
}

impl IterativeTypeFunctionTypeVisitor {
  pub(crate) fn is_cyclic<TID>(&self, ty: TID) -> bool
  where
    TID: IdRaw,
  {
    let ty = ty.raw();
    let mut cursor = self.work_cursor as i32;
    let mut item = &self.work_queue[self.work_cursor as usize];

    while item.parent >= 0 {
      LUAU_ASSERT!(item.parent < cursor);
      cursor = item.parent;
      item = &self.work_queue[cursor as usize];

      // C++ `if (*item == ty)`：等价于 `item.asType()/asTypePack()` 分派后的
      // 指针比较；`t` 本身即 id 值，此处直接比对原始指针。
      if item.t == ty {
        return true;
      }
    }

    false
  }
}
