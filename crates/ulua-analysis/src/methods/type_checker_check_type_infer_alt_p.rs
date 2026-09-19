use ulua_ast::records::ast_stat_type_function::AstStatTypeFunction;

use crate::{
  enums::control_flow::ControlFlow,
  records::{generic_error::GenericError, type_checker::TypeChecker},
  type_aliases::{scope_ptr_type::ScopePtr, type_error_data::TypeErrorData},
};

impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_type_function(
    &mut self,
    _scope: &ScopePtr,
    typefunction: &AstStatTypeFunction,
  ) -> ControlFlow {
    self.report_error_location_type_error_data(
      &typefunction.base.base.location,
      TypeErrorData::GenericError(GenericError::new(
        "This syntax is not supported".to_string(),
      )),
    );

    ControlFlow::None
  }
}
