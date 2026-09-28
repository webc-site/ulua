use ulua_common::enums::luau_builtin_function::{LBF_BUFFER_WRITEINTEGER, LuauBuiltinFunction};

use crate::records::builtin_info::BuiltinInfo;

/// 与 C++ `getBuiltinInfo` 的静态表同型：按判别值连续索引（0..=132），
/// 编译期常量，零运行时构造。S = `BuiltinInfo::FLAG_NONE_SAFE`。
const S: u32 = BuiltinInfo::FLAG_NONE_SAFE;

/// 构造表项的 const 助手（编译期展开，无运行时代码）
const fn info(params: i32, results: i32, flags: u32) -> BuiltinInfo {
  BuiltinInfo {
    params,
    results,
    flags,
  }
}

const K_BUILTIN_INFO: [BuiltinInfo; LBF_BUFFER_WRITEINTEGER as usize + 1] = [
  info(-1, -1, 0), // LbfNone
  info(-1, -1, 0), // LbfAssert
  info(1, 1, S),   // LbfMathAbs
  info(1, 1, S),   // LbfMathAcos
  info(1, 1, S),   // LbfMathAsin
  info(2, 1, S),   // LbfMathAtan2
  info(1, 1, S),   // LbfMathAtan
  info(1, 1, S),   // LbfMathCeil
  info(1, 1, S),   // LbfMathCosh
  info(1, 1, S),   // LbfMathCos
  info(1, 1, S),   // LbfMathDeg
  info(1, 1, S),   // LbfMathExp
  info(1, 1, S),   // LbfMathFloor
  info(2, 1, S),   // LbfMathFmod
  info(1, 2, S),   // LbfMathFrexp
  info(2, 1, S),   // LbfMathLdexp
  info(1, 1, S),   // LbfMathLog10
  info(-1, 1, 0),  // LbfMathLog
  info(-1, 1, 0),  // LbfMathMax
  info(-1, 1, 0),  // LbfMathMin
  info(1, 2, S),   // LbfMathModf
  info(2, 1, S),   // LbfMathPow
  info(1, 1, S),   // LbfMathRad
  info(1, 1, S),   // LbfMathSinh
  info(1, 1, S),   // LbfMathSin
  info(1, 1, S),   // LbfMathSqrt
  info(1, 1, S),   // LbfMathTanh
  info(1, 1, S),   // LbfMathTan
  info(2, 1, S),   // LbfBit32Arshift
  info(-1, 1, 0),  // LbfBit32Band
  info(1, 1, S),   // LbfBit32Bnot
  info(-1, 1, 0),  // LbfBit32Bor
  info(-1, 1, 0),  // LbfBit32Bxor
  info(-1, 1, 0),  // LbfBit32Btest
  info(-1, 1, 0),  // LbfBit32Extract
  info(2, 1, S),   // LbfBit32Lrotate
  info(2, 1, S),   // LbfBit32Lshift
  info(-1, 1, 0),  // LbfBit32Replace
  info(2, 1, S),   // LbfBit32Rrotate
  info(2, 1, S),   // LbfBit32Rshift
  info(1, 1, 0),   // LbfType
  info(-1, -1, 0), // LbfStringByte
  info(-1, 1, 0),  // LbfStringChar
  info(1, 1, S),   // LbfStringLen
  info(1, 1, 0),   // LbfTypeof
  info(-1, 1, 0),  // LbfStringSub
  info(3, 1, S),   // LbfMathClamp
  info(1, 1, S),   // LbfMathSign
  info(1, 1, S),   // LbfMathRound
  info(3, 1, 0),   // LbfRawset
  info(2, 1, 0),   // LbfRawget
  info(2, 1, 0),   // LbfRawequal
  info(-1, 0, 0),  // LbfTableInsert
  info(-1, -1, 0), // LbfTableUnpack
  info(-1, 1, 0),  // LbfVector
  info(1, 1, S),   // LbfBit32Countlz
  info(1, 1, S),   // LbfBit32Countrz
  info(-1, -1, 0), // LbfSelectVararg
  info(1, 1, S),   // LbfRawlen
  info(3, 1, S),   // LbfBit32Extractk
  info(1, 1, 0),   // LbfGetmetatable
  info(2, 1, 0),   // LbfSetmetatable
  info(-1, 1, 0),  // LbfTonumber
  info(1, 1, 0),   // LbfTostring
  info(1, 1, S),   // LbfBit32Byteswap
  info(2, 1, S),   // LbfBufferReadi8
  info(2, 1, S),   // LbfBufferReadu8
  info(3, 0, S),   // LbfBufferWriteu8
  info(2, 1, S),   // LbfBufferReadi16
  info(2, 1, S),   // LbfBufferReadu16
  info(3, 0, S),   // LbfBufferWriteu16
  info(2, 1, S),   // LbfBufferReadi32
  info(2, 1, S),   // LbfBufferReadu32
  info(3, 0, S),   // LbfBufferWriteu32
  info(2, 1, S),   // LbfBufferReadf32
  info(3, 0, S),   // LbfBufferWritef32
  info(2, 1, S),   // LbfBufferReadf64
  info(3, 0, S),   // LbfBufferWritef64
  info(1, 1, S),   // LbfVectorMagnitude
  info(1, 1, S),   // LbfVectorNormalize
  info(2, 1, S),   // LbfVectorCross
  info(2, 1, S),   // LbfVectorDot
  info(1, 1, S),   // LbfVectorFloor
  info(1, 1, S),   // LbfVectorCeil
  info(1, 1, S),   // LbfVectorAbs
  info(1, 1, S),   // LbfVectorSign
  info(3, 1, S),   // LbfVectorClamp
  info(-1, 1, 0),  // LbfVectorMin
  info(-1, 1, 0),  // LbfVectorMax
  info(3, 1, S),   // LbfMathLerp
  info(3, 1, S),   // LbfVectorLerp
  info(1, 1, S),   // LbfMathIsnan
  info(1, 1, S),   // LbfMathIsinf
  info(1, 1, S),   // LbfMathIsfinite
  info(1, 1, S),   // LbfIntegerCreate
  info(1, 1, S),   // LbfIntegerTonumber
  info(1, 1, S),   // LbfIntegerNeg
  info(2, 1, S),   // LbfIntegerAdd
  info(2, 1, S),   // LbfIntegerSub
  info(2, 1, S),   // LbfIntegerMul
  info(2, 1, S),   // LbfIntegerDiv
  info(-1, 1, 0),  // LbfIntegerMin
  info(-1, 1, 0),  // LbfIntegerMax
  info(2, 1, S),   // LbfIntegerRem
  info(2, 1, S),   // LbfIntegerIdiv
  info(2, 1, S),   // LbfIntegerUdiv
  info(2, 1, S),   // LbfIntegerUrem
  info(2, 1, S),   // LbfIntegerMod
  info(3, 1, S),   // LbfIntegerClamp
  info(-1, 1, 0),  // LbfIntegerBand
  info(-1, 1, 0),  // LbfIntegerBor
  info(1, 1, S),   // LbfIntegerBnot
  info(-1, 1, 0),  // LbfIntegerBxor
  info(2, 1, S),   // LbfIntegerLt
  info(2, 1, S),   // LbfIntegerLe
  info(2, 1, S),   // LbfIntegerUlt
  info(2, 1, S),   // LbfIntegerUle
  info(2, 1, S),   // LbfIntegerGt
  info(2, 1, S),   // LbfIntegerGe
  info(2, 1, S),   // LbfIntegerUgt
  info(2, 1, S),   // LbfIntegerUge
  info(2, 1, S),   // LbfIntegerLshift
  info(2, 1, S),   // LbfIntegerRshift
  info(2, 1, S),   // LbfIntegerArshift
  info(2, 1, S),   // LbfIntegerLrotate
  info(2, 1, S),   // LbfIntegerRrotate
  info(-1, 1, 0),  // LbfIntegerExtract
  info(-1, 1, 0),  // LbfIntegerBtest
  info(1, 1, S),   // LbfIntegerCountrz
  info(1, 1, S),   // LbfIntegerCountlz
  info(1, 1, S),   // LbfIntegerBswap
  info(2, 1, S),   // LbfBufferReadinteger
  info(3, 0, S),   // LbfBufferWriteinteger
];

const _: () = {
  // 表必须覆盖全部连续判别值（LbfNone..=LbfBufferWriteinteger），新增变体在此编译期报错
  assert!(K_BUILTIN_INFO.len() == LBF_BUFFER_WRITEINTEGER as usize + 1);
};

pub(crate) fn get_builtin_info(bfid: i32) -> BuiltinInfo {
  // 非法 id（-1/损坏数据）归入 LBF_NONE（params/results = -1），与 C++ switch 的
  // default 分支语义一致；from_id 已把合法域钉死在 0..=最大判别值。
  let index = LuauBuiltinFunction::from_id(bfid).map_or(0, |bf| bf as usize);
  // Safety: index 由 from_id/常量 0 产生，必落在 K_BUILTIN_INFO 长度内。
  unsafe { *K_BUILTIN_INFO.get_unchecked(index) }
}
