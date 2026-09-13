use alloc::vec::Vec;
use core::ptr::{null, null_mut};
use std::collections::HashMap;

use ulua_common::{
  FInt,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  enums::subtyping_variance::SubtypingVariance,
  functions::borrow_constraints::borrow_constraints,
  records::{
    constraint_graph::ConstraintGraph, constraint_set::ConstraintSet,
    constraint_solver::ConstraintSolver, data_flow_graph::DataFlowGraph, dcr_logger::DcrLogger,
    instantiation_signature::InstantiationSignature, module_resolver::ModuleResolver,
    normalizer::Normalizer, require_cycle::RequireCycle,
    subtype_constraint_record::SubtypeConstraintRecord, subtyping::Subtyping,
    to_string_options::ToStringOptions, type_check_limits::TypeCheckLimits, type_fun::TypeFun,
    type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::module_ptr_module::ModulePtr,
};
/// `ConstraintSolver` 主构造参数包，对应 C++ ConstraintSet 版十一参数构造器
/// （ConstraintSolver.h:149-161）。按语义分组：基础设施 / 目标模块 / 依赖与限额。
pub struct SolverParams {
  // —— 基础设施（NotNull 语义的裸指针）——
  pub normalizer: *const Normalizer,
  pub type_function_runtime: *const TypeFunctionRuntime,
  // —— 目标模块 ——
  pub module: ModulePtr,
  pub module_resolver: *const ModuleResolver,
  // —— 依赖与限额 ——
  pub require_cycles: Vec<RequireCycle>,
  pub logger: *mut DcrLogger,
  pub dfg: *const DataFlowGraph,
  pub limits: TypeCheckLimits,
  pub constraint_set: ConstraintSet,
  pub cgraph: *mut ConstraintGraph,
  pub subtyping: *const Subtyping,
}
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn constraint_solver_not_null_normalizer_not_null_type_function_runtime_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
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

    let mut result = ConstraintSolver {
      arena: unsafe { (*normalizer).arena },
      builtin_types: unsafe { (*normalizer).builtin_types },
      ice_reporter: unsafe { (*type_function_runtime).ice.clone() },
      normalizer: normalizer as *mut Normalizer,
      type_function_runtime: type_function_runtime as *mut TypeFunctionRuntime,
      constraint_set,
      constraints: Vec::new(),
      scope_to_function: null_mut(),
      root_scope: null_mut(),
      module: Some(module),
      dfg,
      solver_constraints: Vec::new(),
      solver_constraint_limit: FInt::LuauSolverConstraintLimit.get() as usize,
      unsolved_constraints: Vec::new(),
      deprecated_blocked_constraints: HashMap::new(),
      deprecated_blocked: HashMap::new(),
      instantiated_aliases: DenseHashMap::new(empty_instantiation_signature),
      upper_bound_contributors: DenseHashMap::new(null()),
      deprecated_type_to_constraint_set: HashMap::new(),
      deprecated_constraint_to_mutated_types: DenseHashMap::new(null()),
      uninhabited_type_functions: DenseHashSet::new(null()),
      seen_constraints: DenseHashMap::new(empty_subtype_constraint),
      generalized_types_: DenseHashSet::new(null()),
      generalized_types: null(),
      errors: Vec::new(),
      module_resolver: module_resolver as *mut ModuleResolver,
      require_cycles,
      logger,
      limits: limits.clone(),
      type_functions_to_finalize: DenseHashMap::new(null()),
      opts: ToStringOptions {
        exhaustive: true,
        ..ToStringOptions::default()
      },
      cgraph,
      subtyping: subtyping as *mut Subtyping,
    };

    result.constraints = borrow_constraints(&result.constraint_set.constraints);
    result.scope_to_function = &mut result.constraint_set.scope_to_function as *mut _;
    result.root_scope = result.constraint_set.root_scope;
    result.generalized_types = &result.generalized_types_ as *const _;

    result.init_free_type_tracking();

    result
  }
}
