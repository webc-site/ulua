#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UnwindFunctionDwarf2 {
  pub begin_offset: u32,
  pub end_offset: u32,
  pub fde_entry_start_pos: u32,
}
