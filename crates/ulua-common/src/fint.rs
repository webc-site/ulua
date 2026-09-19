//! 静态 int FastFlags，对应 `fflag`。C++ 将所有
//! `LUAU_FASTINTVARIABLE(...)` 收进 `namespace FInt`；Rust 模块不可开放，
//! 故聚合于此。读取方式 `fint::Flag.get()`。

pub mod _inner {
  // CodeGen/src/CodeGen.cpp
  crate::LUAU_FASTINTVARIABLE!(
    CODEGEN_HEURISTICS_BLOCK_INSTRUCTION_LIMIT,
    CodegenHeuristicsBlockInstructionLimit,
    65_536
  );
  // CodeGen/src/CodeGen.cpp
  crate::LUAU_FASTINTVARIABLE!(
    CODEGEN_HEURISTICS_BLOCK_LIMIT,
    CodegenHeuristicsBlockLimit,
    32_768
  );
  // CodeGen/src/CodeGen.cpp
  crate::LUAU_FASTINTVARIABLE!(
    CODEGEN_HEURISTICS_INSTRUCTION_LIMIT,
    CodegenHeuristicsInstructionLimit,
    1_048_576
  );
  // CodeGen/src/CodeGenContext.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_CODE_GEN_BLOCK_SIZE,
    LuauCodeGenBlockSize,
    4 * 1024 * 1024
  );
  // CodeGen/src/CodeGenContext.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_CODE_GEN_MAX_TOTAL_SIZE,
    LuauCodeGenMaxTotalSize,
    256 * 1024 * 1024
  );
  // Analysis/src/Clone.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_TYPE_CLONE_ITERATION_LIMIT,
    LuauTypeCloneIterationLimit,
    100_000
  );
  // Analysis/src/ToString.cpp
  crate::LUAU_FASTINTVARIABLE!(DEBUG_LUAU_VERBOSE_TYPE_NAMES, DebugLuauVerboseTypeNames, 0);
  // Analysis/src/TypeInfer.cpp
  crate::LUAU_FASTINTVARIABLE!(LUAU_CHECK_RECURSION_LIMIT, LuauCheckRecursionLimit, 300);
  // CodeGen/src/OptimizeConstProp.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_CODE_GEN_LIVE_SLOT_REUSE_LIMIT,
    LuauCodeGenLiveSlotReuseLimit,
    8
  );
  // CodeGen/src/OptimizeConstProp.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_CODE_GEN_MIN_LINEAR_BLOCK_PATH,
    LuauCodeGenMinLinearBlockPath,
    3
  );
  // CodeGen/src/OptimizeConstProp.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_CODE_GEN_REUSE_SLOT_LIMIT,
    LuauCodeGenReuseSlotLimit,
    64
  );
  // CodeGen/src/OptimizeConstProp.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_CODE_GEN_REUSE_UDATA_TAG_LIMIT,
    LuauCodeGenReuseUdataTagLimit,
    64
  );
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTINTVARIABLE!(LUAU_COMPILE_INLINE_DEPTH, LuauCompileInlineDepth, 5);
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_COMPILE_INLINE_THRESHOLD,
    LuauCompileInlineThreshold,
    25
  );
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_COMPILE_INLINE_THRESHOLD_MAX_BOOST,
    LuauCompileInlineThresholdMaxBoost,
    300
  );
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_COMPILE_LOOP_UNROLL_THRESHOLD,
    LuauCompileLoopUnrollThreshold,
    25
  );
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_COMPILE_LOOP_UNROLL_THRESHOLD_MAX_BOOST,
    LuauCompileLoopUnrollThresholdMaxBoost,
    300
  );
  // Analysis/src/Generalization.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_GENERIC_COUNTER_MAX_DEPTH,
    LuauGenericCounterMaxDepth,
    15
  );
  // Analysis/src/Generalization.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_GENERIC_COUNTER_MAX_STEPS,
    LuauGenericCounterMaxSteps,
    1500
  );
  // Analysis/src/Error.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_INDENT_TYPE_MISMATCH_MAX_TYPE_LENGTH,
    LuauIndentTypeMismatchMaxTypeLength,
    10
  );
  // VM/src/lfunc.cpp:10（`LUAU_FASTINTVARIABLE(LuauInlineHitsThreshold, 32)`）
  crate::LUAU_FASTINTVARIABLE!(LUAU_INLINE_HITS_THRESHOLD, LuauInlineHitsThreshold, 32);
  // Analysis/src/NonStrictTypeChecker.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_NON_STRICT_TYPE_CHECKER_RECURSION_LIMIT,
    LuauNonStrictTypeCheckerRecursionLimit,
    300
  );
  // Analysis/src/Normalize.cpp
  crate::LUAU_FASTINTVARIABLE!(LUAU_NORMALIZE_CACHE_LIMIT, LuauNormalizeCacheLimit, 100000);
  // Analysis/src/Normalize.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_NORMALIZER_INITIAL_FUEL,
    LuauNormalizerInitialFuel,
    3000
  );
  // Ast/src/Parser.cpp
  crate::LUAU_FASTINTVARIABLE!(LUAU_PARSE_ERROR_LIMIT, LuauParseErrorLimit, 100);
  // Analysis/src/ConstraintGenerator.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_PRIMITIVE_INFERENCE_IN_TABLE_LIMIT,
    LuauPrimitiveInferenceInTableLimit,
    500
  );
  // Ast/src/Parser.cpp
  crate::LUAU_FASTINTVARIABLE!(LUAU_RECURSION_LIMIT, LuauRecursionLimit, 1000);
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_SOLVER_CONSTRAINT_LIMIT,
    LuauSolverConstraintLimit,
    1000
  );
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTINTVARIABLE!(LUAU_SOLVER_RECURSION_LIMIT, LuauSolverRecursionLimit, 500);
  // Analysis/src/NativeStackGuard.cpp
  crate::LUAU_FASTINTVARIABLE!(LUAU_STACK_GUARD_THRESHOLD, LuauStackGuardThreshold, 1024);
  // Analysis/src/Subtyping.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_SUBTYPING_ITERATION_LIMIT,
    LuauSubtypingIterationLimit,
    20000
  );
  // Analysis/src/Subtyping.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_SUBTYPING_REASONING_LIMIT,
    LuauSubtypingReasoningLimit,
    100
  );
  // Analysis/src/Linter.cpp
  crate::LUAU_FASTINTVARIABLE!(LUAU_SUGGESTION_DISTANCE, LuauSuggestionDistance, 4);
  // Analysis/src/Type.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_TABLE_TYPE_MAXIMUM_STRINGIFIER_LENGTH,
    LuauTableTypeMaximumStringifierLength,
    0
  );
  // Analysis/src/Substitution.cpp
  crate::LUAU_FASTINTVARIABLE!(LUAU_TARJAN_CHILD_LIMIT, LuauTarjanChildLimit, 10000);
  // Analysis/src/Substitution.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_TARJAN_PREALLOCATION_SIZE,
    LuauTarjanPreallocationSize,
    256
  );
  // Analysis/src/TypeInfer.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_TYPE_INFER_ITERATION_LIMIT,
    LuauTypeInferIterationLimit,
    20000
  );
  // Analysis/src/TypeInfer.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_TYPE_INFER_RECURSION_LIMIT,
    LuauTypeInferRecursionLimit,
    165
  );
  // Analysis/src/TypeInfer.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_TYPE_INFER_TYPE_PACK_LOOP_LIMIT,
    LuauTypeInferTypePackLoopLimit,
    5000
  );
  // Ast/src/Parser.cpp
  crate::LUAU_FASTINTVARIABLE!(LUAU_TYPE_LENGTH_LIMIT, LuauTypeLengthLimit, 1000);
  // Analysis/src/Type.cpp
  crate::LUAU_FASTINTVARIABLE!(
    LUAU_TYPE_MAXIMUM_STRINGIFIER_LENGTH,
    LuauTypeMaximumStringifierLength,
    500
  );
  // Analysis/src/TypeInfer.cpp
  crate::LUAU_FASTINTVARIABLE!(LUAU_VISIT_RECURSION_LIMIT, LuauVisitRecursionLimit, 500);
}
pub use _inner::{
  CODEGEN_HEURISTICS_BLOCK_INSTRUCTION_LIMIT as CodegenHeuristicsBlockInstructionLimit,
  CODEGEN_HEURISTICS_BLOCK_LIMIT as CodegenHeuristicsBlockLimit,
  CODEGEN_HEURISTICS_INSTRUCTION_LIMIT as CodegenHeuristicsInstructionLimit,
  DEBUG_LUAU_VERBOSE_TYPE_NAMES as DebugLuauVerboseTypeNames,
  LUAU_CHECK_RECURSION_LIMIT as LuauCheckRecursionLimit,
  LUAU_CODE_GEN_BLOCK_SIZE as LuauCodeGenBlockSize,
  LUAU_CODE_GEN_LIVE_SLOT_REUSE_LIMIT as LuauCodeGenLiveSlotReuseLimit,
  LUAU_CODE_GEN_MAX_TOTAL_SIZE as LuauCodeGenMaxTotalSize,
  LUAU_CODE_GEN_MIN_LINEAR_BLOCK_PATH as LuauCodeGenMinLinearBlockPath,
  LUAU_CODE_GEN_REUSE_SLOT_LIMIT as LuauCodeGenReuseSlotLimit,
  LUAU_CODE_GEN_REUSE_UDATA_TAG_LIMIT as LuauCodeGenReuseUdataTagLimit,
  LUAU_COMPILE_INLINE_DEPTH as LuauCompileInlineDepth,
  LUAU_COMPILE_INLINE_THRESHOLD as LuauCompileInlineThreshold,
  LUAU_COMPILE_INLINE_THRESHOLD_MAX_BOOST as LuauCompileInlineThresholdMaxBoost,
  LUAU_COMPILE_LOOP_UNROLL_THRESHOLD as LuauCompileLoopUnrollThreshold,
  LUAU_COMPILE_LOOP_UNROLL_THRESHOLD_MAX_BOOST as LuauCompileLoopUnrollThresholdMaxBoost,
  LUAU_GENERIC_COUNTER_MAX_DEPTH as LuauGenericCounterMaxDepth,
  LUAU_GENERIC_COUNTER_MAX_STEPS as LuauGenericCounterMaxSteps,
  LUAU_INDENT_TYPE_MISMATCH_MAX_TYPE_LENGTH as LuauIndentTypeMismatchMaxTypeLength,
  LUAU_INLINE_HITS_THRESHOLD as LuauInlineHitsThreshold,
  LUAU_NON_STRICT_TYPE_CHECKER_RECURSION_LIMIT as LuauNonStrictTypeCheckerRecursionLimit,
  LUAU_NORMALIZE_CACHE_LIMIT as LuauNormalizeCacheLimit,
  LUAU_NORMALIZER_INITIAL_FUEL as LuauNormalizerInitialFuel,
  LUAU_PARSE_ERROR_LIMIT as LuauParseErrorLimit,
  LUAU_PRIMITIVE_INFERENCE_IN_TABLE_LIMIT as LuauPrimitiveInferenceInTableLimit,
  LUAU_RECURSION_LIMIT as LuauRecursionLimit,
  LUAU_SOLVER_CONSTRAINT_LIMIT as LuauSolverConstraintLimit,
  LUAU_SOLVER_RECURSION_LIMIT as LuauSolverRecursionLimit,
  LUAU_STACK_GUARD_THRESHOLD as LuauStackGuardThreshold,
  LUAU_SUBTYPING_ITERATION_LIMIT as LuauSubtypingIterationLimit,
  LUAU_SUBTYPING_REASONING_LIMIT as LuauSubtypingReasoningLimit,
  LUAU_SUGGESTION_DISTANCE as LuauSuggestionDistance,
  LUAU_TABLE_TYPE_MAXIMUM_STRINGIFIER_LENGTH as LuauTableTypeMaximumStringifierLength,
  LUAU_TARJAN_CHILD_LIMIT as LuauTarjanChildLimit,
  LUAU_TARJAN_PREALLOCATION_SIZE as LuauTarjanPreallocationSize,
  LUAU_TYPE_CLONE_ITERATION_LIMIT as LuauTypeCloneIterationLimit,
  LUAU_TYPE_INFER_ITERATION_LIMIT as LuauTypeInferIterationLimit,
  LUAU_TYPE_INFER_RECURSION_LIMIT as LuauTypeInferRecursionLimit,
  LUAU_TYPE_INFER_TYPE_PACK_LOOP_LIMIT as LuauTypeInferTypePackLoopLimit,
  LUAU_TYPE_LENGTH_LIMIT as LuauTypeLengthLimit,
  LUAU_TYPE_MAXIMUM_STRINGIFIER_LENGTH as LuauTypeMaximumStringifierLength,
  LUAU_VISIT_RECURSION_LIMIT as LuauVisitRecursionLimit, *,
};

/// C++ `FValue` ctor 的 `list = this` 自注册对应物：把本文件定义的全部
/// flag 挂入 per-type 链表，供 `set_flag_by_name`/`set_all_unless` 按名遍历。
///
/// # Safety
/// 每个 flag 仅注册一次（由 `ensure_flags_registered` 的 `OnceLock` 串行化），
/// 且早于任何并发链表遍历；`set_version` 写入此后仅经 `version()` 读取的槽位。
pub fn register_flags() {
  use crate::fint;
  unsafe {
    fint::CODEGEN_HEURISTICS_BLOCK_INSTRUCTION_LIMIT.register();
    fint::CODEGEN_HEURISTICS_BLOCK_LIMIT.register();
    fint::CODEGEN_HEURISTICS_INSTRUCTION_LIMIT.register();
    fint::LUAU_CODE_GEN_BLOCK_SIZE.register();
    fint::LUAU_CODE_GEN_MAX_TOTAL_SIZE.register();
    fint::LUAU_TYPE_CLONE_ITERATION_LIMIT.register();
    fint::DEBUG_LUAU_VERBOSE_TYPE_NAMES.register();
    fint::LUAU_CHECK_RECURSION_LIMIT.register();
    fint::LUAU_CODE_GEN_LIVE_SLOT_REUSE_LIMIT.register();
    fint::LUAU_CODE_GEN_MIN_LINEAR_BLOCK_PATH.register();
    fint::LUAU_CODE_GEN_REUSE_SLOT_LIMIT.register();
    fint::LUAU_CODE_GEN_REUSE_UDATA_TAG_LIMIT.register();
    fint::LUAU_COMPILE_INLINE_DEPTH.register();
    fint::LUAU_COMPILE_INLINE_THRESHOLD.register();
    fint::LUAU_COMPILE_INLINE_THRESHOLD_MAX_BOOST.register();
    fint::LUAU_COMPILE_LOOP_UNROLL_THRESHOLD.register();
    fint::LUAU_COMPILE_LOOP_UNROLL_THRESHOLD_MAX_BOOST.register();
    fint::LUAU_GENERIC_COUNTER_MAX_DEPTH.register();
    fint::LUAU_GENERIC_COUNTER_MAX_STEPS.register();
    fint::LUAU_INDENT_TYPE_MISMATCH_MAX_TYPE_LENGTH.register();
    fint::LUAU_INLINE_HITS_THRESHOLD.register();
    fint::LUAU_NON_STRICT_TYPE_CHECKER_RECURSION_LIMIT.register();
    fint::LUAU_NORMALIZE_CACHE_LIMIT.register();
    fint::LUAU_NORMALIZER_INITIAL_FUEL.register();
    fint::LUAU_PARSE_ERROR_LIMIT.register();
    fint::LUAU_PRIMITIVE_INFERENCE_IN_TABLE_LIMIT.register();
    fint::LUAU_RECURSION_LIMIT.register();
    fint::LUAU_SOLVER_CONSTRAINT_LIMIT.register();
    fint::LUAU_SOLVER_RECURSION_LIMIT.register();
    fint::LUAU_STACK_GUARD_THRESHOLD.register();
    fint::LUAU_SUBTYPING_ITERATION_LIMIT.register();
    fint::LUAU_SUBTYPING_REASONING_LIMIT.register();
    fint::LUAU_SUGGESTION_DISTANCE.register();
    fint::LUAU_TABLE_TYPE_MAXIMUM_STRINGIFIER_LENGTH.register();
    fint::LUAU_TARJAN_CHILD_LIMIT.register();
    fint::LUAU_TARJAN_PREALLOCATION_SIZE.register();
    fint::LUAU_TYPE_INFER_ITERATION_LIMIT.register();
    fint::LUAU_TYPE_INFER_RECURSION_LIMIT.register();
    fint::LUAU_TYPE_INFER_TYPE_PACK_LOOP_LIMIT.register();
    fint::LUAU_TYPE_LENGTH_LIMIT.register();
    fint::LUAU_TYPE_MAXIMUM_STRINGIFIER_LENGTH.register();
    fint::LUAU_VISIT_RECURSION_LIMIT.register();
  }
}
