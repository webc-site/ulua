use crate::records::global_state::global_State;
#[derive(Debug)]
#[repr(C)]
pub struct ScopedSetGcThreshold {
  pub(crate) global: *mut global_State,
  pub(crate) original_threshold: usize,
}
