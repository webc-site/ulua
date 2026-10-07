//! 动态 int 标志（`dfint` 模块），对应 `fint`。

use crate::macros::fast_flags::luau_flag_module;

luau_flag_module! {
  // Analysis/src/TypeFunction.cpp
  LUAU_DYNAMIC_FASTINTVARIABLE!(LUAU_TYPE_FAMILY_APPLICATION_CARTESIAN_PRODUCT_LIMIT, LuauTypeFamilyApplicationCartesianProductLimit, 5_000);
  // Analysis/src/TypeFunction.cpp
  LUAU_DYNAMIC_FASTINTVARIABLE!(LUAU_TYPE_FAMILY_GRAPH_REDUCTION_MAXIMUM_STEPS, LuauTypeFamilyGraphReductionMaximumSteps, 1_000_000);
  // Analysis/src/TypeFunctionRuntimeBuilder.cpp
  LUAU_DYNAMIC_FASTINTVARIABLE!(LUAU_TYPE_FUNCTION_SERDE_ITERATION_LIMIT, LuauTypeFunctionSerdeIterationLimit, 100_000);
  // Analysis/src/ConstraintGenerator.cpp
  LUAU_DYNAMIC_FASTINTVARIABLE!(LUAU_CONSTRAINT_GENERATOR_RECURSION_LIMIT, LuauConstraintGeneratorRecursionLimit, 300);
  // Analysis/src/Simplify.cpp
  LUAU_DYNAMIC_FASTINTVARIABLE!(LUAU_SIMPLIFICATION_COMPLEXITY_LIMIT, LuauSimplificationComplexityLimit, 8);
  // Analysis/src/BuiltinTypeFunctions.cpp
  LUAU_DYNAMIC_FASTINTVARIABLE!(LUAU_STEP_REFINE_RECURSION_LIMIT, LuauStepRefineRecursionLimit, 64);
  // Analysis/src/Subtyping.cpp
  LUAU_DYNAMIC_FASTINTVARIABLE!(LUAU_SUBTYPING_RECURSION_LIMIT, LuauSubtypingRecursionLimit, 100);
  // Analysis/src/TypeFunction.cpp
  LUAU_DYNAMIC_FASTINTVARIABLE!(LUAU_TYPE_FAMILY_USE_GUESSER_DEPTH, LuauTypeFamilyUseGuesserDepth, -1);
  // Analysis/src/TypePath.cpp
  LUAU_DYNAMIC_FASTINTVARIABLE!(LUAU_TYPE_PATH_MAXIMUM_TRAVERSE_STEPS, LuauTypePathMaximumTraverseSteps, 100);
  // Analysis/src/Simplify.cpp —— cpp 同名旗标仅定义未读取（DYNAMIC_FASTINTVARIABLE
  // 在:20、全文件无 FInt:: 消费点），镜像对账保留，勿按死代码清理
  LUAU_DYNAMIC_FASTINTVARIABLE!(LUAU_TYPE_SIMPLIFICATION_ITERATION_LIMIT, LuauTypeSimplificationIterationLimit, 128);
  // Analysis/src/Unifier2.cpp
  LUAU_DYNAMIC_FASTINTVARIABLE!(LUAU_UNIFIER_RECURSION_LIMIT, LuauUnifierRecursionLimit, 100);
}
