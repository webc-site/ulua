//! @interface-stub
use core::ptr::null_mut;

use ulua_common::FInt::CodegenHeuristicsInstructionLimit;
use ulua_vm::records::proto::Proto;

use crate::{
  enums::{
    code_gen_compilation_result::CodeGenCompilationResult, include_cfg_info::IncludeCfgInfo,
    include_ir_prefix::IncludeIrPrefix, include_reg_flow_info::IncludeRegFlowInfo,
    include_use_info::IncludeUseInfo, target::Target,
  },
  functions::{
    create_native_proto_exec_data_code_gen_context::create_native_proto_exec_data,
    lower_function::{lower_function_a_64, lower_function_x_64},
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions, compilation_options::CompilationOptions,
    ir_builder::IrBuilder, module_helpers::ModuleHelpers,
  },
  type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create_native_function_x_64(
  build: &mut AssemblyBuilderX64,
  helpers: &mut ModuleHelpers,
  proto: *mut Proto,
  total_ir_inst_count: &mut u32,
  options: &CompilationOptions,
  result: &mut CodeGenCompilationResult,
) -> Option<NativeProtoExecDataPtr> {
  unsafe {
    let mut ir = IrBuilder::ir_builder_ir_builder(&options.hooks);
    ir.build_function_ir(proto);

    let inst_count = ir.function.instructions.len() as u32;

    if total_ir_inst_count.wrapping_add(inst_count)
      >= CodegenHeuristicsInstructionLimit.get() as u32
    {
      *result = CodeGenCompilationResult::CodeGenOverflowInstructionLimit;
      return None;
    }

    *total_ir_inst_count = total_ir_inst_count.wrapping_add(inst_count);

    let assembly_options = AssemblyOptions {
      target: Target::default(),
      compilation_options: options.clone(),
      output_binary: false,
      include_assembly: false,
      include_ir: false,
      include_outlined_code: false,
      include_ir_types: false,
      include_ir_prefix: IncludeIrPrefix::default(),
      include_use_info: IncludeUseInfo::default(),
      include_cfg_info: IncludeCfgInfo::default(),
      include_reg_flow_info: IncludeRegFlowInfo::default(),
      annotator: None,
      annotator_context: null_mut(),
    };

    if !lower_function_x_64(
      &mut ir,
      build,
      helpers,
      proto,
      assembly_options,
      null_mut(),
      result,
    ) {
      return None;
    }

    Some(create_native_proto_exec_data(proto, &ir))
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create_native_function_a_64(
  build: &mut AssemblyBuilderA64,
  helpers: &mut ModuleHelpers,
  proto: *mut Proto,
  total_ir_inst_count: &mut u32,
  options: &CompilationOptions,
  result: &mut CodeGenCompilationResult,
) -> Option<NativeProtoExecDataPtr> {
  unsafe {
    let mut ir = IrBuilder::ir_builder_ir_builder(&options.hooks);
    ir.build_function_ir(proto);

    let inst_count = ir.function.instructions.len() as u32;

    if total_ir_inst_count.wrapping_add(inst_count)
      >= CodegenHeuristicsInstructionLimit.get() as u32
    {
      *result = CodeGenCompilationResult::CodeGenOverflowInstructionLimit;
      return None;
    }

    *total_ir_inst_count = total_ir_inst_count.wrapping_add(inst_count);

    let assembly_options = AssemblyOptions {
      target: Target::default(),
      compilation_options: options.clone(),
      output_binary: false,
      include_assembly: false,
      include_ir: false,
      include_outlined_code: false,
      include_ir_types: false,
      include_ir_prefix: IncludeIrPrefix::default(),
      include_use_info: IncludeUseInfo::default(),
      include_cfg_info: IncludeCfgInfo::default(),
      include_reg_flow_info: IncludeRegFlowInfo::default(),
      annotator: None,
      annotator_context: null_mut(),
    };

    if !lower_function_a_64(
      &mut ir,
      build,
      helpers,
      proto,
      assembly_options,
      null_mut(),
      result,
    ) {
      return None;
    }

    Some(create_native_proto_exec_data(proto, &ir))
  }
}
