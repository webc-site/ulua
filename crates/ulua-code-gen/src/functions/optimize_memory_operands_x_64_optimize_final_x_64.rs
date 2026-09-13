use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::replace_inst_operand_ir_utils::replace_ir_function_ir_inst_operand,
  macros::{codegen_assert::CODEGEN_ASSERT, op_a::op_a, op_b::op_b},
  records::{ir_block::IrBlock, ir_function::IrFunction, ir_inst::IrInst},
};

pub fn optimize_memory_operands_x_64_ir_function_ir_block(
  function: &mut IrFunction,
  block: &mut IrBlock,
) {
  CODEGEN_ASSERT!(block.kind != IrBlockKind::Dead);

  let start = block.start;
  let finish = block.finish;

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
            tag_op_a.kind() == IrOpKind::VmReg || tag_op_a.kind() == IrOpKind::VmConst
          } {
            let replacement_op = op_a(tag);
            replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
          }
        }
      }
      IrCmd::CheckTruthy => {
        // Read both operands from inst before any function calls to satisfy borrow checker
        let inst_op_a = op_a(inst);
        let inst_op_b = op_b(inst.clone());

        if inst_op_a.kind() == IrOpKind::Inst {
          let tag = function.inst_op(inst_op_a);

          if tag.use_count == 1 && tag.cmd == IrCmd::LoadTag && {
            let tag_op_a = op_a(tag);
            tag_op_a.kind() == IrOpKind::VmReg || tag_op_a.kind() == IrOpKind::VmConst
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
        let inst_op_b = op_b(inst.clone());
        if inst_op_b.kind() == IrOpKind::Inst {
          let rhs = function.inst_op(inst_op_b);

          if rhs.use_count == 1 && rhs.cmd == IrCmd::LoadDouble && {
            let rhs_op_a = op_a(rhs);
            rhs_op_a.kind() == IrOpKind::VmReg || rhs_op_a.kind() == IrOpKind::VmConst
          } {
            let replacement_op = op_a(rhs);
            replace_ir_function_ir_inst_operand(function, index, 1, replacement_op);
          }
        }
      }
      IrCmd::JumpEqTag => {
        // Read both operands from inst before any function calls to satisfy borrow checker
        let inst_op_a = op_a(inst);
        let inst_op_b = op_b(inst.clone());

        if inst_op_a.kind() == IrOpKind::Inst {
          let tag_a = function.inst_op(inst_op_a);

          if tag_a.use_count == 1 && tag_a.cmd == IrCmd::LoadTag && {
            let tag_a_op_a = op_a(tag_a);
            tag_a_op_a.kind() == IrOpKind::VmReg || tag_a_op_a.kind() == IrOpKind::VmConst
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
            tag_b_op_a.kind() == IrOpKind::VmReg || tag_b_op_a.kind() == IrOpKind::VmConst
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
            arg_op_a.kind() == IrOpKind::VmReg || arg_op_a.kind() == IrOpKind::VmConst
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
