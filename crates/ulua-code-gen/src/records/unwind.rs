#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct UnwindCodeWin {
  pub offset: u8,
  pub opcode_opinfo: u8,
}

impl UnwindCodeWin {
  #[inline]
  pub fn set_opcode(&mut self, value: u8) {
    self.opcode_opinfo = (self.opcode_opinfo & 0xF0) | (value & 0x0F);
  }

  #[inline]
  pub fn set_opinfo(&mut self, value: u8) {
    self.opcode_opinfo = (self.opcode_opinfo & 0x0F) | ((value & 0x0F) << 4);
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UnwindFunctionDwarf2 {
  pub begin_offset: u32,
  pub end_offset: u32,
  pub fde_entry_start_pos: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UnwindFunctionWin {
  pub begin_offset: u32,
  pub end_offset: u32,
  pub unwind_info_offset: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct unw_dynamic_unwind_sections_t {
  pub dso_base: usize,
  pub dwarf_section: usize,
  pub dwarf_section_length: usize,
  pub compact_unwind_section: usize,
  pub compact_unwind_section_length: usize,
}
