use ulua_ast::records::location::Location;

use crate::{
  records::non_strict_type_checker::NonStrictTypeChecker, type_aliases::type_id::TypeId,
};

impl NonStrictTypeChecker {
  pub fn check_for_type_function_inhabitance(
    &mut self,
    _instance: TypeId,
    _location: Location,
  ) -> TypeId {
    _instance
  }
}
