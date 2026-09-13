use core::ptr::null;

use ulua_compiler::records::compile_options::CompileOptions;

use crate::records::global_options::GLOBAL_OPTIONS;

pub fn copts() -> CompileOptions {
  unsafe {
    let opts = &*core::ptr::addr_of!(GLOBAL_OPTIONS);
    CompileOptions {
      optimization_level: opts.optimization_level,
      debug_level: opts.debug_level,
      type_info_level: opts.type_info_level,
      coverage_level: 0,
      vector_lib: opts.vector_lib,
      vector_ctor: opts.vector_ctor,
      vector_type: opts.vector_type,
      mutable_globals: null(),
      userdata_types: null(),
      libraries_with_known_members: null(),
      library_member_type_cb: None,
      library_member_constant_cb: None,
      disabled_builtins: null(),
    }
  }
}
