//! FastFlag 命名空间 `fflag` 模块 —— 静态（非动态）bool 标志。
//! 全 crate 的 `LUAU_FASTFLAGVARIABLE(...)` 定义集中于此，
//! C++ 的 `FFlag::Name` 对应 `crate::fflag::Name.get()`。
//! Rust 模块不像 C++ 命名空间可开放，故按 crate 聚合 ——
//! 见 [`crate::macros::fast_flags`]。
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

use crate::macros::fast_flags::luau_flag_module;

luau_flag_module! {
  // CodeGen/src/IrRegAllocA64.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_CODEGEN_CHAOS_A64, DebugCodegenChaosA64);
  // CodeGen/src/CodeGen.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_CODEGEN_OPT_SIZE, DebugCodegenOptSize);
  // CodeGen/src/OptimizeConstProp.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_ABORTING_CHECKS, DebugLuauAbortingChecks);
  // Analysis/src/Frontend.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_ALWAYS_SHOW_CONSTRAINT_SOLVING_INCOMPLETE, DebugLuauAlwaysShowConstraintSolvingIncomplete);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_ASSERT_ON_FORCED_CONSTRAINT, DebugLuauAssertOnForcedConstraint);
  // Analysis/src/Normalize.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_CHECK_NORMALIZE_INVARIANT, DebugLuauCheckNormalizeInvariant);
  // Analysis/src/DumpCFG.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_DUMP_CFGJSON, DebugLuauDumpCFGJson);
  // Analysis/src/Frontend.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_FORBID_INTERNAL_TYPES, DebugLuauForbidInternalTypes);
  // Analysis/src/Frontend.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_FORCE_OLD_SOLVER, DebugLuauForceOldSolver);
  // Analysis/src/TypeArena.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_FREEZE_ARENA, DebugLuauFreezeArena);
  // Analysis/src/TypeInfer.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_LOG_BINDINGS, DebugLuauLogBindings);
  // Analysis/src/DumpCFG.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_LOG_CFG, DebugLuauLogCFG);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_LOG_SOLVER, DebugLuauLogSolver);
  // Analysis/src/Frontend.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_LOG_SOLVER_TO_JSON, DebugLuauLogSolverToJson);
  // Analysis/src/Frontend.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_LOG_SOLVER_TO_JSON_FILE, DebugLuauLogSolverToJsonFile);
  // Analysis/src/TypeInfer.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_MAGIC_TYPES, DebugLuauMagicTypes);
  // Analysis/src/AutocompleteCore.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_MAGIC_VARIABLE_NAMES, DebugLuauMagicVariableNames);
  // Ast/src/Parser.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_NO_INLINE, DebugLuauNoInline);
  // Analysis/src/Subtyping.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_SUBTYPING_CHECK_PATH_VALIDITY, DebugLuauSubtypingCheckPathValidity);
  // Common/src/TimeTrace.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_TIME_TRACING, DebugLuauTimeTracing);
  // Analysis/src/ToString.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_TO_STRING_NO_LEXICAL_SORT, DebugLuauToStringNoLexicalSort);
  // Ast/src/Parser.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_USER_DEFINED_CLASSES, DebugLuauUserDefinedClasses);
  // VM/src/lvmexecute.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_USER_DEFINED_CLASSES_RUNTIME, DebugLuauUserDefinedClassesRuntime);
  // Analysis/src/TypeChecker2.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_WARN_ON_UNANNOTATED_TOP_LEVEL_FUNCTIONS, DebugLuauWarnOnUnannotatedTopLevelFunctions);
  // VM/src/lmathlib.cpp
  LUAU_FASTFLAGVARIABLE!(FIX_MATH_NOISE_PRECISION, FixMathNoisePrecision);
  // Analysis/src/NonStrictTypeChecker.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_ADD_RECURSION_COUNTER_TO_NON_STRICT_TYPE_CHECKER, LuauAddRecursionCounterToNonStrictTypeChecker);
  // Ast/src/Parser.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_ALLOW_GLOBAL_DECLARATION_TO_BE_CALLED_CLASS, LuauAllowGlobalDeclarationToBeCalledClass);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_ALSO_INSTANTIATE_INFERRED_ARGUMENTS, LuauAlsoInstantiateInferredArguments);
  // Analysis/src/AutocompleteCore.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_AUTOCOMPLETE_CONST, LuauAutocompleteConst);
  // Analysis/src/AutocompleteCore.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_AUTOCOMPLETE_EXPORT, LuauAutocompleteExport);
  // Analysis/src/AutocompleteCore.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_AUTOCOMPLETE_STRING_SINGLETON_INTERSECTION, LuauAutocompleteStringSingletonIntersection);
  // Analysis/src/ExpectedTypeVisitor.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_BIDIRECTIONAL_INFERENCE_BETTER_UNION_HANDLING, LuauBidirectionalInferenceBetterUnionHandling);
  // VM/src/lvmexecute.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  LUAU_FASTFLAGVARIABLE!(LUAU_BACKEDGE_HEAP_CHECK, LuauBackedgeHeapCheck), version = 2;
  // VM/src/lvmexecute.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CALL_FEEDBACK, LuauCallFeedback);
  // Analysis/src/ToString.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_BETTER_INFERRED_GENERIC_NAMES, LuauBetterInferredGenericNames);
  // Analysis/src/ToString.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_BETTER_METATABLE_STRINGIFICATION, LuauBetterMetatableStringification);
  // Analysis/src/TypeChecker2.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CHECK_FUNCTION_STATEMENT_TYPES, LuauCheckFunctionStatementTypes);
  // CodeGen/src/IrTranslateBuiltins.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_BUFFER_INTEGER, LuauCodegenBufferInteger);
  // CodeGen/src/OptimizeDeadStore.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_DSE_PTR_STORE_TAG_CHECK, LuauCodegenDsePtrStoreTagCheck);
  // CodeGen/src/OptimizeDeadStore.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_DSE_RESTORE_HINTS, LuauCodegenDseRestoreHints), version = 2;
  // CodeGen/src/IrLoweringA64.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_FIX_BUFFER_LEN_CHECK, LuauCodegenFixBufferLenCheck);
  // CodeGen/src/IrTranslation.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_INTEGER3, LuauCodegenInteger3);
  // CodeGen/src/IrTranslation.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_INTEGER_COMPARE, LuauCodegenIntegerCompare);
  // CodeGen/src/OptimizeConstProp.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_LOAD_PROPAGATE_ORIGIN, LuauCodegenLoadPropagateOrigin);
  // CodeGen/src/CodeAllocator.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_PROTECT_DATA, LuauCodegenProtectData);
  // CodeGen/src/CodeGenX64.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_SUGGEST_ARGUMENT_REGISTER_X64, LuauCodegenSuggestArgumentRegisterX64);
  // CodeGen/src/OptimizeDeadStore.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_VM_EXIT_SYNC_MULTI_USE, LuauCodegenVmExitSyncMultiUse);
  // CodeGen/src/IrValueLocationTracking.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_DSE_RESTORE_HINT_UPDATE, LuauCodegenDseRestoreHintUpdate);
  // Compiler/src/Compiler.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_DUPTABLE_CONSTANT_PACK2, LuauCompileDuptableConstantPack2);
  // Compiler/src/CostModel.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_FASTCALL3_COST_MODEL, LuauCompileFastcall3CostModel);
  // Compiler/src/Compiler.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_INLINE_TABLE_FUNCTIONS, LuauCompileInlineTableFunctions);
  // Compiler/src/ConstantFolding.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_NO_FOLD_VECTOR_EQ_W, LuauCompileNoFoldVectorEqW);
  // Compiler/src/Compiler.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_CONCAT_TARGET_TOP, LuauCompileConcatTargetTop);
  // Compiler/src/Compiler.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_STRING_INTERP_TARGET_TOP, LuauCompileStringInterpTargetTop);
  // Compiler/src/Types.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_TYPE_ALIASES, LuauCompileTypeAliases);
  // Compiler/src/Types.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_RECURSIVE_ALIASES, LuauCompileRecursiveAliases);
  // Bytecode/src/BytecodeBuilder.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_VIRTUAL_BC_BUILDER, LuauVirtualBcBuilder);
  // Bytecode/src/BytecodeBuilder.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  LUAU_FASTFLAGVARIABLE!(LUAU_BYTECODE_COST_MODEL, LuauBytecodeCostModel), version = 2;
  // Bytecode/src/BytecodeBuilder.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_EMIT_VECTOR_DOUBLE, LuauCompileEmitVectorDouble), version = 2;
  // Bytecode/src/BytecodeBuilder.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_FASTPCALL, LuauCompileFastpcall), version = 2;
  // Analysis/src/BuiltinTypeFunctions.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CONCAT_DOESNT_ALWAYS_RETURN_STRING, LuauConcatDoesntAlwaysReturnString);
  // Analysis/src/Constraint.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CONSTRAINT_GRAPH, LuauConstraintGraph);
  // Require/src/RequireImpl.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CYCLIC_REQUIRE_SHORT_CIRCUIT, LuauCyclicRequireShortCircuit);
  // Ast/src/Parser.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CST_EXPR_GROUP, LuauCstExprGroup);
  // Ast/src/Parser.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CST_TYPE_GROUP, LuauCstTypeGroup);
  // VM/src/lvmexecute.cpp（LUAU_FLAGVERSION(..., 3) 见 register_flags）
  LUAU_FASTFLAGVARIABLE!(LUAU_DIRECT_FIELD_GET, LuauDirectFieldGet), version = 3;
  // Analysis/src/ConstraintGenerator.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_DISALLOW_REDEFINING_BUILTIN_TYPES, LuauDisallowRedefiningBuiltinTypes);
  // Analysis/src/Unifier2.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_DO_NOT_LEAK_GENERICS_IN_INDEXER, LuauDoNotLeakGenericsInIndexer);
  // Compiler/src/Compiler.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_EMIT_CALL_FEEDBACK, LuauEmitCallFeedback);
  // Ast/src/PrettyPrinter.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_ERROR_TOLERANT_PRETTY_PRINTING, LuauErrorTolerantPrettyPrinting);
  // Analysis/src/TypeInfer.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_EXPLICIT_TYPE_INSTANTIATION_SUPPORT, LuauExplicitTypeInstantiationSupport);
  // Ast/src/Parser.cpp（LUAU_FLAGVERSION(..., 4) 见 register_flags）
  LUAU_FASTFLAGVARIABLE!(LUAU_EXPORT_VALUE_SYNTAX, LuauExportValueSyntax), version = 4;
  // Analysis/src/Frontend.cpp（LUAU_FLAGVERSION(..., 2) 见 register_flags）
  LUAU_FASTFLAGVARIABLE!(LUAU_EXPORT_VALUE_TYPECHECK, LuauExportValueTypecheck), version = 2;
  // Analysis/src/Frontend.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_FRONTEND_SOURCE_NODE_ERASE, LuauFrontendSourceNodeErase);
  // Analysis/src/Normalize.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_EXTERN_TYPES_NORMALIZE_WITH_SHAPES, LuauExternTypesNormalizeWithShapes);
  // Analysis/src/Unifier.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_FIX_INDEXER_SUBTYPING_ORDERING, LuauFixIndexerSubtypingOrdering);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_FIX_PROP_READS_ON_METATABLE_TYPES, LuauFixPropReadsOnMetatableTypes);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_FORCE_LESS, LuauForceLess);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_INSTANTIATE_FUNCTION_TYPE_BEFORE_PUSH, LuauInstantiateFunctionTypeBeforePush);
  // Analysis/src/Unifier.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_INSTANTIATE_IN_SUBTYPING, LuauInstantiateInSubtyping);
  // Analysis/src/Instantiation.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_INSTANTIATION_USES_POLARITY, LuauInstantiationUsesPolarity);
  // Compiler/src/Builtins.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_INTEGER_BUFFER_FASTCALLS, LuauIntegerBufferFastcalls);
  // Compiler/src/Builtins.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_INTEGER_FASTCALLS, LuauIntegerFastcalls);
  // VM/src/lintlib.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_INTEGER_LIBRARY, LuauIntegerLibrary);
  // Ast/src/Parser.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_INTEGER_TYPE2, LuauIntegerType2);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_ITERATIVE_INSTANTIATION_QUEUER, LuauIterativeInstantiationQueuer);
  // Analysis/src/TypeChecker2.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_LVALUE_COMPOUND_ASSIGNMENT_VISIT_LHS, LuauLValueCompoundAssignmentVisitLhs);
  // Analysis/src/Unifier2.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_LIMIT_UNIFICATION_RECURSION, LuauLimitUnificationRecursion);
  // CodeGen/src/CodeGenUtils.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_NATIVE_CODE_TARGET_CHECK, LuauNativeCodeTargetCheck);
  // Analysis/src/NonStrictTypeChecker.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_NON_STRICT_MODE_USE_ERROR_SUPRESSING_TAG, LuauNonStrictModeUseErrorSupressingTag);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_OCCURS_CHECK_FOR_ALL_BINDINGS, LuauOccursCheckForAllBindings);
  // Analysis/src/Unifier2.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_PROPAGATE_FREE_TYPES_INTO_UNION_AND_INTERSECTION_BOUNDS, LuauPropagateFreeTypesIntoUnionAndIntersectionBounds);
  // Analysis/src/ConstraintGenerator.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_PROPAGATE_TYPE_ANNOTATIONS_IN_FOR_IN_LOOPS, LuauPropagateTypeAnnotationsInForInLoops);
  // Analysis/src/TypeChecker2.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_PROPERTY_MODIFIER_MISMATCH_ERRORS, LuauPropertyModifierMismatchErrors);
  // Analysis/src/ConstraintGenerator.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_READ_ONLY_INDEXERS, LuauReadOnlyIndexers);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_REFINE_NIL_FROM_TABLE_INDEXER_RESULT_TYPE, LuauRefineNilFromTableIndexerResultType);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_RELAX_CONSTRAINT_ORDERING_FOR_FUNCTION_CHECK, LuauRelaxConstraintOrderingForFunctionCheck);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_REMOVE_CONSTRAINT_SOLVER_EMPLACE, LuauRemoveConstraintSolverEmplace);
  // Analysis/src/Instantiation.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_REPLACER_IS_SOLVER_AGNOSTIC, LuauReplacerIsSolverAgnostic);
  // VM/src/ldo.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_RESUME_RESTORE_CCALLS, LuauResumeRestoreCcalls);
  // Analysis/src/BuiltinDefinitions.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_SILENCE_DYNAMIC_FORMAT_STRING_ERRORS, LuauSilenceDynamicFormatStringErrors);
  // Ast/src/Parser.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_SINGLE_TYPE_OPTIONAL_PACK_RETURNS_ATTRIBUTE_PARENS, LuauSingleTypeOptionalPackReturnsAttributeParens);
  // Ast/src/Parser.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_SOLVER_V2, LuauSolverV2);
  // Analysis/src/Subtyping.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_SUBTYPING_MISSING_PROPERTIES_AS_NIL, LuauSubtypingMissingPropertiesAsNil);
  // Analysis/src/Subtyping.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_SUBTYPING_TABLES_HAS_BETTER_ERROR_SUPPRESSION, LuauSubtypingTablesHasBetterErrorSuppression);
  // Ast/src/Parser.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TABLE_ENTRIES_DONT_NEED_TO_MATCH_INDENT, LuauTableEntriesDontNeedToMatchIndent);
  // Analysis/src/BuiltinDefinitions.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TABLE_FREEZE_CHECK_IS_SUBTYPE, LuauTableFreezeCheckIsSubtype);
  // Analysis/src/ConstraintGenerator.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TIDY_TYPE_PROTOTYPING, LuauTidyTypePrototyping);
  // Analysis/src/Error.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TWEAK_ACCESS_VIOLATION_REPORTING, LuauTweakAccessViolationReporting);
  // Analysis/src/TypeFunctionRuntime.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TYPE_FUNCTION_ROBUSTNESS, LuauTypeFunctionRobustness);
  // Analysis/src/TypeFunctionRuntime.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TYPE_FUNCTION_SERIALIZE_ARG_NAMES, LuauTypeFunctionSerializeArgNames);
  // Analysis/src/TypeFunctionRuntime.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TYPE_FUNCTION_STRUCTURED_ERRORS, LuauTypeFunctionStructuredErrors);
  // Analysis/src/TypeFunctionRuntime.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TYPE_FUNCTION_SUPPORTS_FROZEN, LuauTypeFunctionSupportsFrozen);
  // VM/src/lvmload.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_UDATA_DIRECT_ACCESS6, LuauUdataDirectAccess6);
  // Analysis/src/TypeFunctionRuntime.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_UDTF_TYPE_IS_SUBTYPE_OF, LuauUdtfTypeIsSubtypeOf);
  // Analysis/src/TypeFunctionRuntime.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_UDTF_CREATE_SINGLETON_FIX_ERROR_MESSAGE, LuauUdtfCreateSingletonFixErrorMessage);
  // Analysis/src/TypeFunctionRuntime.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_UDTF_FIX_TYPE_NAME_TYPO, LuauUdtfFixTypeNameTypo);
  // Analysis/src/NativeStackGuard.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_USE_NATIVE_STACK_GUARD, LuauUseNativeStackGuard);
  // Analysis/src/DataFlowGraph.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_VISIT_CALL_TYPE_ARGS_IN_DFG, LuauVisitCallTypeArgsInDfg);
  // VM/src/ldo.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_XPCALL_FIX_MESSAGE_YIELD_PATH, LuauXpcallFixMessageYieldPath);
  // VM/src/lvmexecute.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_YIELD_ITER2, LuauYieldIter2);
  // VM/src/lgcdebug.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_ENUM_MORE_EDGES, LuauEnumMoreEdges);
  // VM/src/lvmload.cpp：字节码读路径跳过内联候选的 cost 字段
  LUAU_FASTFLAGVARIABLE!(LUAU_COST_MODEL, LuauCostModel);
  // CodeGen/src/IrUtils.cpp propagateTagsFromPredecessors 跳过死前驱
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_SKIP_DEAD_PREDECESSOR_TAGS, LuauCodegenSkipDeadPredecessorTags);
  // CodeGen/src/OptimizeConstProp.cpp 收缩替换后仍记录 CSE 映射。
  // 注：本地 vendored cpp 无此旗标定义（前向移植，默认 false 与本地行为一致）
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_SUBSTITUTE_REPLACEMENTS, LuauCodegenSubstituteReplacements);
  // CodeGen/src/OptimizeConstProp.cpp 块间传播 fallback tag
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_PROPAGATE_FALLBACK_TAGS, LuauCodegenPropagateFallbackTags), version = 2;

  // VM/src/lapi.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_COROUTINE_FINALLY, DebugLuauCoroutineFinally);
  // Analysis/src/EmbeddedBuiltinDefinitions.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_COROUTINE_FINALLY_ANALYSIS, DebugLuauCoroutineFinallyAnalysis);
  // Analysis/src/ConstraintGenerator.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_EXACT_TABLE_TYPES, DebugLuauExactTableTypes);
  // tests/Fixture.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_FORCE_EXACT_TABLES, DebugLuauForceExactTables);
  // Ast/src/Parser.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_PARSE_EXACT_TABLES, DebugLuauParseExactTables);
  // tests/Fixture.cpp
  LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_RUN_FAILING_EXACT_TABLE_TESTS, DebugLuauRunFailingExactTableTests);
  // Analysis/src/ConstraintGenerator.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_BIDIRECTIONAL_INFERENCE_SET_METATABLE, LuauBidirectionalInferenceSetMetatable);
  // VM/src/lstate.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_BUFFER_CAGE, LuauBufferCage);
  // VM/src/lvmutils.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CALL_LUAU_TM, LuauCallLuauTm);
  // Analysis/src/TypeChecker2.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CANNOT_ADD_INDEXER_TO_TABLE_PRIMITIVE, LuauCannotAddIndexerToTablePrimitive);
  // Analysis/src/Module.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CLONE_PUBLIC_INTERFACE_RETAIN_TYPE_FUNCTION_SOLVED_STATUS, LuauClonePublicInterfaceRetainTypeFunctionSolvedStatus);
  // CodeGen/src/IrLoweringA64.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_A64_FORG_LOOP_ARRAY, LuauCodegenA64ForgLoopArray);
  // CodeGen/src/IrRegAllocX64.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_X64_INT_SPILL_RESTORE, LuauCodegenX64IntSpillRestore);
  // Compiler/src/Compiler.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_LOOP_UNROLL_ZERO, LuauCompileLoopUnrollZero);
  // Bytecode/src/BytecodeBuilder.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_UNDO_EMIT_ADJUST, LuauCompileUndoEmitAdjust);
  // Analysis/src/ConstraintGenerator.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_EXPERIMENTAL_IF_LOCAL_ANALYSIS, LuauExperimentalIfLocalAnalysis);
  // Ast/src/Parser.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_EXPERIMENTAL_IF_LOCAL_SYNTAX, LuauExperimentalIfLocalSyntax);
  // Analysis/src/Normalize.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_FIX_NORMALIZE_FUNCTION_INTERSECTIONS, LuauFixNormalizeFunctionIntersections);
  // Analysis/src/FragmentAutocomplete.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_FRAGMENT_AC_LOCAL_AUTOCOMPLETE_FIX, LuauFragmentACLocalAutocompleteFix);
  // VM/src/lapi.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_FROZEN_META_BUTTERFLY, LuauFrozenMetaButterfly);
  // VM/src/lgc.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_GC_TRACE_UDATA, LuauGcTraceUdata);
  // Analysis/src/Linter.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_IMPROVE_DEPRECATED_LINT, LuauImproveDeprecatedLint);
  // Analysis/src/Unifier2.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_INFER_READ_ONLY_INDEXERS, LuauInferReadOnlyIndexers);
  // VM/src/lvmload.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_LOAD_REMAP_OPTIONAL_USERDATA, LuauLoadRemapOptionalUserdata);
  // Analysis/src/Constraint.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_REFERENCE_COUNT_INITIALIZER_IS_ITERATIVE, LuauReferenceCountInitializerIsIterative);
  // VM/src/linit.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_SANDBOX_FREEZES_VECTOR_METATABLE, LuauSandboxFreezesVectorMetatable);
  // Analysis/src/TypeChecker2.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_SKIP_UNUSED_TYPE_TRAVERSALS, LuauSkipUnusedTypeTraversals);
  // Analysis/src/TypeChecker2.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_SOUND_GENERIC_MISMATCHES, LuauSoundGenericMismatches);
  // Analysis/src/Subtyping.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_SUBTYPING_SKIP_UNREAD_REASONING, LuauSubtypingSkipUnreadReasoning);
  // VM/src/ltable.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TABLE_ARRAY_ADJUST_CHECK, LuauTableArrayAdjustCheck);
  // VM/src/ltable.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TABLE_ARRAY_SHRINK_ORDER, LuauTableArrayShrinkOrder);
  // Analysis/src/ConstraintSolver.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TRAVERSE_SCOPE_TO_FUNCTION, LuauTraverseScopeToFunction);
  // Analysis/src/UserDefinedTypeFunction.cpp
  LUAU_FASTFLAGVARIABLE!(LUAU_TYPE_FUNCTIONS_RETURN_AFTER_ALL_SERIALIZED, LuauTypeFunctionsReturnAfterAllSerialized);
}
