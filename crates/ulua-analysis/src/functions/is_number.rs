use crate::{
  functions::is_prim::is_prim, records::primitive_type::PrimitiveType,
  type_aliases::type_id::TypeId,
};

pub fn is_number(ty: TypeId) -> bool {
  is_prim(ty, PrimitiveType::NUMBER)
}
