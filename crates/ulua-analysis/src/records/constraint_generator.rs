//! Node: ConstraintGenerator record
//! Source: `Analysis/include/Luau/ConstraintGenerator.h` (hand-ported; fields only)

use alloc::{string::String, vec::Vec};
use core::fmt::{Debug, Formatter, Result};

use ulua_ast::records::{
  ast_expr::AstExpr, ast_local::AstLocal, ast_name::AstName, ast_stat_type_alias::AstStatTypeAlias,
  ast_stat_type_function::AstStatTypeFunction, location::Location,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::{polarity::Polarity, type_context::TypeContext},
  records::{
    builtin_types::BuiltinTypes, constraint_graph::ConstraintGraph, data_flow_graph::DataFlowGraph,
    dcr_logger::DcrLogger, inference::Inference, interior_free_types::InteriorFreeTypes,
    internal_error_reporter::InternalErrorReporter, module_resolver::ModuleResolver,
    normalizer::Normalizer, refinement_arena_refinement::RefinementArena,
    require_cycle::RequireCycle, scope::Scope, set::Set, symbol::Symbol, type_arena::TypeArena,
    type_error::TypeError, type_function_runtime::TypeFunctionRuntime, type_ids::TypeIds,
  },
  type_aliases::{
    constraint_ptr::ConstraintPtr, frontend_callbacks::ModuleScopeCallback,
    module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr, type_id::TypeId,
  },
};
#[derive(Debug)]
pub struct InferredBinding {
  pub scope: *mut Scope,
  pub location: Location,
  pub types: TypeIds,
}

pub struct ConstraintGenerator {
  pub scopes: Vec<(Location, ScopePtr)>,
  pub module: Option<ModulePtr>,
  pub builtin_types: *mut BuiltinTypes,
  pub arena: *mut TypeArena,
  pub root_scope: *mut Scope,
  pub type_context: TypeContext,
  pub inferred_bindings: DenseHashMap<Symbol, InferredBinding>,
  pub constraints: Vec<ConstraintPtr>,
  pub free_types: TypeIds,
  pub scope_to_function: DenseHashMap<*mut Scope, TypeId>,
  pub ast_type_alias_defining_scopes: DenseHashMap<*const AstStatTypeAlias, Option<ScopePtr>>,
  pub dfg: *const DataFlowGraph,
  pub refinement_arena: RefinementArena,
  pub recursion_count: i32,
  pub errors: Vec<TypeError>,
  pub normalizer: *mut Normalizer,
  pub type_function_runtime: *mut TypeFunctionRuntime,
  pub ast_type_function_environment_scopes:
    DenseHashMap<*const AstStatTypeFunction, Option<ScopePtr>>,
  pub module_resolver: *mut ModuleResolver,
  pub ice: *mut InternalErrorReporter,
  pub global_scope: Option<ScopePtr>,
  pub type_function_scope: Option<ScopePtr>,
  pub prepare_module_scope: ModuleScopeCallback,
  pub require_cycles: Vec<RequireCycle>,
  pub local_types: DenseHashMap<TypeId, TypeIds>,
  pub inferred_expr_cache: DenseHashMap<*mut AstExpr, Inference>,
  pub class_decl_records: DenseHashMap<*mut AstLocal, ClassDeclRecord>,
  pub logger: *mut DcrLogger,
  pub recursion_limit_met: bool,
  pub cgraph: *mut ConstraintGraph,
  pub interior_free_types: Vec<InteriorFreeTypes>,
  pub unions_to_simplify: Vec<TypeId>,
  pub uninitialized_globals: Set<AstName>,
  pub polarity: Polarity,
  pub prop_index_pairs_seen: DenseHashMap<(TypeId, String), TypeId>,
  pub large_table_depth: usize,
}

use crate::records::class_decl_record::ClassDeclRecord;

impl Debug for ConstraintGenerator {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("ConstraintGenerator")
      .field("scopes", &self.scopes)
      .field("module", &self.module)
      .field("builtin_types", &self.builtin_types)
      .field("arena", &self.arena)
      .field("root_scope", &self.root_scope)
      .field("type_context", &self.type_context)
      .field("inferred_bindings", &self.inferred_bindings)
      .field("constraints", &self.constraints)
      .field("free_types", &self.free_types)
      .field("scope_to_function", &self.scope_to_function)
      .field(
        "ast_type_alias_defining_scopes",
        &self.ast_type_alias_defining_scopes,
      )
      .field("dfg", &self.dfg)
      .field("refinement_arena", &self.refinement_arena)
      .field("recursion_count", &self.recursion_count)
      .field("errors", &self.errors)
      .field("normalizer", &self.normalizer)
      .field("type_function_runtime", &self.type_function_runtime)
      .field(
        "ast_type_function_environment_scopes",
        &self.ast_type_function_environment_scopes,
      )
      .field("module_resolver", &self.module_resolver)
      .field("ice", &self.ice)
      .field("global_scope", &self.global_scope)
      .field("type_function_scope", &self.type_function_scope)
      .field("prepare_module_scope", &"...")
      .field("require_cycles", &self.require_cycles)
      .field("local_types", &self.local_types)
      .field("inferred_expr_cache", &self.inferred_expr_cache)
      .field("class_decl_records", &self.class_decl_records)
      .field("logger", &self.logger)
      .field("recursion_limit_met", &self.recursion_limit_met)
      .field("cgraph", &self.cgraph)
      .field("interior_free_types", &self.interior_free_types)
      .field("unions_to_simplify", &self.unions_to_simplify)
      .field("uninitialized_globals", &self.uninitialized_globals)
      .field("polarity", &self.polarity)
      .field("prop_index_pairs_seen", &self.prop_index_pairs_seen)
      .field("large_table_depth", &self.large_table_depth)
      .finish()
  }
}
