use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, is_prim::is_prim},
  records::{
    boolean_singleton::BooleanSingleton, primitive_type::PrimitiveType,
    singleton_type::SingletonType, union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariantMember, type_id::TypeId},
};

pub fn is_boolean(ty: TypeId) -> bool {
  if is_prim(ty, PrimitiveType::BOOLEAN) {
    return true;
  }

  if let Some(stv) = get_type_id::<SingletonType>(follow_type_id(ty))
    && BooleanSingleton::get_if(&stv.variant).is_some()
  {
    return true;
  }

  if let Some(utv) = get_type_id::<UnionType>(follow_type_id(ty)) {
    return utv.options.iter().all(|&opt| is_boolean(opt));
  }

  false
}
