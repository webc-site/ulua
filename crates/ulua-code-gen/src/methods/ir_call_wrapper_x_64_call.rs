use crate::{
  enums::{category_x_64::CategoryX64, ir_op_kind::IrOpKind, size_x_64::SizeX64},
  functions::same_underlying_register::same_underlying_register,
  macros::codegen_assert::CODEGEN_ASSERT,
  methods::ir_call_wrapper_x_64_find_non_interfering_argument::ir_call_wrapper_x_64_find_non_interfering_argument,
  records::{
    call_argument::CallArgument, ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_data::K_INVALID_INST_IDX, ir_inst::IrInst, operand_x_64::OperandX64,
    register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64,
  },
};

impl IrCallWrapperX64 {
  pub fn call(&mut self, func: &OperandX64) {
    self.func_op = *func;

    // Free the result register before handling arguments so that no live value is preserved from it
    if self.result_reg != RegisterX64::NOREG {
      unsafe {
        (*self.regs).free_reg(self.result_reg);
      }
    }

    self.count_register_uses();

    for i in 0..(self.arg_count as usize) {
      let arg_ptr: *mut CallArgument = &mut self.args[i];

      let source_op = unsafe { (*arg_ptr).source_op };

      if source_op.kind() != IrOpKind::None {
        let inst: *mut IrInst = unsafe { (*(*self.regs).function).as_inst_op(source_op) };

        if !inst.is_null() {
          // Source registers are recorded separately from source operands in CallArgument.
          // If source is the last use of IrInst, clear the register from the operand.
          if unsafe { (*self.regs).is_last_use_reg(&*inst, self.inst_idx) } {
            unsafe {
              (*inst).reg_x64 = RegisterX64::NOREG;
            }
          } else {
            // If it's not the last use and register is volatile, register ownership
            // is taken, which also spills the operand.
            let reg = unsafe { (*inst).reg_x64 };
            if reg.size() == SizeX64::Xmmword || unsafe { (*self.regs).should_free_gpr(reg) } {
              unsafe {
                (*self.regs).take_reg(reg, K_INVALID_INST_IDX);
              }
            }
          }
        }
      }

      let source = unsafe { (*arg_ptr).source };
      let target = unsafe { (*arg_ptr).target };

      // Immediate values are stored at the end since they are not interfering and target
      // register can still be used temporarily.
      if source.cat == CategoryX64::Imm {
        unsafe {
          (*arg_ptr).candidate = false;
        }
      }
      // Arguments passed through stack can be handled immediately
      else if target.cat == CategoryX64::Mem {
        if source.cat == CategoryX64::Mem {
          let mut tmp = ScopedRegX64 {
            owner: self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp.scoped_reg_x_64_ir_reg_alloc_x_64_size_x_64(
            unsafe { &mut *self.regs },
            target.mem_size,
          );

          self.free_source_registers(unsafe { &mut *arg_ptr });

          if source.mem_size == SizeX64::None {
            unsafe {
              (*self.build).lea_operand_x_64_operand_x_64(OperandX64::reg(tmp.reg), source);
            }
          } else {
            unsafe {
              (*self.build).mov(OperandX64::reg(tmp.reg), source);
            }
          }

          unsafe {
            (*self.build).mov(target, OperandX64::reg(tmp.reg));
          }

          tmp.free();
        } else {
          self.free_source_registers(unsafe { &mut *arg_ptr });

          unsafe {
            (*self.build).mov(target, source);
          }
        }

        unsafe {
          (*arg_ptr).candidate = false;
        }
      }
      // Skip arguments that are already in their place
      else if source.cat == CategoryX64::Reg && same_underlying_register(target.base, source.base)
      {
        self.free_source_registers(unsafe { &mut *arg_ptr });

        // If target is not used as source in other arguments, prevent register allocator
        // from giving it out.
        if self.get_register_uses(target.base) == 0 {
          unsafe {
            (*self.regs).take_reg(target.base, K_INVALID_INST_IDX);
          }
        } else {
          // Otherwise, make sure we won't free it when last source use is completed
          self.add_register_use(target.base);
        }

        unsafe {
          (*arg_ptr).candidate = false;
        }
      }
    }

    // Repeat until we run out of arguments to pass
    loop {
      // Find target argument register that is not an active source
      let candidate = ir_call_wrapper_x_64_find_non_interfering_argument(self);

      if !candidate.is_null() {
        // This section is only for handling register targets
        CODEGEN_ASSERT!(unsafe { (*candidate).target.cat } == CategoryX64::Reg);

        self.free_source_registers(unsafe { &mut *candidate });

        let target_base = unsafe { (*candidate).target.base };
        CODEGEN_ASSERT!(self.get_register_uses(target_base) == 0);
        unsafe {
          (*self.regs).take_reg(target_base, K_INVALID_INST_IDX);
        }

        self.move_to_target(unsafe { &mut *candidate });

        unsafe {
          (*candidate).candidate = false;
        }
      } else {
        // If all registers cross-interfere (rcx <- rdx, rdx <- rcx), one has to be renamed
        let conflict = self.find_conflicting_target();
        if conflict != RegisterX64::NOREG {
          self.rename_conflicting_register(conflict);
        } else {
          for item in self.args.iter().take(self.arg_count as usize) {
            CODEGEN_ASSERT!(!item.candidate);
          }
          break;
        }
      }
    }

    // Handle immediate arguments last
    for i in 0..(self.arg_count as usize) {
      let arg_ptr: *mut CallArgument = &mut self.args[i];

      if unsafe { (*arg_ptr).source.cat } == CategoryX64::Imm {
        // There could be a conflict with the function source register, make this argument
        // a candidate to find it.
        unsafe {
          (*arg_ptr).candidate = true;
        }

        let conflict = self.find_conflicting_target();
        if conflict != RegisterX64::NOREG {
          self.rename_conflicting_register(conflict);
        }

        let target = unsafe { (*arg_ptr).target };
        if target.cat == CategoryX64::Reg {
          unsafe {
            (*self.regs).take_reg(target.base, K_INVALID_INST_IDX);
          }
        }

        self.move_to_target(unsafe { &mut *arg_ptr });

        unsafe {
          (*arg_ptr).candidate = false;
        }
      }
    }

    // Free registers used in the function call
    let func_base = self.func_op.base;
    let func_index = self.func_op.index;
    self.remove_register_use(func_base);
    self.remove_register_use(func_index);

    // Just before the call is made, argument registers are all marked as free in register allocator
    for i in 0..(self.arg_count as usize) {
      if self.args[i].target.cat == CategoryX64::Reg {
        let target_base = self.args[i].target.base;
        unsafe {
          (*self.regs).free_reg(target_base);
        }
      }
    }

    unsafe {
      (*self.regs).preserve_and_free_inst_values();
      (*self.regs).assert_all_free();
    }

    unsafe {
      (*self.build).call_operand_x_64(self.func_op);
    }

    if self.result_reg != RegisterX64::NOREG {
      // Result register was allocated before call was made, we freed it temporarily and taking it back
      unsafe {
        (*self.regs).take_reg(self.result_reg, self.result_inst_idx);
      }

      // Skip move to eax/rax/xmm0 result
      if self.result_reg.index() != 0 {
        let return_reg = RegisterX64 {
          bits: self.result_reg.size() as u8,
        };
        unsafe {
          (*self.build).mov(
            OperandX64::reg(self.result_reg),
            OperandX64::reg(return_reg),
          );
        }
      }
    }
  }
}
