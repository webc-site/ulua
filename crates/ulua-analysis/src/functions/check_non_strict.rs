//! C++ free function `Luau::checkNonStrict(...)`
//! (`Analysis/src/NonStrictTypeChecker.cpp:1287-1320`).

use crate::{
  functions::{copy_errors::copy_errors, freeze::freeze, unfreeze::unfreeze},
  records::{
    builtin_types::BuiltinTypes, data_flow_graph::DataFlowGraph,
    internal_error_reporter::InternalErrorReporter, module::Module,
    non_strict_type_checker::NonStrictTypeChecker, source_module::SourceModule,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::type_error_data::TypeErrorData,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn check_non_strict(
  builtin_types: *mut BuiltinTypes,
  type_function_runtime: *mut TypeFunctionRuntime,
  ice: *mut InternalErrorReporter,
  unifier_state: *mut UnifierSharedState,
  dfg: *const DataFlowGraph,
  limits: *mut TypeCheckLimits,
  source_module: &SourceModule,
  module: *mut Module,
) {
  let mut type_checker = unsafe {
    NonStrictTypeChecker::new(
      &mut (*module).internal_types,
      builtin_types,
      type_function_runtime,
      ice,
      unifier_state,
      dfg,
      limits,
      module,
    )
  };
  unsafe { type_checker.wire_self_pointers() };

  type_checker.visit_ast_stat_block(source_module.root);

  unsafe {
    let module = &mut *module;
    unfreeze(&mut module.interface_types);
    copy_errors(
      &mut module.errors,
      &mut module.interface_types,
      &*builtin_types,
    );

    module
      .errors
      .retain(|err| !matches!(err.data, TypeErrorData::UnknownRequire(_)));

    freeze(&mut module.interface_types);
  }
}
