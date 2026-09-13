use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn fmla(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    // There is no scalar version of FMLA instruction
    // Vector instruction is used for both cases with proper sz bit.

    //                Q U        Sz  Rm    Opcode Rn    Rd
    let op: u32 = 0b0000_1110_0010_0000_1100_1100_0000_0000;
    let q_bit: u32 = 1 << 30;
    let sz_bit: u32 = 1 << 22;

    if dst.kind() == KindA64::D {
      assert!(src1.kind() == KindA64::D && src2.kind() == KindA64::D);

      if self.log_text {
        self.log_append(format_args!(
          " {:<12}sd{},d{},d{}\n",
          "fmla",
          dst.index(),
          src1.index(),
          src2.index()
        ));
      }

      self.place(
        (dst.index() as u32)
          | ((src1.index() as u32) << 5)
          | ((src2.index() as u32) << 16)
          | op
          | q_bit
          | sz_bit,
      );
    } else if dst.kind() == KindA64::S {
      assert!(src1.kind() == KindA64::S && src2.kind() == KindA64::S);

      if self.log_text {
        self.log_append(format_args!(
          " {:<12}ss{},s{},s{}\n",
          "fmla",
          dst.index(),
          src1.index(),
          src2.index()
        ));
      }

      self.place(
        (dst.index() as u32) | ((src1.index() as u32) << 5) | ((src2.index() as u32) << 16) | op,
      );
    } else {
      assert!(dst.kind() == KindA64::Q && src1.kind() == KindA64::Q && src2.kind() == KindA64::Q);

      if self.log_text {
        self.log_append(format_args!(
          " {:<12}sv{}.4s,v{}.4s,v{}.4s\n",
          "fmla",
          dst.index(),
          src1.index(),
          src2.index()
        ));
      }

      self.place(
        (dst.index() as u32)
          | ((src1.index() as u32) << 5)
          | ((src2.index() as u32) << 16)
          | op
          | q_bit,
      );
    }

    self.commit();
  }
}
