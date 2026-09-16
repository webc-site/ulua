use alloc::{string::String, vec::Vec};
use core::ptr::{null, null_mut};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::{solver_mode::SolverMode, type_context::TypeContext},
  records::{
    builtin_types::BuiltinTypes, dcr_logger::DcrLogger, module::Module, normalizer::Normalizer,
    source_module::SourceModule, subtyping::Subtyping, type_arena::TypeArena,
    type_check_limits::TypeCheckLimits, type_checker_2::TypeChecker2,
    type_function_runtime::TypeFunctionRuntime, unifier_shared_state::UnifierSharedState,
  },
};
impl TypeChecker2 {
  /// C++ `TypeChecker2::TypeChecker2(NotNull<BuiltinTypes>,
  /// NotNull<TypeFunctionRuntime>, NotNull<UnifierSharedState>,
  /// NotNull<TypeCheckLimits>, DcrLogger*, const SourceModule*, Module*)`
  /// (`Analysis/src/TypeChecker2.cpp:307`).
  ///
  /// Owned constructor. The C++ member-init list wires two self-referential
  /// pointers — `_subtyping`'s `NotNull<Normalizer>` points at the embedded
  /// `normalizer`, and `subtyping` points at the embedded `_subtyping`. Those
  /// cannot be set here because the returned value is moved into its final
  /// slot, so they are left null and wired by [`TypeChecker2::wire_self_pointers`]
  /// once the `TypeChecker2` lives at a stable address.
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn new(
    builtin_types: *mut BuiltinTypes,
    type_function_runtime: *mut TypeFunctionRuntime,
    unifier_state: *mut UnifierSharedState,
    limits: *mut TypeCheckLimits,
    logger: *mut DcrLogger,
    source_module: *const SourceModule,
    module: *mut Module,
  ) -> Self {
    // ice(unifierState->iceHandler)
    let ice = unsafe { (*unifier_state).ice_handler };

    // &module->internalTypes
    let arena: *mut TypeArena = unsafe { &mut (*module).internal_types };

    // normalizer{&module->internalTypes, builtinTypes, unifierState, SolverMode::New, /* cacheInhabitance */ true}
    let normalizer = Normalizer::new(arena, builtin_types, unifier_state, SolverMode::New, true);

    // _subtyping{builtinTypes, NotNull{&module->internalTypes}, NotNull{&normalizer},
    //            typeFunctionRuntime, NotNull{unifierState->iceHandler}}
    // The NotNull<Normalizer> is wired in `wire_self_pointers` (it must point
    // at the moved-in `normalizer` field).
    let _subtyping =
      Subtyping::subtyping_owned(builtin_types, arena, null_mut(), type_function_runtime, ice);

    TypeChecker2 {
      builtin_types,
      type_function_runtime,
      logger,
      limits,
      ice,
      source_module,
      module,
      type_context: TypeContext::default(),
      stack: Vec::new(),
      function_decl_stack: Vec::new(),
      seen_type_function_instances: DenseHashSet::new(null()),
      normalizer,
      _subtyping,
      // subtyping(&_subtyping) — wired in wire_self_pointers.
      subtyping: null_mut(),
      warned_globals: DenseHashSet::new(String::new()),
    }
  }

  /// Wires the two self-referential pointers the C++ member-init list sets:
  /// `_subtyping.normalizer = &normalizer` and `subtyping = &_subtyping`.
  /// Must be called after the `TypeChecker2` is at its final address (i.e.
  /// after the `new(..)` value has been moved into its storage slot) and
  /// before any use of `subtyping`.
  ///
  /// # Safety
  /// The `TypeChecker2` must not be moved after this call, or the wired
  /// pointers dangle.
  pub unsafe fn wire_self_pointers(&mut self) {
    let normalizer_ptr: *mut Normalizer = &mut self.normalizer;
    self._subtyping.normalizer = normalizer_ptr;
    self.subtyping = &mut self._subtyping as *mut Subtyping;
  }
}
