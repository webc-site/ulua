//! `TypeChecker2::tryStripUnionFromNil`（TypeChecker2.cpp:2087-2106）。
use alloc::vec::Vec;

use crate::{
  functions::{get_type_alt_j::get_type_id, is_nil::is_nil},
  records::{r#type::Type, type_checker_2::TypeChecker2, union_type::UnionType},
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
impl TypeChecker2 {
  pub fn try_strip_union_from_nil(&self, ty: TypeId) -> Option<TypeId> {
    let utv = get_type_id::<UnionType>(ty)?;

    // if (!std::any_of(begin(utv), end(utv), isNil)) return ty;
    if !utv.options.iter().any(|&option| is_nil(option)) {
      return Some(ty);
    }

    let result: Vec<TypeId> = utv
      .options
      .iter()
      .copied()
      .filter(|&option| !is_nil(option))
      .collect();

    if result.is_empty() {
      return None;
    }

    if let [only] = result.as_slice() {
      return Some(*only);
    }

    // SAFETY: self.module 裸指针与类型检查会话同寿（C++ 同契约）；&self 下
    // 经裸指针 place 取 &mut internal_types，与原 C++ const 方法写 arena 等价。
    Some(unsafe {
      (*self.module)
        .internal_types
        .add_type(Type::new(TypeVariant::Union(UnionType { options: result })))
    })
  }
}
