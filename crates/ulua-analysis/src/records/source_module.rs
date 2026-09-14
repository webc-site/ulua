//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Module.h:36:source_module`
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
//!   - type_ref <- record LoadDefinitionFileResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref <- record FrontendOptions (Analysis/include/Luau/Frontend.h)
//!   - type_ref <- record Frontend (Analysis/include/Luau/Frontend.h)
//!   - type_ref <- record GlobalTypes (Analysis/include/Luau/GlobalTypes.h)
//!   - type_ref <- record TypeChecker2 (Analysis/include/Luau/TypeChecker2.h)
//!   - type_ref <- record TypeChecker (Analysis/include/Luau/TypeInfer.h)
//!   - type_ref <- function findAncestryAtPositionForAutocomplete (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findAstAncestryOfPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findNodeAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findExprAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findTypeAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findExpectedTypeAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findBindingLocalStatement (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findBindingAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function findExprOrLocalAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function getDocumentationSymbolAtPosition (Analysis/src/AstQuery.cpp)
//!   - type_ref <- function autocomplete (Analysis/src/Autocomplete.cpp)
//!   - type_ref <- record BuildQueueItem (Analysis/src/Frontend.cpp)
//!   - type_ref <- function parseSourceForModule (Analysis/src/Frontend.cpp)
//!   - type_ref <- method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
//!   - type_ref <- method Frontend::getRequiredScripts (Analysis/src/Frontend.cpp)
//!   - type_ref <- method Frontend::addBuildQueueItems (Analysis/src/Frontend.cpp)
//!   - type_ref <- method Frontend::checkBuildQueueItem (Analysis/src/Frontend.cpp)
//!   - type_ref <- method Frontend::getModuleEnvironment (Analysis/src/Frontend.cpp)
//!   - type_ref <- method Frontend::getSourceModule (Analysis/src/Frontend.cpp)
//!   - type_ref <- method Frontend::getSourceModule (Analysis/src/Frontend.cpp)
//!   - type_ref <- function check (Analysis/src/Frontend.cpp)
//!   - type_ref <- method Frontend::check (Analysis/src/Frontend.cpp)
//!   - type_ref <- method Frontend::getSourceNode (Analysis/src/Frontend.cpp)
//!   - type_ref <- method Frontend::parse (Analysis/src/Frontend.cpp)
//!   - type_ref <- function isWithinComment (Analysis/src/Module.cpp)
//!   - type_ref <- function isWithinHotComment (Analysis/src/Module.cpp)
//!   - type_ref <- function checkNonStrict (Analysis/src/NonStrictTypeChecker.cpp)
//!   - type_ref <- function attachTypeData (Analysis/src/TypeAttach.cpp)
//!   - type_ref <- function check (Analysis/src/TypeChecker2.cpp)
//!   - type_ref <- method TypeChecker2::TypeChecker2 (Analysis/src/TypeChecker2.cpp)
//!   - type_ref <- method TypeChecker::check (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method TypeChecker::checkWithoutRecursionCheck (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- function reportModuleResult (CLI/src/Analyze.cpp)
//!   - type_ref <- method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
//!   - type_ref <- method Fixture::parse (tests/Fixture.cpp)
//!   - type_ref <- method Fixture::tryParse (tests/Fixture.cpp)
//!   - type_ref <- method Fixture::matchParseError (tests/Fixture.cpp)
//!   - type_ref <- method Fixture::matchParseErrorPrefix (tests/Fixture.cpp)
//!   - type_ref <- method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - type_ref <- method Fixture::findTypeAtPosition (tests/Fixture.cpp)
//!   - type_ref <- method Fixture::findTypeAtPosition (tests/Fixture.cpp)
//!   - type_ref <- method Fixture::findExpectedTypeAtPosition (tests/Fixture.cpp)
//!   - type_ref <- method Fixture::decorateWithTypes (tests/Fixture.cpp)
//!   - type_ref <- record Fixture (tests/Fixture.h)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::parseHelper_ (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::parseHelper (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::getSource (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- record FragmentAutocompleteFixtureImpl (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method NormalizeFixture::toNormalizedType (tests/Normalize.test.cpp)
//!   - type_ref <- method SourceModule::SourceModule (Analysis/include/Luau/Module.h)
//! - outgoing:
//!   - type_ref -> method SourceModule::SourceModule (Analysis/include/Luau/Module.h)
//!   - type_ref -> record Allocator (Ast/include/Luau/Allocator.h)
//!   - type_ref -> record AstNameTable (Ast/include/Luau/Lexer.h)
//!   - type_ref -> record ParseError (Ast/include/Luau/ParseResult.h)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
//!   - type_ref -> record HotComment (Ast/include/Luau/ParseResult.h)
//!   - type_ref -> record Comment (Ast/include/Luau/ParseResult.h)
//!   - translates_to -> rust_item SourceModule

// Module.h:36 — hand-ported; field set matches the C++ struct.
use alloc::{string::String, sync::Arc, vec::Vec};

use ulua_ast::{
  enums::mode,
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
    comment::Comment, hot_comment::HotComment, parse_error::ParseError,
  },
};
use ulua_config::type_aliases::module_name::ModuleName;

use crate::enums::type_file_resolver::Type;
#[derive(Debug, Clone)]
pub struct SourceModule {
  pub name: ModuleName, // Module identifier or a filename
  pub human_readable_name: String,
  pub r#type: Type,
  pub environment_name: Option<String>,
  pub cyclic: bool,
  pub allocator: Arc<Allocator>,
  pub names: Arc<AstNameTable>,
  pub parse_errors: Vec<ParseError>,
  pub root: *mut AstStatBlock,
  pub mode: Option<mode::Mode>,
  pub hotcomments: Vec<HotComment>,
  pub comment_locations: Vec<Comment>,
}

unsafe impl Send for SourceModule {}
unsafe impl Sync for SourceModule {}
