use ulua_analysis::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::primitive_type::{PrimitiveType, Type},
  type_aliases::type_id::TypeId,
};

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn get_primitive_type(&mut self, ty: TypeId) -> Option<Type> {
    ulua_common::LUAU_ASSERT!(!ty.is_null());

    let a_type = follow_type_id(ty);
    ulua_common::LUAU_ASSERT!(!a_type.is_null());

    get_type_id::<PrimitiveType>(a_type).map(|pt| pt.r#type)
  }
}
