use ulua_common::{enums::luau_builtin_function::LuauBuiltinFunction as LBF, fflag};
use ulua_vm::{enums::lua_type::LuaType, macros::lua_multret::LUA_MULTRET};

use crate::{
  enums::{
    int_64_binary::Int64Binary, ir_cmd::IrCmd, ir_condition::IrCondition,
    ir_const_kind::IrConstKind, ir_op_kind::IrOpKind,
  },
  functions::{
    is_compatible_constant::is_compatible_constant,
    translate_builtin_2_number_to_number_libm::translate_builtin_2_number_to_number_libm,
    translate_builtin_assert::translate_builtin_assert,
    translate_builtin_bit_32_extract::translate_builtin_bit_32_extract,
    translate_builtin_bit_32_extract_k::translate_builtin_bit_32_extract_k,
    translate_builtin_bit_32_multiarg_op::translate_builtin_bit_32_multiarg_op,
    translate_builtin_bit_32_replace::translate_builtin_bit_32_replace,
    translate_builtin_bit_32_rotate::translate_builtin_bit_32_rotate,
    translate_builtin_bit_32_shift::translate_builtin_bit_32_shift,
    translate_builtin_bit_32_unary::translate_builtin_bit_32_unary,
    translate_builtin_buffer_read::translate_builtin_buffer_read,
    translate_builtin_buffer_write::translate_builtin_buffer_write,
    translate_builtin_int_64_binary::translate_builtin_int_64_binary,
    translate_builtin_int_64_clamp::translate_builtin_int_64_clamp,
    translate_builtin_int_64_compare::translate_builtin_int_64_compare,
    translate_builtin_int_64_create::translate_builtin_int_64_create,
    translate_builtin_int_64_extract::translate_builtin_int_64_extract,
    translate_builtin_int_64_min_max::translate_builtin_int_64_min_max,
    translate_builtin_int_64_multiarg_op::translate_builtin_int_64_multiarg_op,
    translate_builtin_int_64_neg::translate_builtin_int_64_neg,
    translate_builtin_int_64_shift_rotate::translate_builtin_int_64_shift_rotate,
    translate_builtin_int_64_to_number::translate_builtin_int_64_to_number,
    translate_builtin_int_64_unary::translate_builtin_int_64_unary,
    translate_builtin_math_clamp::translate_builtin_math_clamp,
    translate_builtin_math_deg_rad::translate_builtin_math_deg_rad,
    translate_builtin_math_is_nan::translate_builtin_math_is_nan,
    translate_builtin_math_lerp::translate_builtin_math_lerp,
    translate_builtin_math_log::translate_builtin_math_log,
    translate_builtin_math_min_max::translate_builtin_math_min_max,
    translate_builtin_math_unary::translate_builtin_math_unary,
    translate_builtin_number_to_2_number::translate_builtin_number_to_2_number,
    translate_builtin_number_to_number_libm::translate_builtin_number_to_number_libm,
    translate_builtin_string_len::translate_builtin_string_len,
    translate_builtin_table_insert::translate_builtin_table_insert,
    translate_builtin_type::translate_builtin_type,
    translate_builtin_typeof::translate_builtin_typeof,
    translate_builtin_vector::translate_builtin_vector,
    translate_builtin_vector_clamp::translate_builtin_vector_clamp,
    translate_builtin_vector_cross::translate_builtin_vector_cross,
    translate_builtin_vector_dot::translate_builtin_vector_dot,
    translate_builtin_vector_lerp::translate_builtin_vector_lerp,
    translate_builtin_vector_magnitude::translate_builtin_vector_magnitude,
    translate_builtin_vector_map_1::translate_builtin_vector_map_1,
    translate_builtin_vector_map_1_x_4::translate_builtin_vector_map_1_x_4,
    translate_builtin_vector_min_max::translate_builtin_vector_min_max,
    translate_builtin_vector_normalize::translate_builtin_vector_normalize,
  },
  records::{
    builtin_args::BuiltinArgs, builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder,
    ir_op::IrOp,
  },
};

fn no_builtin() -> BuiltinImplResult {
  BuiltinImplResult::NONE_FALLBACK
}

/// fflag 门控：开启时执行翻译闭包，否则回退 VM fallback
fn gated(
  flag: bool,
  build: &mut IrBuilder,
  f: impl FnOnce(&mut IrBuilder) -> BuiltinImplResult,
) -> BuiltinImplResult {
  if flag { f(build) } else { no_builtin() }
}

/// LuauCodegenInteger3 门控便捷封装
fn int3(
  build: &mut IrBuilder,
  f: impl FnOnce(&mut IrBuilder) -> BuiltinImplResult,
) -> BuiltinImplResult {
  gated(fflag::LuauCodegenInteger3.get(), build, f)
}

pub fn translate_builtin(
  build: &mut IrBuilder,
  bfid: i32,
  bargs: BuiltinArgs,
  fallback: IrOp,
) -> BuiltinImplResult {
  let BuiltinArgs {
    ra,
    arg,
    args,
    arg3,
    nparams,
    nresults,
    pcpos,
  } = bargs;

  if nparams == LUA_MULTRET {
    return no_builtin();
  }

  // 未知 bfid → no_builtin（与 C++ switch default 一致）
  let Some(bf) = LBF::from_id(bfid) else {
    return no_builtin();
  };

  if fflag::LuauCodegenInteger3.get()
    && (args.kind() == IrOpKind::Constant || arg3.kind() == IrOpKind::Constant)
  {
    let const_kinds = match bf {
      LBF::LBF_MATH_MIN
      | LBF::LBF_MATH_MAX
      | LBF::LBF_MATH_POW
      | LBF::LBF_MATH_FMOD
      | LBF::LBF_MATH_ATAN2
      | LBF::LBF_MATH_LDEXP
      | LBF::LBF_MATH_LERP
      | LBF::LBF_MATH_CLAMP
      | LBF::LBF_BIT32_BAND
      | LBF::LBF_BIT32_BOR
      | LBF::LBF_BIT32_BXOR
      | LBF::LBF_BIT32_BTEST
      | LBF::LBF_BIT32_LSHIFT
      | LBF::LBF_BIT32_RSHIFT
      | LBF::LBF_BIT32_ARSHIFT
      | LBF::LBF_BIT32_LROTATE
      | LBF::LBF_BIT32_RROTATE
      | LBF::LBF_BIT32_EXTRACT
      | LBF::LBF_BIT32_EXTRACTK
      | LBF::LBF_BIT32_REPLACE
      | LBF::LBF_VECTOR
      | LBF::LBF_TABLE_INSERT
      | LBF::LBF_BUFFER_READI8
      | LBF::LBF_BUFFER_READU8
      | LBF::LBF_BUFFER_WRITEU8
      | LBF::LBF_BUFFER_READI16
      | LBF::LBF_BUFFER_READU16
      | LBF::LBF_BUFFER_WRITEU16
      | LBF::LBF_BUFFER_READI32
      | LBF::LBF_BUFFER_READU32
      | LBF::LBF_BUFFER_WRITEU32
      | LBF::LBF_BUFFER_READF32
      | LBF::LBF_BUFFER_WRITEF32
      | LBF::LBF_BUFFER_READF64
      | LBF::LBF_BUFFER_WRITEF64
      | LBF::LBF_BUFFER_READINTEGER => Some((IrConstKind::Double, IrConstKind::Double)),
      LBF::LBF_BUFFER_WRITEINTEGER => Some((IrConstKind::Double, IrConstKind::Int64)),
      LBF::LBF_INTEGER_ADD
      | LBF::LBF_INTEGER_SUB
      | LBF::LBF_INTEGER_MUL
      | LBF::LBF_INTEGER_DIV
      | LBF::LBF_INTEGER_IDIV
      | LBF::LBF_INTEGER_UDIV
      | LBF::LBF_INTEGER_REM
      | LBF::LBF_INTEGER_UREM
      | LBF::LBF_INTEGER_MOD
      | LBF::LBF_INTEGER_MIN
      | LBF::LBF_INTEGER_MAX
      | LBF::LBF_INTEGER_CLAMP
      | LBF::LBF_INTEGER_LT
      | LBF::LBF_INTEGER_LE
      | LBF::LBF_INTEGER_GT
      | LBF::LBF_INTEGER_GE
      | LBF::LBF_INTEGER_ULT
      | LBF::LBF_INTEGER_ULE
      | LBF::LBF_INTEGER_UGT
      | LBF::LBF_INTEGER_UGE
      | LBF::LBF_INTEGER_BAND
      | LBF::LBF_INTEGER_BOR
      | LBF::LBF_INTEGER_BXOR
      | LBF::LBF_INTEGER_BNOT
      | LBF::LBF_INTEGER_BTEST
      | LBF::LBF_INTEGER_LSHIFT
      | LBF::LBF_INTEGER_RSHIFT
      | LBF::LBF_INTEGER_ARSHIFT
      | LBF::LBF_INTEGER_LROTATE
      | LBF::LBF_INTEGER_RROTATE
      | LBF::LBF_INTEGER_EXTRACT => Some((IrConstKind::Int64, IrConstKind::Int64)),
      _ => None,
    };
    if let Some((k_args, k_arg3)) = const_kinds
      && (!is_compatible_constant(build, args, k_args)
        || !is_compatible_constant(build, arg3, k_arg3))
    {
      return no_builtin();
    }
  }

  // 以下 int3 臂宏须定义在解构之后：模板中的 nparams/ra/arg/… 按宏定义位置解析到本函数局部。

  /// int3 门控标准参数包臂：`f(build, nparams, ra, arg, <pack>, nresults, pcpos[, 尾参…])`，
  /// `<pack>` 选择子 `arg`/`args`/`arg3` 对应不含/含 args/含 args+arg3 三种形参形态。
  macro_rules! i3 {
    ($f:path, arg3 $(, $tail:expr)*) => {
      int3(build, |b| $f(b, nparams, ra, arg, args, arg3, nresults, pcpos $(, $tail)*))
    };
    ($f:path, args $(, $tail:expr)*) => {
      int3(build, |b| $f(b, nparams, ra, arg, args, nresults, pcpos $(, $tail)*))
    };
    ($f:path, arg $(, $tail:expr)*) => {
      int3(build, |b| $f(b, nparams, ra, arg, nresults, pcpos $(, $tail)*))
    };
  }

  /// int3 门控 cmd 前置参数包臂：`f(build, cmd, nparams, ra, arg[, args], nresults, pcpos)`
  macro_rules! i3c {
    ($f:path, $cmd:expr, args) => {
      int3(build, |b| {
        $f(b, $cmd, nparams, ra, arg, args, nresults, pcpos)
      })
    };
    ($f:path, $cmd:expr) => {
      int3(build, |b| $f(b, $cmd, nparams, ra, arg, nresults, pcpos))
    };
  }

  /// int3 门控多参折叠臂：`f(build, cmd, isTestZero, neutral, bargs)`
  macro_rules! i3m {
    ($f:path, $cmd:expr, $test:expr, $neutral:expr) => {
      int3(build, |b| $f(b, $cmd, $test, $neutral, bargs))
    };
  }

  macro_rules! buf_read {
    ($cmd:ident, $size:expr, $conv:ident) => {
      translate_builtin_buffer_read(
        build,
        bargs,
        IrCmd::$cmd,
        $size,
        IrCmd::$conv,
        IrCmd::StoreDouble,
        LuaType::Number as u8,
      )
    };
  }
  macro_rules! buf_write {
    ($cmd:ident, $size:expr, $conv:ident) => {
      translate_builtin_buffer_write(build, bargs, IrCmd::$cmd, $size, IrCmd::$conv, false)
    };
  }

  /// 非 INTEGER3 门控站点直呼臂：形参形态同 i3!（不经 int3 门控包裹；
  /// 尾参插在 `pcpos` 之前，供 `fallback` 形站点使用）。
  macro_rules! d3 {
    ($f:path, arg3 $(, $tail:expr)*) => {
      $f(build, nparams, ra, arg, args, arg3, nresults $(, $tail)*, pcpos)
    };
    ($f:path, args $(, $tail:expr)*) => {
      $f(build, nparams, ra, arg, args, nresults $(, $tail)*, pcpos)
    };
  }

  /// 非 INTEGER3 门控 cmd 前置臂：形参形态同 i3c!。
  macro_rules! d3c {
    ($f:path, $cmd:expr, arg3) => {
      $f(build, $cmd, nparams, ra, arg, args, arg3, nresults, pcpos)
    };
    ($f:path, $cmd:expr, args) => {
      $f(build, $cmd, nparams, ra, arg, args, nresults, pcpos)
    };
    ($f:path, $cmd:expr) => {
      $f(build, $cmd, nparams, ra, arg, nresults, pcpos)
    };
  }

  match bf {
    LBF::LBF_ASSERT => translate_builtin_assert(build, nparams, ra, arg, args, nresults, pcpos),
    LBF::LBF_MATH_DEG => d3c!(translate_builtin_math_deg_rad, IrCmd::DivNum, args),
    LBF::LBF_MATH_RAD => d3c!(translate_builtin_math_deg_rad, IrCmd::MulNum, args),
    LBF::LBF_MATH_LOG => translate_builtin_math_log(build, nparams, ra, arg, args, nresults, pcpos),
    LBF::LBF_MATH_MIN => d3c!(translate_builtin_math_min_max, IrCmd::MinNum, arg3),
    LBF::LBF_MATH_MAX => d3c!(translate_builtin_math_min_max, IrCmd::MaxNum, arg3),
    LBF::LBF_MATH_CLAMP => d3!(translate_builtin_math_clamp, arg3, fallback),
    LBF::LBF_MATH_FLOOR => d3c!(translate_builtin_math_unary, IrCmd::FloorNum),
    LBF::LBF_MATH_CEIL => d3c!(translate_builtin_math_unary, IrCmd::CeilNum),
    LBF::LBF_MATH_SQRT => d3c!(translate_builtin_math_unary, IrCmd::SqrtNum),
    LBF::LBF_MATH_ABS => d3c!(translate_builtin_math_unary, IrCmd::AbsNum),
    LBF::LBF_MATH_ROUND => d3c!(translate_builtin_math_unary, IrCmd::RoundNum),
    bf @ (LBF::LBF_MATH_EXP
    | LBF::LBF_MATH_ASIN
    | LBF::LBF_MATH_SIN
    | LBF::LBF_MATH_SINH
    | LBF::LBF_MATH_ACOS
    | LBF::LBF_MATH_COS
    | LBF::LBF_MATH_COSH
    | LBF::LBF_MATH_ATAN
    | LBF::LBF_MATH_TAN
    | LBF::LBF_MATH_TANH
    | LBF::LBF_MATH_LOG10) => {
      translate_builtin_number_to_number_libm(build, bf, nparams, ra, arg, nresults, pcpos)
    }
    LBF::LBF_MATH_SIGN => d3c!(translate_builtin_math_unary, IrCmd::SignNum),
    bf @ (LBF::LBF_MATH_POW | LBF::LBF_MATH_FMOD | LBF::LBF_MATH_ATAN2 | LBF::LBF_MATH_LDEXP) => {
      translate_builtin_2_number_to_number_libm(build, bf, nparams, ra, arg, args, nresults, pcpos)
    }
    bf @ (LBF::LBF_MATH_FREXP | LBF::LBF_MATH_MODF) => {
      translate_builtin_number_to_2_number(build, bf, nparams, ra, arg, args, nresults, pcpos)
    }
    LBF::LBF_BIT32_BAND => {
      translate_builtin_bit_32_multiarg_op(build, IrCmd::BitandUint, false, bargs)
    }
    LBF::LBF_BIT32_BOR => {
      translate_builtin_bit_32_multiarg_op(build, IrCmd::BitorUint, false, bargs)
    }
    LBF::LBF_BIT32_BXOR => {
      translate_builtin_bit_32_multiarg_op(build, IrCmd::BitxorUint, false, bargs)
    }
    LBF::LBF_BIT32_BTEST => {
      translate_builtin_bit_32_multiarg_op(build, IrCmd::BitandUint, true, bargs)
    }
    LBF::LBF_BIT32_BNOT => d3c!(translate_builtin_bit_32_unary, IrCmd::BitnotUint, args),
    LBF::LBF_BIT32_LSHIFT => d3c!(translate_builtin_bit_32_shift, IrCmd::BitlshiftUint, args),
    LBF::LBF_BIT32_RSHIFT => d3c!(translate_builtin_bit_32_shift, IrCmd::BitrshiftUint, args),
    LBF::LBF_BIT32_ARSHIFT => d3c!(translate_builtin_bit_32_shift, IrCmd::BitarshiftUint, args),
    LBF::LBF_BIT32_LROTATE => d3c!(translate_builtin_bit_32_rotate, IrCmd::BitlrotateUint, args),
    LBF::LBF_BIT32_RROTATE => d3c!(translate_builtin_bit_32_rotate, IrCmd::BitrrotateUint, args),
    LBF::LBF_BIT32_EXTRACT => d3!(translate_builtin_bit_32_extract, arg3),
    LBF::LBF_BIT32_EXTRACTK => d3!(translate_builtin_bit_32_extract_k, args),
    LBF::LBF_BIT32_COUNTLZ => d3c!(translate_builtin_bit_32_unary, IrCmd::BitcountlzUint, args),
    LBF::LBF_BIT32_COUNTRZ => d3c!(translate_builtin_bit_32_unary, IrCmd::BitcountrzUint, args),
    LBF::LBF_BIT32_REPLACE => d3!(translate_builtin_bit_32_replace, arg3),
    LBF::LBF_TYPE => translate_builtin_type(build, nparams, ra, arg, args, nresults),
    LBF::LBF_TYPEOF => translate_builtin_typeof(build, nparams, ra, arg, args, nresults),
    LBF::LBF_VECTOR => d3!(translate_builtin_vector, arg3),
    LBF::LBF_TABLE_INSERT => d3!(translate_builtin_table_insert, args),
    LBF::LBF_STRING_LEN => d3!(translate_builtin_string_len, args),
    LBF::LBF_BIT32_BYTESWAP => d3c!(translate_builtin_bit_32_unary, IrCmd::ByteswapUint, args),
    LBF::LBF_BUFFER_READI8 => buf_read!(BufferReadi8, 1, IntToNum),
    LBF::LBF_BUFFER_READU8 => buf_read!(BufferReadu8, 1, IntToNum),
    LBF::LBF_BUFFER_WRITEU8 => buf_write!(BufferWritei8, 1, NumToUint),
    LBF::LBF_BUFFER_READI16 => buf_read!(BufferReadi16, 2, IntToNum),
    LBF::LBF_BUFFER_READU16 => buf_read!(BufferReadu16, 2, IntToNum),
    LBF::LBF_BUFFER_WRITEU16 => buf_write!(BufferWritei16, 2, NumToUint),
    LBF::LBF_BUFFER_READI32 => buf_read!(BufferReadi32, 4, IntToNum),
    LBF::LBF_BUFFER_READU32 => buf_read!(BufferReadi32, 4, UintToNum),
    LBF::LBF_BUFFER_WRITEU32 => buf_write!(BufferWritei32, 4, NumToUint),
    LBF::LBF_BUFFER_READF32 => buf_read!(BufferReadf32, 4, FloatToNum),
    LBF::LBF_BUFFER_WRITEF32 => buf_write!(BufferWritef32, 4, NumToFloat),
    LBF::LBF_BUFFER_READF64 => buf_read!(BufferReadf64, 8, NOP),
    LBF::LBF_BUFFER_WRITEF64 => buf_write!(BufferWritef64, 8, NOP),
    LBF::LBF_BUFFER_READINTEGER => gated(fflag::LuauCodegenBufferInteger.get(), build, |b| {
      translate_builtin_buffer_read(
        b,
        bargs,
        IrCmd::BufferReadi64,
        8,
        IrCmd::NOP,
        IrCmd::StoreInt64,
        LuaType::Integer as u8,
      )
    }),
    LBF::LBF_BUFFER_WRITEINTEGER => gated(fflag::LuauCodegenBufferInteger.get(), build, |b| {
      translate_builtin_buffer_write(b, bargs, IrCmd::BufferWritei64, 8, IrCmd::NOP, true)
    }),
    LBF::LBF_VECTOR_MAGNITUDE => d3!(translate_builtin_vector_magnitude, arg3),
    LBF::LBF_VECTOR_NORMALIZE => d3!(translate_builtin_vector_normalize, arg3),
    LBF::LBF_VECTOR_CROSS => d3!(translate_builtin_vector_cross, arg3),
    LBF::LBF_VECTOR_DOT => d3!(translate_builtin_vector_dot, arg3),
    LBF::LBF_VECTOR_FLOOR => d3c!(translate_builtin_vector_map_1_x_4, IrCmd::FloorVec, arg3),
    LBF::LBF_VECTOR_CEIL => d3c!(translate_builtin_vector_map_1_x_4, IrCmd::CeilVec, arg3),
    LBF::LBF_VECTOR_ABS => d3c!(translate_builtin_vector_map_1_x_4, IrCmd::AbsVec, arg3),
    LBF::LBF_VECTOR_SIGN => d3c!(translate_builtin_vector_map_1, IrCmd::SignFloat, arg3),
    LBF::LBF_VECTOR_CLAMP => d3!(translate_builtin_vector_clamp, arg3, fallback),
    LBF::LBF_VECTOR_MIN => d3c!(translate_builtin_vector_min_max, IrCmd::MinVec, arg3),
    LBF::LBF_VECTOR_MAX => d3c!(translate_builtin_vector_min_max, IrCmd::MaxVec, arg3),
    LBF::LBF_VECTOR_LERP => d3!(translate_builtin_vector_lerp, arg3),
    LBF::LBF_MATH_LERP => d3!(translate_builtin_math_lerp, arg3, fallback),
    LBF::LBF_MATH_ISNAN => d3!(translate_builtin_math_is_nan, args),
    LBF::LBF_INTEGER_CREATE => i3!(translate_builtin_int_64_create, arg),
    LBF::LBF_INTEGER_TONUMBER => i3!(translate_builtin_int_64_to_number, arg),
    LBF::LBF_INTEGER_ADD => i3!(translate_builtin_int_64_binary, args, Int64Binary::Add),
    LBF::LBF_INTEGER_SUB => i3!(translate_builtin_int_64_binary, args, Int64Binary::Sub),
    LBF::LBF_INTEGER_MUL => i3!(translate_builtin_int_64_binary, args, Int64Binary::Mul),
    LBF::LBF_INTEGER_DIV => i3!(translate_builtin_int_64_binary, args, Int64Binary::Div),
    LBF::LBF_INTEGER_IDIV => i3!(translate_builtin_int_64_binary, args, Int64Binary::Idiv),
    LBF::LBF_INTEGER_UDIV => i3!(translate_builtin_int_64_binary, args, Int64Binary::Udiv),
    LBF::LBF_INTEGER_REM => i3!(translate_builtin_int_64_binary, args, Int64Binary::Rem),
    LBF::LBF_INTEGER_UREM => i3!(translate_builtin_int_64_binary, args, Int64Binary::Urem),
    LBF::LBF_INTEGER_MOD => i3!(translate_builtin_int_64_binary, args, Int64Binary::Mod),
    LBF::LBF_INTEGER_MIN => i3!(translate_builtin_int_64_min_max, arg3, true),
    LBF::LBF_INTEGER_MAX => i3!(translate_builtin_int_64_min_max, arg3, false),
    LBF::LBF_INTEGER_NEG => i3!(translate_builtin_int_64_neg, arg),
    LBF::LBF_INTEGER_CLAMP => i3!(translate_builtin_int_64_clamp, arg3),
    LBF::LBF_INTEGER_LT => i3!(translate_builtin_int_64_compare, args, IrCondition::Less),
    LBF::LBF_INTEGER_LE => i3!(
      translate_builtin_int_64_compare,
      args,
      IrCondition::LessEqual
    ),
    LBF::LBF_INTEGER_GT => i3!(translate_builtin_int_64_compare, args, IrCondition::Greater),
    LBF::LBF_INTEGER_GE => i3!(
      translate_builtin_int_64_compare,
      args,
      IrCondition::GreaterEqual
    ),
    LBF::LBF_INTEGER_ULT => i3!(
      translate_builtin_int_64_compare,
      args,
      IrCondition::UnsignedLess
    ),
    LBF::LBF_INTEGER_ULE => i3!(
      translate_builtin_int_64_compare,
      args,
      IrCondition::UnsignedLessEqual
    ),
    LBF::LBF_INTEGER_UGT => i3!(
      translate_builtin_int_64_compare,
      args,
      IrCondition::UnsignedGreater
    ),
    LBF::LBF_INTEGER_UGE => i3!(
      translate_builtin_int_64_compare,
      args,
      IrCondition::UnsignedGreaterEqual
    ),
    LBF::LBF_INTEGER_BAND => {
      i3m!(
        translate_builtin_int_64_multiarg_op,
        IrCmd::BitandInt64,
        false,
        -1
      )
    }
    LBF::LBF_INTEGER_BOR => {
      i3m!(
        translate_builtin_int_64_multiarg_op,
        IrCmd::BitorInt64,
        false,
        0
      )
    }
    LBF::LBF_INTEGER_BXOR => {
      i3m!(
        translate_builtin_int_64_multiarg_op,
        IrCmd::BitxorInt64,
        false,
        0
      )
    }
    LBF::LBF_INTEGER_BNOT => i3c!(translate_builtin_int_64_unary, IrCmd::BitnotInt64),
    LBF::LBF_INTEGER_BTEST => {
      i3m!(
        translate_builtin_int_64_multiarg_op,
        IrCmd::BitandInt64,
        true,
        -1
      )
    }
    LBF::LBF_INTEGER_LSHIFT => {
      i3c!(
        translate_builtin_int_64_shift_rotate,
        IrCmd::BitlshiftInt64,
        args
      )
    }
    LBF::LBF_INTEGER_RSHIFT => {
      i3c!(
        translate_builtin_int_64_shift_rotate,
        IrCmd::BitrshiftInt64,
        args
      )
    }
    LBF::LBF_INTEGER_ARSHIFT => {
      i3c!(
        translate_builtin_int_64_shift_rotate,
        IrCmd::BitarshiftInt64,
        args
      )
    }
    LBF::LBF_INTEGER_LROTATE => {
      i3c!(
        translate_builtin_int_64_shift_rotate,
        IrCmd::BitlrotateInt64,
        args
      )
    }
    LBF::LBF_INTEGER_RROTATE => {
      i3c!(
        translate_builtin_int_64_shift_rotate,
        IrCmd::BitrrotateInt64,
        args
      )
    }
    LBF::LBF_INTEGER_COUNTLZ => i3c!(translate_builtin_int_64_unary, IrCmd::BitcountlzInt64),
    LBF::LBF_INTEGER_COUNTRZ => i3c!(translate_builtin_int_64_unary, IrCmd::BitcountrzInt64),
    LBF::LBF_INTEGER_BSWAP => i3c!(translate_builtin_int_64_unary, IrCmd::ByteswapInt64),
    LBF::LBF_INTEGER_EXTRACT => i3!(translate_builtin_int_64_extract, arg3),
    _ => no_builtin(),
  }
}
