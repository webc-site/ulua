use alloc::{string::String, vec::Vec};

use crate::records::patch::Patch;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AssemblyBuilderA64 {
  // `data`/`code`/`text` are the public observable output in the C++
  // AssemblyBuilderA64 (tests read `build.code`); the X64 builder already
  // exposes them `pub`.
  pub data: Vec<u8>,
  pub code: Vec<u32>,
  pub text: String,
  pub(crate) log_text: bool,
  pub(crate) features: u32,
  pub(crate) next_label: u32,
  pub(crate) pending_labels: Vec<Patch>,
  pub(crate) label_locations: Vec<u32>,
  pub(crate) finalized: bool,
  pub(crate) overflowed: bool,
  pub(crate) data_pos: usize,
  pub(crate) code_pos: *mut u32,
  pub(crate) code_end: *mut u32,
}
