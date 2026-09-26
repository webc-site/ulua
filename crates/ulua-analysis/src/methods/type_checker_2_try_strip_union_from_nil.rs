//! `TypeChecker2::tryStripUnionFromNil`（TypeChecker2.cpp:2087-2106）的核心
//! 遍历已单源化于 [`crate::functions::try_strip_union_from_nil`]（cpp
//! TypeUtils.cpp 同名 static 函数），此处仅做 arena 接线。
use crate::{
  functions::try_strip_union_from_nil::try_strip_union_from_nil,
  records::type_checker_2::TypeChecker2, type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  pub fn try_strip_union_from_nil(&self, ty: TypeId) -> Option<TypeId> {
    // SAFETY: self.module 裸指针与类型检查会话同寿（C++ 同契约）；&self 下
    // 经裸指针 place 取 &mut internal_types，与原 C++ const 方法写 arena 等价。
    unsafe { try_strip_union_from_nil(&mut (*self.module).internal_types, ty) }
  }
}
