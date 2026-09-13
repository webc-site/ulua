use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    produces_dirty_high_register_bits::produces_dirty_high_register_bits,
    replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst, substitute::substitute,
  },
  macros::{op_a::op_a, op_b::op_b, op_c::op_c},
  records::{
    buffer_load_store_info::BufferLoadStoreInfo, const_prop_state::ConstPropState,
    ir_block::IrBlock, ir_inst::IrInst, ir_op::IrOp,
  },
};

impl ConstPropState {
  pub fn substitute_or_record_buffer_load(
    &mut self,
    block: &mut IrBlock,
    inst_idx: u32,
    load_inst: &mut IrInst,
    access_size: u8,
  ) {
    if op_b(load_inst.clone()).kind() != IrOpKind::Constant {
      return;
    }

    let offset = unsafe { (&*self.function).int_op(op_b(load_inst.clone())) };
    let tag = unsafe { (&*self.function).tag_op(op_c(load_inst.clone())) };
    let address = op_a(load_inst);

    for info in self.buffer_load_store_info.clone() {
      if info.address == address && info.offset == offset && info.tag == tag {
        if info.from_store {
          match load_inst.cmd {
            IrCmd::BufferReadi8 => {
              if info.load_cmd == IrCmd::BufferReadi8 {
                if info.value.kind() == IrOpKind::Inst {
                  replace_ir_function_ir_block_u32_ir_inst(
                    unsafe { &mut *self.function },
                    block,
                    inst_idx,
                    IrInst::ir_inst_new(IrCmd::Sexti8Int, &[info.value]),
                  );
                  self.substitute_or_record(load_inst, inst_idx);
                } else {
                  substitute(unsafe { &mut *self.function }, load_inst, info.value);
                }
                return;
              }
            }
            IrCmd::BufferReadu8 => {
              if info.load_cmd == IrCmd::BufferReadi8 {
                if info.value.kind() == IrOpKind::Inst {
                  let mask = unsafe { (&mut *self.build).const_int(0xff) };
                  replace_ir_function_ir_block_u32_ir_inst(
                    unsafe { &mut *self.function },
                    block,
                    inst_idx,
                    IrInst::ir_inst_new(IrCmd::BitandUint, &[info.value, mask]),
                  );
                  self.substitute_or_record(load_inst, inst_idx);
                } else {
                  let value = unsafe {
                    (&mut *self.build).const_int((&*self.function).int_op(info.value) as u8 as i32)
                  };
                  substitute(unsafe { &mut *self.function }, load_inst, value);
                }
                return;
              }
            }
            IrCmd::BufferReadi16 => {
              if info.load_cmd == IrCmd::BufferReadi16 {
                if info.value.kind() == IrOpKind::Inst {
                  replace_ir_function_ir_block_u32_ir_inst(
                    unsafe { &mut *self.function },
                    block,
                    inst_idx,
                    IrInst::ir_inst_new(IrCmd::Sexti16Int, &[info.value]),
                  );
                  self.substitute_or_record(load_inst, inst_idx);
                } else {
                  substitute(unsafe { &mut *self.function }, load_inst, info.value);
                }
                return;
              }
            }
            IrCmd::BufferReadu16 => {
              if info.load_cmd == IrCmd::BufferReadi16 {
                if info.value.kind() == IrOpKind::Inst {
                  let mask = unsafe { (&mut *self.build).const_int(0xffff) };
                  replace_ir_function_ir_block_u32_ir_inst(
                    unsafe { &mut *self.function },
                    block,
                    inst_idx,
                    IrInst::ir_inst_new(IrCmd::BitandUint, &[info.value, mask]),
                  );
                  self.substitute_or_record(load_inst, inst_idx);
                } else {
                  let value = unsafe {
                    (&mut *self.build).const_int((&*self.function).int_op(info.value) as u16 as i32)
                  };
                  substitute(unsafe { &mut *self.function }, load_inst, value);
                }
                return;
              }
            }
            IrCmd::BufferReadi32 => {
              if info.load_cmd == IrCmd::BufferReadi32 {
                let src = unsafe { (&mut *self.function).as_inst_op(info.value) };
                if !src.is_null() && produces_dirty_high_register_bits(unsafe { (*src).cmd }) {
                  replace_ir_function_ir_block_u32_ir_inst(
                    unsafe { &mut *self.function },
                    block,
                    inst_idx,
                    IrInst::ir_inst_new(IrCmd::TruncateUint, &[info.value]),
                  );
                  self.substitute_or_record(load_inst, inst_idx);
                } else {
                  substitute(unsafe { &mut *self.function }, load_inst, info.value);
                }
                return;
              }
            }
            IrCmd::BufferReadf32 => {
              if info.load_cmd == IrCmd::BufferReadf32 {
                substitute(unsafe { &mut *self.function }, load_inst, info.value);
                return;
              }
            }
            IrCmd::BufferReadf64 => {
              if info.load_cmd == IrCmd::BufferReadf64 {
                substitute(unsafe { &mut *self.function }, load_inst, info.value);
                return;
              }
            }
            IrCmd::BufferReadi64 => {
              if info.load_cmd == IrCmd::BufferReadi64 {
                substitute(unsafe { &mut *self.function }, load_inst, info.value);
                return;
              }
            }
            _ => {
              crate::macros::codegen_assert::CODEGEN_ASSERT!(false);
            }
          }
        } else if info.load_cmd == load_inst.cmd {
          substitute(unsafe { &mut *self.function }, load_inst, info.value);
          return;
        }
      }
    }

    self.buffer_load_store_info.push(BufferLoadStoreInfo {
      load_cmd: load_inst.cmd,
      access_size,
      tag,
      from_store: false,
      address,
      value: IrOp::ir_op_kind_u32(IrOpKind::Inst, unsafe {
        (&*self.function).get_inst_index(load_inst)
      }),
      offset,
    });
  }
}
