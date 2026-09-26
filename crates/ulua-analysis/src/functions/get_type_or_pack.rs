use crate::{
  functions::{get_type::get, get_type_pack::get as get_type_pack},
  type_aliases::{
    type_id::TypeId,
    type_or_pack::{TypeOrPack, TypeOrPackMember},
    type_pack_id::TypePackId,
    type_pack_variant::TypePackVariantMember,
    type_variant::TypeVariantMember,
  },
};

/// `TypeOrPack` 支内载荷只读探针（C++ `get_if<T>(&typeOrPack)`）。
/// 未命中返回 `None`（原 `*const T` 的 null 哨兵即其像）。
pub fn get_type_or_pack_mut<T: TypeOrPackMember>(ty_or_tp: &TypeOrPack) -> Option<&T> {
  T::get_if(ty_or_tp)
}

/// 类型支的 arena 变体载荷只读探针（C++ `get_if<T>(get_if<TypeId>(&typeOrPack))`）。
/// arena 节点有效性契约收口在 [`get`]，故返回 `&'static T`。
pub fn get_type_or_pack<T: TypeVariantMember + 'static>(
  ty_or_tp: &TypeOrPack,
) -> Option<&'static T> {
  TypeId::get_if(ty_or_tp).and_then(|ty| get::<T>(*ty))
}

/// pack 支的 arena 变体载荷只读探针，语义同 [`get_type_or_pack`]。
pub fn get_type_or_pack_mut_2<T: TypePackVariantMember + 'static>(
  ty_or_tp: &TypeOrPack,
) -> Option<&'static T> {
  TypePackId::get_if(ty_or_tp).and_then(|tp| get_type_pack::<T>(*tp))
}
