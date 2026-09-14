use core::{
  ffi::{c_char, c_void},
  mem::transmute,
};

use crate::{
  functions::visit_fde_entries::visit_fde_entries, macros::codegen_assert::CODEGEN_ASSERT,
  records::unwind_builder_dwarf_2::UnwindBuilderDwarf2,
};

#[cfg(not(target_os = "windows"))]
unsafe extern "C" {
  fn __register_frame(begin: *const c_void);
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
unsafe extern "C" {
  fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

#[cfg(target_os = "windows")]
unsafe extern "system" {
  fn RtlAddFunctionTable(
    function_table: *mut core::ffi::c_void,
    entry_count: u32,
    base_address: usize,
  ) -> i32;
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn create_block_unwind_info(
  context: *mut c_void,
  block: *mut u8,
  block_size: usize,
  begin_offset: &mut usize,
) -> *mut c_void {
  unsafe {
    const K_CODE_ALIGNMENT: usize = 32;

    #[cfg(target_os = "windows")]
    let (function_count, builder_begin_offset, unwind_size) = {
      let unwind = &mut *(context.cast::<crate::records::unwind_builder_win::UnwindBuilderWin>());
      let unwind_size = (unwind.get_unwind_info_size(block_size) + (K_CODE_ALIGNMENT - 1))
        & !(K_CODE_ALIGNMENT - 1);

      CODEGEN_ASSERT!(block_size >= unwind_size);

      let function_count = unwind.finalize(block.cast(), unwind_size, block.cast(), block_size);

      (function_count, unwind.get_begin_offset(), unwind_size)
    };

    #[cfg(not(target_os = "windows"))]
    let (builder_begin_offset, unwind_size) = {
      let unwind = &mut *(context.cast::<UnwindBuilderDwarf2>());
      let unwind_size = (unwind.get_unwind_info_size(block_size) + (K_CODE_ALIGNMENT - 1))
        & !(K_CODE_ALIGNMENT - 1);

      CODEGEN_ASSERT!(block_size >= unwind_size);

      unwind.finalize(block.cast(), unwind_size, block.cast(), block_size);

      (unwind.get_begin_offset(), unwind_size)
    };

    #[cfg(target_os = "windows")]
    {
      if RtlAddFunctionTable(block.cast(), function_count as u32, block as usize) == 0 {
        CODEGEN_ASSERT!(false);
        return core::ptr::null_mut();
      }
    }

    #[cfg(not(target_os = "windows"))]
    {
      visit_fde_entries(block.cast(), __register_frame);
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    register_macos_a64_unwind_sections();

    *begin_offset = unwind_size + builder_begin_offset;
    block.cast()
  }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn register_macos_a64_unwind_sections() {
  use std::sync::OnceLock;

  use crate::{
    functions::find_dynamic_unwind_sections::find_dynamic_unwind_sections,
    records::unw_dynamic_unwind_sections_t::unw_dynamic_unwind_sections_t,
    type_aliases::unw_add_find_dynamic_unwind_sections_t::UnwAddFindDynamicUnwindSectionsT,
  };

  unsafe extern "C-unwind" fn find_dynamic_unwind_sections_thunk(
    addr: usize,
    info: *mut unw_dynamic_unwind_sections_t,
  ) -> i32 {
    if info.is_null() {
      return 0;
    }

    find_dynamic_unwind_sections(addr, unsafe { &mut *info })
  }

  static REGISTER_RESULT: OnceLock<i32> = OnceLock::new();

  let result = REGISTER_RESULT.get_or_init(|| unsafe {
    const RTLD_DEFAULT: *mut c_void = -2_isize as *mut c_void;
    let symbol = c"__unw_add_find_dynamic_unwind_sections";
    let add_find_dynamic_unwind_sections: UnwAddFindDynamicUnwindSectionsT =
      transmute(dlsym(RTLD_DEFAULT, symbol.as_ptr()));

    match add_find_dynamic_unwind_sections {
      Some(register) => register(Some(find_dynamic_unwind_sections_thunk)),
      None => 0,
    }
  });

  CODEGEN_ASSERT!(*result == 0);
}
