//! @interface-stub
use alloc::vec::Vec;
use core::{mem::size_of, ptr::null_mut};

use crate::{
  enums::code_gen_compilation_result::CodeGenCompilationResult,
  functions::{
    assemble_helpers_code_gen_a_64::assemble_helpers as assemble_helpers_a_64,
    assemble_helpers_code_gen_x_64::assemble_helpers as assemble_helpers_x_64,
    lower_function::{lower_function_a_64, lower_function_x_64},
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions, ir_builder::IrBuilder, lowering_stats::LoweringStats,
    module_helpers::ModuleHelpers,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_assembly_from_ir_impl_x_64(
  build: &mut AssemblyBuilderX64,
  ir: &mut IrBuilder,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> Vec<u8> {
  unsafe {
    let mut helpers = ModuleHelpers::default();
    assemble_helpers_x_64(build, &mut helpers);

    if !options.include_outlined_code && options.include_assembly {
      build.text.clear();
      build.log_append(format_args!(
        "; skipping {} bytes of outlined helpers\n",
        build.get_code_size().wrapping_mul(size_of::<u8>() as u32)
      ));
    }

    let mut result = CodeGenCompilationResult::Success;

    if !lower_function_x_64(
      ir,
      build,
      &mut helpers,
      null_mut(),
      options.clone(),
      stats,
      &mut result,
    ) && build.log_text
    {
      build.log_append(format_args!("; skipping (can't lower)\n"));
    }

    if build.log_text {
      build.log_append(format_args!("\n"));
    }

    if !build.finalize() {
      return Vec::new();
    }

    if options.output_binary {
      let mut bytes = Vec::with_capacity(build.code.len() + build.data.len());
      bytes.extend_from_slice(&build.code);
      bytes.extend_from_slice(&build.data);
      bytes
    } else {
      build.text.clone().into_bytes()
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_assembly_from_ir_impl_a_64(
  build: &mut AssemblyBuilderA64,
  ir: &mut IrBuilder,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> Vec<u8> {
  unsafe {
    let mut helpers = ModuleHelpers::default();
    assemble_helpers_a_64(build, &mut helpers);

    if !options.include_outlined_code && options.include_assembly {
      build.text.clear();
      build.log_append(format_args!(
        "; skipping {} bytes of outlined helpers\n",
        build.get_code_size().wrapping_mul(size_of::<u32>() as u32)
      ));
    }

    let mut result = CodeGenCompilationResult::Success;

    if !lower_function_a_64(
      ir,
      build,
      &mut helpers,
      null_mut(),
      options.clone(),
      stats,
      &mut result,
    ) && build.log_text
    {
      build.log_append(format_args!("; skipping (can't lower)\n"));
    }

    if build.log_text {
      build.log_append(format_args!("\n"));
    }

    if !build.finalize() {
      return Vec::new();
    }

    if options.output_binary {
      // 指令字按本机字节序展开，等价于 cpp 把 code 数组重解释为 char*
      let mut bytes = Vec::with_capacity(build.code.len() * size_of::<u32>() + build.data.len());
      bytes.extend(build.code.iter().flat_map(|word| word.to_ne_bytes()));
      bytes.extend_from_slice(&build.data);
      bytes
    } else {
      build.text.clone().into_bytes()
    }
  }
}
