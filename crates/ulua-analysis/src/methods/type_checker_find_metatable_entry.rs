use ulua_ast::records::location::Location;

use crate::{
  functions::find_metatable_entry::find_metatable_entry,
  records::type_checker::TypeChecker,
  type_aliases::{error_vec::ErrorVec, type_id::TypeId},
};
impl TypeChecker {
  pub(crate) fn find_metatable_entry(
    &mut self,
    ty: TypeId,
    entry: &str,
    location: &Location,
    add_errors: bool,
  ) -> Option<TypeId> {
    let mut errors: ErrorVec = ErrorVec::new();
    let result = find_metatable_entry(self.builtin_types, &mut errors, ty, entry, *location);
    if add_errors {
      self.report_errors(&errors);
    }
    result
  }
}
