use core::mem::size_of;

use crate::enums::ir_const_kind::IrConstKind;

/// 以带载荷 enum 取代原 `IrConstKind` 标签 + `IrConstValue` union，类型与数据合一，
/// 消除按标签误读 union 字段的 UB 风险。
/// 尺寸与对齐和原 `#[repr(C)]` 结构完全一致（16 字节 / 8），热路径按值拷贝无回退。
/// 不派生 `PartialEq`：`Double` 含 NaN，相等性判断统一走 `ConstantKey`（按位 `u64`）。
#[derive(Clone, Copy, Debug)]
pub enum IrConst {
  Int(i32),
  Int64(i64),
  Uint(u32),
  Double(f64),
  Tag(u8),
  Import(u32),
}

// 尺寸锁死：编译期保证与原标签+union 形态等大，防止未来加字段回退热路径
const _: () = assert!(size_of::<IrConst>() == 16);

impl Default for IrConst {
  fn default() -> Self {
    Self::Int(0)
  }
}

impl IrConst {
  /// 种类标签；仅用于 `ConstantKey` 去重与 `is_compatible_constant` 断言，取值请直配载荷
  #[inline]
  pub fn kind(&self) -> IrConstKind {
    match self {
      Self::Int(_) => IrConstKind::Int,
      Self::Int64(_) => IrConstKind::Int64,
      Self::Uint(_) => IrConstKind::Uint,
      Self::Double(_) => IrConstKind::Double,
      Self::Tag(_) => IrConstKind::Tag,
      Self::Import(_) => IrConstKind::Import,
    }
  }
}
