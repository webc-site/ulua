use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_unreachable::LUAU_UNREACHABLE};

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{
    bc_inst::BcInst, bc_op::BcOp, bc_phi::BcPhi, bc_proj::BcProj, bytecode_builder::K_INVALID_REG,
    bytecode_graph_serializer::BytecodeGraphSerializer,
  },
  type_aliases::reg::Reg,
};

impl<'a> BytecodeGraphSerializer<'a> {
  /// cpp `getRegisterRaw`（BytecodeGraphSerializer.h:62-101）：把 Phi/Inst/Proj
  /// 解析到具体寄存器号，不做溢出检查。
  pub fn get_register_raw(&mut self, op: BcOp) -> u32 {
    // 循环携带 phi（内外累加 phi 互为操作数成环）经 phi.ops 递归不终止，
    // 故先查 makePhi 时记录的 regs；只有无记录的返回值合并 phi（inliner 插入、
    // 无环）才沿首操作数解析。
    match op.kind {
      BcOpKind::Phi => {
        if let Some(&reg) = self.func.regs.get(&op) {
          u32::from(reg)
        } else {
          let (ops_len, first_op);
          {
            let phi: &mut BcPhi = self.func.phi_op(op);
            LUAU_ASSERT!(!phi.ops.is_empty());
            LUAU_ASSERT!(phi.ops[0] != op);

            ops_len = phi.ops.len();
            first_op = phi.ops[0];
          }

          let res = u32::from(self.get_register(first_op));

          // 断言路径：逐一复核各 phi 操作数解析到同一寄存器
          let rest: Vec<BcOp> = {
            let phi: &mut BcPhi = self.func.phi_op(op);
            phi.ops.as_slice()[..ops_len].to_vec()
          };
          for phi_op in rest {
            LUAU_ASSERT!(res == u32::from(self.get_register(phi_op)));
          }

          res
        }
      }
      BcOpKind::Inst => {
        let res = self.func.regs.get(&op);
        LUAU_ASSERT!(res.is_some());
        u32::from(*res.unwrap())
      }
      BcOpKind::Proj => {
        // Avoid holding `&mut` to `self.func` while recursively calling `self.get_register`.
        let proj = {
          let proj: &mut BcProj = self.func.proj_op(op);
          *proj
        };
        u32::from(self.get_register(proj.op)) + proj.index
      }
      BcOpKind::VmReg => op.index,
      _ => {
        LUAU_UNREACHABLE!()
      }
    }
  }

  /// cpp `getRegister`（BytecodeGraphSerializer.h:102-121）：溢出置 error。
  pub fn get_register(&mut self, op: BcOp) -> Reg {
    let raw = self.get_register_raw(op);

    if raw >= K_INVALID_REG {
      LUAU_ASSERT!(false, "register reference overflow");
      self.error = true;
    }

    if raw >= self.func.maxstacksize as u32 {
      LUAU_ASSERT!(false, "register overflows the function stack");
      self.error = true;
    }

    raw as Reg
  }

  /// cpp `getRegInputForRange`（BytecodeGraphSerializer.h:127-153）：取范围
  /// 起始寄存器，按 count 检查 `reg + range` 不越过函数栈顶。
  pub fn get_reg_input_for_range(&mut self, insn: &BcInst, index: u8, count: i32) -> Reg {
    LUAU_ASSERT!((index as usize) < insn.ops.len());

    let reg = self.get_register_raw(insn.ops[index as usize]);

    if !(-1..255).contains(&count) {
      LUAU_ASSERT!(false, "register count overflow");
      self.error = true;
    }

    let range = if count < 0 { 0 } else { count };

    if reg >= K_INVALID_REG {
      LUAU_ASSERT!(false, "register reference overflow");
      self.error = true;
    }

    if reg + range as u32 > self.func.maxstacksize as u32 {
      LUAU_ASSERT!(false, "register overflows the function stack");
      self.error = true;
    }

    reg as Reg
  }
}
