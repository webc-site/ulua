//! 四张 FastFlag 表（`fflag` / `fint` / `dfint` / `dfflag`）与上游 cpp
//! `LUAU_*VARIABLE` 声明的对齐测试，一条清单覆盖全表。
//!
//! 为什么钉这个：flag 默认值本身就是移植语义的一部分。例
//! `LuauInlineHitsThreshold` 上游是 32（`VM/src/lfunc.cpp:10`），移植表里曾写成
//! 3，则第 3 次调用命中即触发内联，与上游行为静默分岔。
//!
//! 三段检查：
//! 1. [`port_reads_declared_default`]：`FValue::get()` 读到的值 == 下方清单内联
//!    的默认值。改 `fint.rs` 等表的初值而不同步清单，这里先红。
//! 2. [`declared_defaults_match_cpp`]：清单内联值与 dynamic 位 == cpp
//!    `LUAU_*VARIABLE` 声明。上游 sync 改了默认值，这里红。cpp 目录在
//!    `.gitignore` 内（上游快照另行同步），故 cpp 根按 `LUAU_CPP_ROOT` 环境变量
//!    →自 `CARGO_MANIFEST_DIR` 逐级上溯找 `cpp/Common/include/Luau/Common.h`
//!    的顺序定位；都找不到时本段跳过并打印说明，另两段不受影响。
//! 3. [`listing_covers_module_tables`]：清单 == `*.rs` 里的宏定义 ==
//!    `register_flags()` 清单，三者逐项相同。Pascal 别名对由宏展开自产
//!    （与定义同 token，一致性由编译器保证），聚合模块仅需
//!    `pub use _inner::*;` 门面，存在性一并查验。新增 flag 漏登记 /
//!    漏别名门面，这里红。

use std::{
  collections::{BTreeMap, BTreeSet},
  env,
  ffi::OsStr,
  fs,
  path::{Path, PathBuf},
};

use ulua_common::{dfflag, dfint, fflag, fint};

/// 四个 flag 模块（同时是 `src/<module>.rs` 的文件名）。
const MODULES: [&str; 4] = ["fflag", "fint", "dfint", "dfflag"];

/// 一条 flag 的对照信息：表名、Rust 常量名、cpp 原名、dynamic 位、默认值。
struct FlagEntry {
  module: &'static str,
  rust: &'static str,
  cpp: &'static str,
  dynamic: bool,
  /// 清单内联的移植默认值（bool 记作 0/1）。
  expected: i64,
  /// 运行时从 `FValue` 读到的当前值。
  value: i64,
}

/// `dfint`/`dfflag` 对应 cpp 的 `LUAU_DYNAMIC_*`，`dynamic` 位为 true。
fn is_dynamic_module(module: &str) -> bool {
  matches!(module, "dfint" | "dfflag")
}

macro_rules! rust_flags {
  ($( $module:ident :: $rust:ident => $cpp:ident $expected:expr , )*) => {
    vec![
      $(
        FlagEntry {
          module: stringify!($module),
          rust: stringify!($rust),
          cpp: stringify!($cpp),
          dynamic: is_dynamic_module(stringify!($module)),
          expected: ($expected) as i64,
          value: ($module::_inner::$rust).get() as i64,
        },
      )*
    ]
  };
}

/// 全表清单（186 条）。生成自四个模块的宏定义，顺序与源文件一致。
fn rust_table() -> Vec<FlagEntry> {
  rust_flags! {
    // ---- fflag.rs ----
    fflag::DEBUG_CODEGEN_CHAOS_A64 => DebugCodegenChaosA64 false,
    fflag::DEBUG_CODEGEN_OPT_SIZE => DebugCodegenOptSize false,
    fflag::DEBUG_LUAU_ABORTING_CHECKS => DebugLuauAbortingChecks false,
    fflag::DEBUG_LUAU_ALWAYS_SHOW_CONSTRAINT_SOLVING_INCOMPLETE => DebugLuauAlwaysShowConstraintSolvingIncomplete false,
    fflag::DEBUG_LUAU_ASSERT_ON_FORCED_CONSTRAINT => DebugLuauAssertOnForcedConstraint false,
    fflag::DEBUG_LUAU_CHECK_NORMALIZE_INVARIANT => DebugLuauCheckNormalizeInvariant false,
    fflag::DEBUG_LUAU_DUMP_CFGJSON => DebugLuauDumpCFGJson false,
    fflag::DEBUG_LUAU_FORBID_INTERNAL_TYPES => DebugLuauForbidInternalTypes false,
    fflag::DEBUG_LUAU_FORCE_ALL_NEW_SOLVER_TESTS => DebugLuauForceAllNewSolverTests false,
    fflag::DEBUG_LUAU_FORCE_ALL_OLD_SOLVER_TESTS => DebugLuauForceAllOldSolverTests false,
    fflag::DEBUG_LUAU_FORCE_NON_STRICT_MODE => DebugLuauForceNonStrictMode false,
    fflag::DEBUG_LUAU_FORCE_OLD_SOLVER => DebugLuauForceOldSolver false,
    fflag::DEBUG_LUAU_FORCE_STRICT_MODE => DebugLuauForceStrictMode false,
    fflag::DEBUG_LUAU_FREEZE_ARENA => DebugLuauFreezeArena false,
    fflag::DEBUG_LUAU_LOG_BINDINGS => DebugLuauLogBindings false,
    fflag::DEBUG_LUAU_LOG_CFG => DebugLuauLogCFG false,
    fflag::DEBUG_LUAU_LOG_SOLVER => DebugLuauLogSolver false,
    fflag::DEBUG_LUAU_LOG_SOLVER_TO_JSON => DebugLuauLogSolverToJson false,
    fflag::DEBUG_LUAU_LOG_SOLVER_TO_JSON_FILE => DebugLuauLogSolverToJsonFile false,
    fflag::DEBUG_LUAU_MAGIC_TYPES => DebugLuauMagicTypes false,
    fflag::DEBUG_LUAU_MAGIC_VARIABLE_NAMES => DebugLuauMagicVariableNames false,
    fflag::DEBUG_LUAU_NO_INLINE => DebugLuauNoInline false,
    fflag::DEBUG_LUAU_SUBTYPING_CHECK_PATH_VALIDITY => DebugLuauSubtypingCheckPathValidity false,
    fflag::DEBUG_LUAU_TIME_TRACING => DebugLuauTimeTracing false,
    fflag::DEBUG_LUAU_TO_STRING_NO_LEXICAL_SORT => DebugLuauToStringNoLexicalSort false,
    fflag::DEBUG_LUAU_USER_DEFINED_CLASSES => DebugLuauUserDefinedClasses false,
    fflag::DEBUG_LUAU_USER_DEFINED_CLASSES_RUNTIME => DebugLuauUserDefinedClassesRuntime false,
    fflag::DEBUG_LUAU_WARN_ON_UNANNOTATED_TOP_LEVEL_FUNCTIONS => DebugLuauWarnOnUnannotatedTopLevelFunctions false,
    fflag::FIX_MATH_NOISE_PRECISION => FixMathNoisePrecision false,
    fflag::LUAU_ADD_RECURSION_COUNTER_TO_NON_STRICT_TYPE_CHECKER => LuauAddRecursionCounterToNonStrictTypeChecker false,
    fflag::LUAU_ALLOW_GLOBAL_DECLARATION_TO_BE_CALLED_CLASS => LuauAllowGlobalDeclarationToBeCalledClass false,
    fflag::LUAU_ALSO_INSTANTIATE_INFERRED_ARGUMENTS => LuauAlsoInstantiateInferredArguments false,
    fflag::LUAU_AUTOCOMPLETE_CONST => LuauAutocompleteConst false,
    fflag::LUAU_AUTOCOMPLETE_EXPORT => LuauAutocompleteExport false,
    fflag::LUAU_AUTOCOMPLETE_STRING_SINGLETON_INTERSECTION => LuauAutocompleteStringSingletonIntersection false,
    fflag::LUAU_BIDIRECTIONAL_INFERENCE_BETTER_UNION_HANDLING => LuauBidirectionalInferenceBetterUnionHandling false,
    fflag::LUAU_BACKEDGE_HEAP_CHECK => LuauBackedgeHeapCheck false,
    fflag::LUAU_CALL_FEEDBACK => LuauCallFeedback false,
    fflag::LUAU_BETTER_METATABLE_STRINGIFICATION => LuauBetterMetatableStringification false,
    fflag::LUAU_CHECK_FUNCTION_STATEMENT_TYPES => LuauCheckFunctionStatementTypes false,
    fflag::LUAU_CODEGEN_BUFFER_INTEGER => LuauCodegenBufferInteger false,
    fflag::LUAU_CODEGEN_DSE_PTR_STORE_TAG_CHECK => LuauCodegenDsePtrStoreTagCheck false,
    fflag::LUAU_CODEGEN_DSE_RESTORE_HINTS => LuauCodegenDseRestoreHints false,
    fflag::LUAU_CODEGEN_FIX_BUFFER_LEN_CHECK => LuauCodegenFixBufferLenCheck false,
    fflag::LUAU_CODEGEN_INTEGER3 => LuauCodegenInteger3 false,
    fflag::LUAU_CODEGEN_INTEGER_COMPARE => LuauCodegenIntegerCompare false,
    fflag::LUAU_CODEGEN_LOAD_PROPAGATE_ORIGIN => LuauCodegenLoadPropagateOrigin false,
    fflag::LUAU_CODEGEN_PROTECT_DATA => LuauCodegenProtectData false,
    fflag::LUAU_CODEGEN_SUGGEST_ARGUMENT_REGISTER_X64 => LuauCodegenSuggestArgumentRegisterX64 false,
    fflag::LUAU_CODEGEN_VM_EXIT_SYNC_MULTI_USE => LuauCodegenVmExitSyncMultiUse false,
    fflag::LUAU_CODEGEN_DSE_RESTORE_HINT_UPDATE => LuauCodegenDseRestoreHintUpdate false,
    fflag::LUAU_COMPILE_DUPTABLE_CONSTANT_PACK2 => LuauCompileDuptableConstantPack2 false,
    fflag::LUAU_COMPILE_FASTCALL3_COST_MODEL => LuauCompileFastcall3CostModel false,
    fflag::LUAU_COMPILE_INLINE_TABLE_FUNCTIONS => LuauCompileInlineTableFunctions false,
    fflag::LUAU_COMPILE_NO_FOLD_VECTOR_EQ_W => LuauCompileNoFoldVectorEqW false,
    fflag::LUAU_COMPILE_CONCAT_TARGET_TOP => LuauCompileConcatTargetTop false,
    fflag::LUAU_COMPILE_STRING_INTERP_TARGET_TOP => LuauCompileStringInterpTargetTop false,
    fflag::LUAU_COMPILE_TYPE_ALIASES => LuauCompileTypeAliases false,
    fflag::LUAU_COMPILE_RECURSIVE_ALIASES => LuauCompileRecursiveAliases false,
    fflag::LUAU_COMPILE_EXPAND_LIMIT => LuauCompileExpandLimit false,
    fflag::LUAU_VIRTUAL_BC_BUILDER => LuauVirtualBcBuilder false,
    fflag::LUAU_BYTECODE_COST_MODEL => LuauBytecodeCostModel false,
    fflag::LUAU_COMPILE_EMIT_VECTOR_DOUBLE => LuauCompileEmitVectorDouble false,
    fflag::LUAU_COMPILE_FASTPCALL => LuauCompileFastpcall false,
    fflag::LUAU_CONCAT_DOESNT_ALWAYS_RETURN_STRING => LuauConcatDoesntAlwaysReturnString false,
    fflag::LUAU_CONSTRAINT_GRAPH => LuauConstraintGraph false,
    fflag::LUAU_CYCLIC_REQUIRE_SHORT_CIRCUIT => LuauCyclicRequireShortCircuit false,
    fflag::LUAU_CST_EXPR_GROUP => LuauCstExprGroup false,
    fflag::LUAU_CST_TYPE_GROUP => LuauCstTypeGroup false,
    fflag::LUAU_DIRECT_FIELD_GET => LuauDirectFieldGet false,
    fflag::LUAU_DISALLOW_REDEFINING_BUILTIN_TYPES => LuauDisallowRedefiningBuiltinTypes false,
    fflag::LUAU_DO_NOT_LEAK_GENERICS_IN_INDEXER => LuauDoNotLeakGenericsInIndexer false,
    fflag::LUAU_EMIT_CALL_FEEDBACK => LuauEmitCallFeedback false,
    fflag::LUAU_ERROR_TOLERANT_PRETTY_PRINTING => LuauErrorTolerantPrettyPrinting false,
    fflag::LUAU_EXPLICIT_TYPE_INSTANTIATION_SUPPORT => LuauExplicitTypeInstantiationSupport false,
    fflag::LUAU_EXPORT_VALUE_SYNTAX => LuauExportValueSyntax false,
    fflag::LUAU_EXPORT_VALUE_TYPECHECK => LuauExportValueTypecheck false,
    fflag::LUAU_FRONTEND_SOURCE_NODE_ERASE => LuauFrontendSourceNodeErase false,
    fflag::LUAU_EXTERN_TYPES_NORMALIZE_WITH_SHAPES => LuauExternTypesNormalizeWithShapes false,
    fflag::LUAU_FIX_INDEXER_SUBTYPING_ORDERING => LuauFixIndexerSubtypingOrdering false,
    fflag::LUAU_FIX_PROP_READS_ON_METATABLE_TYPES => LuauFixPropReadsOnMetatableTypes false,
    fflag::LUAU_FORCE_LESS => LuauForceLess false,
    fflag::LUAU_INSTANTIATE_FUNCTION_TYPE_BEFORE_PUSH => LuauInstantiateFunctionTypeBeforePush false,
    fflag::LUAU_INSTANTIATE_IN_SUBTYPING => LuauInstantiateInSubtyping false,
    fflag::LUAU_INSTANTIATION_USES_POLARITY => LuauInstantiationUsesPolarity false,
    fflag::LUAU_INTEGER_BUFFER_FASTCALLS => LuauIntegerBufferFastcalls false,
    fflag::LUAU_INTEGER_FASTCALLS => LuauIntegerFastcalls false,
    fflag::LUAU_INTEGER_LIBRARY => LuauIntegerLibrary false,
    fflag::LUAU_INTEGER_TYPE2 => LuauIntegerType2 false,
    fflag::LUAU_ITERATIVE_INSTANTIATION_QUEUER => LuauIterativeInstantiationQueuer false,
    fflag::LUAU_LVALUE_COMPOUND_ASSIGNMENT_VISIT_LHS => LuauLValueCompoundAssignmentVisitLhs false,
    fflag::LUAU_LIMIT_UNIFICATION_RECURSION => LuauLimitUnificationRecursion false,
    fflag::LUAU_NATIVE_CODE_TARGET_CHECK => LuauNativeCodeTargetCheck false,
    fflag::LUAU_NON_STRICT_MODE_USE_ERROR_SUPRESSING_TAG => LuauNonStrictModeUseErrorSupressingTag false,
    fflag::LUAU_OCCURS_CHECK_FOR_ALL_BINDINGS => LuauOccursCheckForAllBindings false,
    fflag::LUAU_PROPAGATE_FREE_TYPES_INTO_UNION_AND_INTERSECTION_BOUNDS => LuauPropagateFreeTypesIntoUnionAndIntersectionBounds false,
    fflag::LUAU_PROPAGATE_TYPE_ANNOTATIONS_IN_FOR_IN_LOOPS => LuauPropagateTypeAnnotationsInForInLoops false,
    fflag::LUAU_PROPERTY_MODIFIER_MISMATCH_ERRORS => LuauPropertyModifierMismatchErrors false,
    fflag::LUAU_READ_ONLY_INDEXERS => LuauReadOnlyIndexers false,
    fflag::LUAU_REFINE_NIL_FROM_TABLE_INDEXER_RESULT_TYPE => LuauRefineNilFromTableIndexerResultType false,
    fflag::LUAU_RELAX_CONSTRAINT_ORDERING_FOR_FUNCTION_CHECK => LuauRelaxConstraintOrderingForFunctionCheck false,
    fflag::LUAU_REMOVE_CONSTRAINT_SOLVER_EMPLACE => LuauRemoveConstraintSolverEmplace false,
    fflag::LUAU_REPLACER_IS_SOLVER_AGNOSTIC => LuauReplacerIsSolverAgnostic false,
    fflag::LUAU_RESUME_RESTORE_CCALLS => LuauResumeRestoreCcalls false,
    fflag::LUAU_SILENCE_DYNAMIC_FORMAT_STRING_ERRORS => LuauSilenceDynamicFormatStringErrors false,
    fflag::LUAU_SINGLE_TYPE_OPTIONAL_PACK_RETURNS_ATTRIBUTE_PARENS => LuauSingleTypeOptionalPackReturnsAttributeParens false,
    fflag::LUAU_SOLVER_V2 => LuauSolverV2 false,
    fflag::LUAU_SUBTYPING_MISSING_PROPERTIES_AS_NIL => LuauSubtypingMissingPropertiesAsNil false,
    fflag::LUAU_SUBTYPING_TABLES_HAS_BETTER_ERROR_SUPPRESSION => LuauSubtypingTablesHasBetterErrorSuppression false,
    fflag::LUAU_TABLE_ENTRIES_DONT_NEED_TO_MATCH_INDENT => LuauTableEntriesDontNeedToMatchIndent false,
    fflag::LUAU_TABLE_FREEZE_CHECK_IS_SUBTYPE => LuauTableFreezeCheckIsSubtype false,
    fflag::LUAU_TIDY_TYPE_PROTOTYPING => LuauTidyTypePrototyping false,
    fflag::LUAU_TWEAK_ACCESS_VIOLATION_REPORTING => LuauTweakAccessViolationReporting false,
    fflag::LUAU_TYPE_FUNCTION_ROBUSTNESS => LuauTypeFunctionRobustness false,
    fflag::LUAU_TYPE_FUNCTION_SERIALIZE_ARG_NAMES => LuauTypeFunctionSerializeArgNames false,
    fflag::LUAU_TYPE_FUNCTION_STRUCTURED_ERRORS => LuauTypeFunctionStructuredErrors false,
    fflag::LUAU_TYPE_FUNCTION_SUPPORTS_FROZEN => LuauTypeFunctionSupportsFrozen false,
    fflag::LUAU_UDATA_DIRECT_ACCESS6 => LuauUdataDirectAccess6 false,
    fflag::LUAU_UDTF_TYPE_IS_SUBTYPE_OF => LuauUdtfTypeIsSubtypeOf false,
    fflag::LUAU_UDTF_CREATE_SINGLETON_FIX_ERROR_MESSAGE => LuauUdtfCreateSingletonFixErrorMessage false,
    fflag::LUAU_UDTF_FIX_TYPE_NAME_TYPO => LuauUdtfFixTypeNameTypo false,
    fflag::LUAU_USE_NATIVE_STACK_GUARD => LuauUseNativeStackGuard false,
    fflag::LUAU_VISIT_CALL_TYPE_ARGS_IN_DFG => LuauVisitCallTypeArgsInDfg false,
    fflag::LUAU_XPCALL_FIX_MESSAGE_YIELD_PATH => LuauXpcallFixMessageYieldPath false,
    fflag::LUAU_YIELD_ITER2 => LuauYieldIter2 false,
    fflag::LUAU_COST_MODEL => LuauCostModel false,
    fflag::LUAU_CODEGEN_SKIP_DEAD_PREDECESSOR_TAGS => LuauCodegenSkipDeadPredecessorTags false,
    fflag::LUAU_CODEGEN_SUBSTITUTE_REPLACEMENTS => LuauCodegenSubstituteReplacements false,
    fflag::LUAU_CODEGEN_PROPAGATE_FALLBACK_TAGS => LuauCodegenPropagateFallbackTags false,
    fflag::LUAU_ENUM_MORE_EDGES => LuauEnumMoreEdges false,
    // ---- fint.rs ----
    fint::CODEGEN_HEURISTICS_BLOCK_INSTRUCTION_LIMIT => CodegenHeuristicsBlockInstructionLimit 65536,
    fint::CODEGEN_HEURISTICS_BLOCK_LIMIT => CodegenHeuristicsBlockLimit 32768,
    fint::CODEGEN_HEURISTICS_INSTRUCTION_LIMIT => CodegenHeuristicsInstructionLimit 1048576,
    fint::LUAU_CODE_GEN_BLOCK_SIZE => LuauCodeGenBlockSize 4194304,
    fint::LUAU_CODE_GEN_MAX_TOTAL_SIZE => LuauCodeGenMaxTotalSize 268435456,
    fint::LUAU_TYPE_CLONE_ITERATION_LIMIT => LuauTypeCloneIterationLimit 100000,
    fint::DEBUG_LUAU_VERBOSE_TYPE_NAMES => DebugLuauVerboseTypeNames 0,
    fint::LUAU_CHECK_RECURSION_LIMIT => LuauCheckRecursionLimit 300,
    fint::LUAU_CODE_GEN_LIVE_SLOT_REUSE_LIMIT => LuauCodeGenLiveSlotReuseLimit 8,
    fint::LUAU_CODE_GEN_MIN_LINEAR_BLOCK_PATH => LuauCodeGenMinLinearBlockPath 3,
    fint::LUAU_CODE_GEN_REUSE_SLOT_LIMIT => LuauCodeGenReuseSlotLimit 64,
    fint::LUAU_CODE_GEN_REUSE_UDATA_TAG_LIMIT => LuauCodeGenReuseUdataTagLimit 64,
    fint::LUAU_COMPILE_INLINE_DEPTH => LuauCompileInlineDepth 5,
    fint::LUAU_COMPILE_INLINE_THRESHOLD => LuauCompileInlineThreshold 25,
    fint::LUAU_COMPILE_INLINE_THRESHOLD_MAX_BOOST => LuauCompileInlineThresholdMaxBoost 300,
    fint::LUAU_COMPILE_LOOP_UNROLL_THRESHOLD => LuauCompileLoopUnrollThreshold 25,
    fint::LUAU_COMPILE_LOOP_UNROLL_THRESHOLD_MAX_BOOST => LuauCompileLoopUnrollThresholdMaxBoost 300,
    fint::LUAU_GENERIC_COUNTER_MAX_DEPTH => LuauGenericCounterMaxDepth 15,
    fint::LUAU_GENERIC_COUNTER_MAX_STEPS => LuauGenericCounterMaxSteps 1500,
    fint::LUAU_INDENT_TYPE_MISMATCH_MAX_TYPE_LENGTH => LuauIndentTypeMismatchMaxTypeLength 10,
    fint::LUAU_INLINE_HITS_THRESHOLD => LuauInlineHitsThreshold 32,
    fint::LUAU_NON_STRICT_TYPE_CHECKER_RECURSION_LIMIT => LuauNonStrictTypeCheckerRecursionLimit 300,
    fint::LUAU_NORMALIZE_CACHE_LIMIT => LuauNormalizeCacheLimit 100000,
    fint::LUAU_NORMALIZER_INITIAL_FUEL => LuauNormalizerInitialFuel 3000,
    fint::LUAU_PARSE_ERROR_LIMIT => LuauParseErrorLimit 100,
    fint::LUAU_PRIMITIVE_INFERENCE_IN_TABLE_LIMIT => LuauPrimitiveInferenceInTableLimit 500,
    fint::LUAU_RECURSION_LIMIT => LuauRecursionLimit 1000,
    fint::LUAU_SOLVER_CONSTRAINT_LIMIT => LuauSolverConstraintLimit 1000,
    fint::LUAU_SOLVER_RECURSION_LIMIT => LuauSolverRecursionLimit 500,
    fint::LUAU_STACK_GUARD_THRESHOLD => LuauStackGuardThreshold 1024,
    fint::LUAU_SUBTYPING_ITERATION_LIMIT => LuauSubtypingIterationLimit 20000,
    fint::LUAU_SUBTYPING_REASONING_LIMIT => LuauSubtypingReasoningLimit 100,
    fint::LUAU_SUGGESTION_DISTANCE => LuauSuggestionDistance 4,
    fint::LUAU_TABLE_TYPE_MAXIMUM_STRINGIFIER_LENGTH => LuauTableTypeMaximumStringifierLength 0,
    fint::LUAU_TARJAN_CHILD_LIMIT => LuauTarjanChildLimit 10000,
    fint::LUAU_TARJAN_PREALLOCATION_SIZE => LuauTarjanPreallocationSize 256,
    fint::LUAU_TYPE_INFER_ITERATION_LIMIT => LuauTypeInferIterationLimit 20000,
    fint::LUAU_TYPE_INFER_RECURSION_LIMIT => LuauTypeInferRecursionLimit 165,
    fint::LUAU_TYPE_INFER_TYPE_PACK_LOOP_LIMIT => LuauTypeInferTypePackLoopLimit 5000,
    fint::LUAU_TYPE_LENGTH_LIMIT => LuauTypeLengthLimit 1000,
    fint::LUAU_TYPE_MAXIMUM_STRINGIFIER_LENGTH => LuauTypeMaximumStringifierLength 500,
    fint::LUAU_VISIT_RECURSION_LIMIT => LuauVisitRecursionLimit 500,
    // ---- dfint.rs ----
    dfint::LUAU_TYPE_FAMILY_APPLICATION_CARTESIAN_PRODUCT_LIMIT => LuauTypeFamilyApplicationCartesianProductLimit 5000,
    dfint::LUAU_TYPE_FAMILY_GRAPH_REDUCTION_MAXIMUM_STEPS => LuauTypeFamilyGraphReductionMaximumSteps 1000000,
    dfint::LUAU_TYPE_FUNCTION_SERDE_ITERATION_LIMIT => LuauTypeFunctionSerdeIterationLimit 100000,
    dfint::LUAU_CONSTRAINT_GENERATOR_RECURSION_LIMIT => LuauConstraintGeneratorRecursionLimit 300,
    dfint::LUAU_SIMPLIFICATION_COMPLEXITY_LIMIT => LuauSimplificationComplexityLimit 8,
    dfint::LUAU_STEP_REFINE_RECURSION_LIMIT => LuauStepRefineRecursionLimit 64,
    dfint::LUAU_SUBTYPING_RECURSION_LIMIT => LuauSubtypingRecursionLimit 100,
    dfint::LUAU_TYPE_FAMILY_USE_GUESSER_DEPTH => LuauTypeFamilyUseGuesserDepth -1,
    dfint::LUAU_TYPE_PATH_MAXIMUM_TRAVERSE_STEPS => LuauTypePathMaximumTraverseSteps 100,
    dfint::LUAU_TYPE_SIMPLIFICATION_ITERATION_LIMIT => LuauTypeSimplificationIterationLimit 128,
    dfint::LUAU_UNIFIER_RECURSION_LIMIT => LuauUnifierRecursionLimit 100,
    // ---- dfflag.rs ----
    dfflag::ADD_RETURN_EXECTARGET_CHECK => AddReturnExectargetCheck false,
    dfflag::DEBUG_LUAU_REPORT_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX => DebugLuauReportReturnTypeVariadicWithTypeSuffix false,
    dfflag::LUAU_SELF_IS_SELF_AND_ALWAYS_SELF => LuauSelfIsSelfAndAlwaysSelf false,
  }
}

#[test]
fn port_reads_declared_default() {
  let drift: Vec<String> = rust_table()
    .iter()
    .filter(|f| f.value != f.expected)
    .map(|f| {
      format!(
        "{}::{} (`{}`) 读到 {}，清单声明 {}",
        f.module, f.rust, f.cpp, f.value, f.expected
      )
    })
    .collect();
  assert!(
    drift.is_empty(),
    "运行期默认值与清单不一致（{} 项）:\n{}",
    drift.len(),
    drift.join("\n")
  );
}

/// cpp 侧 `LUAU_*VARIABLE` 的一条定义。
struct CppDef {
  /// 默认值原文（`false` / `1'048'576` / `4 * 1024 * 1024` 等）。
  default: String,
  dynamic: bool,
  /// `相对路径:行号`，失败信息里定位用。
  at: String,
}

/// 一条 `LUAU_*VARIABLE` 声明的解析结果（Rust 源与 cpp 共用）。
struct Declaration {
  /// 顶层逗号切分并 trim 后的实参。
  args: Vec<String>,
  dynamic: bool,
  /// 声明在文本中的字符偏移，用于报错时算行号。
  offset: usize,
}

/// cpp `Common/include/Luau/Common.h:134-172` 的四个 *定义* 宏；
/// `LUAU_FASTFLAG` / `LUAU_FASTINT` / `LUAU_DYNAMIC_FAST*` 是 `extern` 声明，
/// 不带默认值，故不采。
const FLAG_MACROS: [(&str, bool); 4] = [
  ("LUAU_FASTFLAGVARIABLE", false),
  ("LUAU_FASTINTVARIABLE", false),
  ("LUAU_DYNAMIC_FASTFLAGVARIABLE", true),
  ("LUAU_DYNAMIC_FASTINTVARIABLE", true),
];

/// 采出 `text` 里全部 `LUAU_*VARIABLE(!)(...)` 声明；`rust_style` 为宏名后带
/// `!` 的 Rust 源形态。`#define` 行（`Common.h` 里的宏本体）跳过。
fn flag_definitions(text: &str, rust_style: bool) -> Vec<Declaration> {
  let mut out = Vec::new();
  for (macro_name, dynamic) in FLAG_MACROS {
    let needle = if rust_style {
      format!("{macro_name}!(")
    } else {
      format!("{macro_name}(")
    };
    for (offset, _) in text.match_indices(&needle) {
      let line_start = text[..offset].rfind('\n').map_or(0, |i| i + 1);
      if text[line_start..offset].trim_start().starts_with("#define") {
        continue;
      }
      let open = offset + needle.len();
      let inner = paren_contents(text, open);
      out.push(Declaration {
        args: split_top_level(inner)
          .into_iter()
          .map(str::to_owned)
          .collect(),
        dynamic,
        offset,
      });
    }
  }
  out
}

/// `open` 指向左括号之后，返回括号内的原文（不含外层括号）。
fn paren_contents(text: &str, open: usize) -> &str {
  let bytes = text.as_bytes();
  let mut depth = 1usize;
  let mut i = open;
  while i < bytes.len() {
    match bytes[i] {
      b'(' => depth += 1,
      b')' => {
        depth -= 1;
        if depth == 0 {
          break;
        }
      }
      _ => {}
    }
    i += 1;
  }
  &text[open..i]
}

/// 按顶层（括号深度 0）逗号切分。
fn split_top_level(args: &str) -> Vec<&str> {
  let mut out = Vec::new();
  let mut depth = 0i32;
  let mut start = 0usize;
  for (i, ch) in args.char_indices() {
    match ch {
      '(' => depth += 1,
      ')' => depth -= 1,
      ',' if depth == 0 => {
        out.push(args[start..i].trim());
        start = i + 1;
      }
      _ => {}
    }
  }
  let tail = args[start..].trim();
  if !tail.is_empty() {
    out.push(tail);
  }
  out
}

/// 剥掉 `//` 与块注释（注释掉的 flag 声明不算数，如上游偶尔留档的
/// `// LUAU_FASTFLAG(...)`）；注释内的换行保留，使行号与原文对齐。
fn strip_comments(src: &str) -> String {
  let chars: Vec<char> = src.chars().collect();
  let mut out = String::with_capacity(src.len());
  let mut i = 0usize;
  while i < chars.len() {
    let ch = chars[i];
    let next = chars.get(i + 1).copied();
    if ch == '/' && next == Some('/') {
      while i < chars.len() && chars[i] != '\n' {
        i += 1;
      }
      continue;
    }
    if ch == '/' && next == Some('*') {
      i += 2;
      while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
        if chars[i] == '\n' {
          out.push('\n');
        }
        i += 1;
      }
      i += 2;
      continue;
    }
    if ch == '"' || ch == '\'' {
      out.push(ch);
      i += 1;
      while i < chars.len() && chars[i] != ch {
        if chars[i] == '\\' && i + 1 < chars.len() {
          out.push(chars[i]);
          i += 1;
        }
        out.push(chars[i]);
        i += 1;
      }
      if i < chars.len() {
        out.push(chars[i]);
        i += 1;
      }
      continue;
    }
    out.push(ch);
    i += 1;
  }
  out
}

/// `offset` 处的行号（1 基）。
fn line_of(text: &str, offset: usize) -> usize {
  text[..offset].bytes().filter(|b| *b == b'\n').count() + 1
}

/// cpp 默认值字面量求值：`false`/`true`、十进制（可带 `'` 或 `_` 位分隔）、
/// 以及 `4 * 1024 * 1024` 形的乘积。无法解析返回 `None`。
fn parse_default(text: &str) -> Option<i64> {
  match text.trim() {
    "false" => return Some(0),
    "true" => return Some(1),
    _ => {}
  }
  let mut acc: i64 = 1;
  for factor in text.split('*') {
    let token = factor.trim().replace(['\'', '_'], "");
    acc = acc.checked_mul(token.parse::<i64>().ok()?)?;
  }
  Some(acc)
}

fn manifest_dir() -> PathBuf {
  let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  if manifest.join("src").is_dir() {
    return manifest;
  }
  if let Ok(cwd) = env::current_dir() {
    if cwd.join("crates/ulua-common/src").is_dir() {
      return cwd.join("crates/ulua-common");
    }
    if cwd.join("src").is_dir() {
      return cwd;
    }
  }
  manifest
}

/// cpp 根目录：`LUAU_CPP_ROOT` 优先，否则自本 crate 目录逐级上溯找
/// `cpp/Common/include/Luau/Common.h`（主仓与工作树都能命中主仓的 `cpp/`）。
fn cpp_root() -> Option<PathBuf> {
  const MARKER: &str = "Common/include/Luau/Common.h";
  if let Some(from_env) = env::var_os("LUAU_CPP_ROOT") {
    let candidate = PathBuf::from(from_env);
    if candidate.join(MARKER).is_file() {
      return Some(candidate);
    }
  }
  let mut dir = manifest_dir();
  loop {
    let candidate = dir.join("cpp");
    if candidate.join(MARKER).is_file() {
      return Some(candidate);
    }
    if !dir.pop() {
      break;
    }
  }
  if let Ok(cwd) = env::current_dir() {
    let mut dir = cwd;
    loop {
      let candidate = dir.join("cpp");
      if candidate.join(MARKER).is_file() {
        return Some(candidate);
      }
      if !dir.pop() {
        break;
      }
    }
  }
  None
}

/// 递归采 `*.cpp`/`*.h`/`*.inl`（上游 flag 定义只可能在这三类文件里）。
fn collect_sources(dir: &Path, out: &mut Vec<PathBuf>) {
  let Ok(entries) = fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    let file_type = match entry.file_type() {
      Ok(file_type) => file_type,
      Err(_) => continue,
    };
    if file_type.is_dir() {
      collect_sources(&path, out);
    } else if matches!(
      path.extension().and_then(OsStr::to_str),
      Some("cpp" | "h" | "inl")
    ) {
      out.push(path);
    }
  }
}

/// 扫描 cpp 全部 `LUAU_*VARIABLE` 定义（同名取字典序首个；上游仅
/// `Common.h` 的宏本体重复）。
fn scan_cpp_definitions(root: &Path) -> BTreeMap<String, CppDef> {
  let mut files = Vec::new();
  collect_sources(root, &mut files);
  files.sort();

  let mut out: BTreeMap<String, CppDef> = BTreeMap::new();
  for file in &files {
    let Ok(raw) = fs::read_to_string(file) else {
      continue; // 非 UTF-8 的上游文件不含 flag 定义
    };
    // 粗筛四个定义宏名：`LUAU_DYNAMIC_FAST*` 的原文不含 `LUAU_FAST` 子串，
    // 只按后者过滤会漏掉纯 DFFlag 定义文件（如 Require/src/RequireNavigator.cpp）
    if !FLAG_MACROS
      .iter()
      .any(|(macro_name, _)| raw.contains(macro_name))
    {
      continue;
    }
    let text = strip_comments(&raw);
    let rel = file.strip_prefix(root).unwrap_or(file.as_path());
    for def in flag_definitions(&text, false) {
      let (name, default) = match def.args.as_slice() {
        [name] => (name.clone(), String::from("false")),
        [name, default] => (name.clone(), default.clone()),
        other => panic!(
          "{}:{} 实参形态异常: {other:?}",
          rel.display(),
          line_of(&text, def.offset)
        ),
      };
      if out.contains_key(&name) {
        continue;
      }
      out.insert(
        name,
        CppDef {
          default,
          dynamic: def.dynamic,
          at: format!("{}:{}", rel.display(), line_of(&text, def.offset)),
        },
      );
    }
  }
  out
}

#[test]
fn declared_defaults_match_cpp() {
  let Some(root) = cpp_root() else {
    eprintln!(
      "skip declared_defaults_match_cpp: 未找到上游 cpp 快照（设 LUAU_CPP_ROOT，\
       或同步仓库根的 cpp/ 后本用例即生效）"
    );
    return;
  };
  let defs = scan_cpp_definitions(&root);
  // 采空 = 目录或扫描逻辑有问题，必须报错而不是让下面的比对空转通过。
  assert!(
    !defs.is_empty(),
    "在 {} 下没采到任何 LUAU_*VARIABLE 定义，扫描逻辑或 cpp 快照有问题",
    root.display()
  );
  // 反向健全性：清单里至少有 100 条能在 cpp 里对上号。
  let matched = rust_table()
    .iter()
    .filter(|f| defs.contains_key(f.cpp))
    .count();
  assert!(
    matched > 100,
    "仅 {matched} 条 flag 在 cpp 找到定义，扫描结果可疑"
  );

  let mut drift = Vec::new();
  for f in rust_table() {
    let Some(def) = defs.get(f.cpp) else {
      // 上游已删/改名的 flag 只可能是 `fflag`：`LUAU_FASTFLAGVARIABLE(flag)` 的
      // 默认值由宏体固定为 false，没有可对的业务数值。其余三表的 flag 必须
      // 能在 cpp 里找到定义，否则就是名字漂了。
      if f.module != "fflag" {
        drift.push(format!(
          "{}::{} (`{}`): cpp 无 LUAU_*VARIABLE 定义",
          f.module, f.rust, f.cpp
        ));
      }
      continue;
    };
    if def.dynamic != f.dynamic {
      drift.push(format!(
        "{}: dynamic 位不一致，cpp {} = {}，移植 {} = {}",
        f.cpp, def.at, def.dynamic, f.module, f.dynamic
      ));
    }
    match parse_default(&def.default) {
      Some(value) if value == f.expected => {}
      Some(value) => drift.push(format!(
        "{}: cpp 默认值 {}（{}）≠ 移植 {}（{}::{}）",
        f.cpp, value, def.at, f.expected, f.module, f.rust
      )),
      None => drift.push(format!(
        "{}: cpp 默认值 {:?} 无法解析（{}）",
        f.cpp, def.default, def.at
      )),
    }
  }
  assert!(
    drift.is_empty(),
    "与上游 cpp 默认值不一致（{} 项）:\n{}",
    drift.len(),
    drift.join("\n")
  );
}

/// 读 `src/<module>.rs` 原文。
fn read_module_source(module: &str) -> String {
  let path = manifest_dir().join("src").join(format!("{module}.rs"));
  fs::read_to_string(&path).unwrap_or_else(|err| panic!("读 {} 失败: {err}", path.display()))
}

/// 源文件里的宏定义 → `(Rust 常量名, cpp 原名)`。
fn source_definitions(text: &str, module: &str) -> BTreeSet<(String, String)> {
  let stripped = strip_comments(text);
  flag_definitions(&stripped, true)
    .iter()
    .map(|def| match def.args.as_slice() {
      [name] => (name.clone(), name.clone()),
      [rust, cpp] | [rust, cpp, _] => (rust.clone(), cpp.clone()),
      other => panic!("{module}.rs 宏实参形态异常: {other:?}"),
    })
    .collect()
}

/// `register_flags()` 里 `.register()` 的常量名集合。
fn registered_idents(text: &str, module: &str) -> BTreeSet<String> {
  let marker = "fn register_flags()";
  let start = text
    .find(marker)
    .unwrap_or_else(|| panic!("{module}.rs 缺 {marker}"));
  let mut out = BTreeSet::new();
  // 只扫描真实代码：注释行里出现的 `.register()` 字样（例如 `// Safety:` 前置条件论证）
  // 不是注册调用；同时容忍 rustfmt 把长链式调用折行（`.register()` 落到下一行）。
  let code = text[start..]
    .lines()
    .filter(|l| !l.trim_start().starts_with("//"))
    .collect::<Vec<_>>()
    .join("\n");
  let mut cursor = code.as_str();
  while let Some(pos) = cursor.find(".register(") {
    let ident: String = cursor[..pos]
      .trim_end()
      .chars()
      .rev()
      .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
      .collect();
    out.insert(ident.chars().rev().collect());
    cursor = &cursor[pos + ".register(".len()..];
  }
  out
}

/// 聚合模块的 glob 别名门面（别名对由 `LUAU_*VARIABLE!` 宏展开自产，
/// 与宏定义同 token 生成，一致性由编译器保证，无需再对清单）。
const ALIAS_FACADE: &str = "pub use _inner::*;";

fn alias_facade_present(text: &str, module: &str) {
  assert!(
    text.contains(ALIAS_FACADE),
    "{module}.rs 缺 {ALIAS_FACADE} 别名门面（`fflag::PascalName` 读取路径将断）"
  );
}

#[test]
fn listing_covers_module_tables() {
  let table = rust_table();
  let unique: BTreeSet<&str> = table.iter().map(|f| f.cpp).collect();
  assert_eq!(unique.len(), table.len(), "本用例清单内 cpp 名重复");

  for module in MODULES {
    let text = read_module_source(module);
    let source = source_definitions(&text, module);
    let listing: BTreeSet<(String, String)> = table
      .iter()
      .filter(|f| f.module == module)
      .map(|f| (f.rust.to_owned(), f.cpp.to_owned()))
      .collect();
    assert_eq!(
      listing, source,
      "{module}.rs: 本用例清单与该文件的宏定义不一致"
    );

    let registered = registered_idents(&text, module);
    let names: BTreeSet<String> = source.iter().map(|(rust, _)| rust.clone()).collect();
    assert_eq!(
      registered, names,
      "{module}.rs: register_flags() 清单与宏定义不一致（漏注册则 `--fflags=` 按名遍历看不到该 flag）"
    );

    alias_facade_present(&text, module);
  }
}
