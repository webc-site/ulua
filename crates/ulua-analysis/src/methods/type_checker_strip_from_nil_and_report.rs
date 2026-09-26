use ulua_ast::records::location::Location;

use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type, is_prim::is_nil},
  records::{
    optional_value_access::OptionalValueAccess, type_checker::TypeChecker, union_type::UnionType,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeChecker {
  pub fn strip_from_nil_and_report(&mut self, ty: TypeId, location: &Location) -> TypeId {
    let ty = follow_type::follow(ty);

    // C++ `std::any_of(begin(utv), end(utv), isNil)` — UnionTypeIterator 防环展平并 follow。
    if let Some(utv) = get_type::get::<UnionType>(ty)
      && !begin_union_type(utv).any(is_nil)
    {
      return ty;
    }

    if let Some(stripped_union) = self.try_strip_union_from_nil(ty) {
      self.report_error_location_type_error_data(
        location,
        TypeErrorData::OptionalValueAccess(OptionalValueAccess { optional: ty }),
      );
      return follow_type::follow(stripped_union);
    }

    ty
  }
}
