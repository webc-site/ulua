use crate::records::standalone_code_gen_context::StandaloneCodeGenContext;

impl StandaloneCodeGenContext {
  pub fn on_close_state(&mut self) {
    // The StandaloneCodeGenContext is owned by the one VM that owns it, so when
    // that VM is destroyed, we destroy *this as well:
    unsafe {
      let _ = Box::from_raw(self);
    }
  }
}

pub fn standalone_code_gen_context_on_close_state(this: &mut StandaloneCodeGenContext) {
  this.on_close_state();
}
