use crate::{enums::code_gen_counter::CodeGenCounter, records::ir_lowering_x_64::IrLoweringX64};

impl IrLoweringX64 {
  pub fn ir_lowering_x_64_alloc_and_increment_counter_at(
    &mut self,
    kind: CodeGenCounter,
    pcpos: u32,
  ) {
    unsafe {
      if !(*self.function).record_counters {
        return;
      }

      if (*self.build).log_text {
        (*self.build).log_append(format_args!(
          "; counter kind {} at pcpos {}\n",
          kind as u32, pcpos
        ));
      }

      (*self.function).extra_native_data.push(kind as u32);
      (*self.function).extra_native_data.push(pcpos);
      self.increment_counter_at((*self.function).extra_native_data.len());
      (*self.function).extra_native_data.push(0);
      (*self.function).extra_native_data.push(0);
    }
  }
}
