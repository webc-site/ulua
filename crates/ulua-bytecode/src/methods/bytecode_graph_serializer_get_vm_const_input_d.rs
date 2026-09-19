use crate::records::{bc_inst::BcInst, bytecode_graph_serializer::BytecodeGraphSerializer};

impl<'a> BytecodeGraphSerializer<'a> {
  /// cpp `getVmConstInputD`（BytecodeGraphSerializer.h:228-236）：D 槽 16 位
  /// 有符号常量索引，超出 `0x7fff` 时置 error。
  pub fn get_vm_const_input_d(&mut self, insn: &mut BcInst, index: u8) -> u16 {
    let cid: u32 = self.get_vm_const_input_raw(insn, index);

    if cid > 0x7fff {
      self.error = true;
    }

    cid as u16
  }
}
