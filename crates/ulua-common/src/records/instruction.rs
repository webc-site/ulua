use core::{
  fmt,
  ops::Deref,
  slice::{from_raw_parts, from_raw_parts_mut},
};

use crate::{
  enums::luau_opcode::LuauOpcode,
  functions::{get_jump_target::get_jump_target, read::BytecodeRead},
};

/// 字节码指令强类型透明包装结构体（封装底层裸 `u32` 字）。
///
/// 遵循 Luau 字节码字编码格式：
/// ```text
/// 31          24 23          16 15           8 7            0
/// ┌──────────────┬──────────────┬──────────────┬──────────────┐
/// │      C       │      B       │      A       │     OP       │  ABC
/// └──────────────┴──────────────┴──────────────┴──────────────┘
/// ┌─────────────────────────────┬──────────────┬──────────────┐
/// │              D              │      A       │     OP       │  AD
/// └─────────────────────────────┴──────────────┴──────────────┘
/// ┌────────────────────────────────────────────┬──────────────┐
/// │                     E                      │     OP       │  E
/// └────────────────────────────────────────────┴──────────────┘
/// ```
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Instruction(pub u32);

impl Instruction {
  /// 构造新的指令透明包装。
  #[inline(always)]
  pub const fn new(raw: u32) -> Self {
    Self(raw)
  }

  /// 获取底层裸 `u32` 字。
  #[inline(always)]
  pub const fn raw(self) -> u32 {
    self.0
  }

  /// 提取操作码字节（低 8 位）。
  #[inline(always)]
  pub const fn op(self) -> u8 {
    (self.0 & 0xFF) as u8
  }

  /// 提取 A 字段（8..16 位）。
  #[inline(always)]
  pub const fn a(self) -> u8 {
    ((self.0 >> 8) & 0xFF) as u8
  }

  /// 提取 B 字段（16..24 位）。
  #[inline(always)]
  pub const fn b(self) -> u8 {
    ((self.0 >> 16) & 0xFF) as u8
  }

  /// 提取 C 字段（24..32 位）。
  #[inline(always)]
  pub const fn c(self) -> u8 {
    ((self.0 >> 24) & 0xFF) as u8
  }

  /// 提取 D 字段（16 位有符号跳转偏移，16..32 位）。
  #[inline(always)]
  pub const fn d(self) -> i16 {
    (self.0 >> 16) as i16
  }

  /// 提取 E 字段（24 位有符号跳转偏移，符号扩展至 32 位，8..32 位）。
  #[inline(always)]
  pub const fn e(self) -> i32 {
    (self.0 as i32) >> 8
  }

  /// 尝试转换为强类型 `LuauOpcode` 枚举（超出有效范围返回 `None`）。
  #[inline(always)]
  pub fn opcode(self) -> Option<LuauOpcode> {
    LuauOpcode::from_repr(self.op())
  }

  /// 转换为强类型 `LuauOpcode` 枚举（超出有效范围钳制为 `LuauOpcode::LopNop`）。
  #[inline(always)]
  pub fn luau_opcode(self) -> LuauOpcode {
    LuauOpcode::from(self.op())
  }

  /// 计算以当前指令为基准的跳转目标 PC（与 `get_jump_target` 逻辑完全一致）。
  #[inline(always)]
  pub fn jump_target(self, pc: u32) -> i32 {
    get_jump_target(self.0, pc)
  }

  // --- AUX 扩展字段提取方法 ---

  /// 提取 AUX 指令的 A 字段（低 8 位）。
  #[inline(always)]
  pub const fn aux_a(self) -> u8 {
    (self.0 & 0xFF) as u8
  }

  /// 提取 AUX 指令的 B 字段（8..16 位）。
  #[inline(always)]
  pub const fn aux_b(self) -> u8 {
    ((self.0 >> 8) & 0xFF) as u8
  }

  /// 提取 AUX 指令常量/变量 24 位索引（低 24 位）。
  #[inline(always)]
  pub const fn aux_kv(self) -> u32 {
    self.0 & 0x00FF_FFFF
  }

  /// 提取 AUX 指令常量/变量 16 位索引（低 16 位）。
  #[inline(always)]
  pub const fn aux_kv16(self) -> u16 {
    (self.0 & 0xFFFF) as u16
  }

  /// 提取 AUX 指令槽位（高 16 位）。
  #[inline(always)]
  pub const fn aux_slot(self) -> u32 {
    self.0 >> 16
  }

  /// 提取 AUX 指令布尔标志（最低位）。
  #[inline(always)]
  pub const fn aux_kb(self) -> u32 {
    self.0 & 0x1
  }

  /// 提取 AUX 指令取反标志（最高位符号位）。
  #[inline(always)]
  pub const fn aux_not(self) -> u32 {
    self.0 >> 31
  }

  // --- 编码构造方法 ---

  /// 编码 ABC 格式指令。
  #[inline(always)]
  pub const fn encode_abc(op: LuauOpcode, a: u8, b: u8, c: u8) -> Self {
    Self((op as u32) | ((a as u32) << 8) | ((b as u32) << 16) | ((c as u32) << 24))
  }

  /// 编码 AD 格式指令。
  #[inline(always)]
  pub const fn encode_ad(op: LuauOpcode, a: u8, d: i16) -> Self {
    Self((op as u32) | ((a as u32) << 8) | (((d as u16) as u32) << 16))
  }

  /// 编码 E 格式指令。
  #[inline(always)]
  pub const fn encode_e(op: LuauOpcode, e: i32) -> Self {
    Self((op as u32) | (((e as u32) & 0x00FF_FFFF) << 8))
  }

  // --- 切片零成本视图转换 ---

  /// 将底层裸 `u32` 切片零成本转换为 `Instruction` 切片视图。
  #[inline(always)]
  pub fn from_slice(slice: &[u32]) -> &[Self] {
    // SAFETY: Instruction 是 #[repr(transparent)] 包装 u32，内存对齐与布局完全相同。
    unsafe { from_raw_parts(slice.as_ptr() as *const Self, slice.len()) }
  }

  /// 将底层裸 `u32` 可变切片零成本转换为 `Instruction` 可变切片视图。
  #[inline(always)]
  pub fn from_slice_mut(slice: &mut [u32]) -> &mut [Self] {
    // SAFETY: Instruction 是 #[repr(transparent)] 包装 u32，内存对齐与布局完全相同。
    unsafe { from_raw_parts_mut(slice.as_mut_ptr() as *mut Self, slice.len()) }
  }

  /// 将 `Instruction` 切片零成本转换为底层裸 `u32` 切片视图。
  #[inline(always)]
  pub fn as_raw_slice(slice: &[Self]) -> &[u32] {
    // SAFETY: Instruction 是 #[repr(transparent)] 包装 u32，内存对齐与布局完全相同。
    unsafe { from_raw_parts(slice.as_ptr() as *const u32, slice.len()) }
  }

  /// 将 `Instruction` 可变切片零成本转换为底层裸 `u32` 可变切片视图。
  #[inline(always)]
  pub fn as_raw_slice_mut(slice: &mut [Self]) -> &mut [u32] {
    // SAFETY: Instruction 是 #[repr(transparent)] 包装 u32，内存对齐与布局完全相同。
    unsafe { from_raw_parts_mut(slice.as_mut_ptr() as *mut u32, slice.len()) }
  }
}

impl From<u32> for Instruction {
  #[inline(always)]
  fn from(raw: u32) -> Self {
    Self(raw)
  }
}

impl From<Instruction> for u32 {
  #[inline(always)]
  fn from(insn: Instruction) -> Self {
    insn.0
  }
}

impl Deref for Instruction {
  type Target = u32;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl AsRef<u32> for Instruction {
  #[inline(always)]
  fn as_ref(&self) -> &u32 {
    &self.0
  }
}

impl AsMut<u32> for Instruction {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut u32 {
    &mut self.0
  }
}

impl BytecodeRead for Instruction {
  #[inline(always)]
  fn from_bytes(bytes: &[u8]) -> Self {
    Self(u32::from_bytes(bytes))
  }
}

impl fmt::Display for Instruction {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl fmt::LowerHex for Instruction {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::LowerHex::fmt(&self.0, f)
  }
}

impl fmt::UpperHex for Instruction {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::UpperHex::fmt(&self.0, f)
  }
}

impl fmt::Binary for Instruction {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Binary::fmt(&self.0, f)
  }
}
