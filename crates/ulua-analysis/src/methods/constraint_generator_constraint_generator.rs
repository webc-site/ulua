use alloc::{string::String, vec::Vec};
use core::ptr::{NonNull, null, null_mut};

use ulua_ast::records::ast_name::AstName;
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::{polarity::Polarity, type_context::TypeContext},
  records::{
    builtin_types::BuiltinTypes, constraint_generator::ConstraintGenerator,
    constraint_graph::ConstraintGraph, data_flow_graph::DataFlowGraph, dcr_logger::DcrLogger,
    internal_error_reporter::InternalErrorReporter, module_resolver::ModuleResolver,
    normalizer::Normalizer, refinement_arena_refinement::RefinementArena,
    require_cycle::RequireCycle, set::Set, symbol::Symbol,
    type_function_runtime::TypeFunctionRuntime, type_ids::TypeIds, typed_allocator::TypedAllocator,
  },
  type_aliases::{
    frontend_callbacks::ModuleScopeCallback, module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr,
  },
};
/// `ConstraintGenerator` 构造参数包，对应 C++ 十三参数构造器
/// （ConstraintGenerator.h:164-179）。按语义分组：目标模块 / 基础设施 /
/// 作用域 / 回调与日志 / 依赖。
pub struct ConstraintGeneratorArgs<'a> {
  // —— 目标模块 ——
  pub module: ModulePtr,
  // —— 基础设施（NotNull 语义）——
  pub normalizer: NonNull<Normalizer>,
  pub type_function_runtime: NonNull<TypeFunctionRuntime>,
  pub module_resolver: NonNull<ModuleResolver>,
  pub builtin_types: NonNull<BuiltinTypes>,
  pub ice: NonNull<InternalErrorReporter>,
  // —— 作用域 ——
  pub global_scope: ScopePtr,
  pub type_function_scope: ScopePtr,
  // —— 回调与日志 ——
  pub prepare_module_scope: ModuleScopeCallback,
  pub logger: *mut DcrLogger,
  // —— 依赖 ——
  pub dfg: NonNull<DataFlowGraph>,
  pub require_cycles: &'a [RequireCycle],
  pub cgraph: *mut ConstraintGraph,
}
impl ConstraintGenerator {
  pub fn new(args: ConstraintGeneratorArgs<'_>) -> Self {
    // 按原参数顺序解包，保持与 C++ 一一对应。
    let ConstraintGeneratorArgs {
      module,
      normalizer,
      type_function_runtime,
      module_resolver,
      builtin_types,
      ice,
      global_scope,
      type_function_scope,
      prepare_module_scope,
      logger,
      dfg,
      require_cycles,
      cgraph,
    } = args;
    let normalizer_ref = unsafe { normalizer.as_ref() };
    let arena = { normalizer_ref.arena };

    let result = ConstraintGenerator {
      scopes: Vec::new(),
      module: Some(module),
      builtin_types: builtin_types.as_ptr(),
      arena,
      root_scope: null_mut(),
      type_context: TypeContext::default(),
      inferred_bindings: DenseHashMap::new(Symbol::default()),
      constraints: Vec::new(),
      free_types: TypeIds::new(),
      scope_to_function: DenseHashMap::new(null_mut()),
      ast_type_alias_defining_scopes: DenseHashMap::new(null()),
      dfg: dfg.as_ptr(),
      refinement_arena: RefinementArena {
        allocator: TypedAllocator::default(),
      },
      recursion_count: 0,
      errors: Vec::new(),
      normalizer: normalizer.as_ptr(),
      type_function_runtime: type_function_runtime.as_ptr(),
      ast_type_function_environment_scopes: DenseHashMap::new(null()),
      module_resolver: module_resolver.as_ptr(),
      ice: ice.as_ptr(),
      global_scope: Some(global_scope),
      type_function_scope: Some(type_function_scope),
      prepare_module_scope,
      require_cycles: require_cycles.to_vec(),
      local_types: DenseHashMap::new(null()),
      inferred_expr_cache: DenseHashMap::new(null_mut()),
      class_decl_records: DenseHashMap::new(null_mut()),
      logger,
      recursion_limit_met: false,
      cgraph,
      interior_free_types: Vec::new(),
      unions_to_simplify: Vec::new(),
      uninitialized_globals: Set::new(AstName::default()),
      polarity: Polarity::default(),
      prop_index_pairs_seen: DenseHashMap::new((null(), String::new())),
      large_table_depth: 0,
    };

    LUAU_ASSERT!(result.module.is_some());

    result
  }
}
