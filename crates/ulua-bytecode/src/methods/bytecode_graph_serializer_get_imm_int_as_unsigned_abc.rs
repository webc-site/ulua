use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_imm_kind::BcImmKind,
  records::{bc_imm::BcImm, bc_inst::BcInst, bytecode_graph_serializer::BytecodeGraphSerializer},
};

impl<'a> BytecodeGraphSerializer<'a> {
  /// cpp `getImmIntAsUnsignedABC`（BytecodeGraphSerializer.h:171-182）：ABC 槽
  /// 无符号立即数（bias 用于 `imm - 1` 偏移），超出 `u8` 范围时置 error，
  /// 仍返回截断值。
  pub fn get_imm_int_as_unsigned_abc(&mut self, insn: &mut BcInst, index: u8, bias: i32) -> u8 {
    let imm: &mut BcImm = self.get_imm(insn, index);
    LUAU_ASSERT!(imm.kind == BcImmKind::Int);
    // cpp: int64_t result = int64_t(imm.valueInt) + bias;
    let result: i64 = i64::from(unsafe { imm.value.value_int }) + i64::from(bias);

    if u8::try_from(result).map(i64::from) != Ok(result) {
      self.error = true;
    }

    result as u8
  }
}
