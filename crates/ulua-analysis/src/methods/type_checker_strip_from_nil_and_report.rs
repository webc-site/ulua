use ulua_ast::records::location::Location;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, is_nil::is_nil},
  records::{
    optional_value_access::OptionalValueAccess, type_checker::TypeChecker, union_type::UnionType,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeChecker {
  pub fn strip_from_nil_and_report(&mut self, ty: TypeId, location: &Location) -> TypeId {
    let ty = follow_type_id(ty);

    if let Some(utv) = get_type_id::<UnionType>(ty)
      && !utv.options.iter().any(|&t| is_nil(t))
    {
      return ty;
    }

    if let Some(stripped_union) = self.try_strip_union_from_nil(ty) {
      self.report_error_location_type_error_data(
        location,
        TypeErrorData::OptionalValueAccess(OptionalValueAccess { optional: ty }),
      );
      return follow_type_id(stripped_union);
    }

    ty
  }
}
