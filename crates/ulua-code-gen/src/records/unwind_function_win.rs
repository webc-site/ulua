#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UnwindFunctionWin {
  pub begin_offset: u32,
  pub end_offset: u32,
  pub unwind_info_offset: u32,
}
