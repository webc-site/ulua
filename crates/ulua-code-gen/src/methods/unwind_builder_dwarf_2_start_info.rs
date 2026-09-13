use core::ptr::null_mut;

use crate::{
  enums::arch::Arch,
  functions::{
    align_position::align_position, define_cfa_expression::define_cfa_expression,
    define_saved_register_location::define_saved_register_location, writeu_8::writeu_8,
    writeu_32::writeu_32, writeuleb_128::writeuleb_128,
  },
  macros::{
    codegen_assert::CODEGEN_ASSERT, dw_reg_a_64_lr::DW_REG_A64_LR, dw_reg_a_64_sp::DW_REG_A64_SP,
    dw_reg_x_64_ra::DW_REG_X64_RA, dw_reg_x_64_rsp::DW_REG_X64_RSP,
  },
  records::unwind_builder_dwarf_2::UnwindBuilderDwarf2,
};

impl UnwindBuilderDwarf2 {
  pub fn start_info(&mut self, arch: Arch) {
    CODEGEN_ASSERT!(arch == Arch::A64 || arch == Arch::X64);

    self.begin_offset = 0;
    self.unwind_functions.clear();
    self.pos = self.raw_data.as_mut_ptr();
    self.fde_entry_start = null_mut();

    let cie_length = self.pos;
    unsafe {
      self.pos = writeu_32(self.pos, 0); // Length (to be filled later)
    }

    unsafe {
      self.pos = writeu_32(self.pos, 0); // CIE id. 0 -- .eh_frame
    }
    unsafe {
      self.pos = writeu_8(self.pos, 1); // Version
    }

    unsafe {
      self.pos = writeu_8(self.pos, 0); // CIE augmentation String ""
    }

    let ra = if arch == Arch::A64 {
      DW_REG_A64_LR
    } else {
      DW_REG_X64_RA
    };

    unsafe {
      self.pos = writeuleb_128(self.pos, Self::K_CODE_ALIGN_FACTOR as u64); // Code align factor
    }
    unsafe {
      self.pos = writeuleb_128(self.pos, (-Self::K_DATA_ALIGN_FACTOR & 0x7f) as u64);
      // Data align factor of (as signed LEB128)
    }
    unsafe {
      self.pos = writeu_8(self.pos, ra as u8); // Return address register
    }

    // Optional CIE augmentation section (not present)

    // Call frame instructions (common for all FDEs)
    if arch == Arch::A64 {
      unsafe {
        self.pos = define_cfa_expression(self.pos, DW_REG_A64_SP, 0); // Define CFA to be the sp
      }
    } else {
      unsafe {
        self.pos = define_cfa_expression(self.pos, DW_REG_X64_RSP, 8); // Define CFA to be the rsp + 8
      }
      unsafe {
        self.pos = define_saved_register_location(self.pos, DW_REG_X64_RA, 8);
        // Define return address register (RA) to be located at CFA - 8
      }
    }

    unsafe {
      self.pos = align_position(cie_length, self.pos);
    }
    unsafe {
      writeu_32(
        cie_length,
        (self.pos as usize - cie_length as usize - 4) as u32,
      ); // Length field itself is excluded from length
    }
  }
}
