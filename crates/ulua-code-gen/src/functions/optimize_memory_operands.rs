//! Source: `CodeGen/src/OptimizeFinalX64.cpp:134`
//!
//! 顶层驱动：对每个存活 block（跳过 Dead 与 ExitSync）
//! 运行逐 block 的内存操作数优化。

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::ir::replace_ir_function_ir_inst_operand,
  macros::{
    codegen_assert::CODEGEN_ASSERT,
    ir_operand::{op_a, op_b_ref},
  },
  records::{ir_function::IrFunction, ir_inst::IrInst},
};

fn optimize_memory_operands_block(function: &mut IrFunction, start: u32, finish: u32) {
  for index in start..=finish {
    CODEGEN_ASSERT!(index < function.instructions.len() as u32);
    let inst: &mut IrInst = &mut function.instructions[index as usize];

    match inst.cmd {
      IrCmd::CheckTag => {
        let inst_op_a = op_a(inst);
        if inst_op_a.kind() == IrOpKind::Inst {
          let tag = function.inst_op(inst_op_a);

          if tag.use_count == 1 && tag.cmd == IrCmd::LoadTag && {
            let tag_op_a = op_a(tag);
            matches!(tag_op_a.kind(), IrOpKind::VmReg | IrOpKind::VmConst)
          } {
            let replacement_op = op_a(tag);
            replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
          }
        }
      }
      IrCmd::CheckTruthy => {
        // 在任何函数调用前先从 inst 读出两个操作数，以满足借用检查器
        let inst_op_a = op_a(inst);
        let inst_op_b = op_b_ref(inst);

        if inst_op_a.kind() == IrOpKind::Inst {
          let tag = function.inst_op(inst_op_a);

          if tag.use_count == 1 && tag.cmd == IrCmd::LoadTag && {
            let tag_op_a = op_a(tag);
            matches!(tag_op_a.kind(), IrOpKind::VmReg | IrOpKind::VmConst)
          } {
            let replacement_op = op_a(tag);
            replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
          }
        }

        if inst_op_b.kind() == IrOpKind::Inst {
          let value = function.inst_op(inst_op_b);

          if value.use_count == 1 && value.cmd == IrCmd::LoadInt {
            let replacement_op = op_a(value);
            replace_ir_function_ir_inst_operand(function, index, 1, replacement_op);
          }
        }
      }
      IrCmd::AddNum
      | IrCmd::SubNum
      | IrCmd::MulNum
      | IrCmd::DivNum
      | IrCmd::IdivNum
      | IrCmd::ModNum
      | IrCmd::MinNum
      | IrCmd::MaxNum => {
        let inst_op_b = op_b_ref(inst);
        if inst_op_b.kind() == IrOpKind::Inst {
          let rhs = function.inst_op(inst_op_b);

          if rhs.use_count == 1 && rhs.cmd == IrCmd::LoadDouble && {
            let rhs_op_a = op_a(rhs);
            matches!(rhs_op_a.kind(), IrOpKind::VmReg | IrOpKind::VmConst)
          } {
            let replacement_op = op_a(rhs);
            replace_ir_function_ir_inst_operand(function, index, 1, replacement_op);
          }
        }
      }
      IrCmd::JumpEqTag => {
        // 在任何函数调用前先从 inst 读出两个操作数，以满足借用检查器
        let inst_op_a = op_a(inst);
        let inst_op_b = op_b_ref(inst);

        if inst_op_a.kind() == IrOpKind::Inst {
          let tag_a = function.inst_op(inst_op_a);

          if tag_a.use_count == 1 && tag_a.cmd == IrCmd::LoadTag && {
            let tag_a_op_a = op_a(tag_a);
            matches!(tag_a_op_a.kind(), IrOpKind::VmReg | IrOpKind::VmConst)
          } {
            let replacement_op = op_a(tag_a);
            replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
            continue;
          }
        }

        if inst_op_b.kind() == IrOpKind::Inst {
          let tag_b = function.inst_op(inst_op_b);

          if tag_b.use_count == 1 && tag_b.cmd == IrCmd::LoadTag && {
            let tag_b_op_a = op_a(tag_b);
            matches!(tag_b_op_a.kind(), IrOpKind::VmReg | IrOpKind::VmConst)
          } {
            let replacement_op = op_a(tag_b);
            function.instructions[index as usize]
              .ops
              .as_mut_slice()
              .swap(0, 1);

            replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
          }
        }
      }
      IrCmd::JumpCmpNum => {
        let inst_op_a = op_a(inst);
        if inst_op_a.kind() == IrOpKind::Inst {
          let num = function.inst_op(inst_op_a);

          if num.use_count == 1 && num.cmd == IrCmd::LoadDouble {
            let replacement_op = op_a(num);
            replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
          }
        }
      }
      IrCmd::FloorNum | IrCmd::CeilNum | IrCmd::RoundNum | IrCmd::SqrtNum | IrCmd::AbsNum => {
        let inst_op_a = op_a(inst);
        if inst_op_a.kind() == IrOpKind::Inst {
          let arg = function.inst_op(inst_op_a);

          if arg.use_count == 1 && arg.cmd == IrCmd::LoadDouble && {
            let arg_op_a = op_a(arg);
            matches!(arg_op_a.kind(), IrOpKind::VmReg | IrOpKind::VmConst)
          } {
            let replacement_op = op_a(arg);
            replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
          }
        }
      }
      _ => {}
    }
  }
}

pub fn optimize_memory_operands(function: &mut IrFunction) {
  // 跳过 Dead block。把 load 内联进其消费者会破坏 ExitSync block
  // 中 VM exit sync info argOps 列出的操作数，故也跳过。
  // 块下标仅作游标使用（不进入下游），先收集存活块的指令区间再逐个优化，
  // 从而在按区间可变借用 function 时释放对 blocks 的只读借用。
  let live_ranges: Vec<(u32, u32)> = function
    .blocks
    .iter()
    .filter(|block| !matches!(block.kind, IrBlockKind::Dead | IrBlockKind::ExitSync))
    .map(|block| (block.start, block.finish))
    .collect();

  for (start, finish) in live_ranges {
    optimize_memory_operands_block(function, start, finish);
  }
}
