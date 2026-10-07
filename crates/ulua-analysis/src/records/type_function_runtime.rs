use alloc::{string::String, vec::Vec};

use ulua_ast::records::ast_stat_type_function::AstStatTypeFunction;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    internal_error_reporter::InternalErrorReporter, type_check_limits::TypeCheckLimits,
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
    type_function_type::TypeFunctionType, type_function_type_pack_var::TypeFunctionTypePackVar,
    typed_allocator::TypedAllocator,
  },
  type_aliases::{scope_ptr_type::ScopePtr, state_ref::StateRef},
};

// Non-copyable in C++ (owns TypedAllocator arenas) — Debug only.
#[derive(Debug)]
pub struct TypeFunctionRuntime {
  pub(crate) ice: InternalErrorReporter,
  pub(crate) limits: TypeCheckLimits,
  pub(crate) type_arena: TypedAllocator<TypeFunctionType>,
  pub(crate) type_pack_arena: TypedAllocator<TypeFunctionTypePackVar>,
  pub(crate) state: StateRef,
  pub(crate) initialized: DenseHashSet<*mut AstStatTypeFunction>,
  pub(crate) allow_evaluation: bool,
  pub(crate) root_scope: ScopePtr,
  pub(crate) messages: Vec<String>,
  pub(crate) runtime_builder: *mut TypeFunctionRuntimeBuilderState,
}
