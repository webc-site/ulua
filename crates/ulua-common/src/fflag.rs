//! FastFlag 命名空间 `FFlag::` —— 静态（非动态）bool 标志。
//! 全 crate 的 `LUAU_FASTFLAGVARIABLE(...)` 定义集中于此，
//! C++ 的 `FFlag::Name` 对应 `crate::FFlag::Name.get()`。
//! Rust 模块不像 C++ 命名空间可开放，故按 crate 聚合 ——
//! 见 `crate::macros::luau_fastflagvariable`。

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
  // tests/Fixture.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_FORCE_ALL_NEW_SOLVER_TESTS,
    DebugLuauForceAllNewSolverTests
  );
  // tests/Fixture.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_FORCE_ALL_OLD_SOLVER_TESTS,
    DebugLuauForceAllOldSolverTests
  );
  // Analysis/src/Frontend.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    DEBUG_LUAU_FORCE_NON_STRICT_MODE,
    DebugLuauForceNonStrictMode
  );
  // Analysis/src/Frontend.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_FORCE_OLD_SOLVER, DebugLuauForceOldSolver);
  // Analysis/src/Frontend.cpp
  crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_FORCE_STRICT_MODE, DebugLuauForceStrictMode);
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
  // VM/src/lvmexecute.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CALL_FEEDBACK, LuauCallFeedback);
  // Analysis/src/TypeChecker2.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CHECK_FUNCTION_STATEMENT_TYPES,
    LuauCheckFunctionStatementTypes
  );
  // VM/src/lvmexecute.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CLOSURE_USAGE_COUNTER, LuauClosureUsageCounter);
  // CodeGen/src/EmitInstructionX64.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODE_GEN_CALL_WRAPPER_EMIT_INST,
    LuauCodeGenCallWrapperEmitInst
  );
  // CodeGen/src/IrTranslateBuiltins.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_BUFFER_INTEGER, LuauCodegenBufferInteger);
  // CodeGen/src/OptimizeDeadStore.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_DSE_PTR_STORE_TAG_CHECK,
    LuauCodegenDsePtrStoreTagCheck
  );
  // CodeGen/src/OptimizeDeadStore.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_DSE_RESTORE_HINTS, LuauCodegenDseRestoreHints);
  // CodeGen/src/OptimizeConstProp.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_EXTRA_TABLE_OPTS, LuauCodegenExtraTableOpts);
  // CodeGen/src/IrLoweringA64.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_FIX_BUFFER_LEN_CHECK,
    LuauCodegenFixBufferLenCheck
  );
  // CodeGen/src/IrValueLocationTracking.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_FORWARD_REMATERIALIZE,
    LuauCodegenForwardRematerialize
  );
  // CodeGen/src/CodeAllocator.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_FREE_BLOCKS, LuauCodegenFreeBlocks);
  // CodeGen/src/CodeGen.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_INTEGER2, LuauCodegenInteger2);
  // CodeGen/src/IrTranslateBuiltins.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_INTEGER_ARG3_FIX, LuauCodegenIntegerArg3Fix);
  // CodeGen/src/IrTranslation.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_INTEGER_FASTCALL2K,
    LuauCodegenIntegerFastcall2k
  );
  // CodeGen/src/OptimizeConstProp.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_LINEAR_SETUP_ENTRY_STATE3,
    LuauCodegenLinearSetupEntryState3
  );
  // CodeGen/src/OptimizeConstProp.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_LOAD_PROPAGATE_ORIGIN,
    LuauCodegenLoadPropagateOrigin
  );
  // CodeGen/src/CodeAllocator.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_PROTECT_DATA, LuauCodegenProtectData);
  // CodeGen/src/OptimizeConstProp.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_RECORD_ALL_BLOCK_EXIT_INFO,
    LuauCodegenRecordAllBlockExitInfo
  );
  // CodeGen/src/BytecodeAnalysis.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_REG_TAG2, LuauCodegenRegTag2);
  // CodeGen/src/CodeGenX64.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_SUGGEST_ARGUMENT_REGISTER_X64,
    LuauCodegenSuggestArgumentRegisterX64
  );
  // CodeGen/src/IrAnalysis.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_VM_EXIT_SYNC, LuauCodegenVmExitSync);
  // CodeGen/src/OptimizeDeadStore.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_VM_EXIT_SYNC_FIX, LuauCodegenVmExitSyncFix);
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
  // Compiler/src/ConstantFolding.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_FOLD_OPTIMIZE, LuauCompileFoldOptimize);
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_COMPILE_INLINE_TABLE_FUNCTIONS,
    LuauCompileInlineTableFunctions
  );
  // Compiler/src/ConstantFolding.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_COMPILE_NEW_TABLE_MUTATION_TRACKER,
    LuauCompileNewTableMutationTracker
  );
  // Compiler/src/ConstantFolding.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_COMPILE_PROPAGATE_TABLE_PROPS2,
    LuauCompilePropagateTableProps2
  );
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_CONCAT_TARGET_TOP, LuauCompileConcatTargetTop);
  // Compiler/src/Compiler.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_COMPILE_STRING_INTERP_TARGET_TOP,
    LuauCompileStringInterpTargetTop
  );
  // Compiler/src/Types.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_TYPE_ALIASES, LuauCompileTypeAliases);
  // Bytecode/src/BytecodeBuilder.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_UDATA_DIRECT, LuauCompileUdataDirect);
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
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CST_EXPR_GROUP, LuauCstExprGroup);
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_CST_TYPE_GROUP, LuauCstTypeGroup);
  // VM/src/lvmexecute.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_DIRECT_FIELD_GET, LuauDirectFieldGet);
  // Analysis/src/ConstraintGenerator.cpp
  crate::LUAU_FASTFLAGVARIABLE!(
    LUAU_DISALLOW_REDEFINING_BUILTIN_TYPES,
    LuauDisallowRedefiningBuiltinTypes
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
  // Ast/src/Parser.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_EXPORT_VALUE_SYNTAX, LuauExportValueSyntax);
  // Analysis/src/Frontend.cpp
  crate::LUAU_FASTFLAGVARIABLE!(LUAU_EXPORT_VALUE_TYPECHECK, LuauExportValueTypecheck);
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
}
pub use _inner::{
  DEBUG_CODEGEN_CHAOS_A64 as DebugCodegenChaosA64, DEBUG_CODEGEN_OPT_SIZE as DebugCodegenOptSize,
  DEBUG_LUAU_ABORTING_CHECKS as DebugLuauAbortingChecks,
  DEBUG_LUAU_ALWAYS_SHOW_CONSTRAINT_SOLVING_INCOMPLETE as DebugLuauAlwaysShowConstraintSolvingIncomplete,
  DEBUG_LUAU_ASSERT_ON_FORCED_CONSTRAINT as DebugLuauAssertOnForcedConstraint,
  DEBUG_LUAU_CHECK_NORMALIZE_INVARIANT as DebugLuauCheckNormalizeInvariant,
  DEBUG_LUAU_DUMP_CFGJSON as DebugLuauDumpCFGJson,
  DEBUG_LUAU_FORBID_INTERNAL_TYPES as DebugLuauForbidInternalTypes,
  DEBUG_LUAU_FORCE_ALL_NEW_SOLVER_TESTS as DebugLuauForceAllNewSolverTests,
  DEBUG_LUAU_FORCE_ALL_OLD_SOLVER_TESTS as DebugLuauForceAllOldSolverTests,
  DEBUG_LUAU_FORCE_NON_STRICT_MODE as DebugLuauForceNonStrictMode,
  DEBUG_LUAU_FORCE_OLD_SOLVER as DebugLuauForceOldSolver,
  DEBUG_LUAU_FORCE_STRICT_MODE as DebugLuauForceStrictMode,
  DEBUG_LUAU_FREEZE_ARENA as DebugLuauFreezeArena, DEBUG_LUAU_LOG_BINDINGS as DebugLuauLogBindings,
  DEBUG_LUAU_LOG_CFG as DebugLuauLogCFG, DEBUG_LUAU_LOG_SOLVER as DebugLuauLogSolver,
  DEBUG_LUAU_LOG_SOLVER_TO_JSON as DebugLuauLogSolverToJson,
  DEBUG_LUAU_LOG_SOLVER_TO_JSON_FILE as DebugLuauLogSolverToJsonFile,
  DEBUG_LUAU_MAGIC_TYPES as DebugLuauMagicTypes,
  DEBUG_LUAU_MAGIC_VARIABLE_NAMES as DebugLuauMagicVariableNames,
  DEBUG_LUAU_NO_INLINE as DebugLuauNoInline,
  DEBUG_LUAU_SUBTYPING_CHECK_PATH_VALIDITY as DebugLuauSubtypingCheckPathValidity,
  DEBUG_LUAU_TIME_TRACING as DebugLuauTimeTracing,
  DEBUG_LUAU_TO_STRING_NO_LEXICAL_SORT as DebugLuauToStringNoLexicalSort,
  DEBUG_LUAU_USER_DEFINED_CLASSES as DebugLuauUserDefinedClasses,
  DEBUG_LUAU_USER_DEFINED_CLASSES_RUNTIME as DebugLuauUserDefinedClassesRuntime,
  FIX_MATH_NOISE_PRECISION as FixMathNoisePrecision,
  LUAU_ADD_RECURSION_COUNTER_TO_NON_STRICT_TYPE_CHECKER as LuauAddRecursionCounterToNonStrictTypeChecker,
  LUAU_ALLOW_GLOBAL_DECLARATION_TO_BE_CALLED_CLASS as LuauAllowGlobalDeclarationToBeCalledClass,
  LUAU_ALSO_INSTANTIATE_INFERRED_ARGUMENTS as LuauAlsoInstantiateInferredArguments,
  LUAU_AUTOCOMPLETE_CONST as LuauAutocompleteConst,
  LUAU_AUTOCOMPLETE_EXPORT as LuauAutocompleteExport,
  LUAU_AUTOCOMPLETE_STRING_SINGLETON_INTERSECTION as LuauAutocompleteStringSingletonIntersection,
  LUAU_BIDIRECTIONAL_INFERENCE_BETTER_UNION_HANDLING as LuauBidirectionalInferenceBetterUnionHandling,
  LUAU_BYTECODE_COST_MODEL as LuauBytecodeCostModel, LUAU_CALL_FEEDBACK as LuauCallFeedback,
  LUAU_CHECK_FUNCTION_STATEMENT_TYPES as LuauCheckFunctionStatementTypes,
  LUAU_CLOSURE_USAGE_COUNTER as LuauClosureUsageCounter,
  LUAU_CODE_GEN_CALL_WRAPPER_EMIT_INST as LuauCodeGenCallWrapperEmitInst,
  LUAU_CODEGEN_BUFFER_INTEGER as LuauCodegenBufferInteger,
  LUAU_CODEGEN_DSE_PTR_STORE_TAG_CHECK as LuauCodegenDsePtrStoreTagCheck,
  LUAU_CODEGEN_DSE_RESTORE_HINT_UPDATE as LuauCodegenDseRestoreHintUpdate,
  LUAU_CODEGEN_DSE_RESTORE_HINTS as LuauCodegenDseRestoreHints,
  LUAU_CODEGEN_EXTRA_TABLE_OPTS as LuauCodegenExtraTableOpts,
  LUAU_CODEGEN_FIX_BUFFER_LEN_CHECK as LuauCodegenFixBufferLenCheck,
  LUAU_CODEGEN_FORWARD_REMATERIALIZE as LuauCodegenForwardRematerialize,
  LUAU_CODEGEN_FREE_BLOCKS as LuauCodegenFreeBlocks,
  LUAU_CODEGEN_INTEGER_ARG3_FIX as LuauCodegenIntegerArg3Fix,
  LUAU_CODEGEN_INTEGER_FASTCALL2K as LuauCodegenIntegerFastcall2k,
  LUAU_CODEGEN_INTEGER2 as LuauCodegenInteger2,
  LUAU_CODEGEN_LINEAR_SETUP_ENTRY_STATE3 as LuauCodegenLinearSetupEntryState3,
  LUAU_CODEGEN_LOAD_PROPAGATE_ORIGIN as LuauCodegenLoadPropagateOrigin,
  LUAU_CODEGEN_PROTECT_DATA as LuauCodegenProtectData,
  LUAU_CODEGEN_RECORD_ALL_BLOCK_EXIT_INFO as LuauCodegenRecordAllBlockExitInfo,
  LUAU_CODEGEN_REG_TAG2 as LuauCodegenRegTag2,
  LUAU_CODEGEN_SUGGEST_ARGUMENT_REGISTER_X64 as LuauCodegenSuggestArgumentRegisterX64,
  LUAU_CODEGEN_VM_EXIT_SYNC as LuauCodegenVmExitSync,
  LUAU_CODEGEN_VM_EXIT_SYNC_FIX as LuauCodegenVmExitSyncFix,
  LUAU_COMPILE_CONCAT_TARGET_TOP as LuauCompileConcatTargetTop,
  LUAU_COMPILE_DUPTABLE_CONSTANT_PACK2 as LuauCompileDuptableConstantPack2,
  LUAU_COMPILE_EMIT_VECTOR_DOUBLE as LuauCompileEmitVectorDouble,
  LUAU_COMPILE_EXPAND_LIMIT as LuauCompileExpandLimit,
  LUAU_COMPILE_FASTCALL3_COST_MODEL as LuauCompileFastcall3CostModel,
  LUAU_COMPILE_FASTPCALL as LuauCompileFastpcall,
  LUAU_COMPILE_FOLD_OPTIMIZE as LuauCompileFoldOptimize,
  LUAU_COMPILE_INLINE_TABLE_FUNCTIONS as LuauCompileInlineTableFunctions,
  LUAU_COMPILE_NEW_TABLE_MUTATION_TRACKER as LuauCompileNewTableMutationTracker,
  LUAU_COMPILE_PROPAGATE_TABLE_PROPS2 as LuauCompilePropagateTableProps2,
  LUAU_COMPILE_STRING_INTERP_TARGET_TOP as LuauCompileStringInterpTargetTop,
  LUAU_COMPILE_TYPE_ALIASES as LuauCompileTypeAliases,
  LUAU_COMPILE_UDATA_DIRECT as LuauCompileUdataDirect,
  LUAU_CONCAT_DOESNT_ALWAYS_RETURN_STRING as LuauConcatDoesntAlwaysReturnString,
  LUAU_CONSTRAINT_GRAPH as LuauConstraintGraph, LUAU_CST_EXPR_GROUP as LuauCstExprGroup,
  LUAU_CST_TYPE_GROUP as LuauCstTypeGroup, LUAU_DIRECT_FIELD_GET as LuauDirectFieldGet,
  LUAU_DISALLOW_REDEFINING_BUILTIN_TYPES as LuauDisallowRedefiningBuiltinTypes,
  LUAU_EMIT_CALL_FEEDBACK as LuauEmitCallFeedback, LUAU_ENUM_MORE_EDGES as LuauEnumMoreEdges,
  LUAU_ERROR_TOLERANT_PRETTY_PRINTING as LuauErrorTolerantPrettyPrinting,
  LUAU_EXPLICIT_TYPE_INSTANTIATION_SUPPORT as LuauExplicitTypeInstantiationSupport,
  LUAU_EXPORT_VALUE_SYNTAX as LuauExportValueSyntax,
  LUAU_EXPORT_VALUE_TYPECHECK as LuauExportValueTypecheck,
  LUAU_EXTERN_TYPES_NORMALIZE_WITH_SHAPES as LuauExternTypesNormalizeWithShapes,
  LUAU_FIX_INDEXER_SUBTYPING_ORDERING as LuauFixIndexerSubtypingOrdering,
  LUAU_FIX_PROP_READS_ON_METATABLE_TYPES as LuauFixPropReadsOnMetatableTypes,
  LUAU_FORCE_LESS as LuauForceLess,
  LUAU_INSTANTIATE_FUNCTION_TYPE_BEFORE_PUSH as LuauInstantiateFunctionTypeBeforePush,
  LUAU_INSTANTIATE_IN_SUBTYPING as LuauInstantiateInSubtyping,
  LUAU_INSTANTIATION_USES_POLARITY as LuauInstantiationUsesPolarity,
  LUAU_INTEGER_BUFFER_FASTCALLS as LuauIntegerBufferFastcalls,
  LUAU_INTEGER_FASTCALLS as LuauIntegerFastcalls, LUAU_INTEGER_LIBRARY as LuauIntegerLibrary,
  LUAU_INTEGER_TYPE2 as LuauIntegerType2,
  LUAU_ITERATIVE_INSTANTIATION_QUEUER as LuauIterativeInstantiationQueuer,
  LUAU_LIMIT_UNIFICATION_RECURSION as LuauLimitUnificationRecursion,
  LUAU_LVALUE_COMPOUND_ASSIGNMENT_VISIT_LHS as LuauLValueCompoundAssignmentVisitLhs,
  LUAU_NATIVE_CODE_TARGET_CHECK as LuauNativeCodeTargetCheck,
  LUAU_NON_STRICT_MODE_USE_ERROR_SUPRESSING_TAG as LuauNonStrictModeUseErrorSupressingTag,
  LUAU_OCCURS_CHECK_FOR_ALL_BINDINGS as LuauOccursCheckForAllBindings,
  LUAU_PROPAGATE_FREE_TYPES_INTO_UNION_AND_INTERSECTION_BOUNDS as LuauPropagateFreeTypesIntoUnionAndIntersectionBounds,
  LUAU_PROPAGATE_TYPE_ANNOTATIONS_IN_FOR_IN_LOOPS as LuauPropagateTypeAnnotationsInForInLoops,
  LUAU_PROPERTY_MODIFIER_MISMATCH_ERRORS as LuauPropertyModifierMismatchErrors,
  LUAU_READ_ONLY_INDEXERS as LuauReadOnlyIndexers,
  LUAU_REFINE_NIL_FROM_TABLE_INDEXER_RESULT_TYPE as LuauRefineNilFromTableIndexerResultType,
  LUAU_RELAX_CONSTRAINT_ORDERING_FOR_FUNCTION_CHECK as LuauRelaxConstraintOrderingForFunctionCheck,
  LUAU_REMOVE_CONSTRAINT_SOLVER_EMPLACE as LuauRemoveConstraintSolverEmplace,
  LUAU_REPLACER_IS_SOLVER_AGNOSTIC as LuauReplacerIsSolverAgnostic,
  LUAU_RESUME_RESTORE_CCALLS as LuauResumeRestoreCcalls,
  LUAU_SILENCE_DYNAMIC_FORMAT_STRING_ERRORS as LuauSilenceDynamicFormatStringErrors,
  LUAU_SINGLE_TYPE_OPTIONAL_PACK_RETURNS_ATTRIBUTE_PARENS as LuauSingleTypeOptionalPackReturnsAttributeParens,
  LUAU_SOLVER_V2 as LuauSolverV2,
  LUAU_SUBTYPING_MISSING_PROPERTIES_AS_NIL as LuauSubtypingMissingPropertiesAsNil,
  LUAU_SUBTYPING_TABLES_HAS_BETTER_ERROR_SUPPRESSION as LuauSubtypingTablesHasBetterErrorSuppression,
  LUAU_TABLE_ENTRIES_DONT_NEED_TO_MATCH_INDENT as LuauTableEntriesDontNeedToMatchIndent,
  LUAU_TABLE_FREEZE_CHECK_IS_SUBTYPE as LuauTableFreezeCheckIsSubtype,
  LUAU_TIDY_TYPE_PROTOTYPING as LuauTidyTypePrototyping,
  LUAU_TWEAK_ACCESS_VIOLATION_REPORTING as LuauTweakAccessViolationReporting,
  LUAU_TYPE_FUNCTION_ROBUSTNESS as LuauTypeFunctionRobustness,
  LUAU_TYPE_FUNCTION_SERIALIZE_ARG_NAMES as LuauTypeFunctionSerializeArgNames,
  LUAU_TYPE_FUNCTION_STRUCTURED_ERRORS as LuauTypeFunctionStructuredErrors,
  LUAU_TYPE_FUNCTION_SUPPORTS_FROZEN as LuauTypeFunctionSupportsFrozen,
  LUAU_UDATA_DIRECT_ACCESS6 as LuauUdataDirectAccess6,
  LUAU_UDTF_CREATE_SINGLETON_FIX_ERROR_MESSAGE as LuauUdtfCreateSingletonFixErrorMessage,
  LUAU_UDTF_FIX_TYPE_NAME_TYPO as LuauUdtfFixTypeNameTypo,
  LUAU_UDTF_TYPE_IS_SUBTYPE_OF as LuauUdtfTypeIsSubtypeOf,
  LUAU_USE_NATIVE_STACK_GUARD as LuauUseNativeStackGuard,
  LUAU_VIRTUAL_BC_BUILDER as LuauVirtualBcBuilder,
  LUAU_VISIT_CALL_TYPE_ARGS_IN_DFG as LuauVisitCallTypeArgsInDfg,
  LUAU_XPCALL_FIX_MESSAGE_YIELD_PATH as LuauXpcallFixMessageYieldPath,
  LUAU_YIELD_ITER2 as LuauYieldIter2, *,
};

/// C++ `FValue` ctor 的 `list = this` 自注册对应物：把本文件定义的全部
/// flag 挂入 per-type 链表，供 `set_flag_by_name`/`set_all_unless` 按名遍历。
///
/// # Safety
/// 每个 flag 仅注册一次（由 `ensure_flags_registered` 的 `OnceLock` 串行化），
/// 且早于任何并发链表遍历；`set_version` 写入此后仅经 `version()` 读取的槽位。
pub fn register_flags() {
  use crate::FFlag;
  unsafe {
    FFlag::DEBUG_CODEGEN_CHAOS_A64.register();
    FFlag::DEBUG_CODEGEN_OPT_SIZE.register();
    FFlag::DEBUG_LUAU_ABORTING_CHECKS.register();
    FFlag::DEBUG_LUAU_ALWAYS_SHOW_CONSTRAINT_SOLVING_INCOMPLETE.register();
    FFlag::DEBUG_LUAU_ASSERT_ON_FORCED_CONSTRAINT.register();
    FFlag::DEBUG_LUAU_CHECK_NORMALIZE_INVARIANT.register();
    FFlag::DEBUG_LUAU_DUMP_CFGJSON.register();
    FFlag::DEBUG_LUAU_FORBID_INTERNAL_TYPES.register();
    FFlag::DEBUG_LUAU_FORCE_ALL_NEW_SOLVER_TESTS.register();
    FFlag::DEBUG_LUAU_FORCE_ALL_OLD_SOLVER_TESTS.register();
    FFlag::DEBUG_LUAU_FORCE_NON_STRICT_MODE.register();
    FFlag::DEBUG_LUAU_FORCE_OLD_SOLVER.register();
    FFlag::DEBUG_LUAU_FORCE_STRICT_MODE.register();
    FFlag::DEBUG_LUAU_FREEZE_ARENA.register();
    FFlag::DEBUG_LUAU_LOG_BINDINGS.register();
    FFlag::DEBUG_LUAU_LOG_CFG.register();
    FFlag::DEBUG_LUAU_LOG_SOLVER.register();
    FFlag::DEBUG_LUAU_LOG_SOLVER_TO_JSON.register();
    FFlag::DEBUG_LUAU_LOG_SOLVER_TO_JSON_FILE.register();
    FFlag::DEBUG_LUAU_MAGIC_TYPES.register();
    FFlag::DEBUG_LUAU_MAGIC_VARIABLE_NAMES.register();
    FFlag::DEBUG_LUAU_NO_INLINE.register();
    FFlag::DEBUG_LUAU_SUBTYPING_CHECK_PATH_VALIDITY.register();
    FFlag::DEBUG_LUAU_TIME_TRACING.register();
    FFlag::DEBUG_LUAU_TO_STRING_NO_LEXICAL_SORT.register();
    FFlag::DEBUG_LUAU_USER_DEFINED_CLASSES.register();
    FFlag::DEBUG_LUAU_USER_DEFINED_CLASSES_RUNTIME.register();
    FFlag::FIX_MATH_NOISE_PRECISION.register();
    FFlag::LUAU_ADD_RECURSION_COUNTER_TO_NON_STRICT_TYPE_CHECKER.register();
    FFlag::LUAU_ALLOW_GLOBAL_DECLARATION_TO_BE_CALLED_CLASS.register();
    FFlag::LUAU_ALSO_INSTANTIATE_INFERRED_ARGUMENTS.register();
    FFlag::LUAU_AUTOCOMPLETE_CONST.register();
    FFlag::LUAU_AUTOCOMPLETE_EXPORT.register();
    FFlag::LUAU_AUTOCOMPLETE_STRING_SINGLETON_INTERSECTION.register();
    FFlag::LUAU_BIDIRECTIONAL_INFERENCE_BETTER_UNION_HANDLING.register();
    FFlag::LUAU_CALL_FEEDBACK.register();
    FFlag::LUAU_CHECK_FUNCTION_STATEMENT_TYPES.register();
    FFlag::LUAU_CLOSURE_USAGE_COUNTER.register();
    FFlag::LUAU_CODE_GEN_CALL_WRAPPER_EMIT_INST.register();
    FFlag::LUAU_CODEGEN_BUFFER_INTEGER.register();
    FFlag::LUAU_CODEGEN_DSE_PTR_STORE_TAG_CHECK.register();
    FFlag::LUAU_CODEGEN_DSE_RESTORE_HINTS.register();
    FFlag::LUAU_CODEGEN_EXTRA_TABLE_OPTS.register();
    FFlag::LUAU_CODEGEN_FIX_BUFFER_LEN_CHECK.register();
    FFlag::LUAU_CODEGEN_FORWARD_REMATERIALIZE.register();
    FFlag::LUAU_CODEGEN_FREE_BLOCKS.register();
    FFlag::LUAU_CODEGEN_INTEGER2.register();
    FFlag::LUAU_CODEGEN_INTEGER_ARG3_FIX.register();
    FFlag::LUAU_CODEGEN_INTEGER_FASTCALL2K.register();
    FFlag::LUAU_CODEGEN_LINEAR_SETUP_ENTRY_STATE3.register();
    FFlag::LUAU_CODEGEN_LOAD_PROPAGATE_ORIGIN.register();
    FFlag::LUAU_CODEGEN_PROTECT_DATA.register();
    FFlag::LUAU_CODEGEN_RECORD_ALL_BLOCK_EXIT_INFO.register();
    FFlag::LUAU_CODEGEN_REG_TAG2.register();
    FFlag::LUAU_CODEGEN_SUGGEST_ARGUMENT_REGISTER_X64.register();
    FFlag::LUAU_CODEGEN_VM_EXIT_SYNC.register();
    FFlag::LUAU_CODEGEN_VM_EXIT_SYNC_FIX.register();
    FFlag::LUAU_CODEGEN_DSE_RESTORE_HINT_UPDATE.register();
    FFlag::LUAU_COMPILE_DUPTABLE_CONSTANT_PACK2.register();
    FFlag::LUAU_COMPILE_FASTCALL3_COST_MODEL.register();
    FFlag::LUAU_COMPILE_FOLD_OPTIMIZE.register();
    FFlag::LUAU_COMPILE_INLINE_TABLE_FUNCTIONS.register();
    FFlag::LUAU_COMPILE_NEW_TABLE_MUTATION_TRACKER.register();
    FFlag::LUAU_COMPILE_PROPAGATE_TABLE_PROPS2.register();
    FFlag::LUAU_COMPILE_CONCAT_TARGET_TOP.register();
    FFlag::LUAU_COMPILE_STRING_INTERP_TARGET_TOP.register();
    FFlag::LUAU_COMPILE_TYPE_ALIASES.register();
    FFlag::LUAU_COMPILE_UDATA_DIRECT.register();
    FFlag::LUAU_COMPILE_EXPAND_LIMIT.register();
    FFlag::LUAU_VIRTUAL_BC_BUILDER.register();
    FFlag::LUAU_BYTECODE_COST_MODEL.register();
    FFlag::LUAU_BYTECODE_COST_MODEL.set_version(2);
    FFlag::LUAU_COMPILE_EMIT_VECTOR_DOUBLE.register();
    FFlag::LUAU_COMPILE_EMIT_VECTOR_DOUBLE.set_version(2);
    FFlag::LUAU_COMPILE_FASTPCALL.register();
    FFlag::LUAU_COMPILE_FASTPCALL.set_version(2);
    FFlag::LUAU_CONCAT_DOESNT_ALWAYS_RETURN_STRING.register();
    FFlag::LUAU_CONSTRAINT_GRAPH.register();
    FFlag::LUAU_CST_EXPR_GROUP.register();
    FFlag::LUAU_CST_TYPE_GROUP.register();
    FFlag::LUAU_DIRECT_FIELD_GET.register();
    FFlag::LUAU_DISALLOW_REDEFINING_BUILTIN_TYPES.register();
    FFlag::LUAU_EMIT_CALL_FEEDBACK.register();
    FFlag::LUAU_ERROR_TOLERANT_PRETTY_PRINTING.register();
    FFlag::LUAU_EXPLICIT_TYPE_INSTANTIATION_SUPPORT.register();
    FFlag::LUAU_EXPORT_VALUE_SYNTAX.register();
    FFlag::LUAU_EXPORT_VALUE_TYPECHECK.register();
    FFlag::LUAU_EXTERN_TYPES_NORMALIZE_WITH_SHAPES.register();
    FFlag::LUAU_FIX_INDEXER_SUBTYPING_ORDERING.register();
    FFlag::LUAU_FIX_PROP_READS_ON_METATABLE_TYPES.register();
    FFlag::LUAU_FORCE_LESS.register();
    FFlag::LUAU_INSTANTIATE_FUNCTION_TYPE_BEFORE_PUSH.register();
    FFlag::LUAU_INSTANTIATE_IN_SUBTYPING.register();
    FFlag::LUAU_INSTANTIATION_USES_POLARITY.register();
    FFlag::LUAU_INTEGER_BUFFER_FASTCALLS.register();
    FFlag::LUAU_INTEGER_FASTCALLS.register();
    FFlag::LUAU_INTEGER_LIBRARY.register();
    FFlag::LUAU_INTEGER_TYPE2.register();
    FFlag::LUAU_ITERATIVE_INSTANTIATION_QUEUER.register();
    FFlag::LUAU_LVALUE_COMPOUND_ASSIGNMENT_VISIT_LHS.register();
    FFlag::LUAU_LIMIT_UNIFICATION_RECURSION.register();
    FFlag::LUAU_NATIVE_CODE_TARGET_CHECK.register();
    FFlag::LUAU_NON_STRICT_MODE_USE_ERROR_SUPRESSING_TAG.register();
    FFlag::LUAU_OCCURS_CHECK_FOR_ALL_BINDINGS.register();
    FFlag::LUAU_PROPAGATE_FREE_TYPES_INTO_UNION_AND_INTERSECTION_BOUNDS.register();
    FFlag::LUAU_PROPAGATE_TYPE_ANNOTATIONS_IN_FOR_IN_LOOPS.register();
    FFlag::LUAU_PROPERTY_MODIFIER_MISMATCH_ERRORS.register();
    FFlag::LUAU_READ_ONLY_INDEXERS.register();
    FFlag::LUAU_REFINE_NIL_FROM_TABLE_INDEXER_RESULT_TYPE.register();
    FFlag::LUAU_RELAX_CONSTRAINT_ORDERING_FOR_FUNCTION_CHECK.register();
    FFlag::LUAU_REMOVE_CONSTRAINT_SOLVER_EMPLACE.register();
    FFlag::LUAU_REPLACER_IS_SOLVER_AGNOSTIC.register();
    FFlag::LUAU_RESUME_RESTORE_CCALLS.register();
    FFlag::LUAU_SILENCE_DYNAMIC_FORMAT_STRING_ERRORS.register();
    FFlag::LUAU_SINGLE_TYPE_OPTIONAL_PACK_RETURNS_ATTRIBUTE_PARENS.register();
    FFlag::LUAU_SOLVER_V2.register();
    FFlag::LUAU_SUBTYPING_MISSING_PROPERTIES_AS_NIL.register();
    FFlag::LUAU_SUBTYPING_TABLES_HAS_BETTER_ERROR_SUPPRESSION.register();
    FFlag::LUAU_TABLE_ENTRIES_DONT_NEED_TO_MATCH_INDENT.register();
    FFlag::LUAU_TABLE_FREEZE_CHECK_IS_SUBTYPE.register();
    FFlag::LUAU_TIDY_TYPE_PROTOTYPING.register();
    FFlag::LUAU_TWEAK_ACCESS_VIOLATION_REPORTING.register();
    FFlag::LUAU_TYPE_FUNCTION_ROBUSTNESS.register();
    FFlag::LUAU_TYPE_FUNCTION_SERIALIZE_ARG_NAMES.register();
    FFlag::LUAU_TYPE_FUNCTION_STRUCTURED_ERRORS.register();
    FFlag::LUAU_TYPE_FUNCTION_SUPPORTS_FROZEN.register();
    FFlag::LUAU_UDATA_DIRECT_ACCESS6.register();
    FFlag::LUAU_UDTF_TYPE_IS_SUBTYPE_OF.register();
    FFlag::LUAU_UDTF_CREATE_SINGLETON_FIX_ERROR_MESSAGE.register();
    FFlag::LUAU_UDTF_FIX_TYPE_NAME_TYPO.register();
    FFlag::LUAU_USE_NATIVE_STACK_GUARD.register();
    FFlag::LUAU_VISIT_CALL_TYPE_ARGS_IN_DFG.register();
    FFlag::LUAU_XPCALL_FIX_MESSAGE_YIELD_PATH.register();
    FFlag::LUAU_YIELD_ITER2.register();
    FFlag::LUAU_ENUM_MORE_EDGES.register();
  }
}
