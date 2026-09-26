#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::FromRepr)]
pub enum TMS {
  TmIndex,
  TmNewIndex,
  TmMode,
  TmNameCall,
  TmCall,
  TmIter,
  TmLen,

  TmEq, // last tag method with `fast' access

  TmAdd,
  TmSub,
  TmMul,
  TmDiv,
  TmIDiv,
  TmMod,
  TmPow,
  TmUnm,

  TmLt,
  TmLe,
  TmConcat,
  TmType,
  TmMetaTable,

  TmN, // number of elements in the enum
}

impl TMS {
  /// checked 构造辅助（仿 `LuauBuiltinFunction::from_id` 风格）：TMS 是判别值 0..=21
  /// 连续、`#[repr(u32)]` 的枚举。`TmN`（21）是元素计数哨兵，不是真实
  /// 元方法，故同样视为无效输入。
  #[inline]
  pub const fn from_u32(value: u32) -> Option<TMS> {
    match Self::from_repr(value) {
      Some(TMS::TmN) | None => None,
      valid => valid,
    }
  }
}
