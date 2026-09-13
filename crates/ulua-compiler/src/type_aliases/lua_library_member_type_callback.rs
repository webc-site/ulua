use core::ffi::{c_char, c_int};

pub type LuaLibraryMemberTypeCallback =
  Option<unsafe extern "C-unwind" fn(library: *const c_char, member: *const c_char) -> c_int>;
