#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ControlFlow {
  Bits0 = 0b00000,
  None = 0b00001,
  Returns = 0b00010,
  Bits3 = 0b00011,
  Throws = 0b00100,
  Bits5 = 0b00101,
  Bits6 = 0b00110,
  Bits7 = 0b00111,
  Breaks = 0b01000,
  Bits9 = 0b01001,
  Bits10 = 0b01010,
  Bits11 = 0b01011,
  Bits12 = 0b01100,
  Bits13 = 0b01101,
  Bits14 = 0b01110,
  Bits15 = 0b01111,
  Continues = 0b10000,
  Bits17 = 0b10001,
  Bits18 = 0b10010,
  Bits19 = 0b10011,
  Bits20 = 0b10100,
  Bits21 = 0b10101,
  Bits22 = 0b10110,
  Bits23 = 0b10111,
  Bits24 = 0b11000,
  Bits25 = 0b11001,
  Bits26 = 0b11010,
  Bits27 = 0b11011,
  Bits28 = 0b11100,
  Bits29 = 0b11101,
  Bits30 = 0b11110,
  Bits31 = 0b11111,
}

impl ControlFlow {
  pub const ZERO: Self = Self::Bits0;

  /// C++ `ControlFlow(int(a) & int(b))` / `int(a) | int(b)`（`Analysis/include/
  /// Luau/ControlFlow.h:21-29`）：任意 int 位型合法，无断言路径。Rust 侧枚举
  /// 仅 5 位可表示，以 5 位掩码 + 常量位型表实现；可达输入（按位运算结果 ≤
  /// 0b11111）与 C++ 语义逐位一致。
  pub fn from_bits(bits: u32) -> Self {
    // 位型表：下标 i ↔ 枚举值 i，覆盖 0..=31 全部 32 个位型。
    const BIT_PATTERNS: [ControlFlow; 32] = [
      ControlFlow::Bits0,
      ControlFlow::None,
      ControlFlow::Returns,
      ControlFlow::Bits3,
      ControlFlow::Throws,
      ControlFlow::Bits5,
      ControlFlow::Bits6,
      ControlFlow::Bits7,
      ControlFlow::Breaks,
      ControlFlow::Bits9,
      ControlFlow::Bits10,
      ControlFlow::Bits11,
      ControlFlow::Bits12,
      ControlFlow::Bits13,
      ControlFlow::Bits14,
      ControlFlow::Bits15,
      ControlFlow::Continues,
      ControlFlow::Bits17,
      ControlFlow::Bits18,
      ControlFlow::Bits19,
      ControlFlow::Bits20,
      ControlFlow::Bits21,
      ControlFlow::Bits22,
      ControlFlow::Bits23,
      ControlFlow::Bits24,
      ControlFlow::Bits25,
      ControlFlow::Bits26,
      ControlFlow::Bits27,
      ControlFlow::Bits28,
      ControlFlow::Bits29,
      ControlFlow::Bits30,
      ControlFlow::Bits31,
    ];
    // SAFETY: `bits & 0b1_1111` 上界为 31 < 32，下标恒在表内。
    unsafe { *BIT_PATTERNS.get_unchecked((bits & 0b1_1111) as usize) }
  }
}
