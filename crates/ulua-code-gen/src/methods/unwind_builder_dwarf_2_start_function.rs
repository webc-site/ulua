use crate::{
  functions::{writeu_32::writeu_32, writeu_64::writeu_64},
  records::{
    unwind_builder_dwarf_2::UnwindBuilderDwarf2, unwind_function_dwarf_2::UnwindFunctionDwarf2,
  },
};

impl UnwindBuilderDwarf2 {
  pub fn start_function(&mut self) {
    // End offset is filled in later and everything gets adjusted at the end
    let func = UnwindFunctionDwarf2 {
      begin_offset: 0,
      end_offset: 0,
      fde_entry_start_pos: (self.pos as usize - self.raw_data.as_ptr() as usize) as u32,
    };
    self.unwind_functions.push(func);

    self.fde_entry_start = self.pos; // Will be written at the end
    unsafe {
      self.pos = writeu_32(self.pos, 0); // Length (to be filled later)
      self.pos = writeu_32(
        self.pos,
        (self.pos as usize - self.raw_data.as_ptr() as usize) as u32,
      ); // CIE pointer
      self.pos = writeu_64(self.pos, 0); // Initial location (to be filled later)
      self.pos = writeu_64(self.pos, 0); // Address range (to be filled later)
    }

    // Optional CIE augmentation section (not present)

    // Function call frame instructions to follow
  }
}
