use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;

use crate::records::const_prop_state::ConstPropState;

pub fn handle_builtin_effects(
  state: &mut ConstPropState,
  bfid: LuauBuiltinFunction,
  first_return_reg: u32,
  _nresults: i32,
) {
  // Switch over all values is used to force new items to be handled
  match bfid {
    LuauBuiltinFunction::LBF_NONE
    | LuauBuiltinFunction::LBF_ASSERT
    | LuauBuiltinFunction::LBF_MATH_ABS
    | LuauBuiltinFunction::LBF_MATH_ACOS
    | LuauBuiltinFunction::LBF_MATH_ASIN
    | LuauBuiltinFunction::LBF_MATH_ATAN2
    | LuauBuiltinFunction::LBF_MATH_ATAN
    | LuauBuiltinFunction::LBF_MATH_CEIL
    | LuauBuiltinFunction::LBF_MATH_COSH
    | LuauBuiltinFunction::LBF_MATH_COS
    | LuauBuiltinFunction::LBF_MATH_DEG
    | LuauBuiltinFunction::LBF_MATH_EXP
    | LuauBuiltinFunction::LBF_MATH_FLOOR
    | LuauBuiltinFunction::LBF_MATH_FMOD
    | LuauBuiltinFunction::LBF_MATH_FREXP
    | LuauBuiltinFunction::LBF_MATH_LDEXP
    | LuauBuiltinFunction::LBF_MATH_LOG10
    | LuauBuiltinFunction::LBF_MATH_LOG
    | LuauBuiltinFunction::LBF_MATH_MAX
    | LuauBuiltinFunction::LBF_MATH_MIN
    | LuauBuiltinFunction::LBF_MATH_MODF
    | LuauBuiltinFunction::LBF_MATH_POW
    | LuauBuiltinFunction::LBF_MATH_RAD
    | LuauBuiltinFunction::LBF_MATH_SINH
    | LuauBuiltinFunction::LBF_MATH_SIN
    | LuauBuiltinFunction::LBF_MATH_SQRT
    | LuauBuiltinFunction::LBF_MATH_TANH
    | LuauBuiltinFunction::LBF_MATH_TAN
    | LuauBuiltinFunction::LBF_BIT32_ARSHIFT
    | LuauBuiltinFunction::LBF_BIT32_BAND
    | LuauBuiltinFunction::LBF_BIT32_BNOT
    | LuauBuiltinFunction::LBF_BIT32_BOR
    | LuauBuiltinFunction::LBF_BIT32_BXOR
    | LuauBuiltinFunction::LBF_BIT32_BTEST
    | LuauBuiltinFunction::LBF_BIT32_EXTRACT
    | LuauBuiltinFunction::LBF_BIT32_LROTATE
    | LuauBuiltinFunction::LBF_BIT32_LSHIFT
    | LuauBuiltinFunction::LBF_BIT32_REPLACE
    | LuauBuiltinFunction::LBF_BIT32_RROTATE
    | LuauBuiltinFunction::LBF_BIT32_RSHIFT
    | LuauBuiltinFunction::LBF_TYPE
    | LuauBuiltinFunction::LBF_STRING_BYTE
    | LuauBuiltinFunction::LBF_STRING_CHAR
    | LuauBuiltinFunction::LBF_STRING_LEN
    | LuauBuiltinFunction::LBF_TYPEOF
    | LuauBuiltinFunction::LBF_STRING_SUB
    | LuauBuiltinFunction::LBF_MATH_CLAMP
    | LuauBuiltinFunction::LBF_MATH_SIGN
    | LuauBuiltinFunction::LBF_MATH_ROUND
    | LuauBuiltinFunction::LBF_RAWGET
    | LuauBuiltinFunction::LBF_RAWEQUAL
    | LuauBuiltinFunction::LBF_TABLE_UNPACK
    | LuauBuiltinFunction::LBF_VECTOR
    | LuauBuiltinFunction::LBF_BIT32_COUNTLZ
    | LuauBuiltinFunction::LBF_BIT32_COUNTRZ
    | LuauBuiltinFunction::LBF_SELECT_VARARG
    | LuauBuiltinFunction::LBF_RAWLEN
    | LuauBuiltinFunction::LBF_BIT32_EXTRACTK
    | LuauBuiltinFunction::LBF_GETMETATABLE
    | LuauBuiltinFunction::LBF_TONUMBER
    | LuauBuiltinFunction::LBF_TOSTRING
    | LuauBuiltinFunction::LBF_BIT32_BYTESWAP
    | LuauBuiltinFunction::LBF_BUFFER_READI8
    | LuauBuiltinFunction::LBF_BUFFER_READU8
    | LuauBuiltinFunction::LBF_BUFFER_READI16
    | LuauBuiltinFunction::LBF_BUFFER_READU16
    | LuauBuiltinFunction::LBF_BUFFER_READI32
    | LuauBuiltinFunction::LBF_BUFFER_READU32
    | LuauBuiltinFunction::LBF_BUFFER_READF32
    | LuauBuiltinFunction::LBF_BUFFER_READF64
    | LuauBuiltinFunction::LBF_BUFFER_READINTEGER
    | LuauBuiltinFunction::LBF_VECTOR_MAGNITUDE
    | LuauBuiltinFunction::LBF_VECTOR_NORMALIZE
    | LuauBuiltinFunction::LBF_VECTOR_CROSS
    | LuauBuiltinFunction::LBF_VECTOR_DOT
    | LuauBuiltinFunction::LBF_VECTOR_FLOOR
    | LuauBuiltinFunction::LBF_VECTOR_CEIL
    | LuauBuiltinFunction::LBF_VECTOR_ABS
    | LuauBuiltinFunction::LBF_VECTOR_SIGN
    | LuauBuiltinFunction::LBF_VECTOR_CLAMP
    | LuauBuiltinFunction::LBF_VECTOR_MIN
    | LuauBuiltinFunction::LBF_VECTOR_MAX
    | LuauBuiltinFunction::LBF_VECTOR_LERP
    | LuauBuiltinFunction::LBF_MATH_LERP
    | LuauBuiltinFunction::LBF_MATH_ISNAN
    | LuauBuiltinFunction::LBF_MATH_ISINF
    | LuauBuiltinFunction::LBF_MATH_ISFINITE
    | LuauBuiltinFunction::LBF_INTEGER_ADD
    | LuauBuiltinFunction::LBF_INTEGER_MUL
    | LuauBuiltinFunction::LBF_INTEGER_IDIV
    | LuauBuiltinFunction::LBF_INTEGER_LT
    | LuauBuiltinFunction::LBF_INTEGER_CREATE
    | LuauBuiltinFunction::LBF_INTEGER_MOD
    | LuauBuiltinFunction::LBF_INTEGER_SUB
    | LuauBuiltinFunction::LBF_INTEGER_LE
    | LuauBuiltinFunction::LBF_INTEGER_GT
    | LuauBuiltinFunction::LBF_INTEGER_GE
    | LuauBuiltinFunction::LBF_INTEGER_ULT
    | LuauBuiltinFunction::LBF_INTEGER_ULE
    | LuauBuiltinFunction::LBF_INTEGER_UGT
    | LuauBuiltinFunction::LBF_INTEGER_UGE
    | LuauBuiltinFunction::LBF_INTEGER_DIV
    | LuauBuiltinFunction::LBF_INTEGER_NEG
    | LuauBuiltinFunction::LBF_INTEGER_BSWAP
    | LuauBuiltinFunction::LBF_INTEGER_MIN
    | LuauBuiltinFunction::LBF_INTEGER_MAX
    | LuauBuiltinFunction::LBF_INTEGER_REM
    | LuauBuiltinFunction::LBF_INTEGER_UDIV
    | LuauBuiltinFunction::LBF_INTEGER_UREM
    | LuauBuiltinFunction::LBF_INTEGER_BAND
    | LuauBuiltinFunction::LBF_INTEGER_BOR
    | LuauBuiltinFunction::LBF_INTEGER_BNOT
    | LuauBuiltinFunction::LBF_INTEGER_BXOR
    | LuauBuiltinFunction::LBF_INTEGER_BTEST
    | LuauBuiltinFunction::LBF_INTEGER_COUNTRZ
    | LuauBuiltinFunction::LBF_INTEGER_COUNTLZ
    | LuauBuiltinFunction::LBF_INTEGER_LSHIFT
    | LuauBuiltinFunction::LBF_INTEGER_RSHIFT
    | LuauBuiltinFunction::LBF_INTEGER_ARSHIFT
    | LuauBuiltinFunction::LBF_INTEGER_LROTATE
    | LuauBuiltinFunction::LBF_INTEGER_RROTATE
    | LuauBuiltinFunction::LBF_INTEGER_CLAMP
    | LuauBuiltinFunction::LBF_INTEGER_EXTRACT
    | LuauBuiltinFunction::LBF_INTEGER_TONUMBER => {}
    LuauBuiltinFunction::LBF_BUFFER_WRITEU8
    | LuauBuiltinFunction::LBF_BUFFER_WRITEU16
    | LuauBuiltinFunction::LBF_BUFFER_WRITEU32
    | LuauBuiltinFunction::LBF_BUFFER_WRITEF32
    | LuauBuiltinFunction::LBF_BUFFER_WRITEF64
    | LuauBuiltinFunction::LBF_BUFFER_WRITEINTEGER => {
      state.invalidate_heap_buffer_data();
    }
    LuauBuiltinFunction::LBF_TABLE_INSERT => {
      state.invalidate_heap();
      return; // table.insert does not modify result registers.
    }
    LuauBuiltinFunction::LBF_RAWSET => {
      state.invalidate_heap();
    }
    LuauBuiltinFunction::LBF_SETMETATABLE => {
      state.invalidate_heap();
    }
  }

  // TODO: classify further using switch above, some fastcalls only modify the value, not the tag
  // TODO: fastcalls are different from calls and it might be possible to not invalidate all register starting from return
  state.invalidate_registers_from(first_return_reg as i32);
}
