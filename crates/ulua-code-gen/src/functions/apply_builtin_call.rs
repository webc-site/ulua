use ulua_common::enums::{
  luau_builtin_function::LuauBuiltinFunction, luau_bytecode_type::LuauBytecodeType,
};

use crate::records::bytecode_types::{BytecodeTypes, LBC_TYPE_ANY};

pub const LBC_TYPE_NIL: u8 = LuauBytecodeType::LBC_TYPE_NIL.0 as u8;
pub const LBC_TYPE_BOOLEAN: u8 = LuauBytecodeType::LBC_TYPE_BOOLEAN.0 as u8;
pub const LBC_TYPE_NUMBER: u8 = LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8;
pub const LBC_TYPE_VECTOR: u8 = LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8;
pub const LBC_TYPE_STRING: u8 = LuauBytecodeType::LBC_TYPE_STRING.0 as u8;
pub const LBC_TYPE_TABLE: u8 = LuauBytecodeType::LBC_TYPE_TABLE.0 as u8;
pub const LBC_TYPE_BUFFER: u8 = LuauBytecodeType::LBC_TYPE_BUFFER.0 as u8;
pub const LBC_TYPE_INTEGER: u8 = LuauBytecodeType::LBC_TYPE_INTEGER.0 as u8;

const A: u8 = LBC_TYPE_ANY;
const N: u8 = LBC_TYPE_NUMBER;
const S: u8 = LBC_TYPE_STRING;
const T: u8 = LBC_TYPE_TABLE;
const B: u8 = LBC_TYPE_BUFFER;
const I: u8 = LBC_TYPE_INTEGER;
const V: u8 = LBC_TYPE_VECTOR;
const L: u8 = LBC_TYPE_BOOLEAN;
const Z: u8 = LBC_TYPE_NIL;

#[inline]
pub const fn builtin_bytecode_types(bfid: LuauBuiltinFunction) -> BytecodeTypes {
  let (result, a, b, c) = match bfid {
    // 标量一元数学 / 位取反 / 位统计
    LuauBuiltinFunction::LBF_MATH_ABS
    | LuauBuiltinFunction::LBF_MATH_ACOS
    | LuauBuiltinFunction::LBF_MATH_ASIN
    | LuauBuiltinFunction::LBF_MATH_ATAN
    | LuauBuiltinFunction::LBF_MATH_CEIL
    | LuauBuiltinFunction::LBF_MATH_COSH
    | LuauBuiltinFunction::LBF_MATH_COS
    | LuauBuiltinFunction::LBF_MATH_DEG
    | LuauBuiltinFunction::LBF_MATH_EXP
    | LuauBuiltinFunction::LBF_MATH_FLOOR
    | LuauBuiltinFunction::LBF_MATH_FREXP
    | LuauBuiltinFunction::LBF_MATH_LOG10
    | LuauBuiltinFunction::LBF_MATH_MODF
    | LuauBuiltinFunction::LBF_MATH_RAD
    | LuauBuiltinFunction::LBF_MATH_SINH
    | LuauBuiltinFunction::LBF_MATH_SIN
    | LuauBuiltinFunction::LBF_MATH_SQRT
    | LuauBuiltinFunction::LBF_MATH_TANH
    | LuauBuiltinFunction::LBF_MATH_TAN
    | LuauBuiltinFunction::LBF_MATH_SIGN
    | LuauBuiltinFunction::LBF_MATH_ROUND
    | LuauBuiltinFunction::LBF_BIT32_BNOT
    | LuauBuiltinFunction::LBF_BIT32_COUNTLZ
    | LuauBuiltinFunction::LBF_BIT32_COUNTRZ
    | LuauBuiltinFunction::LBF_BIT32_BYTESWAP => (N, N, A, A),

    // 标量二元数学 / 位移
    LuauBuiltinFunction::LBF_MATH_ATAN2
    | LuauBuiltinFunction::LBF_MATH_FMOD
    | LuauBuiltinFunction::LBF_MATH_LDEXP
    | LuauBuiltinFunction::LBF_MATH_LOG
    | LuauBuiltinFunction::LBF_MATH_POW
    | LuauBuiltinFunction::LBF_BIT32_ARSHIFT
    | LuauBuiltinFunction::LBF_BIT32_LROTATE
    | LuauBuiltinFunction::LBF_BIT32_LSHIFT
    | LuauBuiltinFunction::LBF_BIT32_RROTATE
    | LuauBuiltinFunction::LBF_BIT32_RSHIFT
    | LuauBuiltinFunction::LBF_BIT32_EXTRACTK => (N, N, N, A),

    // 标量三/四元数学 / 位逻辑
    LuauBuiltinFunction::LBF_MATH_MAX
    | LuauBuiltinFunction::LBF_MATH_MIN
    | LuauBuiltinFunction::LBF_MATH_CLAMP
    | LuauBuiltinFunction::LBF_MATH_LERP
    | LuauBuiltinFunction::LBF_BIT32_BAND
    | LuauBuiltinFunction::LBF_BIT32_BOR
    | LuauBuiltinFunction::LBF_BIT32_BXOR
    | LuauBuiltinFunction::LBF_BIT32_BTEST
    | LuauBuiltinFunction::LBF_BIT32_EXTRACT
    | LuauBuiltinFunction::LBF_BIT32_REPLACE => (N, N, N, N),

    // 字符串函数
    LuauBuiltinFunction::LBF_TYPE
    | LuauBuiltinFunction::LBF_TYPEOF
    | LuauBuiltinFunction::LBF_TOSTRING => (S, A, A, A),
    LuauBuiltinFunction::LBF_STRING_BYTE => (N, S, N, A),
    LuauBuiltinFunction::LBF_STRING_CHAR => (S, N, N, N),
    LuauBuiltinFunction::LBF_STRING_LEN => (N, S, A, A),
    LuauBuiltinFunction::LBF_STRING_SUB => (S, S, N, N),

    // 表与元表
    LuauBuiltinFunction::LBF_RAWGET | LuauBuiltinFunction::LBF_RAWSET => (A, T, A, A),
    LuauBuiltinFunction::LBF_RAWEQUAL => (L, A, A, A),
    LuauBuiltinFunction::LBF_TABLE_UNPACK => (A, T, N, A),
    LuauBuiltinFunction::LBF_TABLE_INSERT => (Z, T, A, A),
    LuauBuiltinFunction::LBF_GETMETATABLE => (T, A, A, A),
    LuauBuiltinFunction::LBF_SETMETATABLE => (T, T, T, A),

    // 类型转换 / 检查
    LuauBuiltinFunction::LBF_RAWLEN | LuauBuiltinFunction::LBF_TONUMBER => (N, A, A, A),
    LuauBuiltinFunction::LBF_MATH_ISNAN
    | LuauBuiltinFunction::LBF_MATH_ISINF
    | LuauBuiltinFunction::LBF_MATH_ISFINITE => (L, N, A, A),

    // 向量函数
    LuauBuiltinFunction::LBF_VECTOR => (V, N, N, N),
    LuauBuiltinFunction::LBF_VECTOR_MAGNITUDE => (N, V, A, A),
    LuauBuiltinFunction::LBF_VECTOR_NORMALIZE => (V, V, A, A),
    LuauBuiltinFunction::LBF_VECTOR_DOT => (N, V, V, A),
    LuauBuiltinFunction::LBF_VECTOR_CROSS
    | LuauBuiltinFunction::LBF_VECTOR_FLOOR
    | LuauBuiltinFunction::LBF_VECTOR_CEIL
    | LuauBuiltinFunction::LBF_VECTOR_ABS
    | LuauBuiltinFunction::LBF_VECTOR_SIGN
    | LuauBuiltinFunction::LBF_VECTOR_CLAMP => (V, V, V, A),
    LuauBuiltinFunction::LBF_VECTOR_MIN | LuauBuiltinFunction::LBF_VECTOR_MAX => (V, V, V, V),
    LuauBuiltinFunction::LBF_VECTOR_LERP => (V, V, V, N),

    // Buffer 读写
    LuauBuiltinFunction::LBF_BUFFER_READI8
    | LuauBuiltinFunction::LBF_BUFFER_READU8
    | LuauBuiltinFunction::LBF_BUFFER_READI16
    | LuauBuiltinFunction::LBF_BUFFER_READU16
    | LuauBuiltinFunction::LBF_BUFFER_READI32
    | LuauBuiltinFunction::LBF_BUFFER_READU32
    | LuauBuiltinFunction::LBF_BUFFER_READF32
    | LuauBuiltinFunction::LBF_BUFFER_READF64 => (N, B, N, A),
    LuauBuiltinFunction::LBF_BUFFER_WRITEU8
    | LuauBuiltinFunction::LBF_BUFFER_WRITEU16
    | LuauBuiltinFunction::LBF_BUFFER_WRITEU32
    | LuauBuiltinFunction::LBF_BUFFER_WRITEF32
    | LuauBuiltinFunction::LBF_BUFFER_WRITEF64 => (Z, B, N, N),
    LuauBuiltinFunction::LBF_BUFFER_READINTEGER => (I, B, N, A),
    LuauBuiltinFunction::LBF_BUFFER_WRITEINTEGER => (Z, B, N, I),

    // 64 位整数运算
    LuauBuiltinFunction::LBF_INTEGER_NEG
    | LuauBuiltinFunction::LBF_INTEGER_BSWAP
    | LuauBuiltinFunction::LBF_INTEGER_BNOT
    | LuauBuiltinFunction::LBF_INTEGER_COUNTLZ
    | LuauBuiltinFunction::LBF_INTEGER_COUNTRZ => (I, I, A, A),
    LuauBuiltinFunction::LBF_INTEGER_ADD
    | LuauBuiltinFunction::LBF_INTEGER_SUB
    | LuauBuiltinFunction::LBF_INTEGER_MUL
    | LuauBuiltinFunction::LBF_INTEGER_DIV
    | LuauBuiltinFunction::LBF_INTEGER_IDIV
    | LuauBuiltinFunction::LBF_INTEGER_REM
    | LuauBuiltinFunction::LBF_INTEGER_UDIV
    | LuauBuiltinFunction::LBF_INTEGER_UREM
    | LuauBuiltinFunction::LBF_INTEGER_MOD
    | LuauBuiltinFunction::LBF_INTEGER_LSHIFT
    | LuauBuiltinFunction::LBF_INTEGER_LROTATE
    | LuauBuiltinFunction::LBF_INTEGER_RROTATE
    | LuauBuiltinFunction::LBF_INTEGER_RSHIFT
    | LuauBuiltinFunction::LBF_INTEGER_ARSHIFT => (I, I, I, A),
    LuauBuiltinFunction::LBF_INTEGER_MIN
    | LuauBuiltinFunction::LBF_INTEGER_MAX
    | LuauBuiltinFunction::LBF_INTEGER_BAND
    | LuauBuiltinFunction::LBF_INTEGER_BOR
    | LuauBuiltinFunction::LBF_INTEGER_BXOR
    | LuauBuiltinFunction::LBF_INTEGER_CLAMP
    | LuauBuiltinFunction::LBF_INTEGER_EXTRACT => (I, I, I, I),
    LuauBuiltinFunction::LBF_INTEGER_BTEST => (L, I, I, I),
    LuauBuiltinFunction::LBF_INTEGER_LT
    | LuauBuiltinFunction::LBF_INTEGER_LE
    | LuauBuiltinFunction::LBF_INTEGER_GT
    | LuauBuiltinFunction::LBF_INTEGER_GE
    | LuauBuiltinFunction::LBF_INTEGER_ULT
    | LuauBuiltinFunction::LBF_INTEGER_ULE
    | LuauBuiltinFunction::LBF_INTEGER_UGT
    | LuauBuiltinFunction::LBF_INTEGER_UGE => (L, I, I, A),
    LuauBuiltinFunction::LBF_INTEGER_TONUMBER => (N, I, A, A),
    LuauBuiltinFunction::LBF_INTEGER_CREATE => (I, N, A, A),

    _ => (A, A, A, A),
  };
  BytecodeTypes { result, a, b, c }
}

#[inline]
pub fn apply_builtin_call(bfid: LuauBuiltinFunction, types: &mut BytecodeTypes) {
  *types = builtin_bytecode_types(bfid);
}
