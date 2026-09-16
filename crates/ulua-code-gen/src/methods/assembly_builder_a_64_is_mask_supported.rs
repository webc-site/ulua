use crate::{
  functions::{countlz_bit_utils::countlz, countrz_bit_utils::countrz},
  records::assembly_builder_a_64::AssemblyBuilderA64,
};

impl AssemblyBuilderA64 {
  pub fn is_mask_supported(&mut self, mask: u32) -> bool {
    let lz = countlz(mask);
    let rz = countrz(mask);

    lz + rz > 0 && lz + rz < 32 && (mask >> rz) == (1u32 << (32 - lz - rz)) - 1
  }
}
