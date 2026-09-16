use ulua_common::enums::{luau_builtin_function::LuauBuiltinFunction, luau_bytecode_type};

use crate::records::bytecode_types::{BytecodeTypes, LBC_TYPE_ANY};
pub const LBC_TYPE_NIL: u8 = luau_bytecode_type::LBC_TYPE_NIL.0 as u8;
pub const LBC_TYPE_BOOLEAN: u8 = luau_bytecode_type::LBC_TYPE_BOOLEAN.0 as u8;
pub const LBC_TYPE_NUMBER: u8 = luau_bytecode_type::LBC_TYPE_NUMBER.0 as u8;
pub const LBC_TYPE_VECTOR: u8 = luau_bytecode_type::LBC_TYPE_VECTOR.0 as u8;
pub const LBC_TYPE_STRING: u8 = luau_bytecode_type::LBC_TYPE_STRING.0 as u8;
pub const LBC_TYPE_TABLE: u8 = luau_bytecode_type::LBC_TYPE_TABLE.0 as u8;
pub const LBC_TYPE_BUFFER: u8 = luau_bytecode_type::LBC_TYPE_BUFFER.0 as u8;
pub const LBC_TYPE_INTEGER: u8 = luau_bytecode_type::LBC_TYPE_INTEGER.0 as u8;

pub fn apply_builtin_call(bfid: LuauBuiltinFunction, types: &mut BytecodeTypes) {
  match bfid {
    LuauBuiltinFunction::LBF_NONE | LuauBuiltinFunction::LBF_ASSERT => {
      types.result = LBC_TYPE_ANY;
    }
    LuauBuiltinFunction::LBF_MATH_ABS
    | LuauBuiltinFunction::LBF_MATH_ACOS
    | LuauBuiltinFunction::LBF_MATH_ASIN => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_ATAN2 => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_ATAN
    | LuauBuiltinFunction::LBF_MATH_CEIL
    | LuauBuiltinFunction::LBF_MATH_COSH
    | LuauBuiltinFunction::LBF_MATH_COS
    | LuauBuiltinFunction::LBF_MATH_DEG
    | LuauBuiltinFunction::LBF_MATH_EXP
    | LuauBuiltinFunction::LBF_MATH_FLOOR => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_FMOD => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_FREXP => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_LDEXP => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_LOG10 => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_LOG => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER; // We can mark optional arguments
    }
    LuauBuiltinFunction::LBF_MATH_MAX | LuauBuiltinFunction::LBF_MATH_MIN => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER; // We can mark optional arguments
    }
    LuauBuiltinFunction::LBF_MATH_MODF => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_POW => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_RAD
    | LuauBuiltinFunction::LBF_MATH_SINH
    | LuauBuiltinFunction::LBF_MATH_SIN
    | LuauBuiltinFunction::LBF_MATH_SQRT
    | LuauBuiltinFunction::LBF_MATH_TANH
    | LuauBuiltinFunction::LBF_MATH_TAN => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BIT32_ARSHIFT => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BIT32_BAND => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER; // We can mark optional arguments
    }
    LuauBuiltinFunction::LBF_BIT32_BNOT => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BIT32_BOR
    | LuauBuiltinFunction::LBF_BIT32_BXOR
    | LuauBuiltinFunction::LBF_BIT32_BTEST
    | LuauBuiltinFunction::LBF_BIT32_EXTRACT => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER; // We can mark optional arguments
    }
    LuauBuiltinFunction::LBF_BIT32_LROTATE | LuauBuiltinFunction::LBF_BIT32_LSHIFT => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BIT32_REPLACE => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER; // We can mark optional arguments
    }
    LuauBuiltinFunction::LBF_BIT32_RROTATE | LuauBuiltinFunction::LBF_BIT32_RSHIFT => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_TYPE => {
      types.result = LBC_TYPE_STRING;
    }
    LuauBuiltinFunction::LBF_STRING_BYTE => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_STRING;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_STRING_CHAR => {
      types.result = LBC_TYPE_STRING;
      // We can mark optional arguments
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_STRING_LEN => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_STRING;
    }
    LuauBuiltinFunction::LBF_TYPEOF => {
      types.result = LBC_TYPE_STRING;
    }
    LuauBuiltinFunction::LBF_STRING_SUB => {
      types.result = LBC_TYPE_STRING;
      types.a = LBC_TYPE_STRING;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_CLAMP => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_SIGN => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_ROUND => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_RAWGET => {
      types.result = LBC_TYPE_ANY;
      types.a = LBC_TYPE_TABLE;
    }
    LuauBuiltinFunction::LBF_RAWEQUAL => {
      types.result = LBC_TYPE_BOOLEAN;
    }
    LuauBuiltinFunction::LBF_TABLE_UNPACK => {
      types.result = LBC_TYPE_ANY;
      types.a = LBC_TYPE_TABLE;
      types.b = LBC_TYPE_NUMBER; // We can mark optional arguments
    }
    LuauBuiltinFunction::LBF_VECTOR => {
      types.result = LBC_TYPE_VECTOR;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BIT32_COUNTLZ | LuauBuiltinFunction::LBF_BIT32_COUNTRZ => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_SELECT_VARARG => {
      types.result = LBC_TYPE_ANY;
    }
    LuauBuiltinFunction::LBF_RAWLEN => {
      types.result = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BIT32_EXTRACTK => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_GETMETATABLE => {
      types.result = LBC_TYPE_TABLE;
    }
    LuauBuiltinFunction::LBF_TONUMBER => {
      types.result = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_TOSTRING => {
      types.result = LBC_TYPE_STRING;
    }
    LuauBuiltinFunction::LBF_BIT32_BYTESWAP => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_READI8 | LuauBuiltinFunction::LBF_BUFFER_READU8 => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_WRITEU8 => {
      types.result = LBC_TYPE_NIL;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_READI16 | LuauBuiltinFunction::LBF_BUFFER_READU16 => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_WRITEU16 => {
      types.result = LBC_TYPE_NIL;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_READI32 | LuauBuiltinFunction::LBF_BUFFER_READU32 => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_WRITEU32 => {
      types.result = LBC_TYPE_NIL;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_READF32 => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_WRITEF32 => {
      types.result = LBC_TYPE_NIL;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_READF64 => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_WRITEF64 => {
      types.result = LBC_TYPE_NIL;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_READINTEGER => {
      types.result = LBC_TYPE_INTEGER;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_BUFFER_WRITEINTEGER => {
      types.result = LBC_TYPE_NIL;
      types.a = LBC_TYPE_BUFFER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_INTEGER;
    }
    LuauBuiltinFunction::LBF_TABLE_INSERT => {
      types.result = LBC_TYPE_NIL;
      types.a = LBC_TYPE_TABLE;
    }
    LuauBuiltinFunction::LBF_RAWSET => {
      types.result = LBC_TYPE_ANY;
      types.a = LBC_TYPE_TABLE;
    }
    LuauBuiltinFunction::LBF_SETMETATABLE => {
      types.result = LBC_TYPE_TABLE;
      types.a = LBC_TYPE_TABLE;
      types.b = LBC_TYPE_TABLE;
    }
    LuauBuiltinFunction::LBF_VECTOR_MAGNITUDE => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_VECTOR;
    }
    LuauBuiltinFunction::LBF_VECTOR_NORMALIZE => {
      types.result = LBC_TYPE_VECTOR;
      types.a = LBC_TYPE_VECTOR;
    }
    LuauBuiltinFunction::LBF_VECTOR_CROSS => {
      types.result = LBC_TYPE_VECTOR;
      types.a = LBC_TYPE_VECTOR;
      types.b = LBC_TYPE_VECTOR;
    }
    LuauBuiltinFunction::LBF_VECTOR_DOT => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_VECTOR;
      types.b = LBC_TYPE_VECTOR;
    }
    LuauBuiltinFunction::LBF_VECTOR_FLOOR
    | LuauBuiltinFunction::LBF_VECTOR_CEIL
    | LuauBuiltinFunction::LBF_VECTOR_ABS
    | LuauBuiltinFunction::LBF_VECTOR_SIGN
    | LuauBuiltinFunction::LBF_VECTOR_CLAMP => {
      types.result = LBC_TYPE_VECTOR;
      types.a = LBC_TYPE_VECTOR;
      types.b = LBC_TYPE_VECTOR;
    }
    LuauBuiltinFunction::LBF_VECTOR_MIN | LuauBuiltinFunction::LBF_VECTOR_MAX => {
      types.result = LBC_TYPE_VECTOR;
      types.a = LBC_TYPE_VECTOR;
      types.b = LBC_TYPE_VECTOR;
      types.c = LBC_TYPE_VECTOR; // We can mark optional arguments
    }
    LuauBuiltinFunction::LBF_VECTOR_LERP => {
      types.result = LBC_TYPE_VECTOR;
      types.a = LBC_TYPE_VECTOR;
      types.b = LBC_TYPE_VECTOR;
      types.c = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_LERP => {
      types.result = LBC_TYPE_NUMBER;
      types.a = LBC_TYPE_NUMBER;
      types.b = LBC_TYPE_NUMBER;
      types.c = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_ISNAN => {
      types.result = LBC_TYPE_BOOLEAN;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_ISINF => {
      types.result = LBC_TYPE_BOOLEAN;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_MATH_ISFINITE => {
      types.result = LBC_TYPE_BOOLEAN;
      types.a = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_INTEGER_NEG
    | LuauBuiltinFunction::LBF_INTEGER_BSWAP
    | LuauBuiltinFunction::LBF_INTEGER_BNOT
    | LuauBuiltinFunction::LBF_INTEGER_COUNTLZ
    | LuauBuiltinFunction::LBF_INTEGER_COUNTRZ => {
      types.result = LBC_TYPE_INTEGER;
      types.a = LBC_TYPE_INTEGER;
    }
    LuauBuiltinFunction::LBF_INTEGER_MIN
    | LuauBuiltinFunction::LBF_INTEGER_MAX
    | LuauBuiltinFunction::LBF_INTEGER_BAND
    | LuauBuiltinFunction::LBF_INTEGER_BOR
    | LuauBuiltinFunction::LBF_INTEGER_BXOR => {
      types.a = LBC_TYPE_INTEGER;
      types.b = LBC_TYPE_INTEGER;
      types.c = LBC_TYPE_INTEGER; // We can mark optional arguments
      types.result = LBC_TYPE_INTEGER;
    }
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
    | LuauBuiltinFunction::LBF_INTEGER_ARSHIFT => {
      types.a = LBC_TYPE_INTEGER;
      types.b = LBC_TYPE_INTEGER;
      types.result = LBC_TYPE_INTEGER;
    }
    LuauBuiltinFunction::LBF_INTEGER_CLAMP | LuauBuiltinFunction::LBF_INTEGER_EXTRACT => {
      types.a = LBC_TYPE_INTEGER;
      types.b = LBC_TYPE_INTEGER;
      types.c = LBC_TYPE_INTEGER;
      types.result = LBC_TYPE_INTEGER;
    }
    LuauBuiltinFunction::LBF_INTEGER_BTEST => {
      types.a = LBC_TYPE_INTEGER;
      types.b = LBC_TYPE_INTEGER;
      types.c = LBC_TYPE_INTEGER; // We can mark optional arguments
      types.result = LBC_TYPE_BOOLEAN;
    }
    LuauBuiltinFunction::LBF_INTEGER_LT
    | LuauBuiltinFunction::LBF_INTEGER_LE
    | LuauBuiltinFunction::LBF_INTEGER_GT
    | LuauBuiltinFunction::LBF_INTEGER_GE
    | LuauBuiltinFunction::LBF_INTEGER_ULT
    | LuauBuiltinFunction::LBF_INTEGER_ULE
    | LuauBuiltinFunction::LBF_INTEGER_UGT
    | LuauBuiltinFunction::LBF_INTEGER_UGE => {
      types.a = LBC_TYPE_INTEGER;
      types.b = LBC_TYPE_INTEGER;
      types.result = LBC_TYPE_BOOLEAN;
    }
    LuauBuiltinFunction::LBF_INTEGER_TONUMBER => {
      types.a = LBC_TYPE_INTEGER;
      types.result = LBC_TYPE_NUMBER;
    }
    LuauBuiltinFunction::LBF_INTEGER_CREATE => {
      types.a = LBC_TYPE_NUMBER;
      types.result = LBC_TYPE_INTEGER;
    }
  }
}
