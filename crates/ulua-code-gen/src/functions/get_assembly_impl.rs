//! @interface-stub
use alloc::{string::String, vec::Vec};
use core::{ffi::CStr, fmt::Arguments, mem::size_of, slice::from_raw_parts};

use ulua_common::enums::luau_proto_flag::LuauProtoFlag;
use ulua_vm::{
  macros::{clvalue::clvalue, getstr::getstr},
  records::proto::Proto,
  type_aliases::t_value::TValue,
};

use crate::{
  enums::{
    code_gen_compilation_result::CodeGenCompilationResult, code_gen_flags::CodeGenFlags,
    function_stats_flags::FunctionStatsFlags,
  },
  functions::{
    assemble_helpers_code_gen_a_64::assemble_helpers as assemble_helpers_a_64,
    assemble_helpers_code_gen_x_64::assemble_helpers as assemble_helpers_x_64,
    gather_functions::gather_functions,
    get_instruction_count_code_gen_assembly::get_instruction_count_instruction_size,
    log_function_header::log_function_header,
    log_function_types::log_function_types,
    lower_function::{lower_function_a_64, lower_function_x_64},
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64,
    assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions,
    function_bytecode_summary::FunctionBytecodeSummary,
    function_stats::FunctionStats,
    ir_builder::IrBuilder,
    lowering_stats::{FUNCTION_STATS_ENABLE, LoweringStats},
    module_helpers::ModuleHelpers,
  },
  traits::LogAppend,
};

impl LogAppend for AssemblyBuilderX64 {
  fn log_append(&mut self, args: Arguments<'_>) {
    self.log_append(args);
  }
}

impl LogAppend for AssemblyBuilderA64 {
  fn log_append(&mut self, args: Arguments<'_>) {
    self.log_append(args);
  }
}

/// 统一 X64/A64 汇编器反射接口，对齐上游 CodeGenAssembly.cpp 的
/// `template<typename AssemblyBuilder> getAssemblyImpl`
trait AsmBuilder: LogAppend {
  /// asm 计量步长：X64 按字节、A64 按 4 字节指令字
  const CODE_UNIT: u32;

  fn log_text(&self) -> bool;
  fn text_mut(&mut self) -> &mut String;
  fn finalize(&mut self) -> bool;
  fn get_code_size(&self) -> u32;
  fn get_instruction_count(&self) -> u32;

  /// 代码段字节视图（A64 由 u32 指令字切片转换）
  fn code_bytes(&self) -> &[u8];
  fn data_bytes(&self) -> &[u8];

  fn assemble_helpers(&mut self, helpers: &mut ModuleHelpers);

  /// # Safety
  /// proto 指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  unsafe fn lower_function(
    &mut self,
    ir: &mut IrBuilder,
    helpers: &mut ModuleHelpers,
    proto: *mut Proto,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
    result: &mut CodeGenCompilationResult,
  ) -> bool;
}

impl AsmBuilder for AssemblyBuilderX64 {
  const CODE_UNIT: u32 = size_of::<u8>() as u32;

  fn log_text(&self) -> bool {
    self.log_text
  }

  fn text_mut(&mut self) -> &mut String {
    &mut self.text
  }

  fn finalize(&mut self) -> bool {
    self.finalize()
  }

  fn get_code_size(&self) -> u32 {
    self.get_code_size()
  }

  fn get_instruction_count(&self) -> u32 {
    self.get_instruction_count()
  }

  fn code_bytes(&self) -> &[u8] {
    &self.code
  }

  fn data_bytes(&self) -> &[u8] {
    &self.data
  }

  fn assemble_helpers(&mut self, helpers: &mut ModuleHelpers) {
    assemble_helpers_x_64(self, helpers)
  }

  unsafe fn lower_function(
    &mut self,
    ir: &mut IrBuilder,
    helpers: &mut ModuleHelpers,
    proto: *mut Proto,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
    result: &mut CodeGenCompilationResult,
  ) -> bool {
    unsafe { lower_function_x_64(ir, self, helpers, proto, options, stats, result) }
  }
}

impl AsmBuilder for AssemblyBuilderA64 {
  const CODE_UNIT: u32 = size_of::<u32>() as u32;

  fn log_text(&self) -> bool {
    self.log_text
  }

  fn text_mut(&mut self) -> &mut String {
    &mut self.text
  }

  fn finalize(&mut self) -> bool {
    self.finalize()
  }

  fn get_code_size(&self) -> u32 {
    self.get_code_size()
  }

  fn get_instruction_count(&self) -> u32 {
    self.get_instruction_count()
  }

  fn code_bytes(&self) -> &[u8] {
    // 指令字切片按字节重解释：u32 对齐满足 u8 要求，长度为 len * 4
    unsafe {
      from_raw_parts(
        self.code.as_ptr().cast::<u8>(),
        self.code.len() * size_of::<u32>(),
      )
    }
  }

  fn data_bytes(&self) -> &[u8] {
    &self.data
  }

  fn assemble_helpers(&mut self, helpers: &mut ModuleHelpers) {
    assemble_helpers_a_64(self, helpers)
  }

  unsafe fn lower_function(
    &mut self,
    ir: &mut IrBuilder,
    helpers: &mut ModuleHelpers,
    proto: *mut Proto,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
    result: &mut CodeGenCompilationResult,
  ) -> bool {
    unsafe { lower_function_a_64(ir, self, helpers, proto, options, stats, result) }
  }
}

/// 泛型主体：X64/A64 共用的汇编输出流程
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
unsafe fn get_assembly_impl<B: AsmBuilder>(
  build: &mut B,
  func: *const TValue,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> String {
  unsafe {
    let cl = clvalue!(func);
    let root: *mut Proto = (*cl).inner.l.p;

    if (options.compilation_options.flags & CodeGenFlags::CodeGenOnlyNativeModules as u32) != 0
      && ((*root).flags & LuauProtoFlag::LPF_NATIVE_MODULE as u8) == 0
    {
      build.finalize();
      return String::new();
    }

    let mut protos: Vec<*mut Proto> = Vec::new();
    gather_functions(
      &mut protos,
      root,
      options.compilation_options.flags,
      ((*root).flags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8) != 0,
    );

    protos.retain(|p| !p.is_null());

    if !stats.is_null() {
      (*stats).total_functions += protos.len() as u32;
    }

    if protos.is_empty() {
      build.finalize();
      return String::new();
    }

    let mut helpers = ModuleHelpers::default();
    build.assemble_helpers(&mut helpers);

    if !options.include_outlined_code && options.include_assembly {
      build.text_mut().clear();
      build.log_append(format_args!(
        "; skipping {} bytes of outlined helpers\n",
        build.get_code_size().wrapping_mul(B::CODE_UNIT)
      ));
    }

    for p in protos {
      let mut ir = IrBuilder::ir_builder_ir_builder(&options.compilation_options.hooks);
      ir.build_function_ir(p);
      let mut asm_size = build.get_code_size();
      let mut asm_count = build.get_instruction_count();

      if options.include_assembly || options.include_ir {
        log_function_header(build, p);
      }

      if options.include_ir_types {
        log_function_types(
          build,
          &ir.function,
          options.compilation_options.userdata_types,
        );
      }

      let mut result = CodeGenCompilationResult::Success;

      if !build.lower_function(
        &mut ir,
        &mut helpers,
        p,
        options.clone(),
        stats,
        &mut result,
      ) {
        if build.log_text() {
          build.log_append(format_args!("; skipping (can't lower)\n"));
        }

        asm_size = 0;
        asm_count = 0;

        if !stats.is_null() {
          (*stats).skipped_functions += 1;
        }
      } else {
        asm_size = build.get_code_size().wrapping_sub(asm_size);
        asm_count = build.get_instruction_count().wrapping_sub(asm_count);
      }

      if !stats.is_null() && ((*stats).function_stats_flags & FUNCTION_STATS_ENABLE) != 0 {
        let function_stat_name = if !(*p).debugname.is_null() {
          let name = getstr((*p).debugname as *const _);
          CStr::from_ptr(name).to_string_lossy().into_owned()
        } else if (*p).bytecodeid == (*root).bytecodeid {
          String::from("[top level]")
        } else {
          String::from("[anonymous]")
        };
        let mut function_stat = FunctionStats {
          name: function_stat_name,
          ..Default::default()
        };
        function_stat.line = (*p).linedefined;
        function_stat.bcode_count =
          get_instruction_count_instruction_size((*p).code, (*p).sizecode as u32);
        function_stat.ir_count = ir.function.instructions.len() as u32;
        function_stat.asm_size = asm_size.wrapping_mul(B::CODE_UNIT);
        function_stat.asm_count = asm_count;

        if ((*stats).function_stats_flags & FunctionStatsFlags::FunctionStatsBytecodeSummary as u32)
          != 0
        {
          let summary = FunctionBytecodeSummary::from_proto(p, 0);
          function_stat
            .bytecode_summary
            .push(summary.get_counts(0).clone());
        }

        (*stats).functions.push(function_stat);
      }

      if build.log_text() {
        build.log_append(format_args!("\n"));
      }
    }

    if !build.finalize() {
      return String::new();
    }

    if options.output_binary {
      let mut bytes = Vec::with_capacity(build.code_bytes().len() + build.data_bytes().len());
      bytes.extend_from_slice(build.code_bytes());
      bytes.extend_from_slice(build.data_bytes());
      String::from_utf8_unchecked(bytes)
    } else {
      build.text_mut().clone()
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_assembly_impl_x_64(
  build: &mut AssemblyBuilderX64,
  func: *const TValue,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> String {
  unsafe { get_assembly_impl(build, func, options, stats) }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_assembly_impl_a_64(
  build: &mut AssemblyBuilderA64,
  func: *const TValue,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> String {
  unsafe { get_assembly_impl(build, func, options, stats) }
}
