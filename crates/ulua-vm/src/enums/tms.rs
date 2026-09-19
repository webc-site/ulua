#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
