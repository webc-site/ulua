use crate::records::{
  bc_inst::BcInst, bytecode_builder::K_MAX_CONSTANT_COUNT,
  bytecode_graph_serializer::BytecodeGraphSerializer,
};

impl<'a> BytecodeGraphSerializer<'a> {
  /// cpp `getVmConstInputAux`（BytecodeGraphSerializer.h:248-256）：aux 槽
  /// 常量索引，超出 `kMaxConstantCount` 时置 error。
  pub fn get_vm_const_input_aux(&mut self, insn: &mut BcInst, index: u8) -> u32 {
    let cid = self.get_vm_const_input_raw(insn, index);

    if cid >= K_MAX_CONSTANT_COUNT {
      self.error = true;
    }

    cid
  }
}
