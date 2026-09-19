use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_a_64::{AssemblyBuilderA64, K_MAX_IMMEDIATE},
    register_a_64::RegisterA64,
  },
};

pub fn emit_add_offset(
  build: &mut AssemblyBuilderA64,
  dst: RegisterA64,
  src: RegisterA64,
  offset: usize,
) {
  CODEGEN_ASSERT!(dst != src);
  CODEGEN_ASSERT!(offset <= i32::MAX as usize);

  // cpp: `if (offset <= AssemblyBuilderA64::kMaxImmediate)` —— 在 usize 全域比较后再窄化，
  // 避免高位被截断的 offset 落进立即数分支。
  if offset <= K_MAX_IMMEDIATE {
    build.add_register_a_64_register_a_64_u16(dst, src, offset as u16);
  } else {
    build.mov_register_a_64_i32(dst, offset as i32);
    build.add_register_a_64_register_a_64_register_a_64_i32(dst, dst, src, 0);
  }
}
