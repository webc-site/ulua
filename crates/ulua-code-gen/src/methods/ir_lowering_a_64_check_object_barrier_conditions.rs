use core::mem::offset_of;

use ulua_vm::{
  enums::lua_type::LuaType,
  macros::{bitmask::bit2mask, blackbit::BLACKBIT, white_0_bit::WHITE0BIT, white_1_bit::WHITE1BIT},
  records::g_cheader::GCheader,
  type_aliases::t_value::TValue,
};

use crate::{
  enums::{condition_a_64::ConditionA64, ir_op_kind::IrOpKind, kind_a_64::KindA64},
  functions::{cast_reg::cast_reg, is_gco::is_gco},
  records::{
    ir_lowering_a_64::IrLoweringA64, ir_op::IrOp, label::Label, register_a_64::RegisterA64,
  },
  type_aliases::mem::mem,
};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_check_object_barrier_conditions(
    &mut self,
    object: RegisterA64,
    temp: RegisterA64,
    ra: RegisterA64,
    ra_op: IrOp,
    ratag: i32,
    skip: &mut Label,
  ) {
    let tempw = cast_reg(KindA64::W, temp);

    unsafe {
      if ratag == -1 || !is_gco(ratag as u8) {
        if ra_op.kind() == IrOpKind::Inst {
          (*self.build).umov_4s(tempw, ra, 3);
        } else {
          let addr = self.ir_lowering_a_64_temp_addr(ra_op, offset_of!(TValue, tt) as i32, temp);
          (*self.build).ldr(tempw, addr);
        }

        (*self.build).cmp_register_a_64_u16(tempw, LuaType::String as u16);
        (*self.build).b_condition_a_64_label(ConditionA64::Less, skip);
      }

      (*self.build).ldrb(tempw, mem(object, offset_of!(GCheader, marked) as i32));
      (*self.build).tbz(tempw, BLACKBIT, skip);

      if ra_op.kind() == IrOpKind::Inst {
        (*self.build).fmov_register_a_64_register_a_64(temp, cast_reg(KindA64::D, ra));
      } else {
        let addr = self.ir_lowering_a_64_temp_addr(ra_op, offset_of!(TValue, value) as i32, temp);
        (*self.build).ldr(temp, addr);
      }

      (*self.build).ldrb(tempw, mem(temp, offset_of!(GCheader, marked) as i32));
      (*self.build).tst_register_a_64_u32(tempw, bit2mask(WHITE0BIT, WHITE1BIT as i32) as u32);
      (*self.build).b_condition_a_64_label(ConditionA64::Equal, skip);
    }
  }
}
