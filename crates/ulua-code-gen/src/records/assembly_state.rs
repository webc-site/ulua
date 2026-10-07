use ulua_common::records::dense_hash_table::DenseDefault;

use crate::enums::a_64::Kind;

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

// 注册集合：寄存器分配器的位集状态（对齐 C++ `RegisterSet`）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Set {
  /// 分配器管理的寄存器集（构造时初始化）
  pub base: u32,

  /// 初始集合中的空闲子集
  pub free: u32,

  /// 初始集合中被分配为临时的子集
  pub temp: u32,

  /// 哪条指令定义了哪个寄存器（供 spill 用）；仅非 free 且非 temp 时有效
  pub defs: [u32; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[derive(Default)]
pub struct RegisterSet {
  pub regs: [u64; 4],

  // 若 variadic 序列激活，记录其起始寄存器
  pub vararg_seq: bool,
  pub vararg_start: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[derive(Default)]
pub struct RegisterLink {
  pub reg: u8,
  pub version: u32,
}

impl DenseDefault for RegisterLink {
  fn dense_default() -> Self {
    Self::default()
  }
}
