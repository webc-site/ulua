use core::{ffi::c_void, ptr::copy_nonoverlapping};

use crate::{
  functions::writeu_64::writeu_64, records::unwind_builder_dwarf_2::UnwindBuilderDwarf2,
};

impl UnwindBuilderDwarf2 {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn finalize(
    &self,
    target: *mut u8,
    offset: usize,
    func_address: *mut c_void,
    block_size: usize,
  ) -> usize {
    unsafe {
      copy_nonoverlapping(
        self.raw_data.as_ptr(),
        target,
        self.get_unwind_info_size(block_size),
      );
    }

    let k_full_block_function: u32 = u32::MAX;
    let target_u8 = target;

    // Constants for DWARF FDE offsets
    const K_FDE_INITIAL_LOCATION_OFFSET: usize = 8;
    const K_FDE_ADDRESS_RANGE_OFFSET: usize = 16;

    for func in &self.unwind_functions {
      unsafe {
        let fde_entry = target_u8.add(func.fde_entry_start_pos as usize);

        writeu_64(
          fde_entry.add(K_FDE_INITIAL_LOCATION_OFFSET),
          (func_address as usize as u64) + (offset as u64) + (func.begin_offset as u64),
        );

        let address_range = if func.end_offset == k_full_block_function {
          (block_size as u64) - (offset as u64)
        } else {
          (func.end_offset as u64) - (func.begin_offset as u64)
        };

        write_u_64(fde_entry.add(K_FDE_ADDRESS_RANGE_OFFSET), address_range);
      }
    }

    self.unwind_functions.len()
  }
}

#[inline(always)]
unsafe fn write_u_64(target: *mut u8, value: u64) -> *mut u8 {
  unsafe { writeu_64(target, value) }
}
