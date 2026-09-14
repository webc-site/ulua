use crate::{
  functions::has_side_effects::has_side_effects,
  records::{const_prop_state::ConstPropState, ir_inst::IrInst},
};

impl ConstPropState {
  pub fn get_previous_inst_index(&mut self, inst: &IrInst) -> Option<*mut u32> {
    let prev_idx = {
      let prev_idx = self.value_map.find_mut(inst)?;
      prev_idx as *mut u32
    };

    let prev_inst = unsafe {
      let function = &*self.function;
      &function.instructions[*prev_idx as usize]
    };
    if prev_inst.use_count != 0 || has_side_effects(prev_inst.cmd) {
      Some(prev_idx)
    } else {
      None
    }
  }
}
