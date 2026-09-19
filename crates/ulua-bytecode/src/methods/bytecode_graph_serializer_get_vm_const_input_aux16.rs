use crate::records::{bc_inst::BcInst, bytecode_graph_serializer::BytecodeGraphSerializer};

impl<'a> BytecodeGraphSerializer<'a> {
  /// cpp `getVmConstInputAux16`（BytecodeGraphSerializer.h:238-245）：
  /// 16 位常量索引，超出时置 error。
  pub fn get_vm_const_input_aux16(&mut self, insn: &mut BcInst, index: u8) -> u32 {
    let cid = self.get_vm_const_input_raw(insn, index);

    if cid > 0xffff {
      self.error = true;
    }

    cid
  }
}
