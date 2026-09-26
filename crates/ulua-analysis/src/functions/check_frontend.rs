use alloc::{sync::Arc, vec::Vec};
use core::{
  mem::take,
  ptr::{self, null_mut},
};

use ulua_ast::enums::mode::Mode;
use ulua_common::{
  fflag, fint,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  enums::solver_mode::SolverMode,
  functions::{
    arc_as_mut::arc_as_mut, check_non_strict::check_non_strict,
    check_type_checker_2::check as check_type_checker_2, freeze::freeze,
    synthesize_export_return::synthesize_export_return, unfreeze::unfreeze,
  },
  methods::{
    constraint_generator_constraint_generator::ConstraintGeneratorArgs,
    constraint_solver_constraint_solver_constraint_solver::SolverParams,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, constraint_generator::ConstraintGenerator,
    constraint_graph::ConstraintGraph, constraint_solver::ConstraintSolver,
    data_flow_graph_builder::DataFlowGraphBuilder, frontend_options::FrontendOptions,
    internal_error_reporter::InternalErrorReporter, module::Module,
    module_resolver::ModuleResolverRef, normalizer::Normalizer, require_cycle::RequireCycle,
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
  // —— 基础设施（NotNull 语义句柄化 / 独占借用）——
  pub builtin_types: Handle<BuiltinTypes>,
  pub ice_handler: &'a mut InternalErrorReporter,
  pub module_resolver: ModuleResolverRef,
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

/// 对应 C++ `Frontend::check` 私有重载（新求解器路径，Frontend.cpp）。参数已
/// 全部句柄化/引用化，非空前提由类型编码；函数为安全签名，体内裸指针操作
/// （`module_ptr` 写穿、`source_module.root` 的 AST 遍历等）各收在最小
/// `unsafe` 块并附 `// SAFETY:` 注，其成立依赖 crate 级不变量：
/// - `source_module.root` 非空且其解析 arena 在调用期间存活（DFG/约束生成
///   直接遍历该 AST）；
/// - `builtin_types`/`ice_handler`/`module_resolver` 等会话级实例比本次调用
///   长寿（`Handle` 模块契约）；
/// - 整函数按 crate 不变量单线程驱动：对 `Arc<Module>`/`Arc<Scope>` 的
///   `arc_as_mut` 写穿要求此期间无任何其他线程或别名可变借用。
pub fn check(args: CheckArgs<'_>) -> ModulePtr {
  // 按原参数顺序解包，保持与 C++ 一一对应。
  let CheckArgs {
    source_module,
    mode,
    require_cycles,
    builtin_types,
    ice_handler,
    module_resolver,
    parent_scope,
    type_function_scope,
    prepare_module_scope,
    options,
    limits,
    record_json_log: _,
    stats,
    write_json_log: _,
  } = args;
  let module: ModulePtr = Arc::new(Module::default());
  let module_ptr = arc_as_mut(&module);

  // Safety: `module_ptr` 由 `arc_as_mut(&module)` 从本作用域的 `Arc<Module>`
  // 派生，此刻 `module` 未被任何其他句柄/克隆持有（cg 尚未构造），写穿
  // 等价 C++ 对新建 shared_ptr<Module> 成员的独占初始化；owning_module
  // 自引用记录的是 bump 型 arena 的稳定地址（Module 在返回前不被移动，
  // Arc 指向的堆对象地址固定）。
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
    ice_handler.module_name = source_module.name.to_string();
  }

  // Safety: def_arena/key_arena 是对 module_ptr（上方写穿句柄，Arc 堆对象
  // 地址固定）字段的 &mut 借用；source_module.root 指向解析 arena 内活
  // AST，二者仅在调用表达式内解引用，借用期覆盖 build 全程，符合
  // DataFlowGraphBuilder::build 的裸指针前置条件（ice_handler 为安全
  // 重借用转存的会话独占指针）。
  let mut dfg = unsafe {
    DataFlowGraphBuilder::build(
      source_module.root,
      Handle::from_mut(&mut (*module_ptr).def_arena),
      Handle::from_mut(&mut (*module_ptr).key_arena),
      ptr::from_mut(&mut *ice_handler),
    )
  };

  let mut unifier_state = UnifierSharedState::new(ptr::from_mut(&mut *ice_handler));
  unifier_state.counters.recursion_limit = fint::LuauTypeInferRecursionLimit.get();
  unifier_state.counters.iteration_limit = limits
    .unifier_iteration_limit()
    .unwrap_or_else(|| fint::LuauTypeInferIterationLimit.get());

  // Safety: Normalizer::new 为安全函数，unsafe 仅因对 module_ptr 写穿
  // 句柄解引用取 internal_types 的 &mut（该 arena 在 normalizer 存活期内
  // 固定且被其独占借用）；builtin_types 句柄目标为会话常驻实例。
  let mut normalizer = unsafe {
    Normalizer::new(
      Some(Handle::from_mut(&mut (*module_ptr).internal_types)),
      builtin_types,
      Some(Handle::from_mut(&mut unifier_state)),
      SolverMode::New,
      false,
    )
  };

  let mut type_function_runtime = TypeFunctionRuntime {
    // ice_handler 为安全独占借用，(*_)clone 只读取 InternalErrorReporter 值。
    ice: (*ice_handler).clone(),
    limits: limits.clone(),
    type_arena: TypedAllocator::default(),
    type_pack_arena: TypedAllocator::default(),
    // B 型（Lua C ABI 边界）：`state.0` 为 `lua_State*`，此处留空指针表示
    // 「Lua VM 未启动」——惰性接线唯一由 `prepare_state` 完成（其首行空判即
    // cpp `if (state) return;`，TypeFunctionRuntime.cpp:230），消费方
    // （prepare_state/register_function/user_defined_type_function）均在其后取用。
    state: (null_mut(), None),
    initialized: DenseHashSet::default(),
    allow_evaluation: true,
    root_scope: parent_scope.clone(),
    messages: Vec::new(),
    // B 型（builder 接线槽）：空指针 = 「求值会话未打开」；唯一写入方是
    // `user_defined_type_function` 的 ScopedAssign（对应 cpp `ScopedAssign
    // setRuntimeBuilder(ctx->typeFunctionRuntime->runtimeBuilder, ...)`，
    // TypeFunctionRuntime.cpp:1844 系消费方），解引用仅发生在该登记窗口内。
    runtime_builder: null_mut(),
  };

  let mut cgraph_storage = if fflag::LuauConstraintGraph.get() {
    Some(ConstraintGraph {
      dependencies: DenseHashMap::new(Default::default()),
      reverse_dependencies: DenseHashMap::new(Default::default()),
      constraint_lists: Default::default(),
    })
  } else {
    None
  };
  // B 型（落点字段为 `*mut ConstraintGraph`，records/constraint_generator.rs 与
  // records/constraint_solver.rs 的指针字段契约）：空指针 = `LuauConstraintGraph`
  // 关时不建图；消费方一律以同一 fflag 门控后才解引用（cpp 侧该参数为
  // `NotNull<ConstraintGraph>`，ConstraintGenerator.h:158，Rust 用空指针折叠
  // 「flag 关」态）。
  let cgraph = cgraph_storage
    .as_mut()
    .map(|cgraph| cgraph as *mut ConstraintGraph)
    .unwrap_or(null_mut());

  // Safety: Subtyping::subtyping_owned 为安全构造函数，unsafe 仅来自
  // 对 module_ptr 解引用取 internal_types 的可变借用——该 arena 与
  // normalizer 持有的 internal_types 是同一指针字段，二者借用不重叠
  // （subtyping 只存裸指针副本）。
  let mut subtyping = unsafe {
    Subtyping::subtyping_owned(
      builtin_types,
      Handle::from_mut(&mut (*module_ptr).internal_types),
      &mut normalizer,
      &mut type_function_runtime,
      ptr::from_mut(&mut *ice_handler),
    )
  };

  // C 型：本检查路径从不构造 DcrLogger（cpp Frontend.cpp 同路径亦以空 logger
  // 实参下传），原「具名空指针变量 + from_opt_ptr」折叠为直接传 `None`，
  // 可空性由 `Option<Handle<DcrLogger>>` 字段类型编码。
  let mut cg = ConstraintGenerator::new(ConstraintGeneratorArgs {
    module: module.clone(),
    normalizer: Handle::from_mut(&mut normalizer),
    type_function_runtime: Handle::from_mut(&mut type_function_runtime),
    module_resolver,
    builtin_types,
    ice: Handle::from_mut(&mut *ice_handler),
    global_scope: parent_scope.clone(),
    type_function_scope: type_function_scope.clone(),
    prepare_module_scope,
    logger: None,
    dfg: Handle::from_mut(&mut dfg),
    require_cycles,
    cgraph,
  });

  let constraint_set = cg.run(source_module.root);
  // Safety: module_ptr 写穿句柄有效（Arc 堆对象地址固定）；constraint_set
  // 是 run 的返回值、cg.recursion_limit_met 是普通字段读，均在同一
  // 单线程独占期写入 module.errors，无别名可变借用并存。
  unsafe {
    (*module_ptr).errors = constraint_set.errors.clone();
    (*module_ptr).constraint_generation_did_not_complete = cg.recursion_limit_met;
  }

  // 构造参数已句柄化/引用化，主构造器为安全函数：logger 恒为 `None`（求解器
  // 按可空 DcrLogger 句柄处理，本路径不采集 DCR）、cgraph 为 null 或本地
  // Option 解出的 &mut 指针（独占）；normalizer/type_function_runtime/dfg/
  // subtyping 借自本函数活局部，其句柄副本与 cs 同作用域且借用不冲突。
  let mut cs = ConstraintSolver::constraint_solver_not_null_normalizer_not_null_type_function_runtime_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
    SolverParams {
      normalizer: Handle::from_mut(&mut normalizer),
      type_function_runtime: Handle::from_mut(&mut type_function_runtime),
      module: module.clone(),
      module_resolver,
      require_cycles: require_cycles.to_vec(),
      logger: None,
      dfg: Handle::from_mut(&mut dfg),
      limits: limits.clone(),
      constraint_set,
      cgraph,
      subtyping: Handle::from_mut(&mut subtyping),
    },
  );

  if let Some(seed) = options.randomize_constraint_resolution_seed {
    cs.randomize(seed);
  }

  cs.constraint_solver_run();
  stats.dynamic_constraints_created += cs.solver_constraints.len();

  // Safety: 三处裸指针写均在同一单线程独占期：module_ptr 指向地址
  // 固定的 Arc 堆对象；cg/cs 仍存活，其 errors/scopes 字段读为普通
  // 借用；upper_bound_contributors 换空表用 take 留下 `DenseHashMap::default()`
  // 门面（其哨兵即 null 键，与原 new(null()) 逐位一致、与 cpp 构造语义一致）。
  unsafe {
    (*module_ptr).errors.extend(cs.errors.iter().cloned());
    (*module_ptr).scopes = take(&mut cg.scopes);
    (*module_ptr).r#type = source_module.r#type;
    (*module_ptr).upper_bound_contributors = take(&mut cs.upper_bound_contributors);
  }

  // Safety: module_ptr 写穿句柄有效（见上方初始化注释）。超时分支里
  // `get_module_scope()` 返回 scopes[0] 的 Arc 克隆，arc_as_mut 对同一
  // Scope 堆对象做单线程独占写（crate 已知写穿纪律）；builtin_types
  // 句柄目标为会话常驻实例，error_type/error_type_pack 为其普通字段读。
  if unsafe { (*module_ptr).timeout || (*module_ptr).cancelled } {
    // C++（Frontend.cpp:2428-2436）：求解被中断时，跳过 typecheck 并把模块结果
    // 全部替换为 error 抑制类型，避免向外界泄漏 blocked / pending 类型。
    // Safety: module_ptr 写穿句柄有效（见上方初始化注释）；get_module_scope()
    // 返回 scopes[0] 的 Arc 克隆，arc_as_mut 对其取独占写句柄系 crate 写穿纪律。
    let module_scope_ptr = unsafe { arc_as_mut(&(*module_ptr).get_module_scope()) };
    // Safety: module_scope_ptr 由刚 clone 的 ScopePtr 写穿而来，Scope
    // 堆对象地址固定，此刻仅本线程触碰。
    unsafe { (*module_scope_ptr).return_type = builtin_types.get().error_type_pack };
    // Safety: declared_globals 经 module_ptr 取 &mut，写穿独占期与上方同理。
    for ty in unsafe { &mut (*module_ptr).declared_globals }.values_mut() {
      *ty = builtin_types.get().error_type;
    }
    // Safety: exported_type_bindings 与 declared_globals 同理经 module_ptr
    // 写穿独占取 &mut。
    for tf in unsafe { &mut (*module_ptr).exported_type_bindings }.values_mut() {
      tf.r#type = builtin_types.get().error_type;
    }
  } else {
    match mode {
      Mode::Nonstrict => {
        // 唯一裸实参 module_ptr 为写穿句柄（Arc 堆对象地址固定），比本次检查
        // 长寿且此刻单线程独占；其余实参各有活源——
        // type_function_runtime/unifier_state/ice_handler 为本函数活局部/
        // 独占借用直转，limits.clone() 临时量在此语句内存活，
        // builtin_types 为会话常驻单例句柄。
        check_non_strict(
          builtin_types,
          &mut type_function_runtime,
          ice_handler,
          &mut unifier_state,
          &dfg,
          &limits.clone(),
          source_module,
          module_ptr,
        );
      }
      Mode::Definition | Mode::Strict => {
        // Safety: `module_ptr` 为写穿句柄（Arc 堆对象地址固定），比本次检查
        // 长寿且此刻单线程独占（checker 与收尾块串行使用）；其余实参为受检
        // 引用——logger 恒为 `None`（本路径不采集 DCR，可空性由形参的
        // `Option` 类型承载），source_module 借用覆盖整个调用，其 root AST
        // arena 存活；limits.clone() 临时量在语句内存活。
        check_type_checker_2(
          builtin_types,
          &mut type_function_runtime,
          &mut unifier_state,
          &mut limits.clone(),
          None,
          source_module,
          module_ptr,
        );
      }
      Mode::NoCheck => {}
    }

    if fflag::LuauExportValueSyntax.get() && fflag::LuauExportValueTypecheck.get() {
      // Safety: builtin_types 按函数契约非空存活；module_ptr 为 Arc 堆
      // 对象写穿句柄，此刻无别名可变借用，被调方仅在其上读写类型字段。
      unsafe { synthesize_export_return(builtin_types, module_ptr) };
    }
  }

  // Safety: module_ptr 写穿独占期同前；`errors[0]` 索引由 len()==1 守卫
  // 不会越界；unfreeze/freeze 依次取得 interface_types/internal_types 的
  // &mut，借用互斥且 arena 在 module 返回前保持存活。
  unsafe {
    if (*module_ptr).errors.len() == 1
      && !fflag::DebugLuauAlwaysShowConstraintSolvingIncomplete.get()
      && matches!(
        &(&(*module_ptr).errors)[0].data,
        TypeErrorData::ConstraintSolvingIncompleteError(_)
      )
    {
      (*module_ptr).errors.clear();
    }

    unfreeze(&mut (*module_ptr).interface_types);
    (*module_ptr).clone_public_interface(builtin_types, ice_handler, SolverMode::New);
    freeze(&mut (*module_ptr).internal_types);
    freeze(&mut (*module_ptr).interface_types);
  }

  module
}
