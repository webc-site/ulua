use alloc::vec::Vec;

use ulua_common::{
  FFlag::LuauCodegenVmExitSyncMultiUse,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    add_use::add_use, has_side_effects::has_side_effects, is_unsafe_to_sink::is_unsafe_to_sink,
    remove_use::remove_use, replace_ir_utils::replace_ir_function_ir_op_ir_op,
    visit_arguments::visit_arguments,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_builder::IrBuilder, ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp,
    vm_exit_sync_info::VmExitSyncInfo,
  },
};

fn collect_inputs(inst: &mut IrInst, inputs: &mut Vec<(IrOp, u32)>) {
  visit_arguments(inst, |op| {
    if op.kind() == IrOpKind::Inst {
      if let Some(slot) = inputs.iter_mut().find(|el| el.0 == op) {
        slot.1 += 1;
      } else {
        inputs.push((op, 1));
      }
    }
  });
}

/// cpp `generateVmExitBlocks` 中 `LuauCodegenVmExitSyncMultiUse` 开启时的预处理
/// (CodeGen/src/OptimizeDeadStore.cpp:1497-1554)：
/// 统计各指令在 exit sync（store 参数与已沉入指令参数）中的使用次数，
/// 若 exit 使用次数等于总使用次数，则该指令仅被 exit 使用，可整体沉入 exit 块
fn collect_exit_sync_private_insts(
  function: &mut IrFunction,
  recorded_vm_exit_syncs: &[u32],
) -> DenseHashSet<u32> {
  let mut private_insts: DenseHashSet<u32> = DenseHashSet::new(!0u32);
  let mut exit_inst_use_counts: DenseHashMap<u32, u32> = DenseHashMap::new(!0u32);
  let mut worklist: Vec<u32> = Vec::new();

  // 记录一次 exit 侧使用：计数并压入工作列表（重复入列是安全的）
  fn record_exit_use(op: IrOp, counts: &mut DenseHashMap<u32, u32>, worklist: &mut Vec<u32>) {
    if op.kind() == IrOpKind::Inst {
      let idx = op.index();
      *counts.get_or_insert(idx) += 1;
      worklist.push(idx);
    }
  }

  // 先记录 store 指令自身参数（exit 原始输入）的使用次数
  for &vm_exit_sync_location in recorded_vm_exit_syncs {
    let sync_info = function.vm_exit_info.get_or_insert(vm_exit_sync_location);

    for reg_store in &mut sync_info.reg_stores {
      for record in reg_store.stores.as_mut_slice() {
        visit_arguments(&mut record.backup, |op| {
          record_exit_use(op, &mut exit_inst_use_counts, &mut worklist)
        });
      }
    }
  }

  // 逐个检查工作列表：全部使用都来自 exit sync 的指令标记为 exit 专用，
  // 并把它的参数继续入列，链式依赖由此传播
  while let Some(inst_idx) = worklist.pop() {
    if private_insts.contains(&inst_idx) {
      continue;
    }

    let inst: &mut IrInst = &mut function.instructions[inst_idx as usize];

    let exit_only = match exit_inst_use_counts.find(&inst_idx) {
      Some(&exit_use_count) => {
        inst.use_count as u32 == exit_use_count
          && !has_side_effects(inst.cmd)
          && !is_unsafe_to_sink(inst.cmd)
      }
      None => false,
    };

    if exit_only {
      private_insts.insert(inst_idx);
      visit_arguments(inst, |op| {
        record_exit_use(op, &mut exit_inst_use_counts, &mut worklist)
      });
    }
  }

  private_insts
}

fn redirect(op: &mut IrOp, inst_redir: &DenseHashMap<u32, u32>, inputs: &[(IrOp, u32)]) {
  if op.kind() == IrOpKind::Inst {
    if let Some(new_index) = inst_redir.find(&op.index()) {
      *op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, *new_index);
    } else if !inputs.iter().any(|el| el.0 == *op) {
      // Values can only be used if they are defined in the same block or be an input
      CODEGEN_ASSERT!(false);
    }
  }
}

pub fn generate_vm_exit_blocks(build: &mut IrBuilder, recorded_vm_exit_syncs: &Vec<u32>) {
  // build 与 function 交替可变借用，只能以裸指针绕开借用检查
  let function: *mut IrFunction = &mut build.function;
  let multi_use = LuauCodegenVmExitSyncMultiUse.get();

  // multi-use 模式：预先找出仅被 exit sync 使用的指令集合
  let exit_sync_private_insts = multi_use
    .then(|| collect_exit_sync_private_insts(unsafe { &mut *function }, recorded_vm_exit_syncs));

  for &vm_exit_sync_location in recorded_vm_exit_syncs {
    let sync_info: *mut VmExitSyncInfo = unsafe {
      (*function)
        .vm_exit_info
        .get_or_insert(vm_exit_sync_location)
    };

    if unsafe { (*sync_info).reg_stores.is_empty() } {
      continue;
    }

    // We will be collecting instructions we want to move into the VM exit in reverse order
    let mut store_instructions: Vec<IrInst> = Vec::new();
    let mut arg_instructions: Vec<u32> = Vec::new();
    let mut inputs: Vec<(IrOp, u32)> = Vec::new();
    // Set of inputs we already sunk inside
    let mut sunk_instructions: DenseHashSet<u32> = DenseHashSet::new(!0u32);

    // Start with the store instruction we got
    for reg_store in unsafe { &mut (*sync_info).reg_stores } {
      for record in reg_store.stores.as_slice() {
        let mut backup = record.backup.clone();
        collect_inputs(&mut backup, &mut inputs);
        store_instructions.push(backup);
      }
    }

    // For each input we got, see if we can move it inside the exit block
    // This is possible when one or more exit syncs are the only users of the value
    let mut i = 0usize;
    while i < inputs.len() {
      let (input_op, input_count) = inputs[i];

      let inst: &mut IrInst = unsafe { (*function).inst_op(input_op) };

      // multi-use 模式以预处理结果为准（预处理已校验副作用与可沉入性）
      let can_sink = match exit_sync_private_insts.as_ref() {
        Some(private_insts) => private_insts.contains(&input_op.index()),
        None => {
          inst.use_count as u32 == input_count
            && !has_side_effects(inst.cmd)
            && !is_unsafe_to_sink(inst.cmd)
        }
      };

      if can_sink {
        let inst_idx = input_op.index();

        if multi_use && sunk_instructions.contains(&inst_idx) {
          // 其它沉入已收录过该指令，移到尾部使其先被克隆
          arg_instructions.retain(|&idx| idx != inst_idx);
        }
        arg_instructions.push(inst_idx);
        if multi_use {
          sunk_instructions.insert(inst_idx);
        }

        inputs.remove(i); // Delete this input

        collect_inputs(
          unsafe { &mut (&mut (*function).instructions)[inst_idx as usize] },
          &mut inputs,
        );
      } else {
        i += 1;
      }
    }

    for input in &inputs {
      unsafe {
        (*sync_info).arg_ops.push_back(input.0);
      }
    }

    // We now should have an extracted instruction chain with no side effects in reverse order
    let block_op = build.block(IrBlockKind::ExitSync);
    unsafe {
      (*sync_info).block = block_op;
      *(*function)
        .block_to_vm_exit_map
        .get_or_insert(block_op.index()) = vm_exit_sync_location;
    }
    build.begin_block(block_op);

    let mut inst_redir: DenseHashMap<u32, u32> = DenseHashMap::new(!0u32);

    for &inst_idx in arg_instructions.iter().rev() {
      CODEGEN_ASSERT!((inst_idx as usize) < unsafe { (*function).instructions.len() });
      let mut clone = unsafe { (&(*function).instructions)[inst_idx as usize].clone() };

      // 重定向后再计数：与上游两段循环逐操作数等价，合并为单次遍历
      for op in clone.ops.as_mut_slice() {
        redirect(op, &inst_redir, &inputs);
        add_use(unsafe { &mut *function }, *op);
      }

      // Instructions that referenced the original will have to be adjusted to use the clone
      unsafe {
        *inst_redir.get_or_insert(inst_idx) = (*function).instructions.len() as u32;
      }

      // Reconstruct the fresh clone
      build.inst_ir_cmd_ir_ops(clone.cmd, &clone.ops);
    }

    for mut inst in store_instructions {
      let mut clone = inst.clone();

      // 重定向后再计数：与上游两段循环逐操作数等价，合并为单次遍历
      for op in clone.ops.as_mut_slice() {
        redirect(op, &inst_redir, &inputs);
        add_use(unsafe { &mut *function }, *op);
      }

      // Reconstruct the fresh clone
      build.inst_ir_cmd_ir_ops(clone.cmd, &clone.ops);

      visit_arguments(&mut inst, |op| remove_use(unsafe { &mut *function }, op));
    }

    let vm_exit = unsafe { (*sync_info).vm_exit };
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, vm_exit);

    // Replace guard VM exit with an exit sync block
    let block_for_replace = unsafe { (*sync_info).block };
    let guard = unsafe { &mut (&mut (*function).instructions)[vm_exit_sync_location as usize] };
    for op in guard.ops.as_mut_slice() {
      if op.kind() == IrOpKind::VmExit && *op == vm_exit {
        replace_ir_function_ir_op_ir_op(unsafe { &mut *function }, op, block_for_replace);
        break;
      }
    }
  }
}
