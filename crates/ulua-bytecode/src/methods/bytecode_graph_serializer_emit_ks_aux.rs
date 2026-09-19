use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::records::{bc_inst::BcInst, bytecode_graph_serializer::BytecodeGraphSerializer};

impl<'a> BytecodeGraphSerializer<'a> {
  /// cpp 402-405 / 412-415 / 434-437 三处共用模式：UDATA 变体（GETUDATAKS/
  /// SETUDATAKS/NAMECALLUDATA）aux 低 16 位为 Aux16 常量、上 16 位为 imm 标志；
  /// 普通变体直接走 Aux 常量。`aux_index` 同时是 Aux16 与 Aux 槽位。
  pub fn emit_ks_aux(&mut self, insn: &mut BcInst, aux_index: u8, flags_index: u8) {
    let is_udata = matches!(
      insn.op,
      LuauOpcode::LOP_GETUDATAKS | LuauOpcode::LOP_SETUDATAKS | LuauOpcode::LOP_NAMECALLUDATA
    );
    if is_udata {
      let aux16 = self.get_vm_const_input_aux16(insn, aux_index);
      let flags = self.get_imm_int(insn, flags_index);
      self.bcb.emit_aux(aux16 | (flags as u32) << 16);
    } else {
      let vm_const_input_aux = self.get_vm_const_input_aux(insn, aux_index);
      self.bcb.emit_aux(vm_const_input_aux);
    }
  }
}
