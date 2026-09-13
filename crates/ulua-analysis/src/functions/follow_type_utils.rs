//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeUtils.h:248:follow`
//! Source: `Analysis/include/Luau/TypeUtils.h:247-254` (hand-ported)

use crate::{
  functions::{follow_type::follow_type_id, follow_type_pack::follow_type_pack_id},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// Dispatch of the inner `follow(*ty)` per id type `Ty`: C++ resolves the
/// overload by argument type; Rust needs the trait.
pub trait FollowId: Copy {
  fn follow_id(self) -> Self;
}

impl FollowId for TypeId {
  fn follow_id(self) -> TypeId {
    follow_type_id(self)
  }
}

impl FollowId for TypePackId {
  fn follow_id(self) -> TypePackId {
    // 中转局部变量再调用，解引用由 follow_type_pack_id 内部 unsafe 承担
    let tp = self;
    unsafe { follow_type_pack_id(tp) }
  }
}

/// C++ `template<typename Ty> std::optional<Ty> follow(std::optional<Ty> ty)`.
pub fn follow_optional_ty<Ty: FollowId>(ty: Option<Ty>) -> Option<Ty> {
  ty.map(|ty| ty.follow_id())
}
