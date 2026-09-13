use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{never_type::NeverType, primitive_type::PrimitiveType},
  type_aliases::type_id::TypeId,
};

pub fn is_normalized_thread(ty: TypeId) -> bool {
  if get_type_id::<NeverType>(ty).is_none() {
    true
  } else if let Some(ptv) = get_type_id::<PrimitiveType>(ty).as_ref() {
    ptv.r#type == PrimitiveType::THREAD
  } else {
    false
  }
}
