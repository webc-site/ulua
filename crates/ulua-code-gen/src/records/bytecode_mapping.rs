#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BytecodeMapping {
  pub ir_location: u32,
  pub asm_location: u32,
}
