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
