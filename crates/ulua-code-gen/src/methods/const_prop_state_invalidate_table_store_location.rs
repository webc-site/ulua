use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  macros::{op_a::op_a, op_c::op_c},
  records::{
    const_prop_state::ConstPropState, ir_data::K_INVALID_INST_IDX, ir_inst::IrInst, ir_op::IrOp,
  },
};

impl ConstPropState {
  pub fn invalidate_table_store_location(
    &mut self,
    mut target_addr: IrInst,
    write_offset_op: IrOp,
    tag: u8,
  ) {
    match target_addr.cmd {
      IrCmd::GetSlotNodeAddr => {
        let target_key = op_c(target_addr.clone());

        let keys: Vec<u32> = self
          .hash_value_cache
          .iter()
          .map(|(&pointer_idx, _)| pointer_idx)
          .collect();
        for pointer_idx in keys {
          let address = unsafe { (&(*self.function).instructions)[pointer_idx as usize].clone() };
          if op_c(address) == target_key {
            *self.hash_value_cache.get_or_insert(pointer_idx) = K_INVALID_INST_IDX;
          }
        }

        const K_UNKNOWN_TAG: u8 = 0xff;
        if tag == K_UNKNOWN_TAG || tag == LuaType::Nil as u8 {
          for el in &mut self.check_slot_match_cache {
            let check = unsafe { (&(*self.function).instructions)[el.pointer as usize].clone() };
            let slot_addr_op = op_a(&mut check.clone());
            if slot_addr_op.kind() == IrOpKind::Inst {
              let slot_addr =
                unsafe { (&(*self.function).instructions)[slot_addr_op.index() as usize].clone() };
              if op_c(slot_addr) == target_key {
                el.known_to_not_be_nil = false;
              }
            }
          }
        }
      }
      IrCmd::GetArrAddr => {
        let offset_op = self.get_combined_array_load_offset_op(&mut target_addr, write_offset_op);
        let opt_offset = unsafe { (&mut *self.function).as_int_op(offset_op) };

        if let Some(offset) = opt_offset {
          let mut i = 0;
          while i < self.array_value_cache.len() {
            let entry = &self.array_value_cache[i];
            let remove = entry.offset.kind() != IrOpKind::Constant
              || unsafe { (&*self.function).int_op(entry.offset) } == offset;

            if remove {
              self.array_value_cache.swap_remove(i);
            } else {
              i += 1;
            }
          }
        } else {
          self.array_value_cache.clear();
        }
      }
      IrCmd::TableSetnum => {
        debug_assert!(self.array_value_cache.is_empty());
      }
      _ => {
        debug_assert!(target_addr.cmd == IrCmd::GetClosureUpvalAddr);
      }
    }
  }
}
