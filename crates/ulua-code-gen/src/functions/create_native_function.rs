use core::ptr::null_mut;

use ulua_common::fint::CodegenHeuristicsInstructionLimit;
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

/// 单个 proto 原生编译共用的固定 AssemblyOptions 骨架（文本输出关闭、annotator 置空）。
fn compilation_assembly_options(options: &CompilationOptions) -> AssemblyOptions {
  AssemblyOptions {
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
  }
}

/// 编译单个 proto 为 X64 原生函数：成功返回执行数据句柄，失败返回编译结果错误
/// （出参改返回值）。`total_ir_inst_count` 为跨 proto 累计的指令配额（in/out 累加器）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create_native_function_x_64(
  build: &mut AssemblyBuilderX64,
  helpers: &mut ModuleHelpers,
  proto: *mut Proto,
  total_ir_inst_count: &mut u32,
  options: &CompilationOptions,
) -> Result<NativeProtoExecDataPtr, CodeGenCompilationResult> {
  // 本函数的 unsafe 只在三个被调边界（IR 构建/降级/execdata 生成），均以上层 CodeGen
  // 保证的「proto 存活 + build/helpers/total 活借用」为共同前提，逐处就地标注。
  let mut ir = IrBuilder::ir_builder_ir_builder(&options.hooks);
  // Safety: proto 为待编译的活 X64 L 函数 Proto（调用方 CodeGen 保证），IR 构建期只读。
  unsafe { ir.build_function_ir(proto) };

  let inst_count = ir.function.instructions.len() as u32;

  if total_ir_inst_count.wrapping_add(inst_count) >= CodegenHeuristicsInstructionLimit.get() as u32
  {
    return Err(CodeGenCompilationResult::CodeGenOverflowInstructionLimit);
  }

  *total_ir_inst_count = total_ir_inst_count.wrapping_add(inst_count);

  // Safety: 同上存活前提；annotator_context 传 null 由被调方按 None 处理不解引用。
  unsafe {
    lower_function_x_64(
      &mut ir,
      build,
      helpers,
      proto,
      compilation_assembly_options(options),
      null_mut(),
    )
  }?;

  // Safety: 同上——proto 存活且 IR 已就绪，execdata 只触及这些活对象。
  Ok(unsafe { create_native_proto_exec_data(proto, &ir) })
}

/// 见 X64 版说明（A64 分支）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create_native_function_a_64(
  build: &mut AssemblyBuilderA64,
  helpers: &mut ModuleHelpers,
  proto: *mut Proto,
  total_ir_inst_count: &mut u32,
  options: &CompilationOptions,
) -> Result<NativeProtoExecDataPtr, CodeGenCompilationResult> {
  // 与 x_64 分支同构：unsafe 只在三个被调边界，共用「proto 存活 + 活借用」前提。
  let mut ir = IrBuilder::ir_builder_ir_builder(&options.hooks);
  // Safety: proto 为待编译的活 A64 L 函数 Proto（调用方 CodeGen 保证），IR 构建期只读。
  unsafe { ir.build_function_ir(proto) };

  let inst_count = ir.function.instructions.len() as u32;

  if total_ir_inst_count.wrapping_add(inst_count) >= CodegenHeuristicsInstructionLimit.get() as u32
  {
    return Err(CodeGenCompilationResult::CodeGenOverflowInstructionLimit);
  }

  *total_ir_inst_count = total_ir_inst_count.wrapping_add(inst_count);

  // Safety: 同上存活前提；annotator_context 传 null 由被调方按 None 处理不解引用。
  unsafe {
    lower_function_a_64(
      &mut ir,
      build,
      helpers,
      proto,
      compilation_assembly_options(options),
      null_mut(),
    )
  }?;

  // Safety: 同上——proto 存活且 IR 已就绪，execdata 只触及这些活对象。
  Ok(unsafe { create_native_proto_exec_data(proto, &ir) })
}
