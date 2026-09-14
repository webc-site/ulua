//! Node: TypeChecker2 record
//! Source: `Analysis/include/Luau/TypeChecker2.h` (hand-ported; fields only)

use alloc::{string::String, vec::Vec};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::type_context::TypeContext,
  records::{
    builtin_types::BuiltinTypes, dcr_logger::DcrLogger,
    internal_error_reporter::InternalErrorReporter, module::Module, normalizer::Normalizer,
    scope::Scope, source_module::SourceModule, subtyping::Subtyping,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug)]
pub struct TypeChecker2 {
  pub builtin_types: *mut BuiltinTypes,
  pub type_function_runtime: *mut TypeFunctionRuntime,
  pub logger: *mut DcrLogger,
  pub limits: *mut TypeCheckLimits,
  pub ice: *mut InternalErrorReporter,
  pub source_module: *const SourceModule,
  pub module: *mut Module,
  pub type_context: TypeContext,

  pub stack: Vec<*mut Scope>,
  pub function_decl_stack: Vec<TypeId>,

  pub seen_type_function_instances: DenseHashSet<TypeId>,

  pub normalizer: Normalizer,
  pub _subtyping: Subtyping,
  pub subtyping: *mut Subtyping,

  pub warned_globals: DenseHashSet<String>,
}
