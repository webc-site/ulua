use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn rem(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    // dst must hold the quotient from a preceding sdiv/udiv.
    // dst != src1 because mul clobbers dst before sub reads src1.
    assert!(dst.index() != src1.index());

    // dst = src1 - (dst * src2);
    self.msub(dst, dst, src2, src1);
  }
}
