use crate::records::unw_dynamic_unwind_sections_t::unw_dynamic_unwind_sections_t;

pub type UnwAddFindDynamicUnwindSectionsT = Option<
  unsafe extern "C-unwind" fn(
    find_callback: Option<
      unsafe extern "C-unwind" fn(addr: usize, info: *mut unw_dynamic_unwind_sections_t) -> i32,
    >,
  ) -> i32,
>;
