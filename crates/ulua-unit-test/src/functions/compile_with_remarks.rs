//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Function:Luau.UnitTest:tests/Compiler.test.cpp:146:compile_with_remarks`
//! Source: `tests/Compiler.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Compiler.test.cpp
//! - source_includes:
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Compiler.test.cpp
//!   - calls <- test compiler_cost_model_remarks (tests/Compiler.test.cpp)
//! - outgoing:
//!   - type_ref -> record BytecodeBuilder (Bytecode/include/Luau/BytecodeBuilder.h)
//!   - calls -> method BytecodeBuilder::setDumpFlags (Bytecode/include/Luau/BytecodeBuilder.h)
//!   - calls -> method BytecodeBuilder::setDumpSource (Bytecode/src/BytecodeBuilder.cpp)
//!   - type_ref -> record CompileOptions (Compiler/include/Luau/Compiler.h)
//!   - calls -> method BytecodeBuilder::dumpSourceRemarks (Bytecode/src/BytecodeBuilder.cpp)
//!   - translates_to -> rust_item compileWithRemarks

use alloc::string::String;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};
pub fn compile_with_remarks(source: &str) -> String {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.set_dump_flags(BytecodeBuilder::DUMP_SOURCE | BytecodeBuilder::DUMP_REMARKS);
  bcb.set_dump_source(source);

  let options = CompileOptions {
    optimization_level: 2,
    ..Default::default()
  };
  let owned = String::from(source);
  let parse_options = ParseOptions::default();
  compile_or_throw_bytecode_builder_string_compile_options_parse_options(
    &mut bcb,
    &owned,
    &options,
    &parse_options,
  );

  bcb.dump_source_remarks()
}
