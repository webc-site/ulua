use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::primitive_type::{PrimitiveType, Type},
  type_aliases::type_id::TypeId,
};

pub fn is_prim(ty: TypeId, prim_type: Type) -> bool {
  let followed = follow_type_id(ty);
  get_type_id::<PrimitiveType>(followed).is_some_and(|p| p.r#type == prim_type)
}
