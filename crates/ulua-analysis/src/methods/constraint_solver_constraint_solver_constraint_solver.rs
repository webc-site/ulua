use alloc::vec::Vec;
use core::ptr::{NonNull, null, null_mut};

use ulua_common::{
  fint,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  enums::subtyping_variance::SubtypingVariance,
  functions::borrow_constraints::borrow_constraints,
  records::{
    arena_handle::Handle, constraint::Constraint, constraint_graph::ConstraintGraph,
    constraint_set::ConstraintSet, constraint_solver::ConstraintSolver,
    data_flow_graph::DataFlowGraph, dcr_logger::DcrLogger,
    instantiation_signature::InstantiationSignature, module_resolver::ModuleResolverRef,
    normalizer::Normalizer, require_cycle::RequireCycle, scope::Scope,
    subtype_constraint_record::SubtypeConstraintRecord, subtyping::Subtyping,
    to_string_options::ToStringOptions, type_check_limits::TypeCheckLimits, type_fun::TypeFun,
    type_function_runtime::TypeFunctionRuntime, type_ids::TypeIds,
  },
  type_aliases::{collections::HashMap, module_ptr_module::ModulePtr, type_id::TypeId},
};

/// `ConstraintSolver` 主构造参数包，对应 C++ ConstraintSet 版十一参数构造器
/// （ConstraintSolver.h:149-161）。按语义分组：基础设施 / 目标模块 / 依赖与限额。
pub struct SolverParams {
  // —— 基础设施（NotNull 语义，句柄化；解析器按 cpp `ModuleResolver*` 以
  // [`ModuleResolverRef`] 具体分派句柄按值传入）——
  pub normalizer: Handle<Normalizer>,
  pub type_function_runtime: Handle<TypeFunctionRuntime>,
  // —— 目标模块 ——
  pub module: ModulePtr,
  pub module_resolver: ModuleResolverRef,
  // —— 依赖与限额 ——
  pub require_cycles: Vec<RequireCycle>,
  pub logger: Option<Handle<DcrLogger>>,
  pub dfg: Handle<DataFlowGraph>,
  pub limits: TypeCheckLimits,
  pub constraint_set: ConstraintSet,
  pub cgraph: *mut ConstraintGraph,
  pub subtyping: Handle<Subtyping>,
}
impl ConstraintSolver {
  /// 构造期不再解引用任何裸指针：`Handle` 字段仅存地址，`module_resolver`
  /// 本身即薄指针分派句柄，按值转存；解引用契约收敛在 `Handle` 模块与
  /// `cgraph`（可空 `ConstraintGraph*`，cpp 同语义）各自的类型级说明中。
  pub fn constraint_solver_not_null_normalizer_not_null_type_function_runtime_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
    params: SolverParams,
  ) -> Self {
    // 按原参数顺序解包，保持与 C++ 一一对应。
    let SolverParams {
      normalizer,
      type_function_runtime,
      module,
      module_resolver,
      require_cycles,
      logger,
      dfg,
      limits,
      constraint_set,
      cgraph,
      subtyping,
    } = params;
    let empty_instantiation_signature = InstantiationSignature {
      fn_sig: TypeFun {
        type_params: Vec::new(),
        type_pack_params: Vec::new(),
        r#type: null(),
        definition_location: None,
      },
      arguments: Vec::new(),
      pack_arguments: Vec::new(),
    };
    let empty_subtype_constraint = SubtypeConstraintRecord {
      sub_ty: null(),
      super_ty: null(),
      variance: SubtypingVariance::Invalid,
    };

    // `root_scope` 为 Copy 的裸 scope 句柄：先取出再入字面量直接初始化，
    // 免掉「先置空、构造后再补写」的冗余占位（C 型）。
    let root_scope = constraint_set.root_scope;

    let mut result = ConstraintSolver {
      // arena/builtin_types 沿用 normalizer 的句柄字段（cpp `normalizer->arena`
      // 直译），构造期只读拷出，normalizer 比本 solver 长寿；arena 的 None 属
      // 接线违例，expect 确定性 panic（与原 `Handle::from_ptr(null)` 等价）。
      arena: normalizer
        .get()
        .arena
        .expect("ConstraintSolver 构造要求 Normalizer.arena 已接线"),
      builtin_types: normalizer.get().builtin_types,
      // type_function_runtime 同为 Frontend 持有、比 solver 长寿的 NotNull 会话级
      // runtime；此处仅只读其 `ice` 内嵌字段并克隆句柄（单线程串行）。
      ice_reporter: type_function_runtime.get().ice.clone(),
      normalizer,
      type_function_runtime,
      constraint_set,
      constraints: Vec::new(),
      // 占位（非可空语义）：该指针指向下方 `constraint_set` 字段内部，自引用
      // 无法在字面量内求值，紧随的接线行即覆盖，其间无任何读取。
      scope_to_function: null_mut(),
      root_scope,
      module: Some(module),
      dfg: dfg.as_ptr(),
      solver_constraints: Vec::new(),
      solver_constraint_limit: fint::LuauSolverConstraintLimit.get() as usize,
      unsolved_constraints: Vec::new(),
      deprecated_blocked_constraints: HashMap::new(),
      deprecated_blocked: HashMap::new(),
      instantiated_aliases: DenseHashMap::new(empty_instantiation_signature),
      upper_bound_contributors: DenseHashMap::default(),
      deprecated_type_to_constraint_set: HashMap::new(),
      deprecated_constraint_to_mutated_types: DenseHashMap::default(),
      uninhabited_type_functions: DenseHashSet::default(),
      seen_constraints: DenseHashMap::new(empty_subtype_constraint),
      generalized_types_: DenseHashSet::default(),
      generalized_types: null(),
      errors: Vec::new(),
      module_resolver,
      require_cycles,
      // B 型（落点字段 `*mut DcrLogger`，records/constraint_solver.rs:71）：
      // 空指针 = 未开启 DCR 采集（SolverParams 以 `Option<Handle>` 表达命中与否，
      // 此处按字段形态折叠）；消费方 constraint_solver_run/block_constraint_solver
      // 判空后才 capture。
      logger: logger.map_or(null_mut(), |logger| logger.as_ptr()),
      limits: limits.clone(),
      type_functions_to_finalize: DenseHashMap::default(),
      opts: ToStringOptions {
        exhaustive: true,
        ..ToStringOptions::default()
      },
      cgraph,
      subtyping: subtyping.as_ptr(),
    };

    result.constraints = borrow_constraints(&result.constraint_set.constraints);
    result.scope_to_function = &mut result.constraint_set.scope_to_function as *mut _;
    result.generalized_types = &result.generalized_types_ as *const _;

    result.init_free_type_tracking();

    result
  }
}

/// `ConstraintSolver` fragment 构造参数包，对应 C++ 旧版十三参数构造器
/// （ConstraintSolver.h:164-178，TODO CLI-169086 待淘汰）。按语义分组：
/// 基础设施 / fragment 约束 / 目标模块 / 依赖与限额。
pub(crate) struct FragmentSolverParams {
  // —— 基础设施（NotNull 语义，句柄化；解析器按 cpp `ModuleResolver*` 以
  // [`ModuleResolverRef`] 具体分派句柄按值传入）——
  pub normalizer: Handle<Normalizer>,
  pub type_function_runtime: Handle<TypeFunctionRuntime>,
  // —— fragment 的约束与作用域映射 ——
  pub root_scope: NonNull<Scope>,
  pub constraints: Vec<NonNull<Constraint>>,
  pub scope_to_function: NonNull<DenseHashMap<*mut Scope, TypeId>>,
  // —— 目标模块 ——
  pub module: ModulePtr,
  pub module_resolver: ModuleResolverRef,
  // —— 依赖与限额 ——
  pub require_cycles: Vec<RequireCycle>,
  pub logger: Option<Handle<DcrLogger>>,
  pub dfg: Handle<DataFlowGraph>,
  pub limits: TypeCheckLimits,
  pub cgraph: *mut ConstraintGraph,
  pub subtyping: Handle<Subtyping>,
}
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证 `normalizer`、`type_function_runtime`、`logger`、`dfg`、
  /// `subtyping` 句柄目标与 `root_scope`/`constraints`/`scope_to_function`
  /// 所指对象有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn constraint_solver_not_null_normalizer_not_null_type_function_runtime_not_null_scope_vector_not_null_constraint_not_null_dense_hash_map_scope_type_id_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
    params: FragmentSolverParams,
  ) -> Self {
    // 按原参数顺序解包，保持与 C++ 一一对应。
    let FragmentSolverParams {
      normalizer,
      type_function_runtime,
      root_scope,
      constraints,
      scope_to_function,
      module,
      module_resolver,
      require_cycles,
      logger,
      dfg,
      limits,
      cgraph,
      subtyping,
    } = params;
    // C++ `constraintSet{rootScope}` aggregate-initializes only `rootScope`;
    // every other field is default-constructed.
    let mut constraint_set = ConstraintSet {
      root_scope: root_scope.as_ptr(),
      constraints: Vec::new(),
      free_types: TypeIds::new(),
      scope_to_function: DenseHashMap::default(),
      errors: Vec::new(),
    };
    constraint_set.constraints = constraints.iter().map(|c| c.as_ptr()).collect::<Vec<_>>();
    constraint_set.root_scope = root_scope.as_ptr();

    // 句柄与 `require_cycles`/`limits` 等字段原样转发给主构造器；主构造器现仅
    // 做 safe 的句柄转存与只读字段拷贝（Handle 解引用契约见 arena_handle
    // 模块头）。module_resolver 为 `Copy` 的 [`ModuleResolverRef`] 分派句柄，
    // 按值下传即与 cpp NotNull 共享别名同构。
    let mut result = ConstraintSolver::constraint_solver_not_null_normalizer_not_null_type_function_runtime_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
      SolverParams {
        normalizer,
        type_function_runtime,
        module,
        module_resolver,
        require_cycles,
        logger,
        dfg,
        limits: limits.clone(),
        constraint_set,
        cgraph,
        subtyping,
      },
    );

    // The delegated base constructor already populated `result.constraints`
    // (from `constraint_set.constraints`, which holds the same pointers as the
    // `constraints` argument) and already ran `init_free_type_tracking()`.
    // Re-running it here would double-insert every constraint and trip the
    // `LUAU_ASSERT(fresh1)` in `deprecated_constraint_to_mutated_types`. We only
    // need to point the solver at the fragment generator's `scopeToFunction` and
    // `rootScope` (C++ `NotNull{&cg.scopeToFunction}`, `NotNull(cg.rootScope)`).
    result.scope_to_function = scope_to_function.as_ptr();
    result.root_scope = root_scope.as_ptr();
    result.solver_constraint_limit = fint::LuauSolverConstraintLimit.get() as usize;
    // `module_resolver` 已由上方委托的主构造器按值转存（Copy 句柄），无需重写。
    // 折叠语义同主构造器处注释（B 型：可空 DcrLogger 字段，未开启即空）。
    result.logger = logger.map_or(null_mut(), |logger| logger.as_ptr());
    result.limits = limits;
    result.cgraph = cgraph;
    result.subtyping = subtyping.as_ptr();

    result
  }
}
