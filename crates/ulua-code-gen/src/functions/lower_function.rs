use ulua_common::{
  clock_shim::monotonic_seconds,
  fflag::DebugCodegenOptSize,
  fint::{CodegenHeuristicsBlockInstructionLimit, CodegenHeuristicsBlockLimit},
};
use ulua_vm::records::proto::Proto;

use crate::{
  enums::{
    code_gen_compilation_result::CodeGenCompilationResult, ir_block_kind::IrBlockKind,
    ir_cmd::IrCmd,
  },
  functions::{
    compute_cfg_block_edges::compute_cfg_block_edges,
    compute_cfg_info::compute_cfg_info,
    const_prop_in_block_chains::const_prop_in_block_chains,
    count_instructions_with_cmd::count_instructions_with_cmd,
    create_linear_blocks::create_linear_blocks,
    get_sorted_block_order::get_sorted_block_order,
    jit_rng_seed::jit_rng_seed,
    kill_unused_blocks::kill_unused_blocks,
    lower_ir_code_gen_lower::{
      lower_ir_a_64_assembly_builder_a_64_ir_builder_vector_u32_module_helpers_proto_assembly_options_lowering_stats,
      lower_ir_x_64_assembly_builder_x_64_ir_builder_vector_u32_module_helpers_proto_assembly_options_lowering_stats,
    },
    mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains,
    update_last_use_locations::update_last_use_locations,
    with_lowering_stats::with_lowering_stats,
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions, ir_builder::IrBuilder, lowering_stats::LoweringStats,
    module_helpers::ModuleHelpers,
  },
};

/// lowering 全流程的单一泛型主体（对齐 cpp CodeGenLower.h 的
/// `template<typename AssemblyBuilder> lowerFunction`）：IR 优化 + 生成机器码。
///
/// X64/A64 两平台流程逐语句一致，唯一差异是末尾注入的平台专属 `lower_ir` 回调。
/// 除该回调外全部步骤均为安全函数；可空 `stats` 裸指针的解引用统一走
/// [`with_lowering_stats`] 单点门面，`proto` 仅做判空与取址、不在本函数内解引用。
fn lower_function_common<B>(
  ir: &mut IrBuilder,
  build: &mut B,
  helpers: &mut ModuleHelpers,
  proto: *mut Proto,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
  lower_ir: unsafe fn(
    &mut B,
    &mut IrBuilder,
    &[u32],
    &mut ModuleHelpers,
    *mut Proto,
    AssemblyOptions,
    *mut LoweringStats,
  ) -> bool,
) -> Result<(), CodeGenCompilationResult> {
  // 裸指针字段赋值与判空均不解引用，属安全操作
  ir.function.stats = stats;
  ir.function.record_counters = options.compilation_options.record_counters;

  if options.compilation_options.nop_padding && !proto.is_null() {
    ir.function.jit_rng_state = jit_rng_seed(proto as usize);
  }

  kill_unused_blocks(&mut ir.function);

  let mut pre_opt_block_count = 0u32;
  let mut max_block_instructions = 0u32;

  for block in &ir.function.blocks {
    if block.kind != IrBlockKind::Dead {
      pre_opt_block_count += 1;
    }

    let block_instructions = block.finish.wrapping_sub(block.start);
    max_block_instructions = max_block_instructions.max(block_instructions);
  }

  // cpp 同款：先记统计再查启发式上限，保证超限提前返回时统计仍可见
  with_lowering_stats(stats, |s| {
    s.blocks_pre_opt += pre_opt_block_count;
    s.max_block_instructions = max_block_instructions;
  });

  if pre_opt_block_count >= CodegenHeuristicsBlockLimit.get() as u32 {
    return Err(CodeGenCompilationResult::CodeGenOverflowBlockLimit);
  }

  if max_block_instructions >= CodegenHeuristicsBlockInstructionLimit.get() as u32 {
    return Err(CodeGenCompilationResult::CodeGenOverflowBlockInstructionLimit);
  }

  compute_cfg_info(&mut ir.function);

  const_prop_in_block_chains(ir);

  if !DebugCodegenOptSize.get() {
    let mut start_time = 0.0;
    let mut const_prop_instruction_count = 0u32;

    // stats 为空时闭包不执行，天然跳过无谓的指令计数遍历（cpp `if (stats)` 同款惰性）
    with_lowering_stats(stats, |_| {
      const_prop_instruction_count =
        count_instructions_with_cmd(&ir.function.instructions, IrCmd::SUBSTITUTE);
      start_time = monotonic_seconds();
    });

    create_linear_blocks(ir);

    with_lowering_stats(stats, |s| {
      s.block_linearization_stats.time_seconds += monotonic_seconds() - start_time;
      const_prop_instruction_count =
        count_instructions_with_cmd(&ir.function.instructions, IrCmd::SUBSTITUTE)
          .wrapping_sub(const_prop_instruction_count);
      s.block_linearization_stats.const_prop_instruction_count += const_prop_instruction_count;
    });
  }

  mark_dead_stores_in_block_chains(ir);

  compute_cfg_block_edges(&mut ir.function);

  let sorted_blocks = get_sorted_block_order(&mut ir.function);

  update_last_use_locations(&mut ir.function, &sorted_blocks);

  with_lowering_stats(stats, |s| {
    for block in &ir.function.blocks {
      if block.kind != IrBlockKind::Dead {
        s.blocks_post_opt += 1;
      }
    }
  });

  // Safety: `lower_ir` 依各平台实现自身的 `# Safety` 契约接收参数——`build`/`ir`/`helpers`
  // 为本函数存活引用的独占借用（此前的 stats 临时借用均已按 NLL 结束）；`proto` 只被被调方
  // 判空使用（null 或存活由调用方契约保证）；`stats` 透传同一可空裸指针，被调方的每次解引用
  // 都与其判空守卫配套，且本时刻不存在并存 &mut 别名（单线程串行）。
  let result = unsafe { lower_ir(build, ir, &sorted_blocks, helpers, proto, options, stats) };

  if result {
    Ok(())
  } else {
    Err(CodeGenCompilationResult::CodeGenLoweringFailure)
  }
}

/// lowering 主入口（X64）：IR → 机器码。失败时返回对应的编译结果错误（出参改返回值）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lower_function_x_64(
  ir: &mut IrBuilder,
  build: &mut AssemblyBuilderX64,
  helpers: &mut ModuleHelpers,
  proto: *mut Proto,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> Result<(), CodeGenCompilationResult> {
  // 本 `unsafe fn` 的指针参数仅原样透传给安全主体 `lower_function_common`，
  // 其内部唯一的解引用时刻由该函数的 `// Safety` 契约约束。
  lower_function_common(
    ir,
    build,
    helpers,
    proto,
    options,
    stats,
    lower_ir_x_64_assembly_builder_x_64_ir_builder_vector_u32_module_helpers_proto_assembly_options_lowering_stats,
  )
}

/// lowering 主入口（A64）：IR → 机器码。语义与 X64 版逐路径一致（cpp 同款模板共用主体）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lower_function_a_64(
  ir: &mut IrBuilder,
  build: &mut AssemblyBuilderA64,
  helpers: &mut ModuleHelpers,
  proto: *mut Proto,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> Result<(), CodeGenCompilationResult> {
  // 同 X64 版：指针参数原样透传给安全主体，本函数头 `# Safety` 契约不变。
  lower_function_common(
    ir,
    build,
    helpers,
    proto,
    options,
    stats,
    lower_ir_a_64_assembly_builder_a_64_ir_builder_vector_u32_module_helpers_proto_assembly_options_lowering_stats,
  )
}
