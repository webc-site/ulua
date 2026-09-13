//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:109:cfg_fixture`
//! Source: `tests/ControlFlowGraph.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/ControlFlowGraph.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/ControlFlowGraph.test.cpp
//!   - type_ref <- method CFGFixture::parse (tests/ControlFlowGraph.test.cpp)
//!   - type_ref <- method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref <- method CFGFixture::getDefinitionAtPos (tests/ControlFlowGraph.test.cpp)
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record Allocator (Ast/include/Luau/Allocator.h)
//!   - type_ref -> record AstNameTable (Ast/include/Luau/Lexer.h)
//!   - type_ref -> record CFGAllocator (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item CFGFixture

use core::ptr::null_mut;

use ulua_analysis::records::cfg_allocator::CfgAllocator;
use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
};
use ulua_common::FFlag;

use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
#[derive(Debug)]
pub struct CfgFixture {
  pub allocator: Allocator,
  pub names: AstNameTable,
  pub cfg_allocator: CfgAllocator,
  pub root: *mut AstStatBlock,
  pub freeze_arena: ScopedFastFlag,
}

impl Default for CfgFixture {
  fn default() -> Self {
    let mut allocator = Allocator::new();
    let names = AstNameTable::new(&mut allocator);

    Self {
      allocator,
      names,
      cfg_allocator: CfgAllocator::default(),
      root: null_mut(),
      freeze_arena: ScopedFastFlag::new(&FFlag::DebugLuauFreezeArena, true),
    }
  }
}

impl CfgFixture {
  pub fn new() -> Self {
    Self::default()
  }
}
