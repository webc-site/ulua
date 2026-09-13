use alloc::string::String;
use core::ffi::{CStr, c_char, c_void};
use std::{
  io::{Write, stdout},
  panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_cli_lib::functions::read_file::read_file;
use ulua_code_gen::{
  enums::{
    code_gen_flags::CodeGenFlags, include_cfg_info::IncludeCfgInfo,
    include_ir_prefix::IncludeIrPrefix, include_reg_flow_info::IncludeRegFlowInfo,
    include_use_info::IncludeUseInfo, target::Target,
  },
  records::{assembly_options::AssemblyOptions, compilation_options::CompilationOptions},
};
use ulua_common::functions::get_clock::get_clock;
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
  records::compile_error::CompileError,
};

use crate::{
  enums::compile_format::CompileFormat,
  functions::{
    annotate_instruction::annotate_instruction, copts::copts,
    get_codegen_assembly::get_codegen_assembly, record_delta_time::record_delta_time,
    report_error_compile::report_error_c_char_luau_parse_error,
    report_error_compile_alt_b::report_error_c_char_luau_compile_error,
  },
  records::{compile_stats::CompileStats, global_options::GLOBAL_OPTIONS},
};

pub fn compile_file(
  name: *const c_char,
  format: CompileFormat,
  assembly_target: Target,
  stats: &mut CompileStats,
  dump_constants: bool,
) -> bool {
  let mut currts = get_clock();
  let name_cstr = unsafe { CStr::from_ptr(name) };
  let name_str = name_cstr.to_string_lossy();
  let name_with_nul = String::from_utf8_lossy(name_cstr.to_bytes_with_nul()).into_owned();

  let Some(source) = read_file(&name_with_nul) else {
    eprintln!("Error opening {}", name_str);
    return false;
  };

  stats.read_time += record_delta_time(&mut currts);

  let result = catch_unwind(AssertUnwindSafe(|| {
    let mut bcb = BytecodeBuilder::new(None);

    let mut options = AssemblyOptions {
      target: assembly_target,
      compilation_options: CompilationOptions::default(),
      output_binary: format == CompileFormat::CodegenNull,
      include_assembly: false,
      include_ir: false,
      include_outlined_code: false,
      include_ir_types: false,
      include_ir_prefix: IncludeIrPrefix::default(),
      include_use_info: IncludeUseInfo::default(),
      include_cfg_info: IncludeCfgInfo::default(),
      include_reg_flow_info: IncludeRegFlowInfo::default(),
      annotator: Some(annotate_instruction),
      annotator_context: &mut bcb as *mut BytecodeBuilder as *mut c_void,
    };
    options.compilation_options.flags = CodeGenFlags::CODE_GEN_COLD_FUNCTIONS as u32;

    if !options.output_binary {
      options.include_assembly = format != CompileFormat::CodegenIr;
      options.include_ir = format != CompileFormat::CodegenAsm;
      options.include_ir_types = format != CompileFormat::CodegenAsm;
      options.include_outlined_code = format == CompileFormat::CodegenVerbose;
    }

    if format == CompileFormat::Text {
      let mut flags = BytecodeBuilder::DUMP_CODE
        | BytecodeBuilder::DUMP_SOURCE
        | BytecodeBuilder::DUMP_LOCALS
        | BytecodeBuilder::DUMP_REMARKS
        | BytecodeBuilder::DUMP_TYPES;
      if dump_constants {
        flags |= BytecodeBuilder::DUMP_CONSTANTS;
      }
      bcb.set_dump_flags(flags);
      bcb.set_dump_source(&source);
    } else if format == CompileFormat::Remarks {
      bcb.set_dump_flags(BytecodeBuilder::DUMP_SOURCE | BytecodeBuilder::DUMP_REMARKS);
      bcb.set_dump_source(&source);
    } else if format == CompileFormat::Codegen
      || format == CompileFormat::CodegenAsm
      || format == CompileFormat::CodegenIr
      || format == CompileFormat::CodegenVerbose
    {
      bcb.set_dump_flags(
        BytecodeBuilder::DUMP_CODE
          | BytecodeBuilder::DUMP_SOURCE
          | BytecodeBuilder::DUMP_LOCALS
          | BytecodeBuilder::DUMP_REMARKS,
      );
      bcb.set_dump_source(&source);
    }

    stats.misc_time += record_delta_time(&mut currts);

    let mut allocator = Allocator::new();
    let mut names = AstNameTable::new(&mut allocator);
    let parse_options = ParseOptions {
      store_cst_data: unsafe { (*core::ptr::addr_of!(GLOBAL_OPTIONS)).parse_cst },
      ..ParseOptions::default()
    };

    let parse_result = Parser::parse(
      source.as_str(),
      source.len(),
      &mut names,
      &mut allocator,
      parse_options,
    );

    if !parse_result.errors.is_empty() {
      for error in &parse_result.errors {
        report_error_c_char_luau_parse_error(name, error);
      }
      return false;
    }

    stats.lines += parse_result.lines;
    stats.parse_time += record_delta_time(&mut currts);

    if unsafe { (*core::ptr::addr_of!(GLOBAL_OPTIONS)).only_parse } {
      return true;
    }

    let compile_options = copts();
    compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
      &mut bcb,
      &parse_result,
      &mut names,
      &compile_options,
    );

    stats.bytecode += bcb.get_bytecode().len();
    stats.bytecode_instruction_count = bcb.get_total_instruction_count();
    stats.compile_time += record_delta_time(&mut currts);

    match format {
      CompileFormat::Text => {
        print!("{}", bcb.dump_everything());
      }
      CompileFormat::Remarks => {
        print!("{}", bcb.dump_source_remarks());
      }
      CompileFormat::Binary => {
        let _ = stdout().write_all(bcb.get_bytecode().as_bytes());
      }
      CompileFormat::Codegen
      | CompileFormat::CodegenAsm
      | CompileFormat::CodegenIr
      | CompileFormat::CodegenVerbose => {
        print!(
          "{}",
          get_codegen_assembly(name, bcb.get_bytecode(), options, &mut stats.lower_stats)
        );
      }
      CompileFormat::CodegenNull => {
        let assembly =
          get_codegen_assembly(name, bcb.get_bytecode(), options, &mut stats.lower_stats);
        stats.codegen += assembly.len();
        stats.codegen_time += record_delta_time(&mut currts);
      }
      CompileFormat::Null => {}
    }

    true
  }));

  match result {
    Ok(success) => success,
    Err(payload) => {
      if let Some(error) = payload.downcast_ref::<CompileError>() {
        report_error_c_char_luau_compile_error(name, error);
        false
      } else {
        resume_unwind(payload);
      }
    }
  }
}
