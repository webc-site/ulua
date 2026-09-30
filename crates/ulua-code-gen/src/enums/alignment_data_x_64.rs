#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[derive(Default)]
pub enum AlignmentDataX64 {
  #[default]
  Nop,
  Int3,
  Ud2,
}
