use core::{ffi::c_longlong, mem::size_of};
pub const SZINT: i32 = size_of::<c_longlong>() as i32;
