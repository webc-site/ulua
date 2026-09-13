use core::ffi::CStr;

use ulua_common::{FFlag, enums::luau_builtin_function::*};

use crate::records::{builtin::Builtin, compile_options::CompileOptions};

pub fn get_builtin_function_id(builtin: &Builtin, options: &CompileOptions) -> i32 {
  if builtin.is_global("assert") {
    return LBF_ASSERT as i32;
  }

  if builtin.is_global("type") {
    return LBF_TYPE as i32;
  }

  if builtin.is_global("typeof") {
    return LBF_TYPEOF as i32;
  }

  if builtin.is_global("rawset") {
    return LBF_RAWSET as i32;
  }
  if builtin.is_global("rawget") {
    return LBF_RAWGET as i32;
  }
  if builtin.is_global("rawequal") {
    return LBF_RAWEQUAL as i32;
  }
  if builtin.is_global("rawlen") {
    return LBF_RAWLEN as i32;
  }

  if builtin.is_global("unpack") {
    return LBF_TABLE_UNPACK as i32;
  }

  if builtin.is_global("select") {
    return LBF_SELECT_VARARG as i32;
  }

  if builtin.is_global("getmetatable") {
    return LBF_GETMETATABLE as i32;
  }
  if builtin.is_global("setmetatable") {
    return LBF_SETMETATABLE as i32;
  }

  if builtin.is_global("tonumber") {
    return LBF_TONUMBER as i32;
  }
  if builtin.is_global("tostring") {
    return LBF_TOSTRING as i32;
  }

  if builtin.object == "math" {
    match builtin.method.as_bytes() {
      b"abs" => return LBF_MATH_ABS as i32,
      b"acos" => return LBF_MATH_ACOS as i32,
      b"asin" => return LBF_MATH_ASIN as i32,
      b"atan2" => return LBF_MATH_ATAN2 as i32,
      b"atan" => return LBF_MATH_ATAN as i32,
      b"ceil" => return LBF_MATH_CEIL as i32,
      b"cosh" => return LBF_MATH_COSH as i32,
      b"cos" => return LBF_MATH_COS as i32,
      b"deg" => return LBF_MATH_DEG as i32,
      b"exp" => return LBF_MATH_EXP as i32,
      b"floor" => return LBF_MATH_FLOOR as i32,
      b"fmod" => return LBF_MATH_FMOD as i32,
      b"frexp" => return LBF_MATH_FREXP as i32,
      b"ldexp" => return LBF_MATH_LDEXP as i32,
      b"log10" => return LBF_MATH_LOG10 as i32,
      b"log" => return LBF_MATH_LOG as i32,
      b"max" => return LBF_MATH_MAX as i32,
      b"min" => return LBF_MATH_MIN as i32,
      b"modf" => return LBF_MATH_MODF as i32,
      b"pow" => return LBF_MATH_POW as i32,
      b"rad" => return LBF_MATH_RAD as i32,
      b"sinh" => return LBF_MATH_SINH as i32,
      b"sin" => return LBF_MATH_SIN as i32,
      b"sqrt" => return LBF_MATH_SQRT as i32,
      b"tanh" => return LBF_MATH_TANH as i32,
      b"tan" => return LBF_MATH_TAN as i32,
      b"clamp" => return LBF_MATH_CLAMP as i32,
      b"sign" => return LBF_MATH_SIGN as i32,
      b"round" => return LBF_MATH_ROUND as i32,
      b"lerp" => return LBF_MATH_LERP as i32,
      b"isnan" => return LBF_MATH_ISNAN as i32,
      b"isinf" => return LBF_MATH_ISINF as i32,
      b"isfinite" => return LBF_MATH_ISFINITE as i32,
      _ => {}
    }
  }

  if builtin.object == "bit32" {
    match builtin.method.as_bytes() {
      b"arshift" => return LBF_BIT32_ARSHIFT as i32,
      b"band" => return LBF_BIT32_BAND as i32,
      b"bnot" => return LBF_BIT32_BNOT as i32,
      b"bor" => return LBF_BIT32_BOR as i32,
      b"bxor" => return LBF_BIT32_BXOR as i32,
      b"btest" => return LBF_BIT32_BTEST as i32,
      b"extract" => return LBF_BIT32_EXTRACT as i32,
      b"lrotate" => return LBF_BIT32_LROTATE as i32,
      b"lshift" => return LBF_BIT32_LSHIFT as i32,
      b"replace" => return LBF_BIT32_REPLACE as i32,
      b"rrotate" => return LBF_BIT32_RROTATE as i32,
      b"rshift" => return LBF_BIT32_RSHIFT as i32,
      b"countlz" => return LBF_BIT32_COUNTLZ as i32,
      b"countrz" => return LBF_BIT32_COUNTRZ as i32,
      b"byteswap" => return LBF_BIT32_BYTESWAP as i32,
      _ => {}
    }
  }

  if builtin.object == "string" {
    match builtin.method.as_bytes() {
      b"byte" => return LBF_STRING_BYTE as i32,
      b"char" => return LBF_STRING_CHAR as i32,
      b"len" => return LBF_STRING_LEN as i32,
      b"sub" => return LBF_STRING_SUB as i32,
      _ => {}
    }
  }

  if builtin.object == "table" {
    match builtin.method.as_bytes() {
      b"insert" => return LBF_TABLE_INSERT as i32,
      b"unpack" => return LBF_TABLE_UNPACK as i32,
      _ => {}
    }
  }

  if builtin.object == "buffer" || builtin.object == "Buffer" {
    match builtin.method.as_bytes() {
      b"readi8" => return LBF_BUFFER_READI8 as i32,
      b"readu8" => return LBF_BUFFER_READU8 as i32,
      b"writei8" | b"writeu8" => return LBF_BUFFER_WRITEU8 as i32,
      b"readi16" => return LBF_BUFFER_READI16 as i32,
      b"readu16" => return LBF_BUFFER_READU16 as i32,
      b"writei16" | b"writeu16" => return LBF_BUFFER_WRITEU16 as i32,
      b"readi32" => return LBF_BUFFER_READI32 as i32,
      b"readu32" => return LBF_BUFFER_READU32 as i32,
      b"writei32" | b"writeu32" => return LBF_BUFFER_WRITEU32 as i32,
      b"readf32" => return LBF_BUFFER_READF32 as i32,
      b"writef32" => return LBF_BUFFER_WRITEF32 as i32,
      b"readf64" => return LBF_BUFFER_READF64 as i32,
      b"writef64" => return LBF_BUFFER_WRITEF64 as i32,
      b"readinteger"
        if FFlag::LuauIntegerFastcalls.get() && FFlag::LuauIntegerBufferFastcalls.get() =>
      {
        return LBF_BUFFER_READINTEGER as i32;
      }
      b"writeinteger"
        if FFlag::LuauIntegerFastcalls.get() && FFlag::LuauIntegerBufferFastcalls.get() =>
      {
        return LBF_BUFFER_WRITEINTEGER as i32;
      }
      _ => {}
    }
  }

  if builtin.object == "vector" {
    match builtin.method.as_bytes() {
      b"create" => return LBF_VECTOR as i32,
      b"magnitude" => return LBF_VECTOR_MAGNITUDE as i32,
      b"normalize" => return LBF_VECTOR_NORMALIZE as i32,
      b"cross" => return LBF_VECTOR_CROSS as i32,
      b"dot" => return LBF_VECTOR_DOT as i32,
      b"floor" => return LBF_VECTOR_FLOOR as i32,
      b"ceil" => return LBF_VECTOR_CEIL as i32,
      b"abs" => return LBF_VECTOR_ABS as i32,
      b"sign" => return LBF_VECTOR_SIGN as i32,
      b"clamp" => return LBF_VECTOR_CLAMP as i32,
      b"min" => return LBF_VECTOR_MIN as i32,
      b"max" => return LBF_VECTOR_MAX as i32,
      b"lerp" => return LBF_VECTOR_LERP as i32,
      _ => {}
    }
  }

  if FFlag::LuauIntegerFastcalls.get() && builtin.object == "integer" {
    match builtin.method.as_bytes() {
      b"add" => return LBF_INTEGER_ADD as i32,
      b"sub" => return LBF_INTEGER_SUB as i32,
      b"mod" => return LBF_INTEGER_MOD as i32,
      b"mul" => return LBF_INTEGER_MUL as i32,
      b"div" => return LBF_INTEGER_DIV as i32,
      b"idiv" => return LBF_INTEGER_IDIV as i32,
      b"udiv" => return LBF_INTEGER_UDIV as i32,
      b"rem" => return LBF_INTEGER_REM as i32,
      b"urem" => return LBF_INTEGER_UREM as i32,
      b"min" => return LBF_INTEGER_MIN as i32,
      b"max" => return LBF_INTEGER_MAX as i32,
      b"neg" => return LBF_INTEGER_NEG as i32,
      b"create" => return LBF_INTEGER_CREATE as i32,
      b"clamp" => return LBF_INTEGER_CLAMP as i32,
      b"band" => return LBF_INTEGER_BAND as i32,
      b"bor" => return LBF_INTEGER_BOR as i32,
      b"bxor" => return LBF_INTEGER_BXOR as i32,
      b"bnot" => return LBF_INTEGER_BNOT as i32,
      b"btest" => return LBF_INTEGER_BTEST as i32,
      b"bswap" => return LBF_INTEGER_BSWAP as i32,
      b"lt" => return LBF_INTEGER_LT as i32,
      b"le" => return LBF_INTEGER_LE as i32,
      b"ult" => return LBF_INTEGER_ULT as i32,
      b"ule" => return LBF_INTEGER_ULE as i32,
      b"gt" => return LBF_INTEGER_GT as i32,
      b"ge" => return LBF_INTEGER_GE as i32,
      b"ugt" => return LBF_INTEGER_UGT as i32,
      b"uge" => return LBF_INTEGER_UGE as i32,
      b"lshift" => return LBF_INTEGER_LSHIFT as i32,
      b"rshift" => return LBF_INTEGER_RSHIFT as i32,
      b"arshift" => return LBF_INTEGER_ARSHIFT as i32,
      b"lrotate" => return LBF_INTEGER_LROTATE as i32,
      b"rrotate" => return LBF_INTEGER_RROTATE as i32,
      b"countrz" => return LBF_INTEGER_COUNTRZ as i32,
      b"countlz" => return LBF_INTEGER_COUNTLZ as i32,
      b"extract" => return LBF_INTEGER_EXTRACT as i32,
      b"tonumber" => return LBF_INTEGER_TONUMBER as i32,
      _ => {}
    }
  }

  if !options.vector_ctor.is_null() {
    let vector_ctor = unsafe { CStr::from_ptr(options.vector_ctor) }
      .to_str()
      .unwrap_or("");
    if !options.vector_lib.is_null() {
      let vector_lib = unsafe { CStr::from_ptr(options.vector_lib) }
        .to_str()
        .unwrap_or("");
      if builtin.is_method(vector_lib, vector_ctor) {
        return LBF_VECTOR as i32;
      }
    } else if builtin.is_global(vector_ctor) {
      return LBF_VECTOR as i32;
    }
  }

  -1
}
