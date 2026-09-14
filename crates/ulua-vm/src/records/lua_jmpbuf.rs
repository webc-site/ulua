use core::ffi::c_int;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct lua_jmpbuf {
  pub prev: *mut lua_jmpbuf,
  pub status: c_int,
  pub buf: [c_int; 64],
}

// Note: jmp_buf is a platform-specific array type used for non-local jumps.
// In a wasm32-unknown-unknown or portable context where libc is unavailable,
// we provide a sufficiently sized buffer to satisfy the struct layout for the VM's
// internal longjmp-based error recovery pointers.
