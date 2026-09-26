//! `TypeChecker::tryStripUnionFromNil` 的核心遍历已单源化于
//! [`crate::functions::try_strip_union_from_nil`]（cpp TypeUtils.cpp 同名
//! static 函数），此处仅做 arena 接线。
use crate::{
  functions::{arc_as_mut::arc_as_mut, try_strip_union_from_nil::try_strip_union_from_nil},
  records::type_checker::TypeChecker,
  type_aliases::type_id::TypeId,
};

impl TypeChecker {
  pub fn try_strip_union_from_nil(&mut self, ty: TypeId) -> Option<TypeId> {
    // SAFETY: current_module 在类型检查期间独占（与 self.add_type 同一降级
    // 路径：arc_as_mut 短时重建 &mut，借用止于本次调用，对应 C++ 直接持有
    // module->internal_types）。
    unsafe {
      let module = arc_as_mut(self.expect_current_module());
      try_strip_union_from_nil(&mut (*module).internal_types, ty)
    }
  }
}
