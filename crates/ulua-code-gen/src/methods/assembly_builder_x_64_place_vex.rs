use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  macros::{avx_3_1::avx_3_1, avx_3_2::avx_3_2, avx_3_3::avx_3_3},
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn place_vex(
    &mut self,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    set_w: bool,
    mode: u8,
    prefix: u8,
  ) {
    // Keep the asserts, but avoid CODEGEN_ASSERT!'s pointer-signature mismatch by
    // using explicit boolean checks that don't route through the macro.
    if !(dst.cat == CategoryX64::Reg) {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !(src1.cat == CategoryX64::Reg) {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !(src2.cat == CategoryX64::Reg || src2.cat == CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    // Several translated AVX call sites pass the RAW x86 opcode-map /
    // mandatory-prefix bytes (0x0F/0x38/0x3A and 0x66/0xF2/0xF3) instead of the
    // narrow VEX field encodings the C++ constants use (AVX_0F=0b00001,
    // AVX_66=0b01, ...). Those overflow the 5-bit mmmmm / 2-bit pp fields and
    // corrupt the surrounding bits. Normalize here (the single VEX choke point);
    // values already in proper field form (0..=3) fall through unchanged.
    let mode = match mode {
      0x0F => 0b00001,
      0x38 => 0b00010,
      0x3A => 0b00011,
      m => m,
    };
    let prefix = match prefix {
      0x66 => 0b01,
      0xF3 => 0b10,
      0xF2 => 0b11,
      p => p,
    };

    // C++ `dst.base.size == ymmword` — the l (256-bit) bit comes from the
    // destination REGISTER's size. A reg operand's `memSize` is always `none`,
    // so the original `dst.memSize` check forced l=0 (wrong for every ymm op).
    let l: u8 = if dst.base.size() == SizeX64::Ymmword {
      1
    } else {
      0
    };

    self.place(avx_3_1());
    self.place(avx_3_2(dst.base, src2.index, src2.base, mode));
    self.place(avx_3_3(set_w, src1.base, l, prefix));
  }
}
