// ptr/返回值为分配器内存块，用 *mut u8 表达字节语义；ud 为用户数据句柄，保留 c_void
use core::ffi::c_void;
pub type LuaAlloc = Option<
  unsafe extern "C-unwind" fn(ud: *mut c_void, ptr: *mut u8, osize: usize, nsize: usize) -> *mut u8,
>;
