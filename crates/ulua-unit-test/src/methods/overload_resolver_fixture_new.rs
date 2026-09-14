use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::ptr::null;

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  records::{
    builtin_types::BuiltinTypes, function_type::FunctionType,
    internal_error_reporter::InternalErrorReporter, intersection_type::IntersectionType,
    normalizer::Normalizer, overload_resolver::OverloadResolver, scope::Scope, r#type::Type,
    type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, unifier_shared_state::UnifierSharedState,
  },
  type_aliases::type_id::TypeId,
};
use ulua_ast::records::{ast_expr_constant_nil::AstExprConstantNil, location::Location};
use ulua_common::{FFlag, records::dense_hash_set::DenseHashSet};

use crate::records::{fixture::Fixture, overload_resolver_fixture::OverloadResolverFixture};
impl OverloadResolverFixture {
  pub fn new() -> Self {
    let mut base = Box::new(Fixture::default());
    let mut arena_ = Box::new(TypeArena::default());
    let arena = arena_.as_mut() as *mut TypeArena;

    let mut builtin_types = Box::new(BuiltinTypes::new());
    let builtin_types_ptr = builtin_types.as_mut() as *mut BuiltinTypes;
    base.builtin_types = builtin_types_ptr;

    let mut ice_reporter = Box::new(InternalErrorReporter::default());
    let ice_reporter_ptr = ice_reporter.as_mut() as *mut InternalErrorReporter;

    let mut limits = Box::new(TypeCheckLimits::default());
    let limits_ptr = limits.as_mut() as *mut TypeCheckLimits;

    let base_ice_ptr = &mut base.ice as *mut InternalErrorReporter;
    let mut shared_state = Box::new(UnifierSharedState::new(base_ice_ptr));
    let shared_state_ptr = shared_state.as_mut() as *mut UnifierSharedState;

    let solver_mode = if !FFlag::DebugLuauForceOldSolver.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    };
    let mut normalizer = Box::new(Normalizer::new(
      arena,
      builtin_types_ptr,
      shared_state_ptr,
      solver_mode,
      false,
    ));
    let normalizer_ptr = normalizer.as_mut() as *mut Normalizer;

    let mut root_scope = Box::new(Scope::scope_type_pack_id(builtin_types.empty_type_pack));
    let root_scope_ptr = root_scope.as_mut() as *mut Scope;

    let runtime_root_scope = Arc::new(Scope::scope_type_pack_id(builtin_types.empty_type_pack));
    let mut type_function_runtime = Box::new(TypeFunctionRuntime::new(
      ice_reporter.as_ref(),
      limits.as_ref(),
      runtime_root_scope,
    ));
    let type_function_runtime_ptr = type_function_runtime.as_mut() as *mut TypeFunctionRuntime;

    let call_location = Location::default();
    let resolver = unsafe {
      OverloadResolver::new(
        builtin_types_ptr,
        arena,
        normalizer_ptr,
        type_function_runtime_ptr,
        root_scope_ptr,
        ice_reporter_ptr,
        limits_ptr,
        call_location,
      )
    };

    let mut k_empty_set = Box::new(DenseHashSet::new(null::<Type>() as TypeId));
    let empty_set = k_empty_set.as_mut() as *mut DenseHashSet<TypeId>;

    let k_dummy_location = Location::default();
    let k_dummy_expr = AstExprConstantNil::new(k_dummy_location);

    let number_type = builtin_types.number_type;
    let string_type = builtin_types.string_type;

    let number_to_number = add_function_type(arena, &[number_type], &[number_type]);
    let number_number_to_number =
      add_function_type(arena, &[number_type, number_type], &[number_type]);
    let number_to_string = add_function_type(arena, &[number_type], &[string_type]);
    let string_to_string = add_function_type(arena, &[string_type], &[string_type]);

    let number_to_number_and_string_to_string = unsafe {
      (*arena).add_type(IntersectionType {
        parts: alloc::vec![number_to_number, string_to_string],
      })
    };
    let number_to_number_and_number_number_to_number = unsafe {
      (*arena).add_type(IntersectionType {
        parts: alloc::vec![number_to_number, number_number_to_number],
      })
    };

    Self {
      arena_,
      arena,
      builtin_types,
      shared_state,
      normalizer,
      ice_reporter,
      limits,
      type_function_runtime,
      root_scope,
      call_location,
      resolver,
      k_empty_set,
      empty_set,
      k_dummy_location,
      k_dummy_expr,
      k_empty_exprs: Vec::new(),
      number_to_number,
      number_number_to_number,
      number_to_string,
      string_to_string,
      number_to_number_and_string_to_string,
      number_to_number_and_number_number_to_number,
      base,
    }
  }
}

impl Default for OverloadResolverFixture {
  fn default() -> Self {
    Self::new()
  }
}

fn add_function_type(arena: *mut TypeArena, args: &[TypeId], rets: &[TypeId]) -> TypeId {
  unsafe {
    let arg_pack = (*arena).add_type_pack_initializer_list_type_id(args);
    let ret_pack = (*arena).add_type_pack_initializer_list_type_id(rets);
    (*arena).add_type(FunctionType::function_type_new(
      arg_pack, ret_pack, None, false,
    ))
  }
}
