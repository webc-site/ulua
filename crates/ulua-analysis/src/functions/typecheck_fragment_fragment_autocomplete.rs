use alloc::{boxed::Box, rc::Rc, string::String, sync::Arc, vec::Vec};
use core::{
  mem::take,
  ptr::{NonNull, null, null_mut},
};
use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_ast::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
    position::Position,
  },
};
use ulua_common::{
  fflag, fint, functions::get_clock::get_clock, macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE,
  records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::{
    fragment_autocomplete_waypoint::FragmentAutocompleteWaypoint,
    fragment_type_check_status::FragmentTypeCheckStatus, solver_mode::SolverMode,
  },
  functions::{
    arc_as_mut::arc_as_mut, borrow_constraints::borrow_constraints,
    clone_types_from_fragment::clone_types_from_fragment, find_closest_scope::find_closest_scope,
    freeze::freeze, get_module_resolver::get_module_resolver,
    is_within_comment_module::is_within_comment, parse_fragment::parse_fragment,
    trace_requires::trace_requires, unfreeze::unfreeze,
  },
  methods::{
    constraint_generator_constraint_generator::ConstraintGeneratorArgs,
    constraint_solver_constraint_solver_constraint_solver::FragmentSolverParams,
  },
  records::{
    arena_handle::Handle,
    clone_state::CloneState,
    constraint::Constraint,
    constraint_generator::ConstraintGenerator,
    constraint_graph::ConstraintGraph,
    constraint_solver::ConstraintSolver,
    data_flow_graph_builder::DataFlowGraphBuilder,
    dcr_logger::DcrLogger,
    expected_type_visitor::ExpectedTypeVisitor,
    fragment_parse_result::FragmentParseResult,
    fragment_type_check_result::FragmentTypeCheckResult,
    frontend::Frontend,
    frontend_options::FrontendOptions,
    i_fragment_autocomplete_reporter::ReporterRef,
    internal_error_reporter::InternalErrorReporter,
    module::Module,
    module_resolver::ModuleResolverRef,
    normalizer::Normalizer,
    scope::Scope,
    scope_registry::{intern_scope, register_scope},
    subtyping::Subtyping,
    type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::{
    frontend_callbacks::ModuleScopeCallback, module_name_type::ModuleName,
    module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr,
  },
};

/// # Safety
/// 对应 cpp `typecheckFragment_`（`FragmentAutocomplete.cpp:1146`）。逐参数契约：
/// - `frontend`：调用期间存活的独占借用；函数内派生的 `ice_handler`、
///   `builtin_types`（NonNull 自指 `builtin_types_` 存储）与 `file_resolver`
///   句柄均以其字段为落点，生命周期覆盖本函数。
/// - `root`：必须指向由本次传入的 `ast_allocator` 分配的 fragment 根
///   `AstStatBlock`（即 `parse_fragment` 成功时的 `FragmentParseResult.root`），
///   且在本函数执行期间不被其他所有者可变访问（allocator 所有权随本调用移交，
///   AST 随之存活）。
/// - `stale`：调用方保活的 `Arc<Module>`；本函数按 cpp 契约对其 arena 做
///   freeze/unfreeze、读取 name/human_readable_name，并可能在超时后写
///   `timeout`——均属 `arc_as_mut` 记录的写穿惯用法，要求单线程独占驱动
///   （见 lib.rs crate 不变量 1/2）。
/// - `closest_scope`：`find_closest_scope(&stale, ...)` 产出的 `Arc<Scope>`，
///   必须属于 `stale` 的作用域树并在调用期间存活。
/// - `_cursor_pos`：与 cpp 签名对齐保留，函数体不使用。
/// - `ast_allocator`：分配了 `root` 的 `Allocator`；所有权被移交给增量模块
///   （`allocator` 字段注入），因此对 `root` 的裸指针解引用在其内存活。
/// - `opts` / `reporter`：普通借用，调用期间存活即可。
pub unsafe fn typecheck_fragment_(
  frontend: &mut Frontend,
  root: *mut AstStatBlock,
  stale: &ModulePtr,
  closest_scope: &ScopePtr,
  // 与 C++ 签名对齐保留该参数，但主体未使用（C++ 同样未使用 cursorPos）
  _cursor_pos: &Position,
  ast_allocator: Box<Allocator>,
  opts: &FrontendOptions,
  reporter: ReporterRef<'_>,
) -> FragmentTypeCheckResult {
  LUAU_TIMETRACE_SCOPE!("Luau::typecheckFragment_", "FragmentAutocomplete");

  let stale_ptr = arc_as_mut(stale);
  // Safety: `stale_ptr` 派生自参数 `stale`（`&Arc<Module>`），该 Arc 在整个调用
  // 内保活此 Module。经裸指针取 `&mut` 做 freeze 是 `arc_as_mut` 契约允许的
  // cpp 直译写穿（等价 cpp:1158-1159 `freeze(*stale->internalTypes)`）：本函数
  // 单线程独占驱动，两个 `&mut` 借用半径均止于各自语句，语句间无并存借用。
  unsafe {
    freeze(&mut (*stale_ptr).internal_types);
    freeze(&mut (*stale_ptr).interface_types);
  }

  let incremental_module: ModulePtr = Arc::new(Module::default());
  let module_ptr = arc_as_mut(&incremental_module);
  // 安全等价改写：经参数 `stale` 的 `Arc<Module>` Deref 只读克隆，替代原先对
  // `(*stale_ptr).name` 的裸指针解引用。
  let module_name = stale.name.clone();
  // Safety: `module_ptr` 指向刚由 `Arc::new(Module::default())` 创建、此刻唯一
  // 强引用为 `incremental_module` 的增量模块；该 Arc 存续至函数末尾并移入返回
  // 结果，其间本函数独占驱动（cpp:1161-1168 直译）。`owning_module` 自引用注入
  // 亦只存裸身份句柄。对 `stale` 模块两字段的读取为瞬态只读借用
  // （cpp:1161-1162 同源读）。
  unsafe {
    (*module_ptr).name = module_name.clone();
    (*module_ptr).human_readable_name =
      String::from("Incremental$") + &(*stale_ptr).human_readable_name;
    (*module_ptr).internal_types.owning_module = module_ptr;
    (*module_ptr).interface_types.owning_module = module_ptr;
    (*module_ptr).allocator = Some(Arc::from(ast_allocator));
    // C++ `Module::names` is a `std::shared_ptr<AstNameTable>`. The fragment
    // root is parsed reusing the stale module's name table
    // (`parseFragment(..., module->names.get(), ...)`,
    // FragmentAutocomplete.cpp:1317), so the incremental module's
    // constraint generation (e.g. `module->names->get(...)` in
    // `prototypeTypeDefinitions`, ConstraintGenerator.cpp:1220) resolves
    // against that same table. Share it here so the AstName lookups operate
    // on the table the fragment AST nodes were interned into.
    (*module_ptr).names = (*stale_ptr).names.clone();
    (*module_ptr).checked_in_new_solver = true;
    unfreeze(&mut (*module_ptr).internal_types);
    unfreeze(&mut (*module_ptr).interface_types);
  }

  // Setup typecheck limits
  let limits = TypeCheckLimits {
    finish_time: opts.module_time_limit_sec.map(|secs| get_clock() + secs),
    cancellation_token: opts.cancellation_token.clone(),
    ..Default::default()
  };

  let ice_handler: *mut InternalErrorReporter = &mut frontend.ice_handler;
  let builtin_types = frontend.builtin_types_handle();

  // Make the shared state for the unifier (recursion + iteration limits)
  let mut unifier_state = UnifierSharedState::new(ice_handler);
  unifier_state.counters.recursion_limit = fint::LuauTypeInferRecursionLimit.get();
  unifier_state.counters.iteration_limit = limits
    .unifier_iteration_limit()
    .unwrap_or_else(|| fint::LuauTypeInferIterationLimit.get());

  // Initialize the normalizer
  // Safety: `module_ptr` 指向存活增量模块（`incremental_module` 强引用持有、本
  // 函数独占驱动），Normalizer::new 仅存裸句柄，&mut 借用半径止于本表达式，
  // 期间无对 internal_types 的其他并存可变访问；`builtin_types` 指向 frontend
  // 自有存储、`&mut unifier_state` 为上方局部值（cpp:1186 构造直译）。
  let mut normalizer = unsafe {
    Normalizer::new(
      Some(Handle::from_mut(&mut (*module_ptr).internal_types)),
      builtin_types,
      Some(Handle::from_mut(&mut unifier_state)),
      SolverMode::New,
      false,
    )
  };

  // User defined type functions runtime
  // 空 `state`（未懒建的 lua_State）与未接线的 `runtime_builder` 由构造器
  // `TypeFunctionRuntime::new` 单点表达——对应 cpp `TypeFunctionRuntime.cpp:58`
  // ctor 初始化 `state(nullptr, dummyStateClose)` 与头文件默认
  // `runtimeBuilder = nullptr`（TypeFunctionRuntime.h:327），本调用点不再手写字面量。
  // Safety: `ice_handler` 指向上方从 `&mut frontend.ice_handler` 取得的字段，
  // `frontend` 借用贯穿本函数；new 内 clone 为语句内瞬态只读。
  let mut type_function_runtime =
    TypeFunctionRuntime::new(unsafe { &*ice_handler }, &limits, closest_scope.clone());
  // 对应 cpp:1195 `typeFunctionRuntime.allowEvaluation =
  // FFlag::LuauFragmentACEnableTypeFunctionEvaluation;`——该 flag 默认 false，
  // Rust 端按默认值定形为关闭。
  type_function_runtime.allow_evaluation = false;

  // Safety: `subtyping_owned` 为 safe fn、仅存裸句柄，本块的 unsafe 只在
  // `(*module_ptr).internal_types` 的解引用——增量模块存活且独占（同上证成）；
  // `normalizer`/`type_function_runtime` 是本函数局部变量（借用半径止于本表达式，
  // 转存后其宿主仍存活至返回），`builtin_types`/`ice_handler` 来源同上
  // （cpp:1192 直译）。
  let mut subtyping = unsafe {
    Subtyping::subtyping_owned(
      builtin_types,
      Handle::from_mut(&mut (*module_ptr).internal_types),
      &mut normalizer,
      &mut type_function_runtime,
      ice_handler,
    )
  };

  // Create a DataFlowGraph just for the surrounding context
  // Safety: `DataFlowGraphBuilder::build` 为 unsafe fn，其契约要求各指针指向
  // 存活对象：`root` 是 `ast_allocator`（已注入增量模块）持有的 fragment AST，
  // def/key arena 借自存活的 `module_ptr`（本函数独占），`ice_handler` 指向
  // frontend 字段。&mut 借用半径均止于本次调用，与 cpp:1198 逐参对应。
  let mut dfg = unsafe {
    DataFlowGraphBuilder::build(
      root,
      Handle::from_mut(&mut (*module_ptr).def_arena),
      Handle::from_mut(&mut (*module_ptr).key_arena),
      ice_handler,
    )
  };
  reporter.report_waypoint(FragmentAutocompleteWaypoint::DfgBuildEnd);

  // requireTrace for the surrounding context. Erased on the way out (ScopedExit).
  // Safety: `root` 指向 `ast_allocator`（已移交增量模块持有）分配的 fragment AST，
  // 本函数独占使用；`&mut` 借用半径止于本次调用（cpp `traceRequires(..., root, ...)`
  // :1208 对非 const `AstStatBlock*` 的直译），返回的 RequireTraceResult 为独立值。
  let trace = unsafe {
    trace_requires(
      frontend.file_resolver_mut(),
      &mut *root,
      module_name.clone(),
      &limits,
    )
  };
  frontend.require_trace.insert(module_name.clone(), trace);

  // resolver 以 cpp `ModuleResolver*` 语义在约束生成器与求解器间共享：
  // [`ModuleResolverRef`] 为 `Copy` 的薄指针分派句柄，按值传递即等价于
  // 复制裸别名；globals 先取克隆，避免与 resolver 对 frontend 的借用交叠。
  let global_scope = frontend.globals.global_scope.clone();
  let type_function_scope = frontend.globals.global_type_function_scope.clone();
  let resolver = ModuleResolverRef::from(get_module_resolver(frontend, Some(opts.clone())));

  // freshChildOfNearestScope = std::make_shared<Scope>(nullptr)
  let mut fresh_scope_value = Scope::scope_type_pack_id(null());
  fresh_scope_value.interior_free_types = Some(Vec::new());
  fresh_scope_value.interior_free_type_packs = Some(Vec::new());
  // Safety: `root` 由 `ast_allocator` 保活（其所有权已注入增量模块），只读
  // location 为语句内 Copy。
  fresh_scope_value.location = unsafe { (*root).base.base.location };
  let fresh_child_of_nearest_scope: ScopePtr = Arc::new(fresh_scope_value);
  register_scope(&fresh_child_of_nearest_scope);
  let fresh_scope_ptr = arc_as_mut(&fresh_child_of_nearest_scope);

  let mut cgraph_storage = if fflag::LuauConstraintGraph.get() {
    Some(ConstraintGraph {
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
    // 「无约束图」是下游 `*mut ConstraintGraph` 字段（records/constraint_generator.rs:68、
    // records/constraint_solver.rs:76）目前的可空表示：cpp 形参为
    // `NotNull<ConstraintGraph>`（ConstraintSolver.cpp:460）本无 null 形态，Rust 移植
    // 保留 `LuauConstraintGraph` flag 关闭 = 空槽语义；字段 Option 化波及其它文件，
    // 不在本批四文件范围内，本调用点无法折成 `Option<&T>`。
    .unwrap_or(null_mut());

  // 无 Dcr 记录器：对应 cpp `typecheckFragment_` 给 ConstraintGenerator /
  // ConstraintSolver 的 `DcrLogger*` 可空形参（ConstraintGenerator.cpp:313、
  // ConstraintSolver.cpp:457）直接传 `nullptr`（FragmentAutocomplete.cpp:1227、1275）。
  // 两个 args 结构的 logger 字段已是 `Option<Handle<DcrLogger>>`，直接用 `None`
  // 表达「未启用」，不再经裸指针 null 哨兵中转（b 类哨兵消失）。
  let logger: Option<Handle<DcrLogger>> = None;

  // Constraint Generator
  let prepare_module_scope: ModuleScopeCallback = Rc::new(|_, _| {});

  let mut cg = ConstraintGenerator::new(ConstraintGeneratorArgs {
    module: incremental_module.clone(),
    normalizer: Handle::from_mut(&mut normalizer),
    type_function_runtime: Handle::from_mut(&mut type_function_runtime),
    module_resolver: resolver,
    builtin_types,
    ice: Handle::from_ptr(ice_handler),
    global_scope,
    type_function_scope,
    prepare_module_scope,
    logger,
    dfg: Handle::from_mut(&mut dfg),
    require_cycles: &[],
    cgraph,
  });

  // `builtin_types` 为 frontend.builtin_types_handle()——该句柄
  // 在 Frontend 构造期指向同一 struct 持有的 `builtin_types_` 存储，随
  // `&mut frontend` 借用存活；get_mut 物化的可变借用半径止于本语句（CloneState
  // 只留句柄副本），其间无对 BuiltinTypes 的其他可变访问。
  let mut clone_state = CloneState::new(builtin_types.get_mut());

  // incrementalModule->scopes.emplace_back(root->location, freshChildOfNearestScope);
  // Safety: `module_ptr` 指向存活增量模块（`incremental_module` 强引用持有），
  // push 为本函数独占写；`root` 读 location 同上由 allocator 保活；
  // `fresh_child_of_nearest_scope` 是本作用域存活的 Arc，clone 只增计数。
  unsafe {
    (*module_ptr).scopes.push((
      (*root).base.base.location,
      fresh_child_of_nearest_scope.clone(),
    ));
  }
  cg.root_scope = Some(fresh_child_of_nearest_scope.clone());

  // Create module-local scope for the type function environment
  let local_type_function_scope: ScopePtr =
    // Safety: type_function_scope 由 frontend.globals.global_type_function_scope
    // 装配期接线（cpp NotNull），本构造点前无清空路径，恒为 Some。
    Arc::new(Scope::new(
      cg
        .type_function_scope
        .as_ref()
        .expect("构造期接线 cpp NotNull，恒为 Some"),
      0,
    ));
  register_scope(&local_type_function_scope);
  // Safety: `local_type_function_scope` 是刚创建、强引用计数为 1 的局部 Arc，
  // 写穿（arc_as_mut 惯用法）半径止于本语句，随后才在下方移交给 runtime。
  unsafe {
    let lhs = arc_as_mut(&local_type_function_scope);
    (*lhs).location = (*root).base.base.location;
  }
  // Safety: `cg.type_function_runtime` 是构造 cg 时由局部 `type_function_runtime`
  // 的 `&mut` 转成的 NonNull，该局部变量存活至函数返回且此刻无并存借用；
  // 写入其 root_scope 字段是独占访问（移交 local_type_function_scope 的所有权
  // 副本，Arc clone 仅增计数）。
  cg.type_function_runtime.get_mut().root_scope = local_type_function_scope;

  reporter.report_waypoint(FragmentAutocompleteWaypoint::CloneAndSquashScopeStart);
  // Safety: `module_ptr`/`root`/`fresh_scope_ptr` 分别由本地 `incremental_module` Arc、
  // 已注入增量模块的 `ast_allocator` 与本地 Arc `fresh_child_of_nearest_scope` 保活到
  // 函数末尾，三者目标互不重叠；两个 `&mut` 的写半径止于下方这一次克隆
  // （cpp:1244-1253 直译）。`dfg` 为上方局部值，`builtin_types` 指向 frontend 自有存储。
  let (dest_arena, fragment_root, dest_scope) = unsafe {
    (
      Handle::from_ptr(&mut (*module_ptr).internal_types),
      &mut *root,
      &mut *fresh_scope_ptr,
    )
  };
  clone_types_from_fragment(
    &mut clone_state,
    closest_scope,
    stale,
    dest_arena,
    &mut dfg,
    builtin_types,
    fragment_root,
    dest_scope,
  );
  reporter.report_waypoint(FragmentAutocompleteWaypoint::CloneAndSquashScopeEnd);

  cg.visit_fragment_root(&fresh_child_of_nearest_scope, root);

  let cg_scopes = take(&mut cg.scopes);
  for p in cg_scopes {
    // Safety: `p` 已从 `cg.scopes` 移出（所有权归本迭代），push 进存活的增量模块
    // （`module_ptr` 独占写，cpp:1260 `emplace_back(std::move(p))` 直译）。
    unsafe {
      (*module_ptr).scopes.push(p);
    }
  }

  reporter.report_waypoint(FragmentAutocompleteWaypoint::ConstraintSolverStart);

  // Initialize the constraint solver and run it.
  // C++ uses the fragment ConstraintSolver constructor:
  //   NotNull(cg.rootScope), borrowConstraints(cg.constraints), NotNull{&cg.scopeToFunction}, ...
  let borrowed = borrow_constraints(&cg.constraints);
  let constraints: Vec<NonNull<Constraint>> = borrowed
    .into_iter()
    .map(|c| NonNull::new(c).expect("constraint must not be null"))
    .collect();
  let cg_root_scope = NonNull::new(arc_as_mut(cg.root())).expect("rootScope must not be null");
  // &mut 引用反推 NonNull 恒非空，免判空分支（cpp NotNull{&cg.scopeToFunction} 同义）。
  let scope_to_function = NonNull::from(&mut cg.scope_to_function);

  let mut cs =
        ConstraintSolver::constraint_solver_not_null_normalizer_not_null_type_function_runtime_not_null_scope_vector_not_null_constraint_not_null_dense_hash_map_scope_type_id_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
            FragmentSolverParams {
            normalizer: Handle::from_mut(&mut normalizer),
            type_function_runtime: Handle::from_mut(&mut type_function_runtime),
            root_scope: cg_root_scope,
            constraints,
            scope_to_function,
            module: incremental_module.clone(),
            module_resolver: resolver,
            require_cycles: Vec::new(),
            logger,
            dfg: Handle::from_mut(&mut dfg),
            limits: limits.clone(),
            cgraph,
            subtyping: Handle::from_mut(&mut subtyping),
        },
    );

  let solver_panicked = catch_unwind(AssertUnwindSafe(|| cs.constraint_solver_run()));
  if solver_panicked.is_err() {
    // C++ catches TimeLimitError / UserCancelError and marks the stale module.
    // Safety: `stale_ptr` 由参数 `stale` Arc 保活（同上 freeze 证成）；写
    // `timeout = true` 为语句内瞬态独占写（cpp:1288 直译）。
    unsafe {
      (*stale_ptr).timeout = true;
    }
  }

  reporter.report_waypoint(FragmentAutocompleteWaypoint::ConstraintSolverEnd);

  // Safety: 各 `&mut` 解引用均借自存活的 `module_ptr`（增量模块 Arc 本地持有、
  // 独占驱动），ExpectedTypeVisitor::new 只存裸句柄，借用半径止于本表达式；
  // `builtin_types` 指向 frontend 自有存储，`fresh_scope_ptr` 指向本地 Arc
  // `fresh_child_of_nearest_scope` 的内容（cpp:1297-1305 直译）。
  let mut etv = unsafe {
    ExpectedTypeVisitor::new(
      &mut (*module_ptr).ast_types,
      &mut (*module_ptr).ast_expected_types,
      &mut (*module_ptr).ast_resolved_types,
      &mut (*module_ptr).ast_overload_resolved_types,
      Handle::from_mut(&mut (*module_ptr).internal_types),
      builtin_types,
      fresh_scope_ptr,
    )
  };
  // Safety: `root` 由 allocator（已移交增量模块）保活，etv 遍历期间本函数对其
  // 独占；&mut 借用半径止于本次 visit（cpp `root->visit(&etv)` 非 const 直译）。
  ast_stat_block_visit(unsafe { &mut *root }, &mut etv);

  // In frontend we would forbid internal types because this is just for autocomplete,
  // we don't actually care. We also don't even need to typecheck - just synthesize types
  // as best as we can.
  // Safety: 两个 arena 的 freeze 经存活的 `module_ptr` 独占写（同开头对
  // `stale_ptr` freeze 的证成），且 etv 的借用已随 visit 结束；
  // `fresh_scope_ptr` 指向本地 Arc
  // `fresh_child_of_nearest_scope` 的内容（存续至移入返回结果），写 parent 前
  // 无任何并存借用该 Scope 的可变引用（cpp:1313-1315 直译）。
  unsafe {
    freeze(&mut (*module_ptr).internal_types);
    freeze(&mut (*module_ptr).interface_types);
    (*fresh_scope_ptr).parent = Some(intern_scope(closest_scope));
  }

  // ScopedExit: erase the requireTrace entry for the incremental module.
  frontend.require_trace.remove(&module_name);

  FragmentTypeCheckResult {
    incremental_module: Some(incremental_module),
    fresh_scope: fresh_child_of_nearest_scope,
    ancestry: Vec::new(),
  }
}

fn empty_result() -> FragmentTypeCheckResult {
  let fresh_scope = Arc::new(Scope::scope_type_pack_id(null()));
  register_scope(&fresh_scope);
  FragmentTypeCheckResult {
    incremental_module: None,
    fresh_scope,
    ancestry: Vec::new(),
  }
}

pub fn typecheck_fragment(
  frontend: &mut Frontend,
  module_name: &ModuleName,
  cursor_pos: &Position,
  opts: Option<FrontendOptions>,
  src: &str,
  fragment_end_position: Option<Position>,
  recent_parse: *mut AstStatBlock,
  reporter: ReporterRef<'_>,
) -> (FragmentTypeCheckStatus, FragmentTypeCheckResult) {
  typecheck_fragment_impl(
    frontend,
    module_name,
    cursor_pos,
    opts,
    src,
    fragment_end_position,
    recent_parse,
    reporter,
  )
}

// 内部实现。`recent_parse` 为裸指针入参，契约与 cpp
// `typecheckFragment(..., AstStatBlock* recentParse, ...)` 一致——须为 null 或指向
// frontend 持有的最近一次成功解析的 AST 根；本函数内所有解引用均限定在紧邻
// Safety 注释标明的语句半径内，module 经 resolver 取回并由本地 Arc 保活。
fn typecheck_fragment_impl(
  frontend: &mut Frontend,
  module_name: &ModuleName,
  cursor_pos: &Position,
  opts: Option<FrontendOptions>,
  src: &str,
  fragment_end_position: Option<Position>,
  recent_parse: *mut AstStatBlock,
  reporter: ReporterRef<'_>,
) -> (FragmentTypeCheckStatus, FragmentTypeCheckResult) {
  LUAU_TIMETRACE_SCOPE!("Luau::typecheckFragment", "FragmentAutocomplete");

  if !frontend.all_module_dependencies_valid(
    module_name,
    opts.as_ref().map(|o| o.for_autocomplete).unwrap_or(false),
  ) {
    return (FragmentTypeCheckStatus::SkipAutocomplete, empty_result());
  }

  let module = {
    let resolver = get_module_resolver(frontend, opts.clone());
    resolver.get_module(module_name)
  };
  // C++: if (!module) LUAU_ASSERT(!"Expected Module for fragment typecheck"); get_module
  // panics on a missing module in this port, mirroring that assertion.

  let module_ptr = arc_as_mut(&module);
  // Safety: `module_ptr` 指向本地 Arc `module`（resolver 取回、存续至函数返回）
  // 的内容；`names` 经 `as_ref().expect(..)` 确认是 Some 后由 `Arc::as_ptr` 取
  // 得，指向该 Arc<AstNameTable> 内容并随其存活（const→mut 为既有 cpp
  // `module->names.get()` 直译）。
  let names: *mut AstNameTable = unsafe {
    Arc::as_ptr(
      (*module_ptr)
        .names
        .as_ref()
        .expect("module must have names"),
    )
    .cast_mut()
  };
  // Safety: `root` 字段是模块类型检查前写入的 AST arena 裸句柄，随 module/
  // frontend 存活；仅作为参数透传给 unsafe 的 parse_fragment，本行不创建引用。
  let stale_root = unsafe { (*module_ptr).root };

  // Safety: 满足 `parse_fragment` 契约——`stale_root` 指向存活 AstStatBlock（如上
  // 证成），`recent_parse` 由调用方按 cpp `typecheckFragment` 的 `recentParse`
  // 语义提供（null 时 callee 首句即早退、不解引用），`names` 指向上述存活的
  // AstNameTable；`src`/`cursor_pos` 为普通借用。
  let try_parse = unsafe {
    parse_fragment(
      stale_root,
      recent_parse,
      names,
      src,
      cursor_pos,
      fragment_end_position,
    )
  };

  let parse_result = match try_parse {
    Some(pr) => pr,
    None => return (FragmentTypeCheckStatus::SkipAutocomplete, empty_result()),
  };

  if is_within_comment(
    &parse_result.comment_locations,
    fragment_end_position.unwrap_or(*cursor_pos),
  ) {
    return (FragmentTypeCheckStatus::SkipAutocomplete, empty_result());
  }

  let frontend_options = opts.unwrap_or_else(|| frontend.options.clone());
  let closest_scope = find_closest_scope(&module, &parse_result.scope_pos);

  // 解构代替 clone：fragment_to_parse / ancestry / alloc 按值转移，避免多余堆分配
  let FragmentParseResult {
    fragment_to_parse,
    root,
    ancestry,
    alloc,
    ..
  } = parse_result;

  // Safety: 逐参满足 `typecheck_fragment_` 的 `# Safety` 契约——`root` 是上方
  // `parse_fragment` 成功结果的 fragment AST 根，`alloc`（解构出的所有权）正是
  // 分配它的 Allocator，随参数移交；`&module` 为其 Arc 本地强引用（存续至
  // 函数末尾），`closest_scope` 由 `find_closest_scope(&module, ..)` 取自同一
  // 模块 scope 树；`frontend`/`&frontend_options`/`reporter` 均为调用帧内存活
  // 借用（cpp:1357-1359 直译）。
  let mut result = unsafe {
    typecheck_fragment_(
      frontend,
      root,
      &module,
      &closest_scope,
      cursor_pos,
      alloc,
      &frontend_options,
      reporter,
    )
  };
  result.ancestry = ancestry;
  reporter.report_fragment_string(&fragment_to_parse);

  (FragmentTypeCheckStatus::Success, result)
}
