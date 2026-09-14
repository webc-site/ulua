use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  macros::{op_a::op_a, op_b::op_b, op_c::op_c, op_d::op_d},
  records::{
    buffer_load_store_info::BufferLoadStoreInfo, const_prop_state::ConstPropState, ir_inst::IrInst,
  },
};

impl ConstPropState {
  pub fn forward_buffer_store_to_load(
    &mut self,
    store_inst: &mut IrInst,
    load_cmd: IrCmd,
    access_size: u8,
  ) {
    let tag = unsafe { &mut *self.function }.tag_op(op_d(store_inst.clone()));

    // Writing at unknown offset removes everything in the same kind of memory (buffer/userdata)
    // For userdata, we could check where the pointer is coming from, but we don't have an example of such usage
    if op_b(store_inst.clone()).kind() != IrOpKind::Constant {
      let mut i = 0;
      while i < self.buffer_load_store_info.len() {
        let info = self.buffer_load_store_info[i];
        if info.tag == tag {
          let last = *self.buffer_load_store_info.last().unwrap();
          self.buffer_load_store_info[i] = last;
          self.buffer_load_store_info.pop();
        } else {
          i += 1;
        }
      }

      return;
    }

    let offset = unsafe { &mut *self.function }.int_op(op_b(store_inst.clone()));

    // Write at a constant offset invalidates that range in every object unless we know the pointers are unrelated
    let mut i = 0;
    while i < self.buffer_load_store_info.len() {
      let intersecting_range = offset + i32::from(access_size)
        > self.buffer_load_store_info[i].offset
        && offset
          < self.buffer_load_store_info[i].offset
            + i32::from(self.buffer_load_store_info[i].access_size);

      if intersecting_range && self.buffer_load_store_info[i].tag == tag {
        let curr_ptr = unsafe { &mut *self.function }.inst_op(op_a(store_inst));
        let info_ptr =
          unsafe { &mut *self.function }.inst_op(self.buffer_load_store_info[i].address);

        // Pointers from separate allocations cannot be the same
        if curr_ptr.cmd == IrCmd::NewUserdata
          && info_ptr.cmd == IrCmd::NewUserdata
          && op_a(store_inst) != self.buffer_load_store_info[i].address
        {
          i += 1;
          continue;
        }

        let last = *self.buffer_load_store_info.last().unwrap();
        self.buffer_load_store_info[i] = last;
        self.buffer_load_store_info.pop();
      } else {
        i += 1;
      }
    }

    let mut value = op_c(store_inst.clone());

    // Store of smaller type will truncate data
    // Dynamic values are handled in 'substituteOrRecordBufferLoad'
    if op_c(store_inst.clone()).kind() == IrOpKind::Constant {
      if load_cmd == IrCmd::BufferReadi8 {
        let v_i32 = unsafe { &mut *self.function }.int_op(op_c(store_inst.clone()));
        value = unsafe { &mut *self.build }.const_int(v_i32 as i8 as i32);
      } else if load_cmd == IrCmd::BufferReadi16 {
        let v_i32 = unsafe { &mut *self.function }.int_op(op_c(store_inst.clone()));
        value = unsafe { &mut *self.build }.const_int(v_i32 as i16 as i32);
      } else if load_cmd == IrCmd::BufferReadf32 {
        let v_f64 = unsafe { &mut *self.function }.double_op(op_c(store_inst.clone()));
        value = unsafe { &mut *self.build }.const_double(v_f64 as f32 as f64);
      }
    }

    // Record this store value for future reuse
    let info = BufferLoadStoreInfo {
      load_cmd,
      access_size,
      tag,
      from_store: true,
      address: op_a(store_inst),
      value,
      offset,
    };

    self.buffer_load_store_info.push(info);
  }
}
