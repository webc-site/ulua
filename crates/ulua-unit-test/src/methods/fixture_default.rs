//! `Default` for the test `Fixture` — port of `Fixture::Fixture(bool=false)`
//! (tests/Fixture.cpp:263) plus the in-class member initializers from
//! tests/Fixture.h:170-184.
//!
//! The two `ScopedFastFlag` members both default to `true` in the C++ in-class
//! initializers (`sff_DebugLuauFreezeArena{FFlag::DebugLuauFreezeArena, true}`,
//! `sff_DebugLuauAlwaysShowConstraintSolvingIncomplete{..., true}`).
//!
//! NOTE on the name table / allocator: `AstNameTable::new` stores a raw
//! `*mut Allocator` into the table. We construct against the *local* `allocator`
//! and then move both into the returned struct, so that stored pointer is left
//! dangling at the struct's final address. That is intentional and safe here:
//! every parse entry point (`parse` / `try_parse` / `match_parse_error`) calls
//! `name_table.rebind_allocator(&mut self.allocator)` before interning, so the
//! table always points at the allocator at its *current* address.
use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_analysis::records::{
  internal_error_reporter::InternalErrorReporter, null_module_resolver::NullModuleResolver,
  type_arena::TypeArena,
};
use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};
use ulua_common::FFlag;

use crate::{
  records::{
    fixture::Fixture, test_config_resolver::TestConfigResolver,
    test_file_resolver::TestFileResolver,
  },
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};
impl Default for Fixture {
  fn default() -> Self {
    let mut allocator = Allocator::new();
    let name_table = AstNameTable::new(&mut allocator);

    Fixture {
      sff_debug_luau_freeze_arena: ScopedFastFlag::new(&FFlag::DebugLuauFreezeArena, true),
      sff_debug_luau_always_show_constraint_solving_incomplete: ScopedFastFlag::new(
        &FFlag::DebugLuauAlwaysShowConstraintSolvingIncomplete,
        true,
      ),
      file_resolver: TestFileResolver::default(),
      config_resolver: TestConfigResolver::default(),
      module_resolver: NullModuleResolver,
      source_module: None,
      ice: InternalErrorReporter::default(),
      allocator,
      name_table,
      arena: TypeArena::default(),
      has_dumped_errors: false,
      for_autocomplete: false,
      frontend: None,
      builtin_types: null_mut(),
      dynamic_scoped_ints: Vec::new(),
    }
  }
}
