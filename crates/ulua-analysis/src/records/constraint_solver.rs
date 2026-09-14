//! Source: `Analysis/include/Luau/ConstraintSolver.h` (hand-ported; fields only)

use alloc::{boxed::Box, vec::Vec};
use core::ffi::c_void;
use std::collections::{HashMap, HashSet};

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    builtin_types::BuiltinTypes, constraint::Constraint, constraint_graph::ConstraintGraph,
    constraint_set::ConstraintSet, data_flow_graph::DataFlowGraph, dcr_logger::DcrLogger,
    hash_instantiation_signature::HashInstantiationSignature,
    hash_subtype_constraint_record::HashSubtypeConstraintRecord,
    instantiation_signature::InstantiationSignature,
    internal_error_reporter::InternalErrorReporter, module_resolver::ModuleResolver,
    normalizer::Normalizer, require_cycle::RequireCycle, scope::Scope,
    subtype_constraint_record::SubtypeConstraintRecord, subtyping::Subtyping,
    to_string_options::ToStringOptions, type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, type_ids::TypeIds,
  },
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, error_vec::ErrorVec, module_ptr_module::ModulePtr,
    type_id::TypeId,
  },
};
#[derive(Debug)]
pub struct ConstraintSolver {
  pub arena: *mut TypeArena,
  pub builtin_types: *mut BuiltinTypes,
  pub ice_reporter: InternalErrorReporter,
  pub normalizer: *mut Normalizer,
  pub type_function_runtime: *mut TypeFunctionRuntime,
  pub constraint_set: ConstraintSet,
  pub constraints: Vec<*mut Constraint>,
  pub scope_to_function: *mut DenseHashMap<*mut Scope, TypeId>,
  pub root_scope: *mut Scope,
  pub module: Option<ModulePtr>,
  pub dfg: *const DataFlowGraph,

  pub solver_constraints: Vec<Box<Constraint>>,
  pub solver_constraint_limit: usize,

  pub unsolved_constraints: Vec<*const Constraint>,

  pub deprecated_blocked_constraints: HashMap<*const Constraint, usize>,
  pub deprecated_blocked: HashMap<BlockedConstraintId, DenseHashSet<*const Constraint>>,

  pub instantiated_aliases:
    DenseHashMap<InstantiationSignature, TypeId, HashInstantiationSignature>,
  pub upper_bound_contributors: DenseHashMap<TypeId, Vec<(Location, TypeId)>>,

  pub deprecated_type_to_constraint_set: HashMap<TypeId, HashSet<*const Constraint>>,
  pub deprecated_constraint_to_mutated_types: DenseHashMap<*const Constraint, TypeIds>,

  pub uninhabited_type_functions: DenseHashSet<*const c_void>,
  pub seen_constraints:
    DenseHashMap<SubtypeConstraintRecord, *mut Constraint, HashSubtypeConstraintRecord>,

  pub generalized_types_: DenseHashSet<TypeId>,
  pub generalized_types: *const DenseHashSet<TypeId>,

  pub errors: ErrorVec,
  pub module_resolver: *mut ModuleResolver,
  pub require_cycles: Vec<RequireCycle>,
  pub logger: *mut DcrLogger,
  pub limits: TypeCheckLimits,
  pub type_functions_to_finalize: DenseHashMap<TypeId, *const Constraint>,

  pub opts: ToStringOptions,
  pub cgraph: *mut ConstraintGraph,
  pub subtyping: *mut Subtyping,
}
