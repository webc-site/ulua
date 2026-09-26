use core::ffi::c_char;
pub const L_ESC: c_char = b'%' as c_char;
pub const SPECIALS: &[u8] = b"^$*+?.([%-";
