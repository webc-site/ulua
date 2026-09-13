use alloc::{sync::Arc, vec::Vec};
use core::{
  mem::{replace, take},
  ptr::{NonNull, null, null_mut},
};

use ulua_ast::enums::mode::Mode;
use ulua_common::{
  FFlag, FInt,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  enums::solver_mode::SolverMode,
  functions::{
    check_non_strict::check_non_strict, check_type_checker_2::check as check_type_checker_2,
    freeze::freeze, synthesize_export_return::synthesize_export_return, unfreeze::unfreeze,
  },
  methods::{
    constraint_generator_constraint_generator::ConstraintGeneratorArgs,
    constraint_solver_constraint_solver_constraint_solver::SolverParams,
  },
  records::{
    builtin_types::BuiltinTypes, constraint_generator::ConstraintGenerator,
    constraint_graph::ConstraintGraph, constraint_solver::ConstraintSolver,
    data_flow_graph_builder::DataFlowGraphBuilder, dcr_logger::DcrLogger,
    file_resolver::FileResolver, frontend_options::FrontendOptions,
    internal_error_reporter::InternalErrorReporter, module::Module,
    module_resolver::ModuleResolver, normalizer::Normalizer, require_cycle::RequireCycle,
    source_module::SourceModule, stats::Stats, subtyping::Subtyping,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
    typed_allocator::TypedAllocator, unifier_shared_state::UnifierSharedState,
  },
  type_aliases::{
    frontend_callbacks::{JsonLogCallback, ModuleScopeCallback},
    module_ptr_module::ModulePtr,
    scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData,
  },
};
/// `Frontend::check`（新求解器路径）的参数包，对应 C++ 私有十五参数
/// `Frontend::check`（Frontend.cpp）。按语义分组：模块上下文 / 基础设施 /
/// 回调 / 选项与限额 / 统计输出。
pub struct CheckArgs<'a> {
  // —— 模块上下文 ——
  pub source_module: &'a SourceModule,
  pub mode: Mode,
  pub require_cycles: &'a [RequireCycle],
  pub parent_scope: &'a ScopePtr,
  pub type_function_scope: &'a ScopePtr,
  // —— 基础设施（NotNull 语义的裸指针）——
  pub builtin_types: *mut BuiltinTypes,
  pub ice_handler: *mut InternalErrorReporter,
  pub module_resolver: *mut ModuleResolver,
  pub file_resolver: *mut FileResolver,
  // —— 回调 ——
  pub prepare_module_scope: ModuleScopeCallback,
  pub write_json_log: JsonLogCallback,
  // —— 选项与限额 ——
  pub options: FrontendOptions,
  pub limits: TypeCheckLimits,
  pub record_json_log: bool,
  // —— 统计输出 ——
  pub stats: &'a mut Stats,
}

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn check(args: CheckArgs<'_>) -> ModulePtr {
  // 按原参数顺序解包，保持与 C++ 一一对应。
  let CheckArgs {
    source_module,
    mode,
    require_cycles,
    builtin_types,
    ice_handler,
    module_resolver,
    file_resolver,
    parent_scope,
    type_function_scope,
    prepare_module_scope,
    options,
    limits,
    record_json_log,
    stats,
    write_json_log,
  } = args;
  // C++ 中这三处参数未读取，保持原语义。
  let _ = file_resolver;
  let _ = record_json_log;
  let _ = write_json_log;
  let module: ModulePtr = Arc::new(Module::default());
  let module_ptr = Arc::as_ptr(&module) as *mut Module;

  unsafe {
    (*module_ptr).checked_in_new_solver = true;
    (*module_ptr).name = source_module.name.clone();
    (*module_ptr).human_readable_name = source_module.human_readable_name.clone();
    (*module_ptr).mode = mode;
    (*module_ptr).internal_types.owning_module = module_ptr;
    (*module_ptr).interface_types.owning_module = module_ptr;
    (*module_ptr).internal_types.collect_singleton_stats = options.collect_type_allocation_stats;
    (*module_ptr).allocator = Some(source_module.allocator.clone());
    (*module_ptr).names = Some(source_module.names.clone());
    (*module_ptr).root = source_module.root;
    (*ice_handler).module_name = source_module.name.clone();
  }

  let mut dfg = unsafe {
    DataFlowGraphBuilder::build(
      source_module.root,
      &mut (*module_ptr).def_arena,
      &mut (*module_ptr).key_arena,
      ice_handler,
    )
  };

  let mut unifier_state = UnifierSharedState::new(ice_handler);
  unifier_state.counters.recursion_limit = FInt::LuauTypeInferRecursionLimit.get();
  unifier_state.counters.iteration_limit = limits
    .unifier_iteration_limit()
    .unwrap_or_else(|| FInt::LuauTypeInferIterationLimit.get());

  let mut normalizer = unsafe {
    Normalizer::new(
      &mut (*module_ptr).internal_types,
      builtin_types,
      &mut unifier_state,
      SolverMode::New,
      false,
    )
  };

  let mut type_function_runtime = TypeFunctionRuntime {
    ice: unsafe { (*ice_handler).clone() },
    limits: limits.clone(),
    type_arena: TypedAllocator::default(),
    type_pack_arena: TypedAllocator::default(),
    state: (null_mut(), None),
    initialized: DenseHashSet::new(null_mut()),
    allow_evaluation: true,
    root_scope: parent_scope.clone(),
    messages: Vec::new(),
    runtime_builder: null_mut(),
  };

  let mut cgraph_storage = if FFlag::LuauConstraintGraph.get() {
    Some(ConstraintGraph {
      builtin_types: NonNull::new(builtin_types).expect("builtinTypes must not be null"),
      dependencies: DenseHashMap::new(Default::default()),
      reverse_dependencies: DenseHashMap::new(Default::default()),
      constraint_lists: Default::default(),
    })
  } else {
    None
  };
  let cgraph = cgraph_storage
    .as_mut()
    .map(|cgraph| cgraph as *mut ConstraintGraph)
    .unwrap_or(null_mut());

  let subtyping = unsafe {
    Subtyping::subtyping_owned(
      builtin_types,
      &mut (*module_ptr).internal_types,
      &mut normalizer,
      &mut type_function_runtime,
      ice_handler,
    )
  };

  let logger: *mut DcrLogger = null_mut();
  let mut cg = ConstraintGenerator::new(ConstraintGeneratorArgs {
    module: module.clone(),
    normalizer: NonNull::new(&mut normalizer).unwrap(),
    type_function_runtime: NonNull::new(&mut type_function_runtime).unwrap(),
    module_resolver: NonNull::new(module_resolver).expect("moduleResolver must not be null"),
    builtin_types: NonNull::new(builtin_types).expect("builtinTypes must not be null"),
    ice: NonNull::new(ice_handler).expect("iceHandler must not be null"),
    global_scope: parent_scope.clone(),
    type_function_scope: type_function_scope.clone(),
    prepare_module_scope,
    logger,
    dfg: NonNull::new(&mut dfg).unwrap(),
    require_cycles,
    cgraph,
  });

  let constraint_set = cg.run(source_module.root);
  unsafe {
    (*module_ptr).errors = constraint_set.errors.clone();
    (*module_ptr).constraint_generation_did_not_complete = cg.recursion_limit_met;
  }

  let mut cs = unsafe {
    ConstraintSolver::constraint_solver_not_null_normalizer_not_null_type_function_runtime_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
            SolverParams {
            normalizer: &normalizer,
            type_function_runtime: &type_function_runtime,
            module: module.clone(),
            module_resolver,
            require_cycles: require_cycles.to_vec(),
            logger,
            dfg: &dfg,
            limits: limits.clone(),
            constraint_set,
            cgraph,
            subtyping: &subtyping,
        },
    )
  };

  if let Some(seed) = options.randomize_constraint_resolution_seed {
    cs.randomize(seed);
  }

  cs.constraint_solver_run();
  stats.dynamic_constraints_created += cs.solver_constraints.len();

  unsafe {
    (*module_ptr).errors.extend(cs.errors.iter().cloned());
    (*module_ptr).scopes = take(&mut cg.scopes);
    (*module_ptr).r#type = source_module.r#type;
    (*module_ptr).upper_bound_contributors =
      replace(&mut cs.upper_bound_contributors, DenseHashMap::new(null()));
  }

  if !unsafe { (*module_ptr).timeout || (*module_ptr).cancelled } {
    match mode {
      Mode::Nonstrict => {
        unsafe {
          check_non_strict(
            builtin_types,
            &mut type_function_runtime,
            ice_handler,
            &mut unifier_state,
            &dfg,
            &mut limits.clone(),
            source_module,
            module_ptr,
          )
        };
      }
      Mode::Definition | Mode::Strict => {
        unsafe {
          check_type_checker_2(
            builtin_types,
            &mut type_function_runtime,
            &mut unifier_state,
            &mut limits.clone(),
            logger,
            source_module,
            module_ptr,
          )
        };
      }
      Mode::NoCheck => {}
    }

    if FFlag::LuauExportValueSyntax.get()
      && FFlag::LuauExportValueTypecheck.get()
      && !unsafe { (*module_ptr).timeout || (*module_ptr).cancelled }
    {
      unsafe { synthesize_export_return(builtin_types, module_ptr) };
    }
  }

  unsafe {
    if (*module_ptr).errors.len() == 1
      && !FFlag::DebugLuauAlwaysShowConstraintSolvingIncomplete.get()
      && matches!(
        &(&(*module_ptr).errors)[0].data,
        TypeErrorData::ConstraintSolvingIncompleteError(_)
      )
    {
      (*module_ptr).errors.clear();
    }

    unfreeze(&mut (*module_ptr).interface_types);
    (*module_ptr).clone_public_interface(builtin_types, &mut *ice_handler, SolverMode::New);
    freeze(&mut (*module_ptr).internal_types);
    freeze(&mut (*module_ptr).interface_types);
  }

  module
}
