//! C++ free function `Luau::check(NotNull<BuiltinTypes>,
//! NotNull<TypeFunctionRuntime>, NotNull<UnifierSharedState>,
//! NotNull<TypeCheckLimits>, DcrLogger*, const SourceModule&, Module*)`
//! (`Analysis/src/TypeChecker2.cpp:286-305`).
use crate::{
  functions::{copy_errors::copy_errors, freeze::freeze, unfreeze::unfreeze},
  records::{
    builtin_types::BuiltinTypes, dcr_logger::DcrLogger, module::Module,
    source_module::SourceModule, type_check_limits::TypeCheckLimits, type_checker_2::TypeChecker2,
    type_function_runtime::TypeFunctionRuntime, unifier_shared_state::UnifierSharedState,
  },
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn check(
  builtin_types: *mut BuiltinTypes,
  type_function_runtime: *mut TypeFunctionRuntime,
  unifier_state: *mut UnifierSharedState,
  limits: *mut TypeCheckLimits,
  logger: *mut DcrLogger,
  source_module: &SourceModule,
  module: *mut Module,
) {
  unsafe {
    // LUAU_TIMETRACE_SCOPE("check", "Typechecking");

    // TypeChecker2 typeChecker{builtinTypes, typeFunctionRuntime, unifierState, limits, logger, &sourceModule, module};
    let mut type_checker = TypeChecker2::new(
      builtin_types,
      type_function_runtime,
      unifier_state,
      limits,
      logger,
      source_module as *const SourceModule,
      module,
    );
    // The C++ member-init list wires `typeChecker`'s self-referential
    // `subtyping`/`_subtyping.normalizer` pointers in place; do that now that
    // `type_checker` lives at its final stack address (it is not moved below).
    type_checker.wire_self_pointers();

    // typeChecker.visit(sourceModule.root);
    type_checker.visit_ast_stat_block(source_module.root);

    // unfreeze(module->interfaceTypes);
    // copyErrors(module->errors, module->interfaceTypes, builtinTypes);
    // freeze(module->interfaceTypes);

    let m = &mut *module;
    unfreeze(&mut m.interface_types);
    copy_errors(&mut m.errors, &mut m.interface_types, &*builtin_types);
    freeze(&mut m.interface_types);
  }
}
