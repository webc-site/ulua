use ulua_ast::records::location::Location;

use crate::records::{
  builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
  normalizer::Normalizer, scope::Scope, subtyping::Subtyping, type_arena::TypeArena,
  type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
};
#[derive(Debug, Clone)]
pub struct OverloadResolver {
  pub(crate) builtin_types: *mut BuiltinTypes,
  pub(crate) arena: *mut TypeArena,
  pub(crate) normalizer: *mut Normalizer,
  pub(crate) type_function_runtime: *mut TypeFunctionRuntime,
  pub(crate) scope: *mut Scope,
  pub(crate) ice: *mut InternalErrorReporter,
  pub(crate) limits: TypeCheckLimits,
  pub(crate) subtyping: Subtyping,
  pub(crate) call_loc: Location,
}
