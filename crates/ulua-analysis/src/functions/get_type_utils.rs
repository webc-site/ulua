use crate::{
  functions::{get_type, get_type_pack},
  type_aliases::{
    type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember,
    type_variant::TypeVariantMember,
  },
};

pub trait GetThroughId<Ty: Copy>: Sized {
  fn get_through(ty: Ty) -> Option<&'static Self>;
}

impl<T: TypeVariantMember + 'static> GetThroughId<TypeId> for T {
  fn get_through(ty: TypeId) -> Option<&'static T> {
    get_type::get::<T>(ty)
  }
}

impl<T: TypePackVariantMember + 'static> GetThroughId<TypePackId> for T {
  fn get_through(tp: TypePackId) -> Option<&'static T> {
    get_type_pack::get::<T>(tp)
  }
}

/// 消除 C 风格裸指针与 unsafe：返回安全引用 `Option<&'static T>`。
/// C++ `template<typename T, typename Ty> const T* get(std::optional<Ty> ty)`.
pub fn get_optional_ty<T: GetThroughId<Ty>, Ty: Copy>(ty: Option<Ty>) -> Option<&'static T> {
  ty.and_then(T::get_through)
}
