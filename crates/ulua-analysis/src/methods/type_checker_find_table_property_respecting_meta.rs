use ulua_ast::records::location::Location;

use crate::{
  enums::value_context::ValueContext,
  functions::find_table_property_respecting_meta_type_utils::find_table_property_respecting_meta,
  records::type_checker::TypeChecker,
  type_aliases::{error_vec::ErrorVec, name_type::Name, type_id::TypeId},
};

impl TypeChecker {
  pub fn find_table_property_respecting_meta(
    &mut self,
    lhs_type: TypeId,
    name: Name,
    location: &Location,
    add_errors: bool,
  ) -> Option<TypeId> {
    let mut errors: ErrorVec = ErrorVec::new();
    let result = find_table_property_respecting_meta(
      self.builtin_types,
      &mut errors,
      lhs_type,
      name.as_str(),
      ValueContext::RValue,
      *location,
      false,
    );
    if add_errors {
      self.report_errors(&errors);
    }
    result
  }
}
