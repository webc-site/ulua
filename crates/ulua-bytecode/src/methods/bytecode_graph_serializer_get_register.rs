use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_unreachable::LUAU_UNREACHABLE};

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{
    bc_inst::BcInst, bc_op::BcOp, bc_proj::BcProj, bytecode_builder::K_INVALID_REG,
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
        // 上游只在无记录分支索引 ops[0]，但断言在取 regs 之前，故先读出首操作数
        let first_op = self.func.phi_op(op).ops.first().copied();
        LUAU_ASSERT!(first_op.is_some());

        if let Some(&reg) = self.func.regs.get(&op) {
          u32::from(reg)
        } else {
          let Some(first_op) = first_op else {
            // cpp 在此处解引用空 ops（release 下 UB）；空 phi 只可能来自损坏的图
            LUAU_ASSERT!(false, "phi without inputs");
            return 0;
          };
          LUAU_ASSERT!(first_op != op);
          u32::from(self.get_register(first_op))
        }
      }
      BcOpKind::Inst => {
        let reg = self.func.regs.get(&op).copied();
        LUAU_ASSERT!(reg.is_some());
        u32::from(reg.unwrap_or(0))
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
