use crate::{
  enums::ir_cmd::IrCmd,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    array_value_entry::ArrayValueEntry, const_prop_state::ConstPropState, ir_inst::IrInst,
    ir_op::IrOp,
  },
};

impl ConstPropState {
  pub fn forward_table_store_to_load(
    &mut self,
    target_addr: &mut IrInst,
    write_offset_op: IrOp,
    inst_idx: u32,
  ) {
    if target_addr.cmd == IrCmd::GetSlotNodeAddr {
      CODEGEN_ASSERT!(unsafe { &*self.function }.int_op(write_offset_op) == 0);

      let function = unsafe { &*self.function };
      let key = function.get_inst_index(target_addr);
      *self.hash_value_cache.get_or_insert(key) = inst_idx;
    } else if target_addr.cmd == IrCmd::GetArrAddr {
      // 与 C++ 一致：复用 getCombinedArrayLoadOffsetOp，避免内联副本漏掉回绕语义
      let offset_op = self.get_combined_array_load_offset_op(target_addr, write_offset_op);

      let function = unsafe { &*self.function };
      let key = function.get_inst_index(target_addr);
      self.array_value_cache.push(ArrayValueEntry {
        pointer: key,
        offset: offset_op,
        value: inst_idx,
      });
    } else {
      CODEGEN_ASSERT!(
        target_addr.cmd == IrCmd::TableSetnum || target_addr.cmd == IrCmd::GetClosureUpvalAddr
      );
    }
  }
}
