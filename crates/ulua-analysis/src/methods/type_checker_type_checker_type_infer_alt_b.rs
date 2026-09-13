use alloc::{string::String, vec::Vec};
use core::ptr::{null, null_mut};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    builtin_types::BuiltinTypes, instantiation::Instantiation,
    internal_error_reporter::InternalErrorReporter, module_resolver::ModuleResolver,
    normalizer::Normalizer, txn_log::TxnLog, type_checker::TypeChecker, type_level::TypeLevel,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl TypeChecker {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn new(
    global_scope: &ScopePtr,
    resolver: *mut ModuleResolver,
    builtin_types: *mut BuiltinTypes,
    ice_handler: *mut InternalErrorReporter,
  ) -> Self {
    let unifier_state = UnifierSharedState::new(ice_handler);

    let mut result = TypeChecker {
      global_scope: global_scope as *const ScopePtr,
      resolver,
      current_module: None,
      builtin_types,
      ice_handler,
      unifier_state,
      normalizer: Normalizer::new(
        null_mut(),
        builtin_types,
        null_mut(),
        SolverMode::Old,
        false,
      ),
      reusable_instantiation: Instantiation::instantiation_new(
        TxnLog::empty(),
        null_mut(),
        builtin_types,
        TypeLevel::default(),
        null_mut(),
      ),
      require_cycles: Vec::new(),
      finish_time: None,
      instantiation_child_limit: None,
      unifier_iteration_limit: None,
      cancellation_token: None,
      prepare_module_scope: None,
      nil_type: unsafe { (*builtin_types).nil_type },
      number_type: unsafe { (*builtin_types).number_type },
      integer_type: unsafe { (*builtin_types).integer_type },
      string_type: unsafe { (*builtin_types).string_type },
      boolean_type: unsafe { (*builtin_types).boolean_type },
      thread_type: unsafe { (*builtin_types).thread_type },
      buffer_type: unsafe { (*builtin_types).buffer_type },
      any_type: unsafe { (*builtin_types).any_type },
      unknown_type: unsafe { (*builtin_types).unknown_type },
      never_type: unsafe { (*builtin_types).never_type },
      any_type_pack: unsafe { (*builtin_types).any_type_pack },
      never_type_pack: unsafe { (*builtin_types).never_type_pack },
      uninhabitable_type_pack: unsafe { (*builtin_types).uninhabitable_type_pack },
      check_recursion_count: 0,
      recursion_count: 0,
      duplicate_type_aliases: DenseHashSet::new((false, String::new())),
      incorrect_extern_type_definitions: DenseHashSet::new(null()),
      deferred_quantification: Vec::new(),
    };

    result.normalizer.shared_state = &mut result.unifier_state;
    result
  }
}
