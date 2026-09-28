//! 「标签 + 匿名 union」建模（BcVmConst / BcImm / Constant）的同形 `as_*` 读取器
//! 收口：三族共 11 个方法体逐字同构（match 活跃变体取载荷，误用路径 debug 断言 +
//! release 一致零值），差异只有名字/变体/返回类型/零值/可见性与各自文档。
//! 断言消息按 `"<方法名> on non-<变体名> <类型名>"` 由 concat 逐字拼接，与原
//! 各方法内的字面量完全一致；`#[inline]` 与文档属性原样透传。

macro_rules! UNION_READER {
  ($label:literal { $($(#[$meta:meta])* $vis:vis fn $name:ident() -> $ret:ty = $variant:ident, $zero:expr);* $(;)? }) => {
    $($(#[$meta])*
      #[inline]
      $vis fn $name(&self) -> $ret {
        match self {
          Self::$variant(value) => *value,
          _ => {
            debug_assert!(
              false,
              concat!(stringify!($name), " on non-", stringify!($variant), " ", $label)
            );
            $zero
          }
        }
      })*
  };
}

pub(crate) use UNION_READER;
