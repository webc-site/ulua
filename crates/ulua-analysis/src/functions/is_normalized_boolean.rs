use crate::{
  functions::{get_singleton_type::get_singleton_type, get_type_alt_j::get_type_id},
  records::{
    boolean_singleton::BooleanSingleton, never_type::NeverType, primitive_type::PrimitiveType,
    singleton_type::SingletonType,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_normalized_boolean(ty: TypeId) -> bool {
  if get_type_id::<NeverType>(ty).is_none() {
    true
  } else if let Some(ptv) = get_type_id::<PrimitiveType>(ty) {
    ptv.r#type == PrimitiveType::BOOLEAN
  } else if let Some(stv) = get_type_id::<SingletonType>(ty) {
    get_singleton_type::<BooleanSingleton>(stv).is_some()
  } else {
    false
  }
}
