extern crate alloc;

pub mod enums;
pub mod functions;
pub mod macros;
pub mod methods;
pub mod records;
pub mod type_aliases;

/// Minimal libc surface for wasm. On `wasm32-unknown-unknown` (no libc) every
/// shim is needed; on libc-bearing wasm (e.g. `wasm32-wasip1`, used to run the
/// suite on a 32-bit-pointer platform) the allocator shims are gated off inside
/// the module so they don't clash with wasi-libc's, while the functions wasi
/// lacks (mmap stubs, etc.) are still provided.
#[cfg(target_arch = "wasm32")]
pub mod wasm_libc;

/// Pure-Rust `strtod` shim for wasm (no libc on `wasm32-unknown-unknown`). The
/// scanning core is unit-tested natively, so the module is also compiled under
/// `test`; only the `#[unsafe(no_mangle)]` C entry point is wasm-gated.
#[cfg(any(target_arch = "wasm32", test))]
pub mod strtod_shim;

// C++ exposes this at namespace scope; codegen_assert! and friends reference
// `ulua_common::assert_call_handler` directly.
pub use functions::assert_call_handler::assert_call_handler;
pub use records::f_value::set_luau_bool_flags;

/// C++ CLI `setLuauFlagsDefault(value)` analog: set every non-Debug FFlag.
/// (Rust statics cannot self-register, so the list is generated explicitly.)
pub fn set_all_flags(value: bool) {
  FFlag::DesugaredArrayTypeReferenceIsEmpty.set(value);
  FFlag::FixMathNoisePrecision.set(value);
  FFlag::LuauAddRecursionCounterToNonStrictTypeChecker.set(value);
  FFlag::LuauAllowGlobalDeclarationToBeCalledClass.set(value);
  FFlag::LuauAlsoInstantiateInferredArguments.set(value);
  FFlag::LuauAutocompleteConst.set(value);
  FFlag::LuauAutocompleteExport.set(value);
  FFlag::LuauAutocompleteStringSingletonIntersection.set(value);
  FFlag::LuauBidirectionalInferenceBetterUnionHandling.set(value);
  FFlag::LuauCallFeedback.set(value);
  FFlag::LuauCheckFunctionStatementTypes.set(value);
  FFlag::LuauClosureUsageCounter.set(value);
  FFlag::LuauCodeGenCallWrapperEmitInst.set(value);
  FFlag::LuauCodegenBufferInteger.set(value);
  FFlag::LuauCodegenDsePtrStoreTagCheck.set(value);
  FFlag::LuauCodegenDseRestoreHints.set(value);
  FFlag::LuauCodegenExtraTableOpts.set(value);
  FFlag::LuauCodegenFixBufferLenCheck.set(value);
  FFlag::LuauCodegenForwardRematerialize.set(value);
  FFlag::LuauCodegenFreeBlocks.set(value);
  FFlag::LuauCodegenInteger2.set(value);
  FFlag::LuauCodegenIntegerArg3Fix.set(value);
  FFlag::LuauCodegenIntegerFastcall2k.set(value);
  FFlag::LuauCodegenLinearSetupEntryState3.set(value);
  FFlag::LuauCodegenLoadPropagateOrigin.set(value);
  FFlag::LuauCodegenNopPadding.set(value);
  FFlag::LuauCodegenProtectData.set(value);
  FFlag::LuauCodegenRecordAllBlockExitInfo.set(value);
  FFlag::LuauCodegenRegTag2.set(value);
  FFlag::LuauCodegenSuggestArgumentRegisterX64.set(value);
  FFlag::LuauCodegenVmExitSync.set(value);
  FFlag::LuauCodegenVmExitSyncFix.set(value);
  FFlag::LuauCompileDuptableConstantPack2.set(value);
  FFlag::LuauCodegenDseRestoreHintUpdate.set(value);
  FFlag::LuauCompileFastcall3CostModel.set(value);
  FFlag::LuauCompileFoldOptimize.set(value);
  FFlag::LuauCompileInlineTableFunctions.set(value);
  FFlag::LuauCompileNewTableMutationTracker.set(value);
  FFlag::LuauCompileNoOptNext.set(value);
  FFlag::LuauCompilePropagateTableProps2.set(value);
  FFlag::LuauCompileConcatTargetTop.set(value);
  FFlag::LuauCompileStringInterpTargetTop.set(value);
  FFlag::LuauCompileTypeAliases.set(value);
  FFlag::LuauCompileUdataDirect.set(value);
  FFlag::LuauConcatDoesntAlwaysReturnString.set(value);
  FFlag::LuauConstraintGraph.set(value);
  FFlag::LuauCstExprGroup.set(value);
  FFlag::LuauCstTypeGroup.set(value);
  FFlag::LuauDirectFieldGet.set(value);
  FFlag::LuauDisallowRedefiningBuiltinTypes.set(value);
  FFlag::LuauEmitCallFeedback.set(value);
  FFlag::LuauErrorTolerantPrettyPrinting.set(value);
  FFlag::LuauExplicitTypeInstantiationSupport.set(value);
  // Experimental "export values" syntax is intentionally NOT enabled here: it
  // is incomplete in this port — a Closure that captures an exported local
  // mis-compiles the upvalue register (the C++ reference handles it), so it can
  // produce out-of-range bytecode. Keep it off (default false) until the
  // export-table/Closure codegen is fixed. Tests that exercise it set the flag
  // explicitly via a scoped override.
  FFlag::LuauExportValueSyntax.set(false);
  FFlag::LuauExportValueTypecheck.set(false);
  FFlag::LuauExternTypesNormalizeWithShapes.set(value);
  FFlag::LuauFixIndexerSubtypingOrdering.set(value);
  FFlag::LuauFixPropReadsOnMetatableTypes.set(value);
  FFlag::LuauInstantiateFunctionTypeBeforePush.set(value);
  FFlag::LuauInstantiateInSubtyping.set(value);
  FFlag::LuauInstantiationUsesPolarity.set(value);
  FFlag::LuauIntegerBufferFastcalls.set(value);
  FFlag::LuauIntegerFastcalls.set(value);
  FFlag::LuauIntegerLibrary.set(value);
  FFlag::LuauIntegerType2.set(value);
  FFlag::LuauIterativeInstantiationQueuer.set(value);
  FFlag::LuauKnowsTheDataModel3.set(value);
  FFlag::LuauLValueCompoundAssignmentVisitLhs.set(value);
  FFlag::LuauLimitUnificationRecursion.set(value);
  FFlag::LuauNativeCodeTargetCheck.set(value);
  FFlag::LuauNonStrictModeUseErrorSupressingTag.set(value);
  FFlag::LuauOccursCheckForAllBindings.set(value);
  FFlag::LuauPropagateFreeTypesIntoUnionAndIntersectionBounds.set(value);
  FFlag::LuauPropagateTypeAnnotationsInForInLoops.set(value);
  FFlag::LuauPropertyModifierMismatchErrors.set(value);
  FFlag::LuauReadOnlyIndexers.set(value);
  FFlag::LuauRefineNilFromTableIndexerResultType.set(value);
  FFlag::LuauRemoveConstraintSolverEmplace.set(value);
  FFlag::LuauReplacerIsSolverAgnostic.set(value);
  FFlag::LuauRequireResolveAliasNullCheck.set(value);
  FFlag::LuauResumeRestoreCcalls.set(value);
  FFlag::LuauSilenceDynamicFormatStringErrors.set(value);
  FFlag::LuauSingleTypeOptionalPackReturnsAttributeParens.set(value);
  FFlag::LuauSolverV2.set(value);
  FFlag::LuauSubtypingMissingPropertiesAsNil.set(value);
  FFlag::LuauSubtypingTablesHasBetterErrorSuppression.set(value);
  FFlag::LuauTableEntriesDontNeedToMatchIndent.set(value);
  FFlag::LuauTableFreezeCheckIsSubtype.set(value);
  FFlag::LuauTidyTypePrototyping.set(value);
  FFlag::LuauTransitiveSubtyping.set(value);
  FFlag::LuauTweakAccessViolationReporting.set(value);
  FFlag::LuauTypeFunctionRobustness.set(value);
  FFlag::LuauTypeFunctionSerializeArgNames.set(value);
  FFlag::LuauTypeFunctionStructuredErrors.set(value);
  FFlag::LuauTypeFunctionSupportsFrozen.set(value);
  FFlag::LuauUdataDirectAccess6.set(value);
  FFlag::LuauUdtfTypeIsSubtypeOf.set(value);
  FFlag::LuauUseNativeStackGuard.set(value);
  FFlag::LuauVisitCallTypeArgsInDfg.set(value);
  FFlag::LuauYieldIter2.set(value);
  FFlag::LuauXpcallFixMessageYieldPath.set(value);
  FFlag::LuauEnumMoreEdges.set(value);
}

/// FastFlag namespace `FFlag::` — static (non-dynamic) bool flags. Definitions
/// from `LUAU_FASTFLAGVARIABLE(...)` across this crate's sources are collected
/// here so C++ reads `FFlag::Name` map to `crate::FFlag::Name.get()`. (Rust
/// modules are not open like C++ namespaces, so the per-crate namespace module
/// is the aggregation point — see `crate::macros::luau_fastflagvariable`.)
pub mod fflag {
  pub mod _inner {
    // CodeGen/src/IrRegAllocA64.cpp
    crate::LUAU_FASTFLAGVARIABLE!(DEBUG_CODEGEN_CHAOS_A64, DebugCodegenChaosA64);
    // CodeGen/src/CodeGen.cpp
    crate::LUAU_FASTFLAGVARIABLE!(DEBUG_CODEGEN_OPT_SIZE, DebugCodegenOptSize);
    // CodeGen/src/CodeGen.cpp
    crate::LUAU_FASTFLAGVARIABLE!(DEBUG_CODEGEN_SKIP_NUMBERING, DebugCodegenSkipNumbering);
    // Analysis/src/FragmentAutocomplete.cpp
    crate::LUAU_FASTFLAGVARIABLE!(
      DEBUG_LOG_FRAGMENTS_FROM_AUTOCOMPLETE,
      DebugLogFragmentsFromAutocomplete
    );
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
    crate::LUAU_FASTFLAGVARIABLE!(
      DEBUG_LUAU_FREEZE_DURING_UNIFICATION,
      DebugLuauFreezeDuringUnification
    );
    // Analysis/src/ConstraintSolver.cpp
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
    // Analysis/src/TypeFunction.cpp
    crate::LUAU_FASTFLAGVARIABLE!(DEBUG_LUAU_LOG_TYPE_FAMILIES, DebugLuauLogTypeFamilies);
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
    // Ast/src/Parser.cpp
    crate::LUAU_FASTFLAGVARIABLE!(
      DESUGARED_ARRAY_TYPE_REFERENCE_IS_EMPTY,
      DesugaredArrayTypeReferenceIsEmpty
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
    // CodeGen/src/CodeGen.cpp
    crate::LUAU_FASTFLAGVARIABLE!(LUAU_CODEGEN_NOP_PADDING, LuauCodegenNopPadding);
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
    // Compiler/src/Compiler.cpp
    crate::LUAU_FASTFLAGVARIABLE!(LUAU_COMPILE_NO_OPT_NEXT, LuauCompileNoOptNext);
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
    // Analysis/src/Frontend.cpp
    crate::LUAU_FASTFLAGVARIABLE!(LUAU_KNOWS_THE_DATA_MODEL3, LuauKnowsTheDataModel3);
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
      LUAU_REMOVE_CONSTRAINT_SOLVER_EMPLACE,
      LuauRemoveConstraintSolverEmplace
    );
    // Analysis/src/Instantiation.cpp
    crate::LUAU_FASTFLAGVARIABLE!(
      LUAU_REPLACER_IS_SOLVER_AGNOSTIC,
      LuauReplacerIsSolverAgnostic
    );
    // Require/src/RequireNavigator.cpp
    crate::LUAU_FASTFLAGVARIABLE!(
      LUAU_REQUIRE_RESOLVE_ALIAS_NULL_CHECK,
      LuauRequireResolveAliasNullCheck
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
    // Analysis/src/Unifier.cpp
    crate::LUAU_FASTFLAGVARIABLE!(LUAU_TRANSITIVE_SUBTYPING, LuauTransitiveSubtyping);
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
    DEBUG_CODEGEN_SKIP_NUMBERING as DebugCodegenSkipNumbering,
    DEBUG_LOG_FRAGMENTS_FROM_AUTOCOMPLETE as DebugLogFragmentsFromAutocomplete,
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
    DEBUG_LUAU_FREEZE_ARENA as DebugLuauFreezeArena,
    DEBUG_LUAU_FREEZE_DURING_UNIFICATION as DebugLuauFreezeDuringUnification,
    DEBUG_LUAU_LOG_BINDINGS as DebugLuauLogBindings, DEBUG_LUAU_LOG_CFG as DebugLuauLogCFG,
    DEBUG_LUAU_LOG_SOLVER as DebugLuauLogSolver,
    DEBUG_LUAU_LOG_SOLVER_TO_JSON as DebugLuauLogSolverToJson,
    DEBUG_LUAU_LOG_SOLVER_TO_JSON_FILE as DebugLuauLogSolverToJsonFile,
    DEBUG_LUAU_LOG_TYPE_FAMILIES as DebugLuauLogTypeFamilies,
    DEBUG_LUAU_MAGIC_TYPES as DebugLuauMagicTypes,
    DEBUG_LUAU_MAGIC_VARIABLE_NAMES as DebugLuauMagicVariableNames,
    DEBUG_LUAU_NO_INLINE as DebugLuauNoInline,
    DEBUG_LUAU_SUBTYPING_CHECK_PATH_VALIDITY as DebugLuauSubtypingCheckPathValidity,
    DEBUG_LUAU_TIME_TRACING as DebugLuauTimeTracing,
    DEBUG_LUAU_TO_STRING_NO_LEXICAL_SORT as DebugLuauToStringNoLexicalSort,
    DEBUG_LUAU_USER_DEFINED_CLASSES as DebugLuauUserDefinedClasses,
    DEBUG_LUAU_USER_DEFINED_CLASSES_RUNTIME as DebugLuauUserDefinedClassesRuntime,
    DESUGARED_ARRAY_TYPE_REFERENCE_IS_EMPTY as DesugaredArrayTypeReferenceIsEmpty,
    FIX_MATH_NOISE_PRECISION as FixMathNoisePrecision,
    LUAU_ADD_RECURSION_COUNTER_TO_NON_STRICT_TYPE_CHECKER as LuauAddRecursionCounterToNonStrictTypeChecker,
    LUAU_ALLOW_GLOBAL_DECLARATION_TO_BE_CALLED_CLASS as LuauAllowGlobalDeclarationToBeCalledClass,
    LUAU_ALSO_INSTANTIATE_INFERRED_ARGUMENTS as LuauAlsoInstantiateInferredArguments,
    LUAU_AUTOCOMPLETE_CONST as LuauAutocompleteConst,
    LUAU_AUTOCOMPLETE_EXPORT as LuauAutocompleteExport,
    LUAU_AUTOCOMPLETE_STRING_SINGLETON_INTERSECTION as LuauAutocompleteStringSingletonIntersection,
    LUAU_BIDIRECTIONAL_INFERENCE_BETTER_UNION_HANDLING as LuauBidirectionalInferenceBetterUnionHandling,
    LUAU_CALL_FEEDBACK as LuauCallFeedback,
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
    LUAU_CODEGEN_NOP_PADDING as LuauCodegenNopPadding,
    LUAU_CODEGEN_PROTECT_DATA as LuauCodegenProtectData,
    LUAU_CODEGEN_RECORD_ALL_BLOCK_EXIT_INFO as LuauCodegenRecordAllBlockExitInfo,
    LUAU_CODEGEN_REG_TAG2 as LuauCodegenRegTag2,
    LUAU_CODEGEN_SUGGEST_ARGUMENT_REGISTER_X64 as LuauCodegenSuggestArgumentRegisterX64,
    LUAU_CODEGEN_VM_EXIT_SYNC as LuauCodegenVmExitSync,
    LUAU_CODEGEN_VM_EXIT_SYNC_FIX as LuauCodegenVmExitSyncFix,
    LUAU_COMPILE_CONCAT_TARGET_TOP as LuauCompileConcatTargetTop,
    LUAU_COMPILE_DUPTABLE_CONSTANT_PACK2 as LuauCompileDuptableConstantPack2,
    LUAU_COMPILE_FASTCALL3_COST_MODEL as LuauCompileFastcall3CostModel,
    LUAU_COMPILE_FOLD_OPTIMIZE as LuauCompileFoldOptimize,
    LUAU_COMPILE_INLINE_TABLE_FUNCTIONS as LuauCompileInlineTableFunctions,
    LUAU_COMPILE_NEW_TABLE_MUTATION_TRACKER as LuauCompileNewTableMutationTracker,
    LUAU_COMPILE_NO_OPT_NEXT as LuauCompileNoOptNext,
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
    LUAU_INSTANTIATE_FUNCTION_TYPE_BEFORE_PUSH as LuauInstantiateFunctionTypeBeforePush,
    LUAU_INSTANTIATE_IN_SUBTYPING as LuauInstantiateInSubtyping,
    LUAU_INSTANTIATION_USES_POLARITY as LuauInstantiationUsesPolarity,
    LUAU_INTEGER_BUFFER_FASTCALLS as LuauIntegerBufferFastcalls,
    LUAU_INTEGER_FASTCALLS as LuauIntegerFastcalls, LUAU_INTEGER_LIBRARY as LuauIntegerLibrary,
    LUAU_INTEGER_TYPE2 as LuauIntegerType2,
    LUAU_ITERATIVE_INSTANTIATION_QUEUER as LuauIterativeInstantiationQueuer,
    LUAU_KNOWS_THE_DATA_MODEL3 as LuauKnowsTheDataModel3,
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
    LUAU_REMOVE_CONSTRAINT_SOLVER_EMPLACE as LuauRemoveConstraintSolverEmplace,
    LUAU_REPLACER_IS_SOLVER_AGNOSTIC as LuauReplacerIsSolverAgnostic,
    LUAU_REQUIRE_RESOLVE_ALIAS_NULL_CHECK as LuauRequireResolveAliasNullCheck,
    LUAU_RESUME_RESTORE_CCALLS as LuauResumeRestoreCcalls,
    LUAU_SILENCE_DYNAMIC_FORMAT_STRING_ERRORS as LuauSilenceDynamicFormatStringErrors,
    LUAU_SINGLE_TYPE_OPTIONAL_PACK_RETURNS_ATTRIBUTE_PARENS as LuauSingleTypeOptionalPackReturnsAttributeParens,
    LUAU_SOLVER_V2 as LuauSolverV2,
    LUAU_SUBTYPING_MISSING_PROPERTIES_AS_NIL as LuauSubtypingMissingPropertiesAsNil,
    LUAU_SUBTYPING_TABLES_HAS_BETTER_ERROR_SUPPRESSION as LuauSubtypingTablesHasBetterErrorSuppression,
    LUAU_TABLE_ENTRIES_DONT_NEED_TO_MATCH_INDENT as LuauTableEntriesDontNeedToMatchIndent,
    LUAU_TABLE_FREEZE_CHECK_IS_SUBTYPE as LuauTableFreezeCheckIsSubtype,
    LUAU_TIDY_TYPE_PROTOTYPING as LuauTidyTypePrototyping,
    LUAU_TRANSITIVE_SUBTYPING as LuauTransitiveSubtyping,
    LUAU_TWEAK_ACCESS_VIOLATION_REPORTING as LuauTweakAccessViolationReporting,
    LUAU_TYPE_FUNCTION_ROBUSTNESS as LuauTypeFunctionRobustness,
    LUAU_TYPE_FUNCTION_SERIALIZE_ARG_NAMES as LuauTypeFunctionSerializeArgNames,
    LUAU_TYPE_FUNCTION_STRUCTURED_ERRORS as LuauTypeFunctionStructuredErrors,
    LUAU_TYPE_FUNCTION_SUPPORTS_FROZEN as LuauTypeFunctionSupportsFrozen,
    LUAU_UDATA_DIRECT_ACCESS6 as LuauUdataDirectAccess6,
    LUAU_UDTF_FIX_TYPE_NAME_TYPO as LuauUdtfFixTypeNameTypo,
    LUAU_UDTF_TYPE_IS_SUBTYPE_OF as LuauUdtfTypeIsSubtypeOf,
    LUAU_USE_NATIVE_STACK_GUARD as LuauUseNativeStackGuard,
    LUAU_VISIT_CALL_TYPE_ARGS_IN_DFG as LuauVisitCallTypeArgsInDfg,
    LUAU_XPCALL_FIX_MESSAGE_YIELD_PATH as LuauXpcallFixMessageYieldPath,
    LUAU_YIELD_ITER2 as LuauYieldIter2, *,
  };
}
pub use fflag as FFlag;

/// Static int FastFlags, mirroring `FFlag`. C++ collects every
/// `LUAU_FASTINTVARIABLE(...)` into `namespace FInt`; Rust modules aren't open,
/// so the consumers' flags are gathered here. Read as `FInt::Flag.get()`.
pub mod fint {
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
    // VM/src/lfunc.cpp
    crate::LUAU_FASTINTVARIABLE!(LUAU_INLINE_HITS_THRESHOLD, LuauInlineHitsThreshold, 3);
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
}
pub use fint as FInt;

/// Dynamic bool flags (`DFFlag::`), mirroring `FFlag`.
pub mod dfflag {
  pub mod _inner {
    // CodeGen/src/EmitCommonX64.cpp
    crate::LUAU_DYNAMIC_FASTFLAGVARIABLE!(
      ADD_RETURN_EXECTARGET_CHECK,
      AddReturnExectargetCheck,
      false
    );
    // Ast/src/Parser.cpp
    crate::LUAU_DYNAMIC_FASTFLAGVARIABLE!(
      DEBUG_LUAU_REPORT_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX,
      DebugLuauReportReturnTypeVariadicWithTypeSuffix,
      false
    );
    // Require/src/RequireNavigator.cpp
    crate::LUAU_DYNAMIC_FASTFLAGVARIABLE!(
      LUAU_REQUIRE_ALIAS_OVERRIDE_ORDER_FIX,
      LuauRequireAliasOverrideOrderFix,
      false
    );
  }
  pub use _inner::{
    ADD_RETURN_EXECTARGET_CHECK as AddReturnExectargetCheck,
    DEBUG_LUAU_REPORT_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX as DebugLuauReportReturnTypeVariadicWithTypeSuffix,
    LUAU_REQUIRE_ALIAS_OVERRIDE_ORDER_FIX as LuauRequireAliasOverrideOrderFix, *,
  };
}
pub use dfflag as DFFlag;

/// Dynamic int flags (`DFInt::`), mirroring `FInt`.
pub mod dfint {
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
}
pub use dfint as DFInt;

#[cfg(test)]
mod fastflag_timetrace_tests {
  use crate::{
    FFlag::DebugLuauTimeTracing,
    LUAU_TIMETRACE_ARGUMENT, LUAU_TIMETRACE_OPTIONAL_TAIL_SCOPE, LUAU_TIMETRACE_SCOPE,
    functions::{
      create_scope_data::create_scope_data, create_token::create_token,
      get_global_context::get_global_context,
    },
    records::thread_context::ThreadContext,
  };

  /// The macro-defined flag reads its default; the TimeTrace consumer macros
  /// expand cleanly as no-ops (default `LUAU_ENABLE_TIME_TRACE` off).
  #[test]
  fn flag_default_and_timetrace_noops() {
    assert!(!DebugLuauTimeTracing.get());
    LUAU_TIMETRACE_SCOPE!("name", "category");
    LUAU_TIMETRACE_OPTIONAL_TAIL_SCOPE!("name", "category", 100);
    LUAU_TIMETRACE_ARGUMENT!("k", "v");
    DebugLuauTimeTracing.set(true);
    assert!(DebugLuauTimeTracing.get());
  }

  #[test]
  fn timetrace_token_and_scope_data() {
    let tok_id = create_scope_data("testScope", "Category");
    let ctx = get_global_context();
    let tok_id2 = create_token(&ctx, "testScope2", "Category2");
    assert_ne!(tok_id, tok_id2);

    let mut thread_ctx = ThreadContext::new();
    thread_ctx.event_enter_u16(tok_id);
    thread_ctx.event_argument("arg_name", "arg_value");
    thread_ctx.event_leave();
    assert_eq!(thread_ctx.events.len(), 4);
  }
}
