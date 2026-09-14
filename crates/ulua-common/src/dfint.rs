//! 动态 int 标志（`DFInt::`），对应 `FInt`。

pub mod _inner {
  // Analysis/src/TypeFunction.cpp
  crate::LUAU_DYNAMIC_FASTINTVARIABLE!(
    LUAU_TYPE_FAMILY_APPLICATION_CARTESIAN_PRODUCT_LIMIT,
    LuauTypeFamilyApplicationCartesianProductLimit,
    5_000
  );
  // Analysis/src/TypeFunction.cpp
  crate::LUAU_DYNAMIC_FASTINTVARIABLE!(
    LUAU_TYPE_FAMILY_GRAPH_REDUCTION_MAXIMUM_STEPS,
    LuauTypeFamilyGraphReductionMaximumSteps,
    1_000_000
  );
  // Analysis/src/TypeFunctionRuntimeBuilder.cpp
  crate::LUAU_DYNAMIC_FASTINTVARIABLE!(
    LUAU_TYPE_FUNCTION_SERDE_ITERATION_LIMIT,
    LuauTypeFunctionSerdeIterationLimit,
    100_000
  );
  // Analysis/src/ConstraintGenerator.cpp
  crate::LUAU_DYNAMIC_FASTINTVARIABLE!(
    LUAU_CONSTRAINT_GENERATOR_RECURSION_LIMIT,
    LuauConstraintGeneratorRecursionLimit,
    300
  );
  // Analysis/src/Simplify.cpp
  crate::LUAU_DYNAMIC_FASTINTVARIABLE!(
    LUAU_SIMPLIFICATION_COMPLEXITY_LIMIT,
    LuauSimplificationComplexityLimit,
    8
  );
  // Analysis/src/BuiltinTypeFunctions.cpp
  crate::LUAU_DYNAMIC_FASTINTVARIABLE!(
    LUAU_STEP_REFINE_RECURSION_LIMIT,
    LuauStepRefineRecursionLimit,
    64
  );
  // Analysis/src/Subtyping.cpp
  crate::LUAU_DYNAMIC_FASTINTVARIABLE!(
    LUAU_SUBTYPING_RECURSION_LIMIT,
    LuauSubtypingRecursionLimit,
    100
  );
  // Analysis/src/TypeFunction.cpp
  crate::LUAU_DYNAMIC_FASTINTVARIABLE!(
    LUAU_TYPE_FAMILY_USE_GUESSER_DEPTH,
    LuauTypeFamilyUseGuesserDepth,
    -1
  );
  // Analysis/src/TypePath.cpp
  crate::LUAU_DYNAMIC_FASTINTVARIABLE!(
    LUAU_TYPE_PATH_MAXIMUM_TRAVERSE_STEPS,
    LuauTypePathMaximumTraverseSteps,
    100
  );
  // Analysis/src/Simplify.cpp
  crate::LUAU_DYNAMIC_FASTINTVARIABLE!(
    LUAU_TYPE_SIMPLIFICATION_ITERATION_LIMIT,
    LuauTypeSimplificationIterationLimit,
    128
  );
  // Analysis/src/Unifier2.cpp
  crate::LUAU_DYNAMIC_FASTINTVARIABLE!(
    LUAU_UNIFIER_RECURSION_LIMIT,
    LuauUnifierRecursionLimit,
    100
  );
}
pub use _inner::{
  LUAU_CONSTRAINT_GENERATOR_RECURSION_LIMIT as LuauConstraintGeneratorRecursionLimit,
  LUAU_SIMPLIFICATION_COMPLEXITY_LIMIT as LuauSimplificationComplexityLimit,
  LUAU_STEP_REFINE_RECURSION_LIMIT as LuauStepRefineRecursionLimit,
  LUAU_SUBTYPING_RECURSION_LIMIT as LuauSubtypingRecursionLimit,
  LUAU_TYPE_FAMILY_APPLICATION_CARTESIAN_PRODUCT_LIMIT as LuauTypeFamilyApplicationCartesianProductLimit,
  LUAU_TYPE_FAMILY_GRAPH_REDUCTION_MAXIMUM_STEPS as LuauTypeFamilyGraphReductionMaximumSteps,
  LUAU_TYPE_FAMILY_USE_GUESSER_DEPTH as LuauTypeFamilyUseGuesserDepth,
  LUAU_TYPE_FUNCTION_SERDE_ITERATION_LIMIT as LuauTypeFunctionSerdeIterationLimit,
  LUAU_TYPE_PATH_MAXIMUM_TRAVERSE_STEPS as LuauTypePathMaximumTraverseSteps,
  LUAU_TYPE_SIMPLIFICATION_ITERATION_LIMIT as LuauTypeSimplificationIterationLimit,
  LUAU_UNIFIER_RECURSION_LIMIT as LuauUnifierRecursionLimit, *,
};

/// C++ `FValue` ctor 的 `list = this` 自注册对应物：把本文件定义的全部
/// flag 挂入 per-type 链表，供 `set_flag_by_name`/`set_all_unless` 按名遍历。
///
/// # Safety
/// 每个 flag 仅注册一次（由 `ensure_flags_registered` 的 `OnceLock` 串行化），
/// 且早于任何并发链表遍历；`set_version` 写入此后仅经 `version()` 读取的槽位。
pub fn register_flags() {
  use crate::DFInt;
  unsafe {
    DFInt::LUAU_TYPE_FAMILY_APPLICATION_CARTESIAN_PRODUCT_LIMIT.register();
    DFInt::LUAU_TYPE_FAMILY_GRAPH_REDUCTION_MAXIMUM_STEPS.register();
    DFInt::LUAU_TYPE_FUNCTION_SERDE_ITERATION_LIMIT.register();
    DFInt::LUAU_CONSTRAINT_GENERATOR_RECURSION_LIMIT.register();
    DFInt::LUAU_SIMPLIFICATION_COMPLEXITY_LIMIT.register();
    DFInt::LUAU_STEP_REFINE_RECURSION_LIMIT.register();
    DFInt::LUAU_SUBTYPING_RECURSION_LIMIT.register();
    DFInt::LUAU_TYPE_FAMILY_USE_GUESSER_DEPTH.register();
    DFInt::LUAU_TYPE_PATH_MAXIMUM_TRAVERSE_STEPS.register();
    DFInt::LUAU_TYPE_SIMPLIFICATION_ITERATION_LIMIT.register();
    DFInt::LUAU_UNIFIER_RECURSION_LIMIT.register();
  }
}
