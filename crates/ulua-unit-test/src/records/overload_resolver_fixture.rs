use alloc::{boxed::Box, vec::Vec};

use ulua_analysis::{
  records::{
    builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
    normalizer::Normalizer, overload_resolver::OverloadResolver, scope::Scope,
    type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, unifier_shared_state::UnifierSharedState,
  },
  type_aliases::type_id::TypeId,
};
use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_constant_nil::AstExprConstantNil, location::Location,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::fixture::Fixture;
#[derive(Debug)]
#[repr(C)]
pub struct OverloadResolverFixture {
  pub arena_: Box<TypeArena>,
  pub arena: *mut TypeArena,
  pub builtin_types: Box<BuiltinTypes>,
  pub shared_state: Box<UnifierSharedState>,
  pub normalizer: Box<Normalizer>,
  pub ice_reporter: Box<InternalErrorReporter>,
  pub limits: Box<TypeCheckLimits>,
  pub type_function_runtime: Box<TypeFunctionRuntime>,
  pub root_scope: Box<Scope>,
  pub call_location: Location,
  pub resolver: OverloadResolver,
  pub k_empty_set: Box<DenseHashSet<TypeId>>,
  pub empty_set: *mut DenseHashSet<TypeId>,
  pub k_dummy_location: Location,
  pub k_dummy_expr: AstExprConstantNil,
  pub k_empty_exprs: Vec<*mut AstExpr>,
  pub number_to_number: TypeId,
  pub number_number_to_number: TypeId,
  pub number_to_string: TypeId,
  pub string_to_string: TypeId,
  pub number_to_number_and_string_to_string: TypeId,
  pub number_to_number_and_number_number_to_number: TypeId,
  pub base: Box<Fixture>,
}
