use core::mem::offset_of;

use ulua_vm::type_aliases::{
  closure::Closure as ClosureAlias, lua_table::LuaTable as LuaTableAlias,
};

use crate::{
  enums::kind_a_64::KindA64,
  functions::cast_reg::cast_reg,
  records::{
    ir_block::IrBlock, ir_lowering_a_64::IrLoweringA64, ir_op::IrOp, label::Label,
    register_a_64::RegisterA64,
  },
  type_aliases::mem::mem,
};

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
  RegisterA64 {
    bits: kind as u8 | (index << 3),
  }
}

const R_CLOSURE: RegisterA64 = reg(KindA64::X, 23);

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_check_safe_env(&mut self, target: IrOp, index: u32, next: &IrBlock) {
    let _ = next;
    let mut fresh = Label { id: 0, location: 0 };
    let temp: RegisterA64 = self.regs.alloc_temp(KindA64::X);
    let tempw: RegisterA64 = cast_reg(KindA64::W, temp);
    unsafe {
      let offset_env = offset_of!(ClosureAlias, env);
      let offset_safeenv = offset_of!(LuaTableAlias, safeenv);
      (*self.build).ldr(temp, mem(R_CLOSURE, offset_env as i32));
      (*self.build).ldrb(tempw, mem(temp, offset_safeenv as i32));
      (*self.build).cbz(
        tempw,
        self.ir_lowering_a_64_get_target_label(target, index, &mut fresh),
      );
      self.ir_lowering_a_64_finalize_target_label(target, index, &mut fresh);
    }
  }
}
