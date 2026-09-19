use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_imm_kind::BcImmKind,
  records::{bc_imm::BcImm, bc_inst::BcInst, bytecode_graph_serializer::BytecodeGraphSerializer},
};

impl<'a> BytecodeGraphSerializer<'a> {
  /// cpp `getImmIntAsSignedD`（BytecodeGraphSerializer.h:184-193）：D 槽
  /// 有符号立即数，超出 `i16` 范围时置 error，仍返回截断值。
  pub fn get_imm_int_as_signed_d(&mut self, insn: &mut BcInst, index: u8) -> i16 {
    let imm: &mut BcImm = self.get_imm(insn, index);
    LUAU_ASSERT!(imm.kind == BcImmKind::Int);
    // cpp: if (int32_t(int16_t(imm.valueInt)) != imm.valueInt) error = true;
    let value: i32 = unsafe { imm.value.value_int };

    if i32::from(value as i16) != value {
      self.error = true;
    }

    value as i16
  }
}
