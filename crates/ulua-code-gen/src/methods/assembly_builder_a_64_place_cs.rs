use crate::{
  enums::{condition_a_64::ConditionA64, kind_a_64::KindA64},
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_cs(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    cond: ConditionA64,
    op: u8,
    opc: u8,
    invert: i32,
  ) {
    if self.log_text {
      self.log_c_char_register_a_64_register_a_64_register_a_64_condition_a_64(
        name, dst, src1, src2, cond,
      );
    }

    // Avoid CODEGEN_ASSERT! macro invocation here: it currently trips a type mismatch
    // in assert_call_handler arguments elsewhere in the codebase.
    debug_assert!(dst.kind() == src1.kind() && dst.kind() == src2.kind());

    let sf = if dst.kind() == KindA64::X {
      0x80000000
    } else {
      0
    };

    let code_for_condition = [
      0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
    ];

    let cond_val = code_for_condition[cond as usize] as u32;

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((opc as u32) << 10)
        | ((cond_val ^ (invert as u32)) << 12)
        | ((src2.index() as u32) << 16)
        | ((op as u32) << 21)
        | sf,
    );
    self.commit();
  }
}
