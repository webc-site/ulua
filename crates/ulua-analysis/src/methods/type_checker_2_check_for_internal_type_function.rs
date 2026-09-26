use ulua_ast::records::location::Location;

use crate::{
  records::{
    generic_type_visitor::GenericTypeVisitorTrait,
    internal_type_function_finder::InternalTypeFunctionFinder,
    pack_where_clause_needed::PackWhereClauseNeeded, type_checker_2::TypeChecker2,
    where_clause_needed::WhereClauseNeeded,
  },
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  pub fn check_for_internal_type_function(&mut self, ty: TypeId, location: Location) {
    let mut finder = InternalTypeFunctionFinder::new(&mut self.function_decl_stack);
    finder.traverse_type_id(ty);

    for internal in finder.internal_functions.iter() {
      if self.should_suppress_uninhabited_type_function_error(*internal) {
        continue;
      }

      self.report_error_type_error_data_location(
        WhereClauseNeeded { ty: *internal }.into(),
        &location,
      );
    }

    for internal in finder.internal_pack_functions.iter() {
      self.report_error_type_error_data_location(
        PackWhereClauseNeeded { tp: *internal }.into(),
        &location,
      );
    }
  }
}
