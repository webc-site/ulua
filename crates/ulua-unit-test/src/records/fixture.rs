//! Node: `cxx:Record:Luau.UnitTest:tests/Fixture.h:116:fixture`
//! Source: `tests/Fixture.h:116-211` (hand-ported, fields only — methods live
//! in their own node files under methods/fixture_*.rs)

use alloc::{boxed::Box, vec::Vec};

use ulua_analysis::records::{
  builtin_types::BuiltinTypes, frontend::Frontend, internal_error_reporter::InternalErrorReporter,
  null_module_resolver::NullModuleResolver, source_module::SourceModule, type_arena::TypeArena,
};
use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};

use crate::{
  records::{test_config_resolver::TestConfigResolver, test_file_resolver::TestFileResolver},
  type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
};

/// C++ `struct Fixture` — the base test fixture every *.test.cpp derives from.
#[derive(Debug)]
pub struct Fixture {
  pub dynamic_scoped_ints: Vec<ScopedFastInt>,

  pub builtin_types: *mut BuiltinTypes,
  pub frontend: Option<Frontend>,
  pub for_autocomplete: bool,

  pub has_dumped_errors: bool,

  pub arena: TypeArena,
  pub name_table: AstNameTable,
  pub allocator: Allocator,
  pub ice: InternalErrorReporter,
  /// C++ `std::unique_ptr<SourceModule>` (null until a parse happens).
  pub source_module: Option<Box<SourceModule>>,
  pub module_resolver: NullModuleResolver,
  pub config_resolver: TestConfigResolver,
  pub file_resolver: TestFileResolver,

  // ScopedFastFlag members are declared first in C++, so their destructors run
  // last. Rust drops fields in declaration order, so keep them at the end.
  pub sff_debug_luau_always_show_constraint_solving_incomplete: ScopedFastFlag,
  pub sff_debug_luau_freeze_arena: ScopedFastFlag,
}
