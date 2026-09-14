use core::{
  ffi::{c_char, c_void},
  mem::size_of,
  ptr::copy_nonoverlapping,
};
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn visit_fde_entries(pos: *mut c_char, cb: unsafe extern "C" fn(*const c_void)) {
  unsafe {
    // C++ Luau uses a *weak* `__unw_add_dynamic_fde` symbol to detect Apple's
    // libunwind: when it is present (Apple) each FDE entry is registered
    // individually; when it is absent (Linux/other, which register the whole
    // block once via `__register_frame`) the block is passed to `cb` directly.
    // A weak extern static is a *strong* undefined reference in Rust and fails to
    // link on Linux ("undefined symbol: __unw_add_dynamic_fde"), so detect the
    // platform at compile time — which is exactly what the weak-symbol presence
    // check amounted to.
    #[cfg(not(target_vendor = "apple"))]
    {
      cb(pos as *const core::ffi::c_void);
    }
    #[cfg(target_vendor = "apple")]
    {
      let mut current_pos = pos;
      loop {
        let mut part_length: u32 = 0;
        copy_nonoverlapping(
          current_pos as *const u8,
          &mut part_length as *mut u32 as *mut u8,
          size_of::<u32>(),
        );

        if part_length == 0 {
          break;
        }

        let mut part_id: u32 = 0;
        copy_nonoverlapping(
          current_pos.add(4) as *const u8,
          &mut part_id as *mut u32 as *mut u8,
          size_of::<u32>(),
        );

        if part_id != 0 {
          cb(current_pos as *const c_void);
        }

        current_pos = current_pos.add(part_length as usize + 4);
      }
    }
  }
}
