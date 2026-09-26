use crate::enums::r#type::Type;

/// cpp `BytecodeBuilder::ConstantKey`（`BytecodeBuilder.h:232-249`）。
///
/// `value`/`extra` 对应 cpp 的 `value`/`extra1`；`extra2`/`extra3` 只被
/// `Type_Vectord` 用到（cpp 注释：Vectorf 的 x/y 在 `value`、z/w 在 `extra1`；
/// Vectord 的 x 在 `value`、y/z/w 在 `extra1/2/3`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantKey {
  pub(crate) r#type: Type,
  pub(crate) value: u64,
  pub(crate) extra: u64,
  pub(crate) extra2: u64,
  pub(crate) extra3: u64,
}
