use core::{ffi::c_char, ptr::null};

#[derive(Debug, Clone, Copy)]
pub struct GlobalOptions {
  pub optimization_level: i32,
  pub debug_level: i32,
  pub type_info_level: i32,

  pub vector_lib: *const c_char,
  pub vector_ctor: *const c_char,
  pub vector_type: *const c_char,

  pub only_parse: bool,
  pub parse_cst: bool,
}

impl Default for GlobalOptions {
  fn default() -> Self {
    Self {
      optimization_level: 1,
      debug_level: 1,
      type_info_level: 0,
      vector_lib: null(),
      vector_ctor: null(),
      vector_type: null(),
      only_parse: false,
      parse_cst: false,
    }
  }
}

pub static mut GLOBAL_OPTIONS: GlobalOptions = GlobalOptions {
  optimization_level: 1,
  debug_level: 1,
  type_info_level: 0,
  vector_lib: null(),
  vector_ctor: null(),
  vector_type: null(),
  only_parse: false,
  parse_cst: false,
};
