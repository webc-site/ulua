//! FastFlag 命名空间 `fflag` 模块 —— 静态（非动态）bool 标志。
//! 全 crate 的 `LUAU_FASTFLAGVARIABLE(...)` 定义集中于此，
//! C++ 的 `FFlag::Name` 对应 `crate::fflag::Name.get()`。
//! Rust 模块不像 C++ 命名空间可开放，故按 crate 聚合 ——
//! 见 `crate::macros::luau_fastflagvariable`。
//!
//! ## 保留裁定与可达性判据（audit-deadcode B1 仲裁落档）
//!
//! 默认 `false` 的旗标**不可**按「默认 false ⇒ 死臂」删除。可达性判据是
//! 三面运行时可达，**任一面可达即保留**：
//! 1. rt 建 VM 批量点亮：`set_luau_bool_flags(true)`（定义于
//!    `records/f_value.rs`，`crates/ulua-rt/src/state.rs` 等入口 call_once 触发），
//!    谓词 [`crate::functions::is_default_enabled_flag`] 只认 `Luau*` 前缀且排除实验性
//!    旗标——`Debug*` 前缀**不会**被 rt 点亮，也不存在其他批量误点 `Debug*` 的
//!    路径（CLI 裸 `--fflags=true` 的排除谓词同样跳过非 `Luau*`，见
//!    `ulua-cli-lib/src/functions/set_luau_flags_flags.rs`）。
//! 2. CLI `--fflags=Name[=bool]` 按名点亮（含 `Debug*`），消费全量注册表。
//! 3. CLI 专用开关（如 `--timetrace` 点亮 `DebugLuauTimeTracing`）。
//!
//! 已拆除的例外仅两族（先例见 git log `r7-dc-flag4`）：
//! `DebugLuauForce{Non,}StrictMode` 对与 `ForceAll{New,Old}SolverTests` 守卫族
//! ——三面皆零可达路径的时代遗留标记。再遇「默认 false 疑似死臂」候选，
//! 先按上述三面逐一核可达性，勿直接删。

pub mod _inner {
  // CodeGen/src/IrRegAllocA64.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_CODEGEN_CHAOS_A64, DebugCodegenChaosA64);
  // CodeGen/src/CodeGen.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_CODEGEN_OPT_SIZE, DebugCodegenOptSize);
  // CodeGen/src/OptimizeConstProp.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_ABORTING_CHECKS, DebugLuauAbortingChecks);
  // Analysis/src/Frontend.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_ALWAYS_SHOW_CONSTRAINT_SOLVING_INCOMPLETE,
    DebugLuauAlwaysShowConstraintSolvingIncomplete
  );
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_ASSERT_ON_FORCED_CONSTRAINT,
    DebugLuauAssertOnForcedConstraint
  );
  // Analysis/src/Normalize.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_CHECK_NORMALIZE_INVARIANT,
    DebugLuauCheckNormalizeInvariant
  );
  // Analysis/src/DumpCFG.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_DUMP_CFGJSON, DebugLuauDumpCFGJson);
  // Analysis/src/Frontend.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_FORBID_INTERNAL_TYPES,
    DebugLuauForbidInternalTypes
  );
  // Analysis/src/Frontend.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_FORCE_OLD_SOLVER, DebugLuauForceOldSolver);
  // Analysis/src/TypeArena.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_FREEZE_ARENA, DebugLuauFreezeArena);
  // Analysis/src/TypeInfer.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_LOG_BINDINGS, DebugLuauLogBindings);
  // Analysis/src/DumpCFG.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_LOG_CFG, DebugLuauLogCFG);
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_LOG_SOLVER, DebugLuauLogSolver);
  // Analysis/src/Frontend.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_LOG_SOLVER_TO_JSON, DebugLuauLogSolverToJson);
  // Analysis/src/Frontend.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_LOG_SOLVER_TO_JSON_FILE,
    DebugLuauLogSolverToJsonFile
  );
  // Analysis/src/TypeInfer.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_MAGIC_TYPES, DebugLuauMagicTypes);
  // Analysis/src/AutocompleteCore.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_MAGIC_VARIABLE_NAMES, DebugLuauMagicVariableNames);
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_NO_INLINE, DebugLuauNoInline);
  // Analysis/src/Subtyping.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_SUBTYPING_CHECK_PATH_VALIDITY,
    DebugLuauSubtypingCheckPathValidity
  );
  // Common/src/TimeTrace.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_TIME_TRACING, DebugLuauTimeTracing);
  // Analysis/src/ToString.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_TO_STRING_NO_LEXICAL_SORT,
    DebugLuauToStringNoLexicalSort
  );
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_USER_DEFINED_CLASSES, DebugLuauUserDefinedClasses);
  // VM/src/lvmexecute.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_USER_DEFINED_CLASSES_RUNTIME,
    DebugLuauUserDefinedClassesRuntime
  );
  // Analysis/src/TypeChecker2.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_WARN_ON_UNANNOTATED_TOP_LEVEL_FUNCTIONS,
    DebugLuauWarnOnUnannotatedTopLevelFunctions
  );
  // VM/src/lmathlib.cpp
  crate::LUAU_FASTFLAGVARIABLE!(FIX_MATH_NOISE_PRECISION, FixMathNoisePrecision);
  // Analysis/src/NonStrictTypeChecker.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_ADD_RECURSION_COUNTER_TO_NON_STRICT_TYPE_CHECKER,
    LuauAddRecursionCounterToNonStrictTypeChecker
  );
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_ALLOW_GLOBAL_DECLARATION_TO_BE_CALLED_CLASS,
    LuauAllowGlobalDeclarationToBeCalledClass
  );
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_ALSO_INSTANTIATE_INFERRED_ARGUMENTS,
    LuauAlsoInstantiateInferredArguments
  );
  // Analysis/src/AutocompleteCore.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_AUTOCOMPLETE_CONST, LuauAutocompleteConst);
  // Analysis/src/AutocompleteCore.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_AUTOCOMPLETE_EXPORT, LuauAutocompleteExport);
  // Analysis/src/AutocompleteCore.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_AUTOCOMPLETE_STRING_SINGLETON_INTERSECTION,
    LuauAutocompleteStringSingletonIntersection
  );
  // Analysis/src/ExpectedTypeVisitor.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_BIDIRECTIONAL_INFERENCE_BETTER_UNION_HANDLING,
    LuauBidirectionalInferenceBetterUnionHandling
  );
  // VM/src/lvmexecute.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_BACKEDGE_HEAP_CHECK, LuauBackedgeHeapCheck);
  // VM/src/lvmexecute.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CALL_FEEDBACK, LuauCallFeedback);
  // Analysis/src/ToString.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_BETTER_METATABLE_STRINGIFICATION,
    LuauBetterMetatableStringification
  );
  // Analysis/src/TypeChecker2.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CHECK_FUNCTION_STATEMENT_TYPES,
    LuauCheckFunctionStatementTypes
  );
  // CodeGen/src/IrTranslateBuiltins.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_BUFFER_INTEGER, LuauCodegenBufferInteger);
  // CodeGen/src/OptimizeDeadStore.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_DSE_PTR_STORE_TAG_CHECK,
    LuauCodegenDsePtrStoreTagCheck
  );
  // CodeGen/src/OptimizeDeadStore.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_DSE_RESTORE_HINTS, LuauCodegenDseRestoreHints);
  // CodeGen/src/IrLoweringA64.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_FIX_BUFFER_LEN_CHECK,
    LuauCodegenFixBufferLenCheck
  );
  // CodeGen/src/IrTranslation.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_INTEGER3, LuauCodegenInteger3);
  // CodeGen/src/IrTranslation.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_INTEGER_COMPARE, LuauCodegenIntegerCompare);
  // CodeGen/src/OptimizeConstProp.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_LOAD_PROPAGATE_ORIGIN,
    LuauCodegenLoadPropagateOrigin
  );
  // CodeGen/src/CodeAllocator.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_PROTECT_DATA, LuauCodegenProtectData);
  // CodeGen/src/CodeGenX64.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_SUGGEST_ARGUMENT_REGISTER_X64,
    LuauCodegenSuggestArgumentRegisterX64
  );
  // CodeGen/src/OptimizeDeadStore.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_VM_EXIT_SYNC_MULTI_USE,
    LuauCodegenVmExitSyncMultiUse
  );
  // CodeGen/src/IrValueLocationTracking.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_DSE_RESTORE_HINT_UPDATE,
    LuauCodegenDseRestoreHintUpdate
  );
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_COMPILE_DUPTABLE_CONSTANT_PACK2,
    LuauCompileDuptableConstantPack2
  );
  // Compiler/src/CostModel.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_COMPILE_FASTCALL3_COST_MODEL,
    LuauCompileFastcall3CostModel
  );
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_COMPILE_INLINE_TABLE_FUNCTIONS,
    LuauCompileInlineTableFunctions
  );
  // Compiler/src/ConstantFolding.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_NO_FOLD_VECTOR_EQ_W, LuauCompileNoFoldVectorEqW);
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_CONCAT_TARGET_TOP, LuauCompileConcatTargetTop);
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_COMPILE_STRING_INTERP_TARGET_TOP,
    LuauCompileStringInterpTargetTop
  );
  // Compiler/src/Types.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_TYPE_ALIASES, LuauCompileTypeAliases);
  // Compiler/src/Types.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_RECURSIVE_ALIASES, LuauCompileRecursiveAliases);
  // Bytecode/src/BytecodeBuilder.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_EXPAND_LIMIT, LuauCompileExpandLimit);
  // Bytecode/src/BytecodeBuilder.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_VIRTUAL_BC_BUILDER, LuauVirtualBcBuilder);
  // Bytecode/src/BytecodeBuilder.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_BYTECODE_COST_MODEL, LuauBytecodeCostModel);
  // Bytecode/src/BytecodeBuilder.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_EMIT_VECTOR_DOUBLE, LuauCompileEmitVectorDouble);
  // Bytecode/src/BytecodeBuilder.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_FASTPCALL, LuauCompileFastpcall);
  // Analysis/src/BuiltinTypeFunctions.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CONCAT_DOESNT_ALWAYS_RETURN_STRING,
    LuauConcatDoesntAlwaysReturnString
  );
  // Analysis/src/Constraint.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CONSTRAINT_GRAPH, LuauConstraintGraph);
  // Require/src/RequireImpl.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CYCLIC_REQUIRE_SHORT_CIRCUIT,
    LuauCyclicRequireShortCircuit
  );
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CST_EXPR_GROUP, LuauCstExprGroup);
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CST_TYPE_GROUP, LuauCstTypeGroup);
  // VM/src/lvmexecute.cpp（LUAU_FLAGVERSION(..., 3) 见 register_flags）
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_DIRECT_FIELD_GET, LuauDirectFieldGet);
  // Analysis/src/ConstraintGenerator.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_DISALLOW_REDEFINING_BUILTIN_TYPES,
    LuauDisallowRedefiningBuiltinTypes
  );
  // Analysis/src/Unifier2.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_DO_NOT_LEAK_GENERICS_IN_INDEXER,
    LuauDoNotLeakGenericsInIndexer
  );
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_EMIT_CALL_FEEDBACK, LuauEmitCallFeedback);
  // Ast/src/PrettyPrinter.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_ERROR_TOLERANT_PRETTY_PRINTING,
    LuauErrorTolerantPrettyPrinting
  );
  // Analysis/src/TypeInfer.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_EXPLICIT_TYPE_INSTANTIATION_SUPPORT,
    LuauExplicitTypeInstantiationSupport
  );
  // Ast/src/Parser.cpp（LUAU_FLAGVERSION(..., 4) 见 register_flags）
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_EXPORT_VALUE_SYNTAX, LuauExportValueSyntax);
  // Analysis/src/Frontend.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_EXPORT_VALUE_TYPECHECK, LuauExportValueTypecheck);
  // Analysis/src/Frontend.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_FRONTEND_SOURCE_NODE_ERASE, LuauFrontendSourceNodeErase);
  // Analysis/src/Normalize.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_EXTERN_TYPES_NORMALIZE_WITH_SHAPES,
    LuauExternTypesNormalizeWithShapes
  );
  // Analysis/src/Unifier.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_FIX_INDEXER_SUBTYPING_ORDERING,
    LuauFixIndexerSubtypingOrdering
  );
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_FIX_PROP_READS_ON_METATABLE_TYPES,
    LuauFixPropReadsOnMetatableTypes
  );
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_FORCE_LESS, LuauForceLess);
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_INSTANTIATE_FUNCTION_TYPE_BEFORE_PUSH,
    LuauInstantiateFunctionTypeBeforePush
  );
  // Analysis/src/Unifier.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_INSTANTIATE_IN_SUBTYPING, LuauInstantiateInSubtyping);
  // Analysis/src/Instantiation.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_INSTANTIATION_USES_POLARITY,
    LuauInstantiationUsesPolarity
  );
  // Compiler/src/Builtins.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_INTEGER_BUFFER_FASTCALLS, LuauIntegerBufferFastcalls);
  // Compiler/src/Builtins.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_INTEGER_FASTCALLS, LuauIntegerFastcalls);
  // VM/src/lintlib.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_INTEGER_LIBRARY, LuauIntegerLibrary);
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_INTEGER_TYPE2, LuauIntegerType2);
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_ITERATIVE_INSTANTIATION_QUEUER,
    LuauIterativeInstantiationQueuer
  );
  // Analysis/src/TypeChecker2.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_LVALUE_COMPOUND_ASSIGNMENT_VISIT_LHS,
    LuauLValueCompoundAssignmentVisitLhs
  );
  // Analysis/src/Unifier2.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_LIMIT_UNIFICATION_RECURSION,
    LuauLimitUnificationRecursion
  );
  // CodeGen/src/CodeGenUtils.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_NATIVE_CODE_TARGET_CHECK, LuauNativeCodeTargetCheck);
  // Analysis/src/NonStrictTypeChecker.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_NON_STRICT_MODE_USE_ERROR_SUPRESSING_TAG,
    LuauNonStrictModeUseErrorSupressingTag
  );
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_OCCURS_CHECK_FOR_ALL_BINDINGS,
    LuauOccursCheckForAllBindings
  );
  // Analysis/src/Unifier2.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_PROPAGATE_FREE_TYPES_INTO_UNION_AND_INTERSECTION_BOUNDS,
    LuauPropagateFreeTypesIntoUnionAndIntersectionBounds
  );
  // Analysis/src/ConstraintGenerator.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_PROPAGATE_TYPE_ANNOTATIONS_IN_FOR_IN_LOOPS,
    LuauPropagateTypeAnnotationsInForInLoops
  );
  // Analysis/src/TypeChecker2.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_PROPERTY_MODIFIER_MISMATCH_ERRORS,
    LuauPropertyModifierMismatchErrors
  );
  // Analysis/src/ConstraintGenerator.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_READ_ONLY_INDEXERS, LuauReadOnlyIndexers);
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_REFINE_NIL_FROM_TABLE_INDEXER_RESULT_TYPE,
    LuauRefineNilFromTableIndexerResultType
  );
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_RELAX_CONSTRAINT_ORDERING_FOR_FUNCTION_CHECK,
    LuauRelaxConstraintOrderingForFunctionCheck
  );
  // Analysis/src/ConstraintSolver.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_REMOVE_CONSTRAINT_SOLVER_EMPLACE,
    LuauRemoveConstraintSolverEmplace
  );
  // Analysis/src/Instantiation.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_REPLACER_IS_SOLVER_AGNOSTIC,
    LuauReplacerIsSolverAgnostic
  );
  // VM/src/ldo.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_RESUME_RESTORE_CCALLS, LuauResumeRestoreCcalls);
  // Analysis/src/BuiltinDefinitions.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_SILENCE_DYNAMIC_FORMAT_STRING_ERRORS,
    LuauSilenceDynamicFormatStringErrors
  );
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_SINGLE_TYPE_OPTIONAL_PACK_RETURNS_ATTRIBUTE_PARENS,
    LuauSingleTypeOptionalPackReturnsAttributeParens
  );
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_SOLVER_V2, LuauSolverV2);
  // Analysis/src/Subtyping.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_SUBTYPING_MISSING_PROPERTIES_AS_NIL,
    LuauSubtypingMissingPropertiesAsNil
  );
  // Analysis/src/Subtyping.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_SUBTYPING_TABLES_HAS_BETTER_ERROR_SUPPRESSION,
    LuauSubtypingTablesHasBetterErrorSuppression
  );
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_TABLE_ENTRIES_DONT_NEED_TO_MATCH_INDENT,
    LuauTableEntriesDontNeedToMatchIndent
  );
  // Analysis/src/BuiltinDefinitions.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_TABLE_FREEZE_CHECK_IS_SUBTYPE,
    LuauTableFreezeCheckIsSubtype
  );
  // Analysis/src/ConstraintGenerator.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_TIDY_TYPE_PROTOTYPING, LuauTidyTypePrototyping);
  // Analysis/src/Error.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_TWEAK_ACCESS_VIOLATION_REPORTING,
    LuauTweakAccessViolationReporting
  );
  // Analysis/src/TypeFunctionRuntime.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_TYPE_FUNCTION_ROBUSTNESS, LuauTypeFunctionRobustness);
  // Analysis/src/TypeFunctionRuntime.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_TYPE_FUNCTION_SERIALIZE_ARG_NAMES,
    LuauTypeFunctionSerializeArgNames
  );
  // Analysis/src/TypeFunctionRuntime.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_TYPE_FUNCTION_STRUCTURED_ERRORS,
    LuauTypeFunctionStructuredErrors
  );
  // Analysis/src/TypeFunctionRuntime.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_TYPE_FUNCTION_SUPPORTS_FROZEN,
    LuauTypeFunctionSupportsFrozen
  );
  // VM/src/lvmload.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_UDATA_DIRECT_ACCESS6, LuauUdataDirectAccess6);
  // Analysis/src/TypeFunctionRuntime.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_UDTF_TYPE_IS_SUBTYPE_OF, LuauUdtfTypeIsSubtypeOf);
  // Analysis/src/TypeFunctionRuntime.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_UDTF_CREATE_SINGLETON_FIX_ERROR_MESSAGE,
    LuauUdtfCreateSingletonFixErrorMessage
  );
  // Analysis/src/TypeFunctionRuntime.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_UDTF_FIX_TYPE_NAME_TYPO, LuauUdtfFixTypeNameTypo);
  // Analysis/src/NativeStackGuard.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_USE_NATIVE_STACK_GUARD, LuauUseNativeStackGuard);
  // Analysis/src/DataFlowGraph.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_VISIT_CALL_TYPE_ARGS_IN_DFG, LuauVisitCallTypeArgsInDfg);
  // VM/src/ldo.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_XPCALL_FIX_MESSAGE_YIELD_PATH,
    LuauXpcallFixMessageYieldPath
  );
  // VM/src/lvmexecute.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_YIELD_ITER2, LuauYieldIter2);
  // VM/src/lgcdebug.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_ENUM_MORE_EDGES, LuauEnumMoreEdges);
  // VM/src/lvmload.cpp：字节码读路径跳过内联候选的 cost 字段
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COST_MODEL, LuauCostModel);
  // CodeGen/src/IrUtils.cpp propagateTagsFromPredecessors 跳过死前驱
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_SKIP_DEAD_PREDECESSOR_TAGS,
    LuauCodegenSkipDeadPredecessorTags
  );
  // CodeGen/src/OptimizeConstProp.cpp 收缩替换后仍记录 CSE 映射。
  // 注：本地 vendored cpp 无此旗标定义（前向移植，默认 false 与本地行为一致）
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_SUBSTITUTE_REPLACEMENTS,
    LuauCodegenSubstituteReplacements
  );
  // CodeGen/src/OptimizeConstProp.cpp 块间传播 fallback tag
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_PROPAGATE_FALLBACK_TAGS,
    LuauCodegenPropagateFallbackTags
  );
}

/// 宏自产的 Pascal 别名（宏展开内含 `pub use X as Y;`）经 glob 一次性再导出；
/// 别名对与宏定义同 token 生成，一致性由编译器而非人工清单保证。
pub use _inner::*;

/// C++ `FValue` ctor 的 `list = this` 自注册对应物：把本文件定义的全部
/// flag 挂入 per-type 注册表，供 `set_flag_by_name`/`set_all_unless` 按名遍历。
/// 仅由 `ensure_flags_registered` 的 `OnceLock` 串行调用一次。仅本 crate 消费，
/// 降 `pub(crate)`（b28 零消费点收口）。
pub(crate) fn register_flags() {
  _inner::DEBUG_CODEGEN_CHAOS_A64.register();
  _inner::DEBUG_CODEGEN_OPT_SIZE.register();
  _inner::DEBUG_LUAU_ABORTING_CHECKS.register();
  _inner::DEBUG_LUAU_ALWAYS_SHOW_CONSTRAINT_SOLVING_INCOMPLETE.register();
  _inner::DEBUG_LUAU_ASSERT_ON_FORCED_CONSTRAINT.register();
  _inner::DEBUG_LUAU_CHECK_NORMALIZE_INVARIANT.register();
  _inner::DEBUG_LUAU_DUMP_CFGJSON.register();
  _inner::DEBUG_LUAU_FORBID_INTERNAL_TYPES.register();
  _inner::DEBUG_LUAU_FORCE_OLD_SOLVER.register();
  _inner::DEBUG_LUAU_FREEZE_ARENA.register();
  _inner::DEBUG_LUAU_LOG_BINDINGS.register();
  _inner::DEBUG_LUAU_LOG_CFG.register();
  _inner::DEBUG_LUAU_LOG_SOLVER.register();
  _inner::DEBUG_LUAU_LOG_SOLVER_TO_JSON.register();
  _inner::DEBUG_LUAU_LOG_SOLVER_TO_JSON_FILE.register();
  _inner::DEBUG_LUAU_MAGIC_TYPES.register();
  _inner::DEBUG_LUAU_MAGIC_VARIABLE_NAMES.register();
  _inner::DEBUG_LUAU_NO_INLINE.register();
  _inner::DEBUG_LUAU_SUBTYPING_CHECK_PATH_VALIDITY.register();
  _inner::DEBUG_LUAU_TIME_TRACING.register();
  _inner::DEBUG_LUAU_TO_STRING_NO_LEXICAL_SORT.register();
  _inner::DEBUG_LUAU_USER_DEFINED_CLASSES.register();
  _inner::DEBUG_LUAU_USER_DEFINED_CLASSES_RUNTIME.register();
  _inner::DEBUG_LUAU_WARN_ON_UNANNOTATED_TOP_LEVEL_FUNCTIONS.register();
  _inner::FIX_MATH_NOISE_PRECISION.register();
  _inner::LUAU_ADD_RECURSION_COUNTER_TO_NON_STRICT_TYPE_CHECKER.register();
  _inner::LUAU_ALLOW_GLOBAL_DECLARATION_TO_BE_CALLED_CLASS.register();
  _inner::LUAU_ALSO_INSTANTIATE_INFERRED_ARGUMENTS.register();
  _inner::LUAU_AUTOCOMPLETE_CONST.register();
  _inner::LUAU_AUTOCOMPLETE_EXPORT.register();
  _inner::LUAU_AUTOCOMPLETE_STRING_SINGLETON_INTERSECTION.register();
  _inner::LUAU_BETTER_METATABLE_STRINGIFICATION.register();
  _inner::LUAU_BIDIRECTIONAL_INFERENCE_BETTER_UNION_HANDLING.register();
  _inner::LUAU_BACKEDGE_HEAP_CHECK.register();
  _inner::LUAU_BACKEDGE_HEAP_CHECK.set_version(2);
  _inner::LUAU_CALL_FEEDBACK.register();
  _inner::LUAU_CHECK_FUNCTION_STATEMENT_TYPES.register();
  _inner::LUAU_CODEGEN_BUFFER_INTEGER.register();
  _inner::LUAU_CODEGEN_DSE_PTR_STORE_TAG_CHECK.register();
  _inner::LUAU_CODEGEN_DSE_RESTORE_HINTS.register();
  _inner::LUAU_CODEGEN_DSE_RESTORE_HINTS.set_version(2);
  _inner::LUAU_CODEGEN_FIX_BUFFER_LEN_CHECK.register();
  _inner::LUAU_CODEGEN_INTEGER3.register();
  _inner::LUAU_CODEGEN_INTEGER_COMPARE.register();
  _inner::LUAU_CODEGEN_LOAD_PROPAGATE_ORIGIN.register();
  _inner::LUAU_CODEGEN_PROTECT_DATA.register();
  _inner::LUAU_CODEGEN_SUGGEST_ARGUMENT_REGISTER_X64.register();
  _inner::LUAU_CODEGEN_VM_EXIT_SYNC_MULTI_USE.register();
  _inner::LUAU_CODEGEN_DSE_RESTORE_HINT_UPDATE.register();
  _inner::LUAU_COMPILE_DUPTABLE_CONSTANT_PACK2.register();
  _inner::LUAU_COMPILE_FASTCALL3_COST_MODEL.register();
  _inner::LUAU_COMPILE_INLINE_TABLE_FUNCTIONS.register();
  _inner::LUAU_COMPILE_NO_FOLD_VECTOR_EQ_W.register();
  _inner::LUAU_COMPILE_CONCAT_TARGET_TOP.register();
  _inner::LUAU_COMPILE_STRING_INTERP_TARGET_TOP.register();
  _inner::LUAU_COMPILE_RECURSIVE_ALIASES.register();
  _inner::LUAU_COMPILE_TYPE_ALIASES.register();
  _inner::LUAU_COMPILE_EXPAND_LIMIT.register();
  _inner::LUAU_VIRTUAL_BC_BUILDER.register();
  _inner::LUAU_BYTECODE_COST_MODEL.register();
  _inner::LUAU_BYTECODE_COST_MODEL.set_version(2);
  _inner::LUAU_COMPILE_EMIT_VECTOR_DOUBLE.register();
  _inner::LUAU_COMPILE_EMIT_VECTOR_DOUBLE.set_version(2);
  _inner::LUAU_COMPILE_FASTPCALL.register();
  _inner::LUAU_COMPILE_FASTPCALL.set_version(2);
  _inner::LUAU_CONCAT_DOESNT_ALWAYS_RETURN_STRING.register();
  _inner::LUAU_CONSTRAINT_GRAPH.register();
  _inner::LUAU_CST_EXPR_GROUP.register();
  _inner::LUAU_CST_TYPE_GROUP.register();
  _inner::LUAU_CYCLIC_REQUIRE_SHORT_CIRCUIT.register();
  _inner::LUAU_DIRECT_FIELD_GET.register();
  _inner::LUAU_DIRECT_FIELD_GET.set_version(3);
  _inner::LUAU_DISALLOW_REDEFINING_BUILTIN_TYPES.register();
  _inner::LUAU_DO_NOT_LEAK_GENERICS_IN_INDEXER.register();
  _inner::LUAU_EMIT_CALL_FEEDBACK.register();
  _inner::LUAU_ERROR_TOLERANT_PRETTY_PRINTING.register();
  _inner::LUAU_EXPLICIT_TYPE_INSTANTIATION_SUPPORT.register();
  _inner::LUAU_EXPORT_VALUE_SYNTAX.register();
  _inner::LUAU_EXPORT_VALUE_SYNTAX.set_version(4);
  _inner::LUAU_EXPORT_VALUE_TYPECHECK.register();
  _inner::LUAU_EXPORT_VALUE_TYPECHECK.set_version(2);
  _inner::LUAU_EXTERN_TYPES_NORMALIZE_WITH_SHAPES.register();
  _inner::LUAU_FIX_INDEXER_SUBTYPING_ORDERING.register();
  _inner::LUAU_FIX_PROP_READS_ON_METATABLE_TYPES.register();
  _inner::LUAU_FORCE_LESS.register();
  _inner::LUAU_FRONTEND_SOURCE_NODE_ERASE.register();
  _inner::LUAU_INSTANTIATE_FUNCTION_TYPE_BEFORE_PUSH.register();
  _inner::LUAU_INSTANTIATE_IN_SUBTYPING.register();
  _inner::LUAU_INSTANTIATION_USES_POLARITY.register();
  _inner::LUAU_INTEGER_BUFFER_FASTCALLS.register();
  _inner::LUAU_INTEGER_FASTCALLS.register();
  _inner::LUAU_INTEGER_LIBRARY.register();
  _inner::LUAU_INTEGER_TYPE2.register();
  _inner::LUAU_ITERATIVE_INSTANTIATION_QUEUER.register();
  _inner::LUAU_LVALUE_COMPOUND_ASSIGNMENT_VISIT_LHS.register();
  _inner::LUAU_LIMIT_UNIFICATION_RECURSION.register();
  _inner::LUAU_NATIVE_CODE_TARGET_CHECK.register();
  _inner::LUAU_NON_STRICT_MODE_USE_ERROR_SUPRESSING_TAG.register();
  _inner::LUAU_OCCURS_CHECK_FOR_ALL_BINDINGS.register();
  _inner::LUAU_PROPAGATE_FREE_TYPES_INTO_UNION_AND_INTERSECTION_BOUNDS.register();
  _inner::LUAU_PROPAGATE_TYPE_ANNOTATIONS_IN_FOR_IN_LOOPS.register();
  _inner::LUAU_PROPERTY_MODIFIER_MISMATCH_ERRORS.register();
  _inner::LUAU_READ_ONLY_INDEXERS.register();
  _inner::LUAU_REFINE_NIL_FROM_TABLE_INDEXER_RESULT_TYPE.register();
  _inner::LUAU_RELAX_CONSTRAINT_ORDERING_FOR_FUNCTION_CHECK.register();
  _inner::LUAU_REMOVE_CONSTRAINT_SOLVER_EMPLACE.register();
  _inner::LUAU_REPLACER_IS_SOLVER_AGNOSTIC.register();
  _inner::LUAU_RESUME_RESTORE_CCALLS.register();
  _inner::LUAU_SILENCE_DYNAMIC_FORMAT_STRING_ERRORS.register();
  _inner::LUAU_SINGLE_TYPE_OPTIONAL_PACK_RETURNS_ATTRIBUTE_PARENS.register();
  _inner::LUAU_SOLVER_V2.register();
  _inner::LUAU_SUBTYPING_MISSING_PROPERTIES_AS_NIL.register();
  _inner::LUAU_SUBTYPING_TABLES_HAS_BETTER_ERROR_SUPPRESSION.register();
  _inner::LUAU_TABLE_ENTRIES_DONT_NEED_TO_MATCH_INDENT.register();
  _inner::LUAU_TABLE_FREEZE_CHECK_IS_SUBTYPE.register();
  _inner::LUAU_TIDY_TYPE_PROTOTYPING.register();
  _inner::LUAU_TWEAK_ACCESS_VIOLATION_REPORTING.register();
  _inner::LUAU_TYPE_FUNCTION_ROBUSTNESS.register();
  _inner::LUAU_TYPE_FUNCTION_SERIALIZE_ARG_NAMES.register();
  _inner::LUAU_TYPE_FUNCTION_STRUCTURED_ERRORS.register();
  _inner::LUAU_TYPE_FUNCTION_SUPPORTS_FROZEN.register();
  _inner::LUAU_UDATA_DIRECT_ACCESS6.register();
  _inner::LUAU_UDTF_TYPE_IS_SUBTYPE_OF.register();
  _inner::LUAU_UDTF_CREATE_SINGLETON_FIX_ERROR_MESSAGE.register();
  _inner::LUAU_UDTF_FIX_TYPE_NAME_TYPO.register();
  _inner::LUAU_USE_NATIVE_STACK_GUARD.register();
  _inner::LUAU_VISIT_CALL_TYPE_ARGS_IN_DFG.register();
  _inner::LUAU_XPCALL_FIX_MESSAGE_YIELD_PATH.register();
  _inner::LUAU_YIELD_ITER2.register();
  _inner::LUAU_COST_MODEL.register();
  _inner::LUAU_CODEGEN_SKIP_DEAD_PREDECESSOR_TAGS.register();
  _inner::LUAU_CODEGEN_SUBSTITUTE_REPLACEMENTS.register();
  _inner::LUAU_CODEGEN_PROPAGATE_FALLBACK_TAGS.register();
  _inner::LUAU_CODEGEN_PROPAGATE_FALLBACK_TAGS.set_version(2);
  _inner::LUAU_ENUM_MORE_EDGES.register();
}
