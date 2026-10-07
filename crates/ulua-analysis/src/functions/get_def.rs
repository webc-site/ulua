//! Source: `Analysis/include/Luau/Def.h:73-77` (hand-ported)
/// C++ `template<typename T> const T* get(DefId def)`.
use crate::{
  records::def_registry::def_as,
  type_aliases::{def_id_def::DefId, variant::VariantMember},
};

/// 句柄 → 变体成员安全只读引用下转。
/// 消除裸指针与 unsafe，未命中或空句柄返回 `None`。
pub fn get_def_id<'a, T: VariantMember>(def: DefId) -> Option<&'a T> {
  def_as::<T>(def)
}
