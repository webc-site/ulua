use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    register_x_64::RegisterX64, unwind_builder_win::UnwindBuilderWin,
    unwind_code_win::UnwindCodeWin,
  },
};

const UWOP_PUSH_NONVOL: u8 = 0;
const UWOP_ALLOC_LARGE: u8 = 1;
const UWOP_ALLOC_SMALL: u8 = 2;
const UWOP_SET_FPREG: u8 = 3;
const UWOP_SAVE_XMM128: u8 = 8;

fn unwind_code(offset: u8, opcode: u8, opinfo: u8) -> UnwindCodeWin {
  let mut result = UnwindCodeWin {
    offset,
    opcode_opinfo: 0,
  };
  result.set_opcode(opcode);
  result.set_opinfo(opinfo);
  result
}

impl UnwindBuilderWin {
  pub fn prologue_x_64(
    &mut self,
    prologue_size: u32,
    stack_size: u32,
    setup_frame: bool,
    gpr: &[RegisterX64],
    simd: &[RegisterX64],
  ) {
    CODEGEN_ASSERT!(stack_size > 0 && stack_size < 4096 && stack_size.is_multiple_of(8));
    CODEGEN_ASSERT!(prologue_size < 256);

    let mut stack_offset: u32 = 8;
    let mut prologue_offset: u32 = 0;

    if setup_frame {
      stack_offset += 8;
      prologue_offset += 2;
      self.unwind_codes.push(unwind_code(
        prologue_offset as u8,
        UWOP_PUSH_NONVOL,
        RegisterX64::RBP.index(),
      ));

      prologue_offset += 3;
      self.frame_reg = RegisterX64::RBP;
      self.frame_reg_offset = 0;
      self.unwind_codes.push(unwind_code(
        prologue_offset as u8,
        UWOP_SET_FPREG,
        self.frame_reg_offset,
      ));
    }

    for reg in gpr {
      CODEGEN_ASSERT!(reg.size() == SizeX64::Qword);

      stack_offset += 8;
      prologue_offset += 2;
      self.unwind_codes.push(unwind_code(
        prologue_offset as u8,
        UWOP_PUSH_NONVOL,
        reg.index(),
      ));
    }

    CODEGEN_ASSERT!(!setup_frame || simd.is_empty());

    let mut simd_storage_size = simd.len() as u32 * 16;

    if !simd.is_empty() && stack_offset % 16 == 8 {
      simd_storage_size += 8;
    }

    if stack_size <= 128 {
      stack_offset += stack_size;
      prologue_offset += if stack_size == 128 { 7 } else { 4 };
      self.unwind_codes.push(unwind_code(
        prologue_offset as u8,
        UWOP_ALLOC_SMALL,
        ((stack_size - 8) / 8) as u8,
      ));
    } else {
      CODEGEN_ASSERT!(stack_size < 4096);

      stack_offset += stack_size;
      prologue_offset += 7;

      let encoded_offset = (stack_size / 8) as u16;
      let bytes = encoded_offset.to_le_bytes();
      self.unwind_codes.push(UnwindCodeWin {
        offset: bytes[0],
        opcode_opinfo: bytes[1],
      });
      self
        .unwind_codes
        .push(unwind_code(prologue_offset as u8, UWOP_ALLOC_LARGE, 0));
    }

    let mut xmm_store_offset = stack_size - simd_storage_size;

    for reg in simd {
      CODEGEN_ASSERT!(reg.size() == SizeX64::Xmmword);
      CODEGEN_ASSERT!(
        xmm_store_offset.is_multiple_of(16),
        "simd stores have to be performed to aligned locations"
      );

      prologue_offset += if xmm_store_offset >= 128 { 10 } else { 7 };
      self
        .unwind_codes
        .push(unwind_code((xmm_store_offset / 16) as u8, 0, 0));
      self.unwind_codes.push(unwind_code(
        prologue_offset as u8,
        UWOP_SAVE_XMM128,
        reg.index(),
      ));
      xmm_store_offset += 16;
    }

    CODEGEN_ASSERT!(stack_offset.is_multiple_of(16));
    CODEGEN_ASSERT!(prologue_offset == prologue_size);

    self.prolog_size = prologue_size as u8;
  }
}
