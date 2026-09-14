#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RoundingModeX64 {
  RoundToNearestEven = 0b00,
  RoundToNegativeInfinity = 0b01,
  RoundToPositiveInfinity = 0b10,
  RoundToZero = 0b11,
}
