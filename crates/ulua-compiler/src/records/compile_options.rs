use core::{ffi::c_char, ptr::null};

use crate::type_aliases::{
  library_member_constant_callback::LibraryMemberConstantCallback,
  library_member_type_callback::LibraryMemberTypeCallback,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CompileOptions {
  pub optimization_level: i32,
  pub debug_level: i32,
  pub type_info_level: i32,
  pub coverage_level: i32,
  pub vector_lib: *const c_char,
  pub vector_ctor: *const c_char,
  pub vector_type: *const c_char,
  pub mutable_globals: *const *const c_char,
  pub userdata_types: *const *const c_char,
  pub libraries_with_known_members: *const *const c_char,
  pub library_member_type_cb: LibraryMemberTypeCallback,
  pub library_member_constant_cb: LibraryMemberConstantCallback,
  pub disabled_builtins: *const *const c_char,
}

impl Default for CompileOptions {
  fn default() -> Self {
    Self {
      optimization_level: 1,
      debug_level: 1,
      type_info_level: 0,
      coverage_level: 0,
      vector_lib: null(),
      vector_ctor: null(),
      vector_type: null(),
      mutable_globals: null(),
      userdata_types: null(),
      libraries_with_known_members: null(),
      library_member_type_cb: None,
      library_member_constant_cb: None,
      disabled_builtins: null(),
    }
  }
}
