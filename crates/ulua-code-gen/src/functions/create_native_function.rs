use core::ptr::null_mut;

use ulua_common::fint::CodegenHeuristicsInstructionLimit;
use ulua_vm::{
  functions::call_obs::call_obs_hints_for,
  records::{lua_state::LuaState, proto::Proto},
};

use crate::{
  enums::{
    code_gen_compilation_result::CodeGenCompilationResult,
    include_cfg_info::IncludeCfgInfo,
    include_use_info::IncludeUseInfo,
    options::{IncludeIrPrefix, IncludeRegFlowInfo},
    target::Target,
  },
  functions::{
    create_native_proto_exec_data_code_gen_context::create_native_proto_exec_data,
    lower_function::{lower_function_a_64, lower_function_x_64},
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions, compilation_options::CompilationOptions,
    ir_builder::IrBuilder, module::ModuleHelpers,
  },
  type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr,
};

/// 单个 proto 原生编译共用的固定 AssemblyOptions 骨架（文本输出关闭、annotator 置空）。
///
/// `annotator_context: null_mut()` 是 DELIBERATE DEVIATION / 保留理由：该字段
/// （声明在 `records::assembly_options::AssemblyOptions`）是跨 ABI 回调的 opaque
/// 宿主 user-data，唯一消费点在 `functions::lower_impl`——
/// `annotator(options.annotator_context, build.text_mut(), bytecodeid,
/// bc_location as i32)`，被调方类型是 `AnnotatorFn =
/// Option<unsafe extern "C-unwind" fn(context: *mut c_void, result: &mut String,
/// fid: i32, instpos: i32)>`，第一参在 ABI 上钉死为 `void*`。宿主侧
/// （`ulua-compile-cli::compile_file`）正是把 `&mut bcb` 以 `*mut _` 灌进这一槽。
/// 本路径 `annotator: None`（cpp 同名骨架也是 `annotatorContext = nullptr`），
/// 该值随 `None` 一起无消费方；换 `Option<NonNull<c_void>>` 只会在 lower_impl
/// 调用点还原成同一个 null，不减 unsafe。
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
/// `l` 为编译会话宿主态（NAMECALL 阶段字符串常量 intern 的 VM 记账分配入口）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create_native_function_x_64(
  build: &mut AssemblyBuilderX64,
  helpers: &mut ModuleHelpers,
  proto: *mut Proto,
  total_ir_inst_count: &mut u32,
  options: &CompilationOptions,
  l: *mut LuaState,
) -> Result<NativeProtoExecDataPtr, CodeGenCompilationResult> {
  // 本函数的 unsafe 只在三个被调边界（IR 构建/降级/execdata 生成），均以上层 CodeGen
  // 保证的「proto 存活 + build/helpers/total 活借用」为共同前提，逐处就地标注。
  let mut ir = IrBuilder::ir_builder_ir_builder(&options.hooks);
  // NAMECALL 阶段：观测路径内联发射把 callee 字符串常量 intern 进 caller 常量表，
  // 分配须走 VM 记账（编译会话身份字段，见 IrFunction::l 注）。
  ir.set_lua_state(l);

  // JIT call inlining 第 2 阶段：暖重编译时从上一版 execdata 的 COBS 侧表读取
  // CALL 站点观测提示，供发射端以运行时证据替代静态判据。TSFB 类型提示
  // （J1 Phase 2b，GETTABLEKS tag 细化）暂不注入：该通道无运行验证且其特化
  // 面本票发射端不消费（GETTABLEKS 内联受 VmConst 重定位限制未开通）。
  if options.force_recompile && !unsafe { (*proto).execdata.is_null() } {
    ir.set_call_hints(unsafe { call_obs_hints_for(proto) });
  }
  // Safety: proto 为待编译的活 X64 L 函数 Proto（调用方 CodeGen 保证），IR 构建期只读。
  unsafe { ir.build_function_ir(proto) };

  let inst_count = ir.function.instructions.len() as u32;

  if total_ir_inst_count.wrapping_add(inst_count) >= CodegenHeuristicsInstructionLimit.get() as u32
  {
    return Err(CodeGenCompilationResult::CodeGenOverflowInstructionLimit);
  }

  *total_ir_inst_count = total_ir_inst_count.wrapping_add(inst_count);

  // Safety: 同上存活前提；annotator_context 传 null 由被调方按 None 处理不解引用；
  // `proto` 依契约非空存活，此处仅派生只读共享借用（lowering 侧不写原型）。
  unsafe {
    lower_function_x_64(
      &mut ir,
      build,
      helpers,
      Some(&*proto),
      compilation_assembly_options(options),
      None,
    )
  }?;

  // Safety: 同上——proto 存活且 IR 已就绪，execdata 只触及这些活对象。
  Ok(unsafe { create_native_proto_exec_data(proto, &ir, options.force_recompile) })
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
  l: *mut LuaState,
) -> Result<NativeProtoExecDataPtr, CodeGenCompilationResult> {
  // 与 x_64 分支同构：unsafe 只在三个被调边界，共用「proto 存活 + 活借用」前提。
  let mut ir = IrBuilder::ir_builder_ir_builder(&options.hooks);
  // NAMECALL 阶段：同 X64 分支——字符串常量 intern 的 VM 记账分配入口。
  ir.set_lua_state(l);

  // J1 Phase 2b：暖重编译时从上一版 execdata 的 TSFB 侧表读取观测类型提示
  // （GETTABLEKS 站点：pc → (B 寄存器, 观测 tag)），注入分析器做 ANY 细化；
  // 并读 CALL 站点观测提示供 call inlining 发射端替代静态判据。
  if options.force_recompile && !unsafe { (*proto).execdata.is_null() } {
    // ir.set_type_hints(unsafe { tsfb_hints_for(proto) });
    ir.set_call_hints(unsafe { call_obs_hints_for(proto) });
  }
  // Safety: proto 为待编译的活 A64 L 函数 Proto（调用方 CodeGen 保证），IR 构建期只读。
  unsafe { ir.build_function_ir(proto) };

  let inst_count = ir.function.instructions.len() as u32;

  if total_ir_inst_count.wrapping_add(inst_count) >= CodegenHeuristicsInstructionLimit.get() as u32
  {
    return Err(CodeGenCompilationResult::CodeGenOverflowInstructionLimit);
  }

  *total_ir_inst_count = total_ir_inst_count.wrapping_add(inst_count);

  // Safety: 同上存活前提；annotator_context 传 null 由被调方按 None 处理不解引用；
  // `proto` 依契约非空存活，此处仅派生只读共享借用（lowering 侧不写原型）。
  unsafe {
    lower_function_a_64(
      &mut ir,
      build,
      helpers,
      Some(&*proto),
      compilation_assembly_options(options),
      None,
    )
  }?;

  // Safety: 同上——proto 存活且 IR 已就绪，execdata 只触及这些活对象。
  Ok(unsafe { create_native_proto_exec_data(proto, &ir, options.force_recompile) })
}
