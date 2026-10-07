use crate::{
  enums::r#type::Type, macros::union_reader::UNION_READER, records::constant_key::ConstantKey,
};

/// cpp `BytecodeBuilder::Constant` 的 `Type` 标签 + 匿名 union。Rust 端用带载荷的
/// enum 合一建模（同 `ulua-compiler` 的 `Constant` 先例）：标签与数据不可能脱钩，
/// 读错分支的 UB 在类型层面不可表达，全部 unsafe 消除。
///
/// 载荷即 cpp 各 `value*` 字段的值：`String`/`Import`/`Table`/`Closure`/`ClassShape`
/// 存的是下标/id，字符串字节本身在 `string_table`/`debug_strings` 侧管理。
#[derive(Debug, Clone, Copy)]
pub enum Constant {
  Nil,
  Boolean(bool),
  Number(f64),
  Integer(i64),
  /// cpp `Type_Vectorf`
  Vector([f32; 4]),
  /// cpp `Type_Vectord`（`BytecodeBuilder.h:203-214`）
  Vectord([f64; 4]),
  /// 字符串表下标（`string_table`/`debug_strings`）
  String(u32),
  Import(u32),
  /// table shape 下标
  Table(u32),
  /// 子闭包 proto 下标
  Closure(u32),
  /// class shape 下标
  ClassShape(u32),
}

impl Constant {
  /// 变体 → cpp `Type` 标签：供仍按标签分派的调用点（VCONST 断言等）使用，零成本。
  #[inline]
  pub(crate) const fn kind(&self) -> Type {
    match self {
      Self::Nil => Type::Nil,
      Self::Boolean(_) => Type::Boolean,
      Self::Number(_) => Type::Number,
      Self::Integer(_) => Type::Integer,
      Self::Vector(_) => Type::Vector,
      Self::Vectord(_) => Type::Vectord,
      Self::String(_) => Type::String,
      Self::Import(_) => Type::Import,
      Self::Table(_) => Type::Table,
      Self::Closure(_) => Type::Closure,
      Self::ClassShape(_) => Type::ClassShape,
    }
  }

  /// cpp `Constant` → `ConstantKey` 的推导：载荷按 cpp 匿名 union 的同一布局
  /// 打进 key 的 `value`/`extra`/`extra2`/`extra3`（Vector 的 x/y 占 `value`、
  /// z/w 占 `extra`；Vectord 四分量依次占满四字段；其余变体只用 `value`）。
  #[inline]
  pub(crate) fn key(&self) -> ConstantKey {
    let (r#type, value, extra, extra2, extra3) = match self {
      Self::Nil => (Type::Nil, 0, 0, 0, 0),
      Self::Boolean(v) => (Type::Boolean, u64::from(*v), 0, 0, 0),
      Self::Number(v) => (Type::Number, v.to_bits(), 0, 0, 0),
      Self::Integer(v) => (Type::Integer, *v as u64, 0, 0, 0),
      Self::Vector([x, y, z, w]) => (
        Type::Vector,
        u64::from(x.to_bits()) | (u64::from(y.to_bits()) << 32),
        u64::from(z.to_bits()) | (u64::from(w.to_bits()) << 32),
        0,
        0,
      ),
      Self::Vectord([x, y, z, w]) => (
        Type::Vectord,
        x.to_bits(),
        y.to_bits(),
        z.to_bits(),
        w.to_bits(),
      ),
      Self::String(v) => (Type::String, u64::from(*v), 0, 0, 0),
      Self::Import(v) => (Type::Import, u64::from(*v), 0, 0, 0),
      Self::Table(v) => (Type::Table, u64::from(*v), 0, 0, 0),
      Self::Closure(v) => (Type::Closure, u64::from(*v), 0, 0, 0),
      Self::ClassShape(v) => (Type::ClassShape, u64::from(*v), 0, 0, 0),
    };
    ConstantKey {
      r#type,
      value,
      extra,
      extra2,
      extra3,
    }
  }

  UNION_READER!("Constant" {
    /// 按字符串表下标读取活跃变体。
    pub(crate) fn as_string() -> u32 = String, 0;
    /// 按子闭包 proto 下标读取活跃变体。
    pub(crate) fn as_closure() -> u32 = Closure, 0;
  });
}
