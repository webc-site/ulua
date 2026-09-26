use core::ptr::addr_of_mut;
use std::{
  io::Write,
  panic::{AssertUnwindSafe, catch_unwind},
};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_cli_lib::functions::{
  read_file::read_file, report_compile_panic::report_compile_panic_string,
  report_open_error::report_open_error_string, report_parse_errors::collect_parse_errors,
};
use ulua_code_gen::{
  enums::{
    code_gen_flags::CodeGenFlags, include_cfg_info::IncludeCfgInfo,
    include_ir_prefix::IncludeIrPrefix, include_reg_flow_info::IncludeRegFlowInfo,
    include_use_info::IncludeUseInfo, target::Target,
  },
  records::{assembly_options::AssemblyOptions, compilation_options::CompilationOptions},
};
use ulua_common::functions::get_clock::get_clock;
use ulua_compiler::functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options;

use crate::{
  enums::compile_format::CompileFormat,
  functions::{
    annotate_instruction::annotate_instruction, copts::copts,
    get_codegen_assembly::get_codegen_assembly, record_delta_time::record_delta_time,
  },
  records::{
    compile_stats::CompileStats,
    global_options::{GlobalOptions, restore},
  },
};

/// 单文件编译的完整结果：成败标志 + 本应直写 stdout/stderr 的字节 + 本文件私有统计。
///
/// 并行编译时各文件在工作线程产出本结构，主线程按 `files` 原顺序统一
/// `write_all`/`eprint`，保证 CLI 输出（含流内交错次序）与串行版逐字节一致。
pub(crate) struct FileOutcome {
  pub ok: bool,
  /// 原 `print!`/`stdout().write_all` 的直写内容（文本模式为 UTF-8 字节，
  /// Binary 模式为原始字节码）
  pub stdout: Vec<u8>,
  /// 原 `eprintln!` 直写内容（打开失败/解析错误/编译错误，逐条含换行）
  pub stderr: String,
  pub stats: CompileStats,
}

/// cpp `compileFile` (CLI/src/Compile.cpp:300-428)
///
/// 并行化改造：原 `&mut stats` 出参与 stdout/stderr 直写全部折进返回的
/// [`FileOutcome`]，本函数不再持有跨文件状态；`globals` 为 main 线程
/// 参数解析后的快照，入口处 [`restore`] 写入本线程 thread_local，
/// 使 `with`/`copts` 在 rayon 工作线程上读到与主线程一致的选项。
pub(crate) fn compile_file(
  name: &str,
  format: CompileFormat,
  assembly_target: Target,
  globals: &GlobalOptions,
  function_stats: u32,
  dump_constants: bool,
) -> FileOutcome {
  restore(globals);

  let mut outcome = FileOutcome {
    ok: false,
    stdout: Vec::new(),
    stderr: String::new(),
    stats: CompileStats::default(),
  };
  outcome.stats.lower_stats.function_stats_flags = function_stats;

  let mut currts = get_clock();

  let Some(source) = read_file(name) else {
    outcome.stderr = report_open_error_string(name);
    return outcome;
  };

  outcome.stats.read_time += record_delta_time(&mut currts);

  let outcome_ref = &mut outcome;
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
      // cpp `options.annotatorContext = &bcb`。addr_of_mut! 一次性取地址，不派生
      // `&mut`：bcb 之后仍被多次可变借用（set_dump_flags/set_dump_source/finalize…），
      // 若这里用 `&mut bcb as *mut _`，那些借用会作废已存进 options 的指针，
      // codegen 期 annotate_instruction 再解引用即 Stacked Borrows UB。
      // 指针指向本任务栈帧上的 bcb，任务内单线程使用，线程间无共享。
      annotator_context: addr_of_mut!(bcb) as *mut _,
    };
    options.compilation_options.flags = CodeGenFlags::CODE_GEN_COLD_FUNCTIONS as u32;

    if !options.output_binary {
      options.include_assembly = format != CompileFormat::CodegenIr;
      options.include_ir = format != CompileFormat::CodegenAsm;
      options.include_ir_types = format != CompileFormat::CodegenAsm;
      options.include_outlined_code = format == CompileFormat::CodegenVerbose;
      // ISSUE[code-gen.include_reg_spills] TODO(cross-crate): cpp 此处
      // `options.includeRegSpills = globalOptions.dumpRegSpills;` —— 依赖 ulua-code-gen
      // 的 `AssemblyOptions::include_reg_spills` 字段（尚未移植），暂无法透传;
      // CLI 侧已在 main() 解析 --dump-regspills 时向用户提示该旗标未生效，
      // 登记见 docs/CONFORMANCE.md「Known gaps」同名锚点。
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

    outcome_ref.stats.misc_time += record_delta_time(&mut currts);

    // Box 钉堆：AstNameTable/Parser 捕获宿主地址，宿主移动即悬垂。
    // 二者均为本任务（本线程栈）局部量，跨线程无共享。
    let mut allocator = Box::new(Allocator::new());
    let mut names = AstNameTable::new(&mut allocator);
    let parse_options = ParseOptions {
      store_cst_data: globals.parse_cst,
      ..ParseOptions::default()
    };

    let parse_result = Parser::parse(source.as_str(), &mut names, &mut allocator, parse_options);

    if !parse_result.errors.is_empty() {
      outcome_ref.stderr = collect_parse_errors(name, &parse_result.errors);
      return false;
    }

    outcome_ref.stats.lines += parse_result.lines;
    outcome_ref.stats.parse_time += record_delta_time(&mut currts);

    if globals.only_parse {
      return true;
    }

    let compile_options = copts();
    compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
      &mut bcb,
      &parse_result,
      &mut names,
      &compile_options,
    );

    outcome_ref.stats.bytecode += bcb.get_bytecode().len();
    outcome_ref.stats.bytecode_instruction_count = bcb.get_total_instruction_count();
    outcome_ref.stats.compile_time += record_delta_time(&mut currts);

    match format {
      CompileFormat::Text => {
        let _ = write!(outcome_ref.stdout, "{}", bcb.dump_everything());
      }
      CompileFormat::Remarks => {
        let _ = write!(outcome_ref.stdout, "{}", bcb.dump_source_remarks());
      }
      CompileFormat::Binary => {
        let _ = outcome_ref.stdout.write_all(bcb.get_bytecode());
      }
      CompileFormat::Codegen
      | CompileFormat::CodegenAsm
      | CompileFormat::CodegenIr
      | CompileFormat::CodegenVerbose => {
        // 文本模式输出本身即 UTF-8 字节，缓冲进本文件 stdout 段免再解码
        let assembly = get_codegen_assembly(
          name,
          bcb.get_bytecode(),
          options,
          &mut outcome_ref.stats.lower_stats,
          &mut outcome_ref.stderr,
        );
        let _ = outcome_ref.stdout.write_all(&assembly);
      }
      CompileFormat::CodegenNull => {
        let assembly = get_codegen_assembly(
          name,
          bcb.get_bytecode(),
          options,
          &mut outcome_ref.stats.lower_stats,
          &mut outcome_ref.stderr,
        );
        outcome_ref.stats.codegen += assembly.len();
        outcome_ref.stats.codegen_time += record_delta_time(&mut currts);
      }
      CompileFormat::Null => {}
    }

    true
  }));

  // CompileError 负载折算为 stderr 消息（由主线程按序上报）；其余负载原样
  // 续抛（门面单点收口）
  outcome.ok = match report_compile_panic_string(name, result) {
    Ok(ok) => ok,
    Err(message) => {
      outcome.stderr.push_str(&message);
      false
    }
  };

  outcome
}
