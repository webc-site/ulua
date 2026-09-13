use ulua_vm::{
  enums::lua_type::LuaType,
  macros::{
    bitmask::{bit2mask, bitmask},
    blackbit::BLACKBIT,
    white_0_bit::WHITE0BIT,
    white_1_bit::WHITE1BIT,
  },
  records::g_cheader::GCheader,
};

use crate::{
  enums::{condition_x_64::ConditionX64, ir_op_kind::IrOpKind, size_x_64::SizeX64},
  functions::{
    dword_reg::dword_reg, is_gco::is_gco, luau_constant_tag::luau_constant_tag,
    luau_constant_value::luau_constant_value, luau_reg_tag::luau_reg_tag,
    luau_reg_value::luau_reg_value, vm_const_op::vm_const_op, vm_reg_op::vm_reg_op,
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_op::IrOp, label::Label, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

pub fn check_object_barrier_conditions(
  build: &mut AssemblyBuilderX64,
  tmp: RegisterX64,
  object: RegisterX64,
  ra: RegisterX64,
  ra_op: IrOp,
  ratag: i32,
  skip: &mut Label,
) {
  // Barrier should've been optimized away if we know that it's not collectable, checking for correctness
  if ratag == -1 || !is_gco(ratag as u8) {
    // iscollectable(ra)
    if ra_op.kind() == IrOpKind::Inst {
      build.vpextrd(dword_reg(tmp), ra, 3);
      build.cmp(
        OperandX64::reg(dword_reg(tmp)),
        OperandX64::imm(LuaType::String as i32),
      );
    } else {
      let tag = if ra_op.kind() == IrOpKind::VmReg {
        luau_reg_tag(vm_reg_op(ra_op))
      } else {
        luau_constant_tag(vm_const_op(ra_op))
      };
      build.cmp(tag, OperandX64::imm(LuaType::String as i32));
    }

    build.jcc(ConditionX64::Less, skip);
  }

  // isblack(obj2gco(o))
  build.test(
    OperandX64::mem(
      SizeX64::Byte,
      RegisterX64::NOREG,
      1,
      object,
      core::mem::offset_of!(GCheader, marked) as i32,
    ),
    OperandX64::imm(bitmask(BLACKBIT as i32)),
  );
  build.jcc(ConditionX64::Zero, skip);

  // iswhite(gcvalue(ra))
  if ra_op.kind() == IrOpKind::Inst {
    build.vmovq(OperandX64::reg(tmp), OperandX64::reg(ra));
  } else {
    let value = if ra_op.kind() == IrOpKind::VmReg {
      luau_reg_value(vm_reg_op(ra_op))
    } else {
      luau_constant_value(vm_const_op(ra_op))
    };
    build.mov(OperandX64::reg(tmp), value);
  }
  build.test(
    OperandX64::mem(
      SizeX64::Byte,
      RegisterX64::NOREG,
      1,
      tmp,
      core::mem::offset_of!(GCheader, marked) as i32,
    ),
    OperandX64::imm(bit2mask(WHITE0BIT, WHITE1BIT as i32)),
  );
  build.jcc(ConditionX64::Zero, skip);
}
