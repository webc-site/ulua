//! 动态 int 标志（`dfint` 模块），对应 `fint`。

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
  // Analysis/src/Simplify.cpp —— cpp 同名旗标仅定义未读取（DYNAMIC_FASTINTVARIABLE
  // 在:20、全文件无 FInt:: 消费点），镜像对账保留，勿按死代码清理
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

/// 宏自产的 Pascal 别名（宏展开内含 `pub use X as Y;`）经 glob 一次性再导出；
/// 别名对与宏定义同 token 生成，一致性由编译器而非人工清单保证。
pub use _inner::*;

/// C++ `FValue` ctor 的 `list = this` 自注册对应物：把本文件定义的全部
/// flag 挂入 per-type 注册表，供 `set_flag_by_name`/`set_all_unless` 按名遍历。
/// 仅本 crate 消费，降 `pub(crate)`（b28 零消费点收口）。
pub(crate) fn register_flags() {
  _inner::LUAU_TYPE_FAMILY_APPLICATION_CARTESIAN_PRODUCT_LIMIT.register();
  _inner::LUAU_TYPE_FAMILY_GRAPH_REDUCTION_MAXIMUM_STEPS.register();
  _inner::LUAU_TYPE_FUNCTION_SERDE_ITERATION_LIMIT.register();
  _inner::LUAU_CONSTRAINT_GENERATOR_RECURSION_LIMIT.register();
  _inner::LUAU_SIMPLIFICATION_COMPLEXITY_LIMIT.register();
  _inner::LUAU_STEP_REFINE_RECURSION_LIMIT.register();
  _inner::LUAU_SUBTYPING_RECURSION_LIMIT.register();
  _inner::LUAU_TYPE_FAMILY_USE_GUESSER_DEPTH.register();
  _inner::LUAU_TYPE_PATH_MAXIMUM_TRAVERSE_STEPS.register();
  _inner::LUAU_TYPE_SIMPLIFICATION_ITERATION_LIMIT.register();
  _inner::LUAU_UNIFIER_RECURSION_LIMIT.register();
}
