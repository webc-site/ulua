use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_insn_op::LUAU_INSN_OP};
use ulua_vm::records::proto::Proto;

use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    build_bytecode_blocks::build_bytecode_blocks, get_jump_target::get_jump_target,
    get_op_length::get_op_length, is_fast_call::is_fast_call,
  },
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

impl IrBuilder {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn rebuild_bytecode_basic_blocks(&mut self, proto: *mut Proto) {
    unsafe {
      let sizecode = (*proto).sizecode as usize;
      let code = (*proto).code;

      self.inst_index_to_block.resize(sizecode, u32::MAX);

      let mut jump_targets = vec![0; sizecode];

      let mut i: usize = 0;
      while i < sizecode {
        let pc: *const Instruction = code.add(i);
        let op = LuauOpcode::from(LUAU_INSN_OP(*pc) as u8);

        let target = get_jump_target(*pc, i as u32);

        if target >= 0 && !is_fast_call(op) {
          jump_targets[target as usize] = 1;
        }

        i += get_op_length(op) as usize;
        debug_assert!(i <= sizecode);
      }

      jump_targets[0] = 1;

      for j in 0..sizecode {
        if jump_targets[j] != 0 {
          let b = self.block(IrBlockKind::Bytecode);
          self.inst_index_to_block[j] = b.index();
        }
      }

      build_bytecode_blocks(&mut self.function, &jump_targets);
    }
  }
}
