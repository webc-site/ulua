use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

pub fn emit_add_offset(
  build: &mut AssemblyBuilderA64,
  dst: RegisterA64,
  src: RegisterA64,
  offset: usize,
) {
  CODEGEN_ASSERT!(dst != src);
  CODEGEN_ASSERT!(offset <= i32::MAX as usize);

  const K_MAX_IMMEDIATE: u16 = 4095;

  if offset as u16 <= K_MAX_IMMEDIATE {
    build.add_register_a_64_register_a_64_u16(dst, src, offset as u16);
  } else {
    build.mov_register_a_64_i32(dst, offset as i32);
    build.add_register_a_64_register_a_64_register_a_64_i32(dst, dst, src, 0);
  }
}
