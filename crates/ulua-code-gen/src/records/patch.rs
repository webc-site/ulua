use crate::enums::kind::Kind;

/// 对应 cpp AssemblyBuilderA64.h `struct Patch`（kind / label / location 三个独立成员）。
///
/// 此处刻意不做位打包：cpp 的 `label` 是完整 30/32 位值，打包成单 u32 会让 `label << 2`
/// 静默丢掉高位；`Kind` 只有 IMM26/IMM19/IMM14 三个合法值，2 位掩码却能表示 0..=3，
/// 从掩码值 transmute 出 3 即为 UB。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Patch {
  pub(crate) kind: Kind,
  pub(crate) label: u32,
  pub(crate) location: u32,
}
