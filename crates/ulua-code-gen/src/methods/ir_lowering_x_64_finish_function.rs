use core::mem::size_of;

use crate::{
  enums::code_gen_counter::CodeGenCounter,
  functions::dword_reg::dword_reg,
  records::{
    emit_common_x_64::{K_EXTRA_SPILL_SLOTS, K_SPILL_SLOTS},
    ir_lowering_x_64::IrLoweringX64,
    label::Label,
    operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
  type_aliases::instruction_ir_translation::Instruction,
};

const K_VM_EXIT_ENTRY_GUARD_PC: u32 = (1u32 << 28) - 1;

impl IrLoweringX64 {
  pub fn ir_lowering_x_64_finish_function(&mut self) {
    unsafe {
      if (*self.build).log_text {
        (*self.build).log_append(format_args!("; interrupt handlers\n"));
      }

      for i in 0..self.interrupt_handlers.len() {
        let mut handler = self.interrupt_handlers[i];
        (*self.build).set_label_label(&mut handler.self_);
        (*self.build).mov(
          OperandX64::reg(dword_reg(RegisterX64::RAX)),
          OperandX64::imm((handler.pcpos + 1) as i32),
        );
        (*self.build).lea_register_x_64_label(RegisterX64::RBX, &mut handler.next);
        (*self.build).jmp_label(&mut (*self.helpers).interrupt);
      }

      if (*self.build).log_text {
        (*self.build).log_append(format_args!("; exit handlers\n"));
      }

      for i in 0..self.exit_handlers.len() {
        let mut handler = self.exit_handlers[i];
        if handler.pcpos == K_VM_EXIT_ENTRY_GUARD_PC {
          (*self.build).set_label_label(&mut handler.self_);

          self.ir_lowering_x_64_alloc_and_increment_counter_at(CodeGenCounter::VmExitTaken, !0u32);

          (*self.build).jmp_label(&mut (*self.helpers).exit_continue_vm_clear_native_flag);
        } else {
          (*self.build).set_label_label(&mut handler.self_);

          self.ir_lowering_x_64_alloc_and_increment_counter_at(
            CodeGenCounter::VmExitTaken,
            handler.pcpos,
          );

          (*self.build).mov(
            OperandX64::reg(dword_reg(RegisterX64::RDX)),
            OperandX64::imm((handler.pcpos * size_of::<Instruction>() as u32) as i32),
          );
          (*self.build).jmp_label(&mut (*self.helpers).update_pc_and_continue_in_vm);
        }
      }

      let mut end = Label::default();
      (*self.build).set_label(&mut end);
      (*self.function).end_location = (*self.build).get_label_offset(&end);
      (*self.build).ud_2();

      if !self.stats.is_null() {
        if self.regs.max_used_slot > K_SPILL_SLOTS + K_EXTRA_SPILL_SLOTS {
          (*self.stats).reg_alloc_errors += 1;
        }

        if self.regs.max_used_slot > (*self.stats).max_spill_slots_used {
          (*self.stats).max_spill_slots_used = self.regs.max_used_slot;
        }
      }
    }
  }
}
