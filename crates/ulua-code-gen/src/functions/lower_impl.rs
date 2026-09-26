//! lowering 块遍历主体（对齐 cpp CodeGenLower.h 的
//! `template<typename AssemblyBuilder> lowerImpl`）。
//!
//! X64/A64 两平台的块遍历流程逐语句一致，差异只在「发射器行为面」与「lowering 状态面」
//! 两处平台实现，因此收敛为单泛型主体 [`lower_blocks`] + 两个窄接口 trait，
//! 平台入口 [`lower_impl_x_64`] / [`lower_impl_a_64`] 只负责契约透传。

use alloc::{string::String, vec, vec::Vec};
use core::mem::size_of;

use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_vm::records::proto::Proto;

use crate::{
  enums::{
    include_ir_prefix::IncludeIrPrefix, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
    ir_op_kind::IrOpKind,
  },
  functions::{
    any_argument_match::any_argument_match,
    get_bytecode_type_name::UserdataTypes,
    get_next_block::get_next_block,
    is_pseudo::is_pseudo,
    jit_rng_random::jit_rng_random,
    to_string_detailed_ir_dump::{
      to_string_detailed as to_string_detailed_inst, to_string_detailed_block,
    },
    to_string_ir_dump::to_string_bytecode_types,
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64,
    assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions,
    bytecode_types::LBC_TYPE_ANY,
    cfg_info::CfgInfo,
    ir_block::{IrBlock, K_BLOCK_FLAG_SAFE_ENV_CHECK, K_BLOCK_NO_START_PC},
    ir_const::IrConst,
    ir_data::K_INVALID_INST_IDX,
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_lowering_a_64::IrLoweringA64,
    ir_lowering_x_64::IrLoweringX64,
    ir_op::IrOp,
    ir_to_string_context::IrToStringContext,
    label::Label,
    store_location_hint::StoreLocationHint,
    vm_exit_sync_info::VmExitSyncInfo,
  },
  traits::LogAppend,
};

/// 块遍历主体需要的发射器最小行为面（X64/A64 单态分发，无 `dyn`）。
pub(crate) trait LowerBuild: LogAppend {
  /// asm 计量步长：X64 按字节、A64 按 4 字节指令字
  const CODE_UNIT: u32;

  /// asm/IR 文本缓冲
  fn text_mut(&mut self) -> &mut String;
  fn get_code_size(&self) -> u32;
  /// 把已存在的块 label 绑定到当前 asm 位置
  fn bind_label(&mut self, label: &mut Label);
  /// 为当前 asm 位置新建并绑定一个指令 label
  fn set_new_label(&mut self) -> Label;
  fn label_offset(&self, label: &Label) -> u32;
  /// 追加 `bytes` 字节随机 nop 填充（jit_rng 通道）
  fn pad_nop(&mut self, bytes: u32);
}

/// 块遍历主体需要的 lowering 状态面（X64/A64 单态分发）。
pub(crate) trait LowerState {
  fn start_block(&mut self, curr: &IrBlock);
  fn check_safe_env(&mut self, exit: IrOp, index: u32, next: &IrBlock);
  fn lower_inst(&mut self, inst: &mut IrInst, index: u32, next: &IrBlock);
  fn finish_block(&mut self, curr: &IrBlock, next: &IrBlock);
  fn finish_function(&mut self);
  fn has_error(&self) -> bool;
  /// 伪指令的 store-location hint 前向登记：临时把 `regs.curr_inst_idx` 切到本指令
  fn track_store_hint(&mut self, index: u32, hint: &StoreLocationHint);
}

impl LowerBuild for AssemblyBuilderX64 {
  const CODE_UNIT: u32 = size_of::<u8>() as u32;

  fn text_mut(&mut self) -> &mut String {
    &mut self.text
  }

  fn get_code_size(&self) -> u32 {
    self.get_code_size()
  }

  fn bind_label(&mut self, label: &mut Label) {
    self.set_label_label(label);
  }

  fn set_new_label(&mut self) -> Label {
    let mut label = Label::default();
    self.set_label(&mut label);
    label
  }

  fn label_offset(&self, label: &Label) -> u32 {
    self.get_label_offset(label)
  }

  fn pad_nop(&mut self, bytes: u32) {
    self.nop(bytes);
  }
}

impl LowerBuild for AssemblyBuilderA64 {
  const CODE_UNIT: u32 = size_of::<u32>() as u32;

  fn text_mut(&mut self) -> &mut String {
    &mut self.text
  }

  fn get_code_size(&self) -> u32 {
    self.get_code_size()
  }

  fn bind_label(&mut self, label: &mut Label) {
    self.set_label_label(label);
  }

  fn set_new_label(&mut self) -> Label {
    self.set_label()
  }

  fn label_offset(&self, label: &Label) -> u32 {
    self.get_label_offset(label)
  }

  fn pad_nop(&mut self, bytes: u32) {
    self.nop(bytes);
  }
}

impl LowerState for IrLoweringX64 {
  fn start_block(&mut self, curr: &IrBlock) {
    self.start_block(curr);
  }

  fn check_safe_env(&mut self, exit: IrOp, index: u32, next: &IrBlock) {
    self.check_safe_env(exit, index, next);
  }

  fn lower_inst(&mut self, inst: &mut IrInst, index: u32, next: &IrBlock) {
    self.lower_inst(inst, index, next);
  }

  fn finish_block(&mut self, curr: &IrBlock, next: &IrBlock) {
    self.finish_block(curr, next);
  }

  fn finish_function(&mut self) {
    self.ir_lowering_x_64_finish_function();
  }

  fn has_error(&self) -> bool {
    self.has_error()
  }

  fn track_store_hint(&mut self, index: u32, hint: &StoreLocationHint) {
    self.regs.curr_inst_idx = index;
    self.value_tracker.process_store_location_hint(hint);
    self.regs.curr_inst_idx = K_INVALID_INST_IDX;
  }
}

impl LowerState for IrLoweringA64 {
  fn start_block(&mut self, curr: &IrBlock) {
    self.ir_lowering_a_64_start_block(curr);
  }

  fn check_safe_env(&mut self, exit: IrOp, index: u32, next: &IrBlock) {
    self.ir_lowering_a_64_check_safe_env(exit, index, next);
  }

  fn lower_inst(&mut self, inst: &mut IrInst, index: u32, next: &IrBlock) {
    self.ir_lowering_a_64_lower_inst(inst, index, next);
  }

  fn finish_block(&mut self, curr: &IrBlock, next: &IrBlock) {
    self.ir_lowering_a_64_finish_block(curr, next);
  }

  fn finish_function(&mut self) {
    self.ir_lowering_a_64_finish_function();
  }

  fn has_error(&self) -> bool {
    self.ir_lowering_a_64_has_error()
  }

  fn track_store_hint(&mut self, index: u32, hint: &StoreLocationHint) {
    self.regs.curr_inst_idx = index;
    self.value_tracker.process_store_location_hint(hint);
    self.regs.curr_inst_idx = K_INVALID_INST_IDX;
  }
}

/// IR 文本回显所需的只读视图，按字段借用现搭（不让整个 `IrFunction` 被共享借用锁住，
/// 从而与同一时刻对 `instructions`/`blocks[i].label` 的可变借用共存）。
struct IrDump<'a> {
  blocks: &'a Vec<IrBlock>,
  constants: &'a Vec<IrConst>,
  cfg: &'a CfgInfo,
  vm_exit_info: &'a DenseHashMap<u32, VmExitSyncInfo>,
  proto: Option<&'a Proto>,
}

/// 后继块视图：`None`（排序表末尾或其余皆死块）落到 `dummy` 哨兵，与原
/// `getNextBlock` 语义一致。
fn next_block<'a>(
  blocks: &'a [IrBlock],
  next_index: Option<u32>,
  dummy: &'a IrBlock,
) -> &'a IrBlock {
  match next_index {
    Some(index) => &blocks[index as usize],
    None => dummy,
  }
}

/// 单函数 IR 块序列的降低主体：按 `sorted_blocks` 顺序逐块发射，出错时把剩余块
/// 全部置 label 并返回 `false`（cpp `lowerImpl` 同款）。
///
/// 主体不含裸指针：块/指令一律按 `Vec` 下标访问（下标来自 `sorted_blocks` 与块的
/// `[start, finish]` 闭区间，均为 IR 构造期建立的合法不变量），唯一的 `unsafe` 是
/// `options.annotator` 这一 C ABI 回调边界。
fn lower_blocks<B: LowerBuild, L: LowerState>(
  build: &mut B,
  lowering: &mut L,
  function: &mut IrFunction,
  sorted_blocks: &[u32],
  bytecodeid: i32,
  options: &AssemblyOptions,
) -> bool {
  let mut bc_locations = vec![u32::MAX; function.instructions.len() + 1];

  for (i, item) in function.bc_mapping.iter().enumerate() {
    if item.ir_location != u32::MAX {
      bc_locations[item.ir_location as usize] = i as u32;
    }
  }

  let output_enabled = options.include_assembly || options.include_ir;

  let mut text_size = build.text_mut().len();
  let mut code_size = build.get_code_size();
  let mut seen_fallback = false;

  let dummy = IrBlock::default();

  debug_assert!(sorted_blocks[0] == 0);
  debug_assert!(function.entry_block == 0);

  for (i, &block_index) in sorted_blocks.iter().enumerate() {
    let curr = block_index as usize;

    if function.blocks[curr].kind == IrBlockKind::Dead {
      continue;
    }

    debug_assert!(function.blocks[curr].start != u32::MAX);
    debug_assert!(function.blocks[curr].finish != u32::MAX);
    debug_assert!(
      !seen_fallback
        || function.blocks[curr].kind == IrBlockKind::Fallback
        || function.blocks[curr].kind == IrBlockKind::ExitSync
    );

    let is_exit_block = function.blocks[curr].kind == IrBlockKind::Fallback
      || function.blocks[curr].kind == IrBlockKind::ExitSync;

    if is_exit_block && !seen_fallback {
      text_size = build.text_mut().len();
      code_size = build.get_code_size();
      seen_fallback = true;
    }

    if options.include_ir {
      dump_block(
        build,
        IrDump {
          blocks: &function.blocks,
          constants: &function.constants,
          cfg: &function.cfg,
          vm_exit_info: &function.vm_exit_info,
          proto: function.proto_view(),
        },
        curr,
        options,
      );
    }

    function.valid_restore_op_blocks.push(block_index);

    build.bind_label(&mut function.blocks[curr].label);

    if block_index == function.entry_block {
      let offset = build.label_offset(&function.blocks[curr].label);
      function.entry_location = offset;
    }

    lowering.start_block(&function.blocks[curr]);

    let next_index = get_next_block(function, sorted_blocks, i);

    if function.blocks[curr].expected_next_block != u32::MAX {
      debug_assert!(next_index == Some(function.blocks[curr].expected_next_block));
    }

    if (function.blocks[curr].flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
      if options.include_ir {
        if options.include_ir_prefix == IncludeIrPrefix::Yes {
          build.log_append(format_args!("# "));
        }

        build.log_append(format_args!(
          "  implicit CHECK_SAFE_ENV exit({})\n",
          function.blocks[curr].startpc
        ));
      }

      debug_assert!(function.blocks[curr].startpc != K_BLOCK_NO_START_PC);
      let startpc = function.blocks[curr].startpc;
      lowering.check_safe_env(
        IrOp {
          kind_and_index: IrOpKind::VmExit as u32 | (startpc << IrOp::INDEX_SHIFT),
        },
        K_INVALID_INST_IDX,
        next_block(&function.blocks, next_index, &dummy),
      );
    }

    for index in function.blocks[curr].start..=function.blocks[curr].finish {
      debug_assert!((index as usize) < function.instructions.len());

      let bc_location = bc_locations[index as usize];

      if let Some(annotator) = options
        .annotator
        .filter(|_| output_enabled && bc_location != u32::MAX)
      {
        // Safety: `annotator` 是 C ABI 回调，`annotator_context` 由 AssemblyOptions 构造方
        // 保证为该回调可消费的存活指针（或 null），`result` 是本作用域的活借用。
        unsafe {
          annotator(
            options.annotator_context,
            build.text_mut(),
            bytecodeid,
            bc_location as i32,
          )
        };

        let bc_types = function.get_bytecode_types_at(bc_location as i32);

        if bc_types.result != LBC_TYPE_ANY
          || bc_types.a != LBC_TYPE_ANY
          || bc_types.b != LBC_TYPE_ANY
          || bc_types.c != LBC_TYPE_ANY
        {
          to_string_bytecode_types(
            build.text_mut(),
            &bc_types,
            UserdataTypes::new(&options.compilation_options.userdata_types),
          );

          build.log_append(format_args!("\n"));
        }
      }

      if bc_location != u32::MAX {
        let label = if index == function.blocks[curr].start {
          function.blocks[curr].label
        } else {
          build.set_new_label()
        };

        function.bc_mapping[bc_location as usize].asm_location = build.label_offset(&label);
      }

      if is_pseudo(function.instructions[index as usize].cmd) {
        if let Some(hint) = function.find_store_location_hint(index) {
          lowering.track_store_hint(index, hint);
        }

        debug_assert!(function.instructions[index as usize].use_count == 0);
        continue;
      }

      debug_assert!(
        function.instructions[index as usize].last_use == 0
          || function.instructions[index as usize].use_count != 0
      );

      if options.include_ir {
        dump_inst(
          build,
          &IrDump {
            blocks: &function.blocks,
            constants: &function.constants,
            cfg: &function.cfg,
            vm_exit_info: &function.vm_exit_info,
            proto: function.proto_view(),
          },
          &mut function.instructions[index as usize],
          curr,
          index,
          options,
        );
      }

      lowering.lower_inst(
        &mut function.instructions[index as usize],
        index,
        next_block(&function.blocks, next_index, &dummy),
      );

      if lowering.has_error() {
        for &abandoned in sorted_blocks.iter().skip(i + 1) {
          build.bind_label(&mut function.blocks[abandoned as usize].label);
        }

        lowering.finish_function();

        return false;
      }
    }

    let curr_block = &function.blocks[curr];
    lowering.finish_block(curr_block, next_block(&function.blocks, next_index, &dummy));

    pad_block_nop(build, function, curr, next_index, &dummy);

    if options.include_ir && options.include_ir_prefix == IncludeIrPrefix::Yes {
      build.log_append(format_args!("#\n"));
    }

    if function.blocks[curr].expected_next_block == u32::MAX {
      function.valid_restore_op_blocks.clear();
    }
  }

  if !seen_fallback {
    text_size = build.text_mut().len();
    code_size = build.get_code_size();
  }

  lowering.finish_function();

  if output_enabled && !options.include_outlined_code && text_size < build.text_mut().len() {
    build.text_mut().truncate(text_size);

    if options.include_assembly {
      let skipped = build
        .get_code_size()
        .wrapping_sub(code_size)
        .wrapping_mul(B::CODE_UNIT);
      build.log_append(format_args!(
        "; skipping {} bytes of outlined code\n",
        skipped
      ));
    }
  }

  true
}

/// 块级 IR 文本回显（`include_ir` 通道）。
fn dump_block<B: LowerBuild>(
  build: &mut B,
  src: IrDump<'_>,
  curr: usize,
  options: &AssemblyOptions,
) {
  if options.include_ir_prefix == IncludeIrPrefix::Yes {
    build.log_append(format_args!("# "));
  }

  let mut ctx = IrToStringContext {
    result: build.text_mut(),
    blocks: src.blocks,
    constants: src.constants,
    cfg: src.cfg,
    vm_exit_info: src.vm_exit_info,
    proto: src.proto,
  };
  to_string_detailed_block(
    &mut ctx,
    &src.blocks[curr],
    curr as u32,
    options.include_use_info,
    options.include_cfg_info,
    options.include_reg_flow_info,
  );
}

/// 指令级 IR 文本回显（`include_ir` 通道）。
fn dump_inst<B: LowerBuild>(
  build: &mut B,
  src: &IrDump<'_>,
  inst: &mut IrInst,
  curr: usize,
  index: u32,
  options: &AssemblyOptions,
) {
  if options.include_ir_prefix == IncludeIrPrefix::Yes {
    build.log_append(format_args!("# "));
  }

  let mut ctx = IrToStringContext {
    result: build.text_mut(),
    blocks: src.blocks,
    constants: src.constants,
    cfg: src.cfg,
    vm_exit_info: src.vm_exit_info,
    proto: src.proto,
  };
  to_string_detailed_inst(
    &mut ctx,
    &src.blocks[curr],
    curr as u32,
    inst,
    index,
    options.include_use_info,
  );
}

/// jit_rng 通道：块尾按 cpp 同款条件插入 `0..max_nop` 字节随机 nop。
fn pad_block_nop<B: LowerBuild>(
  build: &mut B,
  function: &mut IrFunction,
  curr: usize,
  next_index: Option<u32>,
  dummy: &IrBlock,
) {
  if function.jit_rng_state == 0 {
    return;
  }

  let finish = function.blocks[curr].finish as usize;
  let next = next_block(&function.blocks, next_index, dummy);
  let (next_start, next_use_count) = (next.start, next.use_count);
  let term_cmd = function.instructions[finish].cmd;

  let block_falls_through = any_argument_match(&function.instructions[finish], |op| {
    op.kind() == IrOpKind::Block && function.blocks[op.index() as usize].start == next_start
  });

  if block_falls_through && term_cmd == IrCmd::JUMP && next_use_count == 1 {
    return;
  }

  let max_nop_bytes = if block_falls_through { 4 } else { 8 };
  let nop_bytes = jit_rng_random(&mut function.jit_rng_state) % max_nop_bytes;

  if nop_bytes > 0 {
    build.pad_nop(nop_bytes);
  }
}

/// 单函数 IR 块序列降低（X64）。
///
/// # Safety
/// `build`/`lowering`/`function`/`sorted_blocks` 必须为互不重叠的存活借用，`lowering`
/// 内持有的 build/function 裸指针须指向同批对象，`sorted_blocks` 为 `function.blocks`
/// 的合法下标表（cpp 参考实现的同款前置条件）。
pub unsafe fn lower_impl_x_64(
  build: &mut AssemblyBuilderX64,
  lowering: &mut IrLoweringX64,
  function: &mut IrFunction,
  sorted_blocks: &[u32],
  bytecodeid: i32,
  options: &AssemblyOptions,
) -> bool {
  // 泛型主体本身是安全函数（其内部唯一 `unsafe` 边界是 annotator 的 C ABI 回调，已就地
  // 收窄）；本入口保留 `unsafe fn` 只为向调用方声明上面的借用与下标前置条件。
  lower_blocks(
    build,
    lowering,
    function,
    sorted_blocks,
    bytecodeid,
    options,
  )
}

/// 单函数 IR 块序列降低（A64），语义与 X64 版逐路径一致（cpp 同款模板共用主体）。
///
/// # Safety
/// 与 [`lower_impl_x_64`] 同款：借用互不重叠且存活，`sorted_blocks` 为合法下标表。
pub unsafe fn lower_impl_a_64(
  build: &mut AssemblyBuilderA64,
  lowering: &mut IrLoweringA64,
  function: &mut IrFunction,
  sorted_blocks: &[u32],
  bytecodeid: i32,
  options: &AssemblyOptions,
) -> bool {
  // 同 X64 入口：泛型主体为安全函数，`unsafe fn` 仅承载对调用方的前置条件声明。
  lower_blocks(
    build,
    lowering,
    function,
    sorted_blocks,
    bytecodeid,
    options,
  )
}
