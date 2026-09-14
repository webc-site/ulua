#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LuauBuiltinFunction {
  LbfNone = 0,

  // assert()
  LbfAssert,

  // math.
  LbfMathAbs,
  LbfMathAcos,
  LbfMathAsin,
  LbfMathAtan2,
  LbfMathAtan,
  LbfMathCeil,
  LbfMathCosh,
  LbfMathCos,
  LbfMathDeg,
  LbfMathExp,
  LbfMathFloor,
  LbfMathFmod,
  LbfMathFrexp,
  LbfMathLdexp,
  LbfMathLog10,
  LbfMathLog,
  LbfMathMax,
  LbfMathMin,
  LbfMathModf,
  LbfMathPow,
  LbfMathRad,
  LbfMathSinh,
  LbfMathSin,
  LbfMathSqrt,
  LbfMathTanh,
  LbfMathTan,

  // bit32.
  LbfBit32Arshift,
  LbfBit32Band,
  LbfBit32Bnot,
  LbfBit32Bor,
  LbfBit32Bxor,
  LbfBit32Btest,
  LbfBit32Extract,
  LbfBit32Lrotate,
  LbfBit32Lshift,
  LbfBit32Replace,
  LbfBit32Rrotate,
  LbfBit32Rshift,

  // type()
  LbfType,

  // string.
  LbfStringByte,
  LbfStringChar,
  LbfStringLen,

  // typeof()
  LbfTypeof,

  // string.
  LbfStringSub,

  // math.
  LbfMathClamp,
  LbfMathSign,
  LbfMathRound,

  // raw*
  LbfRawset,
  LbfRawget,
  LbfRawequal,

  // table.
  LbfTableInsert,
  LbfTableUnpack,

  // vector ctor
  LbfVector,

  // bit32.count
  LbfBit32Countlz,
  LbfBit32Countrz,

  // select(_, ...)
  LbfSelectVararg,

  // rawlen
  LbfRawlen,

  // bit32.extract(_, k, k)
  LbfBit32Extractk,

  // get/setmetatable
  LbfGetmetatable,
  LbfSetmetatable,

  // tonumber/tostring
  LbfTonumber,
  LbfTostring,

  // bit32.byteswap(n)
  LbfBit32Byteswap,

  // Buffer.
  LbfBufferReadi8,
  LbfBufferReadu8,
  LbfBufferWriteu8,
  LbfBufferReadi16,
  LbfBufferReadu16,
  LbfBufferWriteu16,
  LbfBufferReadi32,
  LbfBufferReadu32,
  LbfBufferWriteu32,
  LbfBufferReadf32,
  LbfBufferWritef32,
  LbfBufferReadf64,
  LbfBufferWritef64,

  // vector.
  LbfVectorMagnitude,
  LbfVectorNormalize,
  LbfVectorCross,
  LbfVectorDot,
  LbfVectorFloor,
  LbfVectorCeil,
  LbfVectorAbs,
  LbfVectorSign,
  LbfVectorClamp,
  LbfVectorMin,
  LbfVectorMax,

  // math.lerp
  LbfMathLerp,

  // vector.lerp
  LbfVectorLerp,

  // math.
  LbfMathIsnan,
  LbfMathIsinf,
  LbfMathIsfinite,

  // integer
  LbfIntegerCreate,
  LbfIntegerTonumber,
  LbfIntegerNeg,
  LbfIntegerAdd,
  LbfIntegerSub,
  LbfIntegerMul,
  LbfIntegerDiv,
  LbfIntegerMin,
  LbfIntegerMax,
  LbfIntegerRem,
  LbfIntegerIdiv,
  LbfIntegerUdiv,
  LbfIntegerUrem,
  LbfIntegerMod,
  LbfIntegerClamp,
  LbfIntegerBand,
  LbfIntegerBor,
  LbfIntegerBnot,
  LbfIntegerBxor,
  LbfIntegerLt,
  LbfIntegerLe,
  LbfIntegerUlt,
  LbfIntegerUle,
  LbfIntegerGt,
  LbfIntegerGe,
  LbfIntegerUgt,
  LbfIntegerUge,
  LbfIntegerLshift,
  LbfIntegerRshift,
  LbfIntegerArshift,
  LbfIntegerLrotate,
  LbfIntegerRrotate,
  LbfIntegerExtract,
  LbfIntegerBtest,
  LbfIntegerCountrz,
  LbfIntegerCountlz,
  LbfIntegerBswap,

  // Buffer.readinteger / Buffer.writeinteger (int64_t)
  LbfBufferReadinteger,
  LbfBufferWriteinteger,
}

impl LuauBuiltinFunction {
  pub const LBF_NONE: Self = Self::LbfNone;
  pub const LBF_ASSERT: Self = Self::LbfAssert;
  pub const LBF_MATH_ABS: Self = Self::LbfMathAbs;
  pub const LBF_MATH_ACOS: Self = Self::LbfMathAcos;
  pub const LBF_MATH_ASIN: Self = Self::LbfMathAsin;
  pub const LBF_MATH_ATAN2: Self = Self::LbfMathAtan2;
  pub const LBF_MATH_ATAN: Self = Self::LbfMathAtan;
  pub const LBF_MATH_CEIL: Self = Self::LbfMathCeil;
  pub const LBF_MATH_COSH: Self = Self::LbfMathCosh;
  pub const LBF_MATH_COS: Self = Self::LbfMathCos;
  pub const LBF_MATH_DEG: Self = Self::LbfMathDeg;
  pub const LBF_MATH_EXP: Self = Self::LbfMathExp;
  pub const LBF_MATH_FLOOR: Self = Self::LbfMathFloor;
  pub const LBF_MATH_FMOD: Self = Self::LbfMathFmod;
  pub const LBF_MATH_FREXP: Self = Self::LbfMathFrexp;
  pub const LBF_MATH_LDEXP: Self = Self::LbfMathLdexp;
  pub const LBF_MATH_LOG10: Self = Self::LbfMathLog10;
  pub const LBF_MATH_LOG: Self = Self::LbfMathLog;
  pub const LBF_MATH_MAX: Self = Self::LbfMathMax;
  pub const LBF_MATH_MIN: Self = Self::LbfMathMin;
  pub const LBF_MATH_MODF: Self = Self::LbfMathModf;
  pub const LBF_MATH_POW: Self = Self::LbfMathPow;
  pub const LBF_MATH_RAD: Self = Self::LbfMathRad;
  pub const LBF_MATH_SINH: Self = Self::LbfMathSinh;
  pub const LBF_MATH_SIN: Self = Self::LbfMathSin;
  pub const LBF_MATH_SQRT: Self = Self::LbfMathSqrt;
  pub const LBF_MATH_TANH: Self = Self::LbfMathTanh;
  pub const LBF_MATH_TAN: Self = Self::LbfMathTan;
  pub const LBF_BIT32_ARSHIFT: Self = Self::LbfBit32Arshift;
  pub const LBF_BIT32_BAND: Self = Self::LbfBit32Band;
  pub const LBF_BIT32_BNOT: Self = Self::LbfBit32Bnot;
  pub const LBF_BIT32_BOR: Self = Self::LbfBit32Bor;
  pub const LBF_BIT32_BXOR: Self = Self::LbfBit32Bxor;
  pub const LBF_BIT32_BTEST: Self = Self::LbfBit32Btest;
  pub const LBF_BIT32_EXTRACT: Self = Self::LbfBit32Extract;
  pub const LBF_BIT32_LROTATE: Self = Self::LbfBit32Lrotate;
  pub const LBF_BIT32_LSHIFT: Self = Self::LbfBit32Lshift;
  pub const LBF_BIT32_REPLACE: Self = Self::LbfBit32Replace;
  pub const LBF_BIT32_RROTATE: Self = Self::LbfBit32Rrotate;
  pub const LBF_BIT32_RSHIFT: Self = Self::LbfBit32Rshift;
  pub const LBF_TYPE: Self = Self::LbfType;
  pub const LBF_STRING_BYTE: Self = Self::LbfStringByte;
  pub const LBF_STRING_CHAR: Self = Self::LbfStringChar;
  pub const LBF_STRING_LEN: Self = Self::LbfStringLen;
  pub const LBF_TYPEOF: Self = Self::LbfTypeof;
  pub const LBF_STRING_SUB: Self = Self::LbfStringSub;
  pub const LBF_MATH_CLAMP: Self = Self::LbfMathClamp;
  pub const LBF_MATH_SIGN: Self = Self::LbfMathSign;
  pub const LBF_MATH_ROUND: Self = Self::LbfMathRound;
  pub const LBF_RAWSET: Self = Self::LbfRawset;
  pub const LBF_RAWGET: Self = Self::LbfRawget;
  pub const LBF_RAWEQUAL: Self = Self::LbfRawequal;
  pub const LBF_TABLE_INSERT: Self = Self::LbfTableInsert;
  pub const LBF_TABLE_UNPACK: Self = Self::LbfTableUnpack;
  pub const LBF_VECTOR: Self = Self::LbfVector;
  pub const LBF_BIT32_COUNTLZ: Self = Self::LbfBit32Countlz;
  pub const LBF_BIT32_COUNTRZ: Self = Self::LbfBit32Countrz;
  pub const LBF_SELECT_VARARG: Self = Self::LbfSelectVararg;
  pub const LBF_RAWLEN: Self = Self::LbfRawlen;
  pub const LBF_BIT32_EXTRACTK: Self = Self::LbfBit32Extractk;
  pub const LBF_GETMETATABLE: Self = Self::LbfGetmetatable;
  pub const LBF_SETMETATABLE: Self = Self::LbfSetmetatable;
  pub const LBF_TONUMBER: Self = Self::LbfTonumber;
  pub const LBF_TOSTRING: Self = Self::LbfTostring;
  pub const LBF_BIT32_BYTESWAP: Self = Self::LbfBit32Byteswap;
  pub const LBF_BUFFER_READI8: Self = Self::LbfBufferReadi8;
  pub const LBF_BUFFER_READU8: Self = Self::LbfBufferReadu8;
  pub const LBF_BUFFER_WRITEU8: Self = Self::LbfBufferWriteu8;
  pub const LBF_BUFFER_READI16: Self = Self::LbfBufferReadi16;
  pub const LBF_BUFFER_READU16: Self = Self::LbfBufferReadu16;
  pub const LBF_BUFFER_WRITEU16: Self = Self::LbfBufferWriteu16;
  pub const LBF_BUFFER_READI32: Self = Self::LbfBufferReadi32;
  pub const LBF_BUFFER_READU32: Self = Self::LbfBufferReadu32;
  pub const LBF_BUFFER_WRITEU32: Self = Self::LbfBufferWriteu32;
  pub const LBF_BUFFER_READF32: Self = Self::LbfBufferReadf32;
  pub const LBF_BUFFER_WRITEF32: Self = Self::LbfBufferWritef32;
  pub const LBF_BUFFER_READF64: Self = Self::LbfBufferReadf64;
  pub const LBF_BUFFER_WRITEF64: Self = Self::LbfBufferWritef64;
  pub const LBF_VECTOR_MAGNITUDE: Self = Self::LbfVectorMagnitude;
  pub const LBF_VECTOR_NORMALIZE: Self = Self::LbfVectorNormalize;
  pub const LBF_VECTOR_CROSS: Self = Self::LbfVectorCross;
  pub const LBF_VECTOR_DOT: Self = Self::LbfVectorDot;
  pub const LBF_VECTOR_FLOOR: Self = Self::LbfVectorFloor;
  pub const LBF_VECTOR_CEIL: Self = Self::LbfVectorCeil;
  pub const LBF_VECTOR_ABS: Self = Self::LbfVectorAbs;
  pub const LBF_VECTOR_SIGN: Self = Self::LbfVectorSign;
  pub const LBF_VECTOR_CLAMP: Self = Self::LbfVectorClamp;
  pub const LBF_VECTOR_MIN: Self = Self::LbfVectorMin;
  pub const LBF_VECTOR_MAX: Self = Self::LbfVectorMax;
  pub const LBF_MATH_LERP: Self = Self::LbfMathLerp;
  pub const LBF_VECTOR_LERP: Self = Self::LbfVectorLerp;
  pub const LBF_MATH_ISNAN: Self = Self::LbfMathIsnan;
  pub const LBF_MATH_ISINF: Self = Self::LbfMathIsinf;
  pub const LBF_MATH_ISFINITE: Self = Self::LbfMathIsfinite;
  pub const LBF_INTEGER_CREATE: Self = Self::LbfIntegerCreate;
  pub const LBF_INTEGER_TONUMBER: Self = Self::LbfIntegerTonumber;
  pub const LBF_INTEGER_NEG: Self = Self::LbfIntegerNeg;
  pub const LBF_INTEGER_ADD: Self = Self::LbfIntegerAdd;
  pub const LBF_INTEGER_SUB: Self = Self::LbfIntegerSub;
  pub const LBF_INTEGER_MUL: Self = Self::LbfIntegerMul;
  pub const LBF_INTEGER_DIV: Self = Self::LbfIntegerDiv;
  pub const LBF_INTEGER_MIN: Self = Self::LbfIntegerMin;
  pub const LBF_INTEGER_MAX: Self = Self::LbfIntegerMax;
  pub const LBF_INTEGER_REM: Self = Self::LbfIntegerRem;
  pub const LBF_INTEGER_IDIV: Self = Self::LbfIntegerIdiv;
  pub const LBF_INTEGER_UDIV: Self = Self::LbfIntegerUdiv;
  pub const LBF_INTEGER_UREM: Self = Self::LbfIntegerUrem;
  pub const LBF_INTEGER_MOD: Self = Self::LbfIntegerMod;
  pub const LBF_INTEGER_CLAMP: Self = Self::LbfIntegerClamp;
  pub const LBF_INTEGER_BAND: Self = Self::LbfIntegerBand;
  pub const LBF_INTEGER_BOR: Self = Self::LbfIntegerBor;
  pub const LBF_INTEGER_BNOT: Self = Self::LbfIntegerBnot;
  pub const LBF_INTEGER_BXOR: Self = Self::LbfIntegerBxor;
  pub const LBF_INTEGER_LT: Self = Self::LbfIntegerLt;
  pub const LBF_INTEGER_LE: Self = Self::LbfIntegerLe;
  pub const LBF_INTEGER_ULT: Self = Self::LbfIntegerUlt;
  pub const LBF_INTEGER_ULE: Self = Self::LbfIntegerUle;
  pub const LBF_INTEGER_GT: Self = Self::LbfIntegerGt;
  pub const LBF_INTEGER_GE: Self = Self::LbfIntegerGe;
  pub const LBF_INTEGER_UGT: Self = Self::LbfIntegerUgt;
  pub const LBF_INTEGER_UGE: Self = Self::LbfIntegerUge;
  pub const LBF_INTEGER_LSHIFT: Self = Self::LbfIntegerLshift;
  pub const LBF_INTEGER_RSHIFT: Self = Self::LbfIntegerRshift;
  pub const LBF_INTEGER_ARSHIFT: Self = Self::LbfIntegerArshift;
  pub const LBF_INTEGER_LROTATE: Self = Self::LbfIntegerLrotate;
  pub const LBF_INTEGER_RROTATE: Self = Self::LbfIntegerRrotate;
  pub const LBF_INTEGER_EXTRACT: Self = Self::LbfIntegerExtract;
  pub const LBF_INTEGER_BTEST: Self = Self::LbfIntegerBtest;
  pub const LBF_INTEGER_COUNTRZ: Self = Self::LbfIntegerCountrz;
  pub const LBF_INTEGER_COUNTLZ: Self = Self::LbfIntegerCountlz;
  pub const LBF_INTEGER_BSWAP: Self = Self::LbfIntegerBswap;
  pub const LBF_BUFFER_READINTEGER: Self = Self::LbfBufferReadinteger;
  pub const LBF_BUFFER_WRITEINTEGER: Self = Self::LbfBufferWriteinteger;
}

pub const LBF_NONE: LuauBuiltinFunction = LuauBuiltinFunction::LbfNone;
pub const LBF_ASSERT: LuauBuiltinFunction = LuauBuiltinFunction::LbfAssert;
pub const LBF_MATH_ABS: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathAbs;
pub const LBF_MATH_ACOS: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathAcos;
pub const LBF_MATH_ASIN: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathAsin;
pub const LBF_MATH_ATAN2: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathAtan2;
pub const LBF_MATH_ATAN: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathAtan;
pub const LBF_MATH_CEIL: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathCeil;
pub const LBF_MATH_COSH: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathCosh;
pub const LBF_MATH_COS: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathCos;
pub const LBF_MATH_DEG: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathDeg;
pub const LBF_MATH_EXP: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathExp;
pub const LBF_MATH_FLOOR: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathFloor;
pub const LBF_MATH_FMOD: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathFmod;
pub const LBF_MATH_FREXP: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathFrexp;
pub const LBF_MATH_LDEXP: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathLdexp;
pub const LBF_MATH_LOG10: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathLog10;
pub const LBF_MATH_LOG: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathLog;
pub const LBF_MATH_MAX: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathMax;
pub const LBF_MATH_MIN: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathMin;
pub const LBF_MATH_MODF: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathModf;
pub const LBF_MATH_POW: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathPow;
pub const LBF_MATH_RAD: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathRad;
pub const LBF_MATH_SINH: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathSinh;
pub const LBF_MATH_SIN: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathSin;
pub const LBF_MATH_SQRT: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathSqrt;
pub const LBF_MATH_TANH: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathTanh;
pub const LBF_MATH_TAN: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathTan;
pub const LBF_BIT32_ARSHIFT: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Arshift;
pub const LBF_BIT32_BAND: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Band;
pub const LBF_BIT32_BNOT: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Bnot;
pub const LBF_BIT32_BOR: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Bor;
pub const LBF_BIT32_BXOR: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Bxor;
pub const LBF_BIT32_BTEST: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Btest;
pub const LBF_BIT32_EXTRACT: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Extract;
pub const LBF_BIT32_LROTATE: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Lrotate;
pub const LBF_BIT32_LSHIFT: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Lshift;
pub const LBF_BIT32_REPLACE: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Replace;
pub const LBF_BIT32_RROTATE: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Rrotate;
pub const LBF_BIT32_RSHIFT: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Rshift;
pub const LBF_TYPE: LuauBuiltinFunction = LuauBuiltinFunction::LbfType;
pub const LBF_STRING_BYTE: LuauBuiltinFunction = LuauBuiltinFunction::LbfStringByte;
pub const LBF_STRING_CHAR: LuauBuiltinFunction = LuauBuiltinFunction::LbfStringChar;
pub const LBF_STRING_LEN: LuauBuiltinFunction = LuauBuiltinFunction::LbfStringLen;
pub const LBF_TYPEOF: LuauBuiltinFunction = LuauBuiltinFunction::LbfTypeof;
pub const LBF_STRING_SUB: LuauBuiltinFunction = LuauBuiltinFunction::LbfStringSub;
pub const LBF_MATH_CLAMP: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathClamp;
pub const LBF_MATH_SIGN: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathSign;
pub const LBF_MATH_ROUND: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathRound;
pub const LBF_RAWSET: LuauBuiltinFunction = LuauBuiltinFunction::LbfRawset;
pub const LBF_RAWGET: LuauBuiltinFunction = LuauBuiltinFunction::LbfRawget;
pub const LBF_RAWEQUAL: LuauBuiltinFunction = LuauBuiltinFunction::LbfRawequal;
pub const LBF_TABLE_INSERT: LuauBuiltinFunction = LuauBuiltinFunction::LbfTableInsert;
pub const LBF_TABLE_UNPACK: LuauBuiltinFunction = LuauBuiltinFunction::LbfTableUnpack;
pub const LBF_VECTOR: LuauBuiltinFunction = LuauBuiltinFunction::LbfVector;
pub const LBF_BIT32_COUNTLZ: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Countlz;
pub const LBF_BIT32_COUNTRZ: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Countrz;
pub const LBF_SELECT_VARARG: LuauBuiltinFunction = LuauBuiltinFunction::LbfSelectVararg;
pub const LBF_RAWLEN: LuauBuiltinFunction = LuauBuiltinFunction::LbfRawlen;
pub const LBF_BIT32_EXTRACTK: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Extractk;
pub const LBF_GETMETATABLE: LuauBuiltinFunction = LuauBuiltinFunction::LbfGetmetatable;
pub const LBF_SETMETATABLE: LuauBuiltinFunction = LuauBuiltinFunction::LbfSetmetatable;
pub const LBF_TONUMBER: LuauBuiltinFunction = LuauBuiltinFunction::LbfTonumber;
pub const LBF_TOSTRING: LuauBuiltinFunction = LuauBuiltinFunction::LbfTostring;
pub const LBF_BIT32_BYTESWAP: LuauBuiltinFunction = LuauBuiltinFunction::LbfBit32Byteswap;
pub const LBF_BUFFER_READI8: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferReadi8;
pub const LBF_BUFFER_READU8: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferReadu8;
pub const LBF_BUFFER_WRITEU8: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferWriteu8;
pub const LBF_BUFFER_READI16: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferReadi16;
pub const LBF_BUFFER_READU16: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferReadu16;
pub const LBF_BUFFER_WRITEU16: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferWriteu16;
pub const LBF_BUFFER_READI32: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferReadi32;
pub const LBF_BUFFER_READU32: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferReadu32;
pub const LBF_BUFFER_WRITEU32: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferWriteu32;
pub const LBF_BUFFER_READF32: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferReadf32;
pub const LBF_BUFFER_WRITEF32: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferWritef32;
pub const LBF_BUFFER_READF64: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferReadf64;
pub const LBF_BUFFER_WRITEF64: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferWritef64;
pub const LBF_VECTOR_MAGNITUDE: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorMagnitude;
pub const LBF_VECTOR_NORMALIZE: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorNormalize;
pub const LBF_VECTOR_CROSS: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorCross;
pub const LBF_VECTOR_DOT: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorDot;
pub const LBF_VECTOR_FLOOR: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorFloor;
pub const LBF_VECTOR_CEIL: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorCeil;
pub const LBF_VECTOR_ABS: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorAbs;
pub const LBF_VECTOR_SIGN: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorSign;
pub const LBF_VECTOR_CLAMP: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorClamp;
pub const LBF_VECTOR_MIN: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorMin;
pub const LBF_VECTOR_MAX: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorMax;
pub const LBF_MATH_LERP: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathLerp;
pub const LBF_VECTOR_LERP: LuauBuiltinFunction = LuauBuiltinFunction::LbfVectorLerp;
pub const LBF_MATH_ISNAN: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathIsnan;
pub const LBF_MATH_ISINF: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathIsinf;
pub const LBF_MATH_ISFINITE: LuauBuiltinFunction = LuauBuiltinFunction::LbfMathIsfinite;
pub const LBF_INTEGER_CREATE: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerCreate;
pub const LBF_INTEGER_TONUMBER: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerTonumber;
pub const LBF_INTEGER_NEG: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerNeg;
pub const LBF_INTEGER_ADD: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerAdd;
pub const LBF_INTEGER_SUB: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerSub;
pub const LBF_INTEGER_MUL: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerMul;
pub const LBF_INTEGER_DIV: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerDiv;
pub const LBF_INTEGER_MIN: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerMin;
pub const LBF_INTEGER_MAX: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerMax;
pub const LBF_INTEGER_REM: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerRem;
pub const LBF_INTEGER_IDIV: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerIdiv;
pub const LBF_INTEGER_UDIV: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerUdiv;
pub const LBF_INTEGER_UREM: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerUrem;
pub const LBF_INTEGER_MOD: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerMod;
pub const LBF_INTEGER_CLAMP: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerClamp;
pub const LBF_INTEGER_BAND: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerBand;
pub const LBF_INTEGER_BOR: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerBor;
pub const LBF_INTEGER_BNOT: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerBnot;
pub const LBF_INTEGER_BXOR: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerBxor;
pub const LBF_INTEGER_LT: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerLt;
pub const LBF_INTEGER_LE: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerLe;
pub const LBF_INTEGER_ULT: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerUlt;
pub const LBF_INTEGER_ULE: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerUle;
pub const LBF_INTEGER_GT: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerGt;
pub const LBF_INTEGER_GE: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerGe;
pub const LBF_INTEGER_UGT: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerUgt;
pub const LBF_INTEGER_UGE: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerUge;
pub const LBF_INTEGER_LSHIFT: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerLshift;
pub const LBF_INTEGER_RSHIFT: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerRshift;
pub const LBF_INTEGER_ARSHIFT: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerArshift;
pub const LBF_INTEGER_LROTATE: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerLrotate;
pub const LBF_INTEGER_RROTATE: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerRrotate;
pub const LBF_INTEGER_EXTRACT: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerExtract;
pub const LBF_INTEGER_BTEST: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerBtest;
pub const LBF_INTEGER_COUNTRZ: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerCountrz;
pub const LBF_INTEGER_COUNTLZ: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerCountlz;
pub const LBF_INTEGER_BSWAP: LuauBuiltinFunction = LuauBuiltinFunction::LbfIntegerBswap;
pub const LBF_BUFFER_READINTEGER: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferReadinteger;
pub const LBF_BUFFER_WRITEINTEGER: LuauBuiltinFunction = LuauBuiltinFunction::LbfBufferWriteinteger;
pub use LuauBuiltinFunction::*;
