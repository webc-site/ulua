//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/TypeInfer.h:62:type_checker`
//! Source: `Analysis/include/Luau/TypeInfer.h` (hand-ported; fields only)

use alloc::{sync::Arc, vec::Vec};
use core::fmt::{Debug, Formatter, Result};

use ulua_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    builtin_types::BuiltinTypes, frontend_cancellation_token::FrontendCancellationToken,
    hash_bool_name_pair::HashBoolNamePair, instantiation::Instantiation,
    internal_error_reporter::InternalErrorReporter, module_resolver::ModuleResolver,
    normalizer::Normalizer, require_cycle::RequireCycle, unifier_shared_state::UnifierSharedState,
  },
  type_aliases::{
    frontend_callbacks::ModuleScopeCallback, module_ptr_module::ModulePtr, name_type::Name,
    scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId,
  },
};
pub struct TypeChecker {
  pub global_scope: *const ScopePtr, // const ScopePtr&
  pub resolver: *mut ModuleResolver,
  pub current_module: Option<ModulePtr>,
  pub builtin_types: *mut BuiltinTypes, // NotNull<BuiltinTypes>
  pub ice_handler: *mut InternalErrorReporter,
  pub unifier_state: UnifierSharedState,
  pub normalizer: Normalizer,
  pub reusable_instantiation: Instantiation,
  pub require_cycles: Vec<RequireCycle>,
  pub finish_time: Option<f64>,
  pub instantiation_child_limit: Option<i32>,
  pub unifier_iteration_limit: Option<i32>,
  pub cancellation_token: Option<Arc<FrontendCancellationToken>>,
  pub prepare_module_scope: Option<ModuleScopeCallback>,

  pub nil_type: TypeId,
  pub number_type: TypeId,
  pub integer_type: TypeId,
  pub string_type: TypeId,
  pub boolean_type: TypeId,
  pub thread_type: TypeId,
  pub buffer_type: TypeId,
  pub any_type: TypeId,
  pub unknown_type: TypeId,
  pub never_type: TypeId,

  pub any_type_pack: TypePackId,
  pub never_type_pack: TypePackId,
  pub uninhabitable_type_pack: TypePackId,

  pub check_recursion_count: i32,
  pub recursion_count: i32,

  pub duplicate_type_aliases: DenseHashSet<(bool, Name), HashBoolNamePair>,
  pub incorrect_extern_type_definitions: DenseHashSet<*const AstStatDeclareExternType>,
  pub deferred_quantification: Vec<(TypeId, ScopePtr)>,
}

impl Debug for TypeChecker {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("TypeChecker")
      .field("global_scope", &self.global_scope)
      .field("resolver", &self.resolver)
      .field("current_module", &self.current_module)
      .field("builtin_types", &self.builtin_types)
      .field("ice_handler", &self.ice_handler)
      .field("require_cycles", &self.require_cycles)
      .field("finish_time", &self.finish_time)
      .field("instantiation_child_limit", &self.instantiation_child_limit)
      .field("unifier_iteration_limit", &self.unifier_iteration_limit)
      .field("cancellation_token", &self.cancellation_token)
      .field(
        "prepare_module_scope",
        &self.prepare_module_scope.as_ref().map(|_| "..."),
      )
      .field("check_recursion_count", &self.check_recursion_count)
      .field("recursion_count", &self.recursion_count)
      .finish()
  }
}
