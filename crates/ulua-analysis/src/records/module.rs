//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Module.h:76:module`
//! Source: `Analysis/include/Luau/Module.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Module.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Ast/include/Luau/ParseOptions.h
//!   - includes -> source_file Ast/include/Luau/ParseResult.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Module.h
//!   - type_ref <- record FrontendOptions (Analysis/include/Luau/Frontend.h)
//!   - type_ref <- type_alias ModulePtr (Analysis/include/Luau/Module.h)
//!   - type_ref <- type_alias ModulePtr (Analysis/include/Luau/ModuleResolver.h)
//!   - type_ref <- type_alias ModulePtr (Analysis/include/Luau/Normalize.h)
//!   - type_ref <- record UserDefinedFunctionData (Analysis/include/Luau/Type.h)
//!   - type_ref <- record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref <- record TypeChecker2 (Analysis/include/Luau/TypeChecker2.h)
//!   - type_ref <- function findScopeAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findTypeAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findExpectedTypeAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findBindingAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function checkOverloadedDocumentationSymbol (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function getMetatableDocumentation (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function getDocumentationSymbolAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findExpectedTypeAt (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function checkTypeMatch (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function checkTypeCorrectKind (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function autocompleteProps (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function autocompleteProps (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function autocompleteProps (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function autocompleteModuleTypes (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function getLocalTypeInScopeAt (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function functionIsExpectedAt (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function autocompleteTypeNames (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function autocompleteStatement (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function autocompleteExpression (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- function autocompleteExpression (Analysis/src/AutocompleteCore.cpp)
//!   - type_ref <- method ConstraintSolver::resolveModule (Analysis/src/ConstraintSolver.cpp)
//!   - type_ref <- function typecheckFragment_ (Analysis/src/FragmentAutocomplete.cpp)
//!   - type_ref <- function accumulateErrors (Analysis/src/Frontend.cpp)
//!   - type_ref <- function check (Analysis/src/Frontend.cpp)
//!   - type_ref <- method Frontend::parseType (Analysis/src/Frontend.cpp)
//!   - type_ref <- record LintContext (Analysis/src/Linter.cpp)
//!   - type_ref <- function lint (Analysis/src/Linter.cpp)
//!   - type_ref <- record ClonePublicInterface (Analysis/src/Module.cpp)
//!   - type_ref <- method ClonePublicInterface::ClonePublicInterface (Analysis/src/Module.cpp)
//!   - type_ref <- method Module::~Module (Analysis/src/Module.cpp)
//!   - type_ref <- function synthesizeExportReturn (Analysis/src/Module.cpp)
//!   - type_ref <- record NonStrictTypeChecker (Analysis/src/NonStrictTypeChecker.cpp)
//!   - type_ref <- method NonStrictTypeChecker::NonStrictTypeChecker (Analysis/src/NonStrictTypeChecker.cpp)
//!   - type_ref <- function checkNonStrict (Analysis/src/NonStrictTypeChecker.cpp)
//!   - type_ref <- method TypeAttacher::TypeAttacher (Analysis/src/TypeAttach.cpp)
//!   - type_ref <- record TypeAttacher (Analysis/src/TypeAttach.cpp)
//!   - type_ref <- function attachTypeData (Analysis/src/TypeAttach.cpp)
//!   - type_ref <- function check (Analysis/src/TypeChecker2.cpp)
//!   - type_ref <- method TypeChecker2::TypeChecker2 (Analysis/src/TypeChecker2.cpp)
//!   - type_ref <- method TypeChecker::checkWithoutRecursionCheck (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method TypeChecker::checkRequire (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method CliFileResolver::readSource (CLI/src/Analyze.cpp)
//!   - type_ref <- method TestFileResolver::readSource (tests/Fixture.cpp)
//!   - type_ref <- method Fixture::dumpErrors (tests/Fixture.cpp)
//!   - type_ref <- record Fixture (tests/Fixture.h)
//!   - type_ref <- method DemoFileResolver::readSource (CLI/src/Web.cpp)
//!   - type_ref <- method Module::clonePublicInterface (Analysis/src/Module.cpp)
//!   - type_ref <- method Module::hasModuleScope (Analysis/src/Module.cpp)
//!   - type_ref <- method Module::getModuleScope (Analysis/src/Module.cpp)
//! - outgoing:
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record Allocator (Ast/include/Luau/Allocator.h)
//!   - type_ref -> record AstNameTable (Ast/include/Luau/Lexer.h)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> type_alias ScopePtr (Analysis/include/Luau/Module.h)
//!   - type_ref -> record DenseHashMap (Common/include/Luau/DenseHash.h)
//!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstType (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypePack (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
//!   - type_ref -> record Scope (Analysis/include/Luau/Scope.h)
//!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias ErrorVec (Analysis/include/Luau/Error.h)
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
//!   - type_ref -> record DefArena (Analysis/include/Luau/Def.h)
//!   - type_ref -> record RefinementKeyArena (Analysis/include/Luau/DataFlowGraph.h)
//!   - type_ref -> record BuiltinTypes (Analysis/include/Luau/Type.h)
//!   - type_ref -> record InternalErrorReporter (Analysis/include/Luau/Error.h)
//!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item Module

use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use core::ptr::{null, null_mut};
use std::collections::HashMap;

use ulua_ast::{
  enums::mode::Mode,
  records::{
    allocator::Allocator, ast_expr::AstExpr, ast_name_table::AstNameTable, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_type::AstType, ast_type_pack::AstTypePack,
    location::Location,
  },
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::type_file_resolver::Type,
  records::{
    def_arena::DefArena, lint_result::LintResult, refinement_key_arena::RefinementKeyArena,
    scope::Scope, r#type, type_arena::TypeArena, type_fun::TypeFun,
  },
  type_aliases::{
    error_vec::ErrorVec, module_name_type::ModuleName, name_type::Name, scope_ptr_type::ScopePtr,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};
#[derive(Debug)]
pub struct Module {
  pub checked_in_new_solver: bool,

  pub name: ModuleName,
  pub human_readable_name: String,

  pub interface_types: TypeArena,
  pub internal_types: TypeArena,

  pub allocator: Option<Arc<Allocator>>,
  pub names: Option<Arc<AstNameTable>>,
  pub root: *mut AstStatBlock,

  pub scopes: Vec<(Location, ScopePtr)>,

  pub ast_types: DenseHashMap<*const AstExpr, TypeId>,
  pub ast_type_packs: DenseHashMap<*const AstExpr, TypePackId>,
  pub ast_expected_types: DenseHashMap<*const AstExpr, TypeId>,

  pub ast_original_call_types: DenseHashMap<*const AstNode, TypeId>,
  pub ast_overload_resolved_types: DenseHashMap<*const AstNode, TypeId>,
  pub ast_for_in_next_types: DenseHashMap<*const AstNode, TypeId>,

  pub ast_resolved_types: DenseHashMap<*const AstType, TypeId>,
  pub ast_resolved_type_packs: DenseHashMap<*const AstTypePack, TypePackId>,

  pub ast_compound_assign_result_types: DenseHashMap<*const AstStat, TypeId>,

  pub upper_bound_contributors: DenseHashMap<TypeId, Vec<(Location, TypeId)>>,

  pub ast_scopes: DenseHashMap<*const AstNode, *mut Scope>,

  pub type_function_aliases: Vec<Box<TypeFun>>,

  pub declared_globals: HashMap<Name, TypeId>,
  pub errors: ErrorVec,
  pub lint_result: LintResult,
  pub mode: Mode,
  pub r#type: Type,
  pub check_duration_sec: f64,
  pub timeout: bool,
  pub cancelled: bool,

  pub return_type: TypePackId,
  pub exported_type_bindings: HashMap<Name, TypeFun>,

  pub def_arena: DefArena,
  pub key_arena: RefinementKeyArena,

  pub constraint_generation_did_not_complete: bool,
}

impl Default for Module {
  fn default() -> Self {
    // Faithful port of the C++ `Module` default ctor (`Module.h:76`), which
    // default-initializes every member via its in-class initializer.
    Self {
      checked_in_new_solver: false,

      name: ModuleName::new(),
      human_readable_name: String::new(),

      interface_types: TypeArena::default(),
      internal_types: TypeArena::default(),

      allocator: None,
      names: None,
      root: null_mut(),

      scopes: Vec::new(),

      // `DenseHashMap<...>{nullptr}` — pointer-keyed maps use the null
      // sentinel as their empty key.
      ast_types: DenseHashMap::new(null::<AstExpr>()),
      ast_type_packs: DenseHashMap::new(null::<AstExpr>()),
      ast_expected_types: DenseHashMap::new(null::<AstExpr>()),

      ast_original_call_types: DenseHashMap::new(null::<AstNode>()),
      ast_overload_resolved_types: DenseHashMap::new(null::<AstNode>()),
      ast_for_in_next_types: DenseHashMap::new(null::<AstNode>()),

      ast_resolved_types: DenseHashMap::new(null::<AstType>()),
      ast_resolved_type_packs: DenseHashMap::new(null::<AstTypePack>()),

      ast_compound_assign_result_types: DenseHashMap::new(null::<AstStat>()),

      upper_bound_contributors: DenseHashMap::new(null::<r#type::Type>()),

      ast_scopes: DenseHashMap::new(null::<AstNode>()),

      type_function_aliases: Vec::new(),

      declared_globals: HashMap::new(),
      errors: ErrorVec::new(),
      lint_result: LintResult::default(),
      // C++ `Mode mode;` has no in-class initializer; the first enumerator
      // (`NoCheck`) is the deterministic zero-init value.
      mode: Mode::NoCheck,
      // C++ `SourceCode::Type type;` — first enumerator is `None`.
      r#type: Type::None,
      check_duration_sec: 0.0,
      timeout: false,
      cancelled: false,

      // `TypePackId returnType = nullptr;` — TypePackId is `*const TypePackVar`.
      return_type: null(),
      exported_type_bindings: HashMap::new(),

      def_arena: DefArena::default(),
      key_arena: RefinementKeyArena::default(),

      constraint_generation_did_not_complete: true,
    }
  }
}

unsafe impl Send for Module {}
unsafe impl Sync for Module {}
