use alloc::vec::Vec;
use core::ptr::{NonNull, null_mut};

use ulua_common::{FInt, records::dense_hash_map::DenseHashMap};

use super::constraint_solver_constraint_solver_constraint_solver::SolverParams;
use crate::{
  records::{
    constraint::Constraint, constraint_graph::ConstraintGraph, constraint_set::ConstraintSet,
    constraint_solver::ConstraintSolver, data_flow_graph::DataFlowGraph, dcr_logger::DcrLogger,
    module_resolver::ModuleResolver, normalizer::Normalizer, require_cycle::RequireCycle,
    scope::Scope, subtyping::Subtyping, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, type_ids::TypeIds,
  },
  type_aliases::{module_ptr_module::ModulePtr, type_id::TypeId},
};
/// `ConstraintSolver` fragment 构造参数包，对应 C++ 旧版十三参数构造器
/// （ConstraintSolver.h:164-178，TODO CLI-169086 待淘汰）。按语义分组：
/// 基础设施 / fragment 约束 / 目标模块 / 依赖与限额。
pub(crate) struct FragmentSolverParams {
  // —— 基础设施（NotNull 语义的裸指针）——
  pub normalizer: *const Normalizer,
  pub type_function_runtime: *const TypeFunctionRuntime,
  // —— fragment 的约束与作用域映射 ——
  pub root_scope: NonNull<Scope>,
  pub constraints: Vec<NonNull<Constraint>>,
  pub scope_to_function: NonNull<DenseHashMap<*mut Scope, TypeId>>,
  // —— 目标模块 ——
  pub module: ModulePtr,
  pub module_resolver: *const ModuleResolver,
  // —— 依赖与限额 ——
  pub require_cycles: Vec<RequireCycle>,
  pub logger: *mut DcrLogger,
  pub dfg: *const DataFlowGraph,
  pub limits: TypeCheckLimits,
  pub cgraph: *mut ConstraintGraph,
  pub subtyping: *const Subtyping,
}
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证 `normalizer、`type_function_runtime、`module_resolver、`logger、`dfg、`cgraph、`subtyping` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
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
      scope_to_function: DenseHashMap::new(null_mut()),
      errors: Vec::new(),
    };
    constraint_set.constraints = constraints.iter().map(|c| c.as_ptr()).collect::<Vec<_>>();
    constraint_set.root_scope = root_scope.as_ptr();

    let mut result = unsafe {
      ConstraintSolver::constraint_solver_not_null_normalizer_not_null_type_function_runtime_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
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
        })
    };

    // The delegated base constructor already populated `result.constraints`
    // (from `constraint_set.constraints`, which holds the same pointers as the
    // `constraints` argument) and already ran `init_free_type_tracking()`.
    // Re-running it here would double-insert every constraint and trip the
    // `LUAU_ASSERT(fresh1)` in `deprecated_constraint_to_mutated_types`. We only
    // need to point the solver at the fragment generator's `scopeToFunction` and
    // `rootScope` (C++ `NotNull{&cg.scopeToFunction}`, `NotNull(cg.rootScope)`).
    result.scope_to_function = scope_to_function.as_ptr();
    result.root_scope = root_scope.as_ptr();
    result.solver_constraint_limit = FInt::LuauSolverConstraintLimit.get() as usize;
    result.module_resolver = module_resolver as *mut ModuleResolver;
    result.logger = logger;
    result.limits = limits;
    result.cgraph = cgraph;
    result.subtyping = subtyping as *mut Subtyping;

    result
  }
}
