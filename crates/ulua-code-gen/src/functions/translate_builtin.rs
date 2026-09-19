use ulua_common::{enums::luau_builtin_function::LuauBuiltinFunction as LBF, fflag};
use ulua_vm::{enums::lua_type::LuaType, macros::lua_multret::LUA_MULTRET};

use crate::{
  enums::{
    builtin_impl_type::BuiltinImplType, int_64_binary::Int64Binary, ir_cmd::IrCmd,
    ir_condition::IrCondition, ir_const_kind::IrConstKind, ir_op_kind::IrOpKind,
  },
  functions::{
    is_compatible_constant::is_compatible_constant,
    translate_builtin_2_number_to_number_libm::translate_builtin_2_number_to_number_libm,
    translate_builtin_assert::translate_builtin_assert,
    translate_builtin_bit_32_bnot::translate_builtin_bit_32_bnot,
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
    translate_builtin_int_64_bnot::translate_builtin_int_64_bnot,
    translate_builtin_int_64_clamp::translate_builtin_int_64_clamp,
    translate_builtin_int_64_compare::translate_builtin_int_64_compare,
    translate_builtin_int_64_create::translate_builtin_int_64_create,
    translate_builtin_int_64_extract::translate_builtin_int_64_extract,
    translate_builtin_int_64_min_max::translate_builtin_int_64_min_max,
    translate_builtin_int_64_multiarg_op::translate_builtin_int_64_multiarg_op,
    translate_builtin_int_64_neg::translate_builtin_int_64_neg,
    translate_builtin_int_64_rotate::translate_builtin_int_64_rotate,
    translate_builtin_int_64_shift::translate_builtin_int_64_shift,
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
  BuiltinImplResult {
    r#type: BuiltinImplType::None,
    actual_result_count: -1,
  }
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
    match bf {
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
      | LBF::LBF_BUFFER_READINTEGER => {
        if !is_compatible_constant(build, args, IrConstKind::Double) {
          return no_builtin();
        }
        if !is_compatible_constant(build, arg3, IrConstKind::Double) {
          return no_builtin();
        }
      }
      LBF::LBF_BUFFER_WRITEINTEGER => {
        if !is_compatible_constant(build, args, IrConstKind::Double) {
          return no_builtin();
        }
        if !is_compatible_constant(build, arg3, IrConstKind::Int64) {
          return no_builtin();
        }
      }
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
      | LBF::LBF_INTEGER_EXTRACT => {
        if !is_compatible_constant(build, args, IrConstKind::Int64) {
          return no_builtin();
        }
        if !is_compatible_constant(build, arg3, IrConstKind::Int64) {
          return no_builtin();
        }
      }
      _ => {}
    }
  }

  match bf {
    LBF::LBF_ASSERT => translate_builtin_assert(build, nparams, ra, arg, args, nresults, pcpos),
    LBF::LBF_MATH_DEG => translate_builtin_math_deg_rad(
      build,
      IrCmd::DivNum,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_MATH_RAD => translate_builtin_math_deg_rad(
      build,
      IrCmd::MulNum,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_MATH_LOG => translate_builtin_math_log(build, nparams, ra, arg, args, nresults, pcpos),
    LBF::LBF_MATH_MIN => translate_builtin_math_min_max(
      build,
      IrCmd::MinNum,
      nparams,
      ra,
      arg,
      args,
      arg3,
      nresults,
      pcpos,
    ),
    LBF::LBF_MATH_MAX => translate_builtin_math_min_max(
      build,
      IrCmd::MaxNum,
      nparams,
      ra,
      arg,
      args,
      arg3,
      nresults,
      pcpos,
    ),
    LBF::LBF_MATH_CLAMP => translate_builtin_math_clamp(
      build, nparams, ra, arg, args, arg3, nresults, fallback, pcpos,
    ),
    LBF::LBF_MATH_FLOOR => {
      translate_builtin_math_unary(build, IrCmd::FloorNum, nparams, ra, arg, nresults, pcpos)
    }
    LBF::LBF_MATH_CEIL => {
      translate_builtin_math_unary(build, IrCmd::CeilNum, nparams, ra, arg, nresults, pcpos)
    }
    LBF::LBF_MATH_SQRT => {
      translate_builtin_math_unary(build, IrCmd::SqrtNum, nparams, ra, arg, nresults, pcpos)
    }
    LBF::LBF_MATH_ABS => {
      translate_builtin_math_unary(build, IrCmd::AbsNum, nparams, ra, arg, nresults, pcpos)
    }
    LBF::LBF_MATH_ROUND => {
      translate_builtin_math_unary(build, IrCmd::RoundNum, nparams, ra, arg, nresults, pcpos)
    }
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
    LBF::LBF_MATH_SIGN => {
      translate_builtin_math_unary(build, IrCmd::SignNum, nparams, ra, arg, nresults, pcpos)
    }
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
    LBF::LBF_BIT32_BNOT => {
      translate_builtin_bit_32_bnot(build, nparams, ra, arg, args, nresults, pcpos)
    }
    LBF::LBF_BIT32_LSHIFT => translate_builtin_bit_32_shift(
      build,
      IrCmd::BitlshiftUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_BIT32_RSHIFT => translate_builtin_bit_32_shift(
      build,
      IrCmd::BitrshiftUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_BIT32_ARSHIFT => translate_builtin_bit_32_shift(
      build,
      IrCmd::BitarshiftUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_BIT32_LROTATE => translate_builtin_bit_32_rotate(
      build,
      IrCmd::BitlrotateUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_BIT32_RROTATE => translate_builtin_bit_32_rotate(
      build,
      IrCmd::BitrrotateUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_BIT32_EXTRACT => {
      translate_builtin_bit_32_extract(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    LBF::LBF_BIT32_EXTRACTK => {
      translate_builtin_bit_32_extract_k(build, nparams, ra, arg, args, nresults, pcpos)
    }
    LBF::LBF_BIT32_COUNTLZ => translate_builtin_bit_32_unary(
      build,
      IrCmd::BitcountlzUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_BIT32_COUNTRZ => translate_builtin_bit_32_unary(
      build,
      IrCmd::BitcountrzUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_BIT32_REPLACE => {
      translate_builtin_bit_32_replace(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    LBF::LBF_TYPE => translate_builtin_type(build, nparams, ra, arg, args, nresults),
    LBF::LBF_TYPEOF => translate_builtin_typeof(build, nparams, ra, arg, args, nresults),
    LBF::LBF_VECTOR => {
      translate_builtin_vector(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    LBF::LBF_TABLE_INSERT => {
      translate_builtin_table_insert(build, nparams, ra, arg, args, nresults, pcpos)
    }
    LBF::LBF_STRING_LEN => {
      translate_builtin_string_len(build, nparams, ra, arg, args, nresults, pcpos)
    }
    LBF::LBF_BIT32_BYTESWAP => translate_builtin_bit_32_unary(
      build,
      IrCmd::ByteswapUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_BUFFER_READI8 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadi8,
      1,
      IrCmd::IntToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    LBF::LBF_BUFFER_READU8 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadu8,
      1,
      IrCmd::IntToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    LBF::LBF_BUFFER_WRITEU8 => translate_builtin_buffer_write(
      build,
      bargs,
      IrCmd::BufferWritei8,
      1,
      IrCmd::NumToUint,
      false,
    ),
    LBF::LBF_BUFFER_READI16 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadi16,
      2,
      IrCmd::IntToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    LBF::LBF_BUFFER_READU16 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadu16,
      2,
      IrCmd::IntToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    LBF::LBF_BUFFER_WRITEU16 => translate_builtin_buffer_write(
      build,
      bargs,
      IrCmd::BufferWritei16,
      2,
      IrCmd::NumToUint,
      false,
    ),
    LBF::LBF_BUFFER_READI32 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadi32,
      4,
      IrCmd::IntToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    LBF::LBF_BUFFER_READU32 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadi32,
      4,
      IrCmd::UintToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    LBF::LBF_BUFFER_WRITEU32 => translate_builtin_buffer_write(
      build,
      bargs,
      IrCmd::BufferWritei32,
      4,
      IrCmd::NumToUint,
      false,
    ),
    LBF::LBF_BUFFER_READF32 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadf32,
      4,
      IrCmd::FloatToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    LBF::LBF_BUFFER_WRITEF32 => translate_builtin_buffer_write(
      build,
      bargs,
      IrCmd::BufferWritef32,
      4,
      IrCmd::NumToFloat,
      false,
    ),
    LBF::LBF_BUFFER_READF64 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadf64,
      8,
      IrCmd::NOP,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    LBF::LBF_BUFFER_WRITEF64 => {
      translate_builtin_buffer_write(build, bargs, IrCmd::BufferWritef64, 8, IrCmd::NOP, false)
    }
    LBF::LBF_BUFFER_READINTEGER => {
      if fflag::LuauCodegenBufferInteger.get() {
        translate_builtin_buffer_read(
          build,
          bargs,
          IrCmd::BufferReadi64,
          8,
          IrCmd::NOP,
          IrCmd::StoreInt64,
          LuaType::Integer as u8,
        )
      } else {
        no_builtin()
      }
    }
    LBF::LBF_BUFFER_WRITEINTEGER => {
      if fflag::LuauCodegenBufferInteger.get() {
        translate_builtin_buffer_write(build, bargs, IrCmd::BufferWritei64, 8, IrCmd::NOP, true)
      } else {
        no_builtin()
      }
    }
    LBF::LBF_VECTOR_MAGNITUDE => {
      translate_builtin_vector_magnitude(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    LBF::LBF_VECTOR_NORMALIZE => {
      translate_builtin_vector_normalize(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    LBF::LBF_VECTOR_CROSS => {
      translate_builtin_vector_cross(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    LBF::LBF_VECTOR_DOT => {
      translate_builtin_vector_dot(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    LBF::LBF_VECTOR_FLOOR => translate_builtin_vector_map_1_x_4(
      build,
      IrCmd::FloorVec,
      nparams,
      ra,
      arg,
      args,
      arg3,
      nresults,
      pcpos,
    ),
    LBF::LBF_VECTOR_CEIL => translate_builtin_vector_map_1_x_4(
      build,
      IrCmd::CeilVec,
      nparams,
      ra,
      arg,
      args,
      arg3,
      nresults,
      pcpos,
    ),
    LBF::LBF_VECTOR_ABS => translate_builtin_vector_map_1_x_4(
      build,
      IrCmd::AbsVec,
      nparams,
      ra,
      arg,
      args,
      arg3,
      nresults,
      pcpos,
    ),
    LBF::LBF_VECTOR_SIGN => translate_builtin_vector_map_1(
      build,
      IrCmd::SignFloat,
      nparams,
      ra,
      arg,
      args,
      arg3,
      nresults,
      pcpos,
    ),
    LBF::LBF_VECTOR_CLAMP => translate_builtin_vector_clamp(
      build, nparams, ra, arg, args, arg3, nresults, fallback, pcpos,
    ),
    LBF::LBF_VECTOR_MIN => translate_builtin_vector_min_max(
      build,
      IrCmd::MinVec,
      nparams,
      ra,
      arg,
      args,
      arg3,
      nresults,
      pcpos,
    ),
    LBF::LBF_VECTOR_MAX => translate_builtin_vector_min_max(
      build,
      IrCmd::MaxVec,
      nparams,
      ra,
      arg,
      args,
      arg3,
      nresults,
      pcpos,
    ),
    LBF::LBF_VECTOR_LERP => {
      translate_builtin_vector_lerp(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    LBF::LBF_MATH_LERP => translate_builtin_math_lerp(
      build, nparams, ra, arg, args, arg3, nresults, fallback, pcpos,
    ),
    LBF::LBF_MATH_ISNAN => {
      translate_builtin_math_is_nan(build, nparams, ra, arg, args, nresults, pcpos)
    }
    LBF::LBF_INTEGER_CREATE => {
      if fflag::LuauCodegenInteger3.get() {
        translate_builtin_int_64_create(build, nparams, ra, arg, nresults, pcpos)
      } else {
        no_builtin()
      }
    }
    LBF::LBF_INTEGER_TONUMBER => {
      if fflag::LuauCodegenInteger3.get() {
        translate_builtin_int_64_to_number(build, nparams, ra, arg, nresults, pcpos)
      } else {
        no_builtin()
      }
    }
    LBF::LBF_INTEGER_ADD => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Add,
    ),
    LBF::LBF_INTEGER_SUB => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Sub,
    ),
    LBF::LBF_INTEGER_MUL => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Mul,
    ),
    LBF::LBF_INTEGER_DIV => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Div,
    ),
    LBF::LBF_INTEGER_IDIV => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Idiv,
    ),
    LBF::LBF_INTEGER_UDIV => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Udiv,
    ),
    LBF::LBF_INTEGER_REM => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Rem,
    ),
    LBF::LBF_INTEGER_UREM => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Urem,
    ),
    LBF::LBF_INTEGER_MOD => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Mod,
    ),
    LBF::LBF_INTEGER_MIN => {
      if fflag::LuauCodegenInteger3.get() {
        translate_builtin_int_64_min_max(build, nparams, ra, arg, args, arg3, nresults, pcpos, true)
      } else {
        no_builtin()
      }
    }
    LBF::LBF_INTEGER_MAX => {
      if fflag::LuauCodegenInteger3.get() {
        translate_builtin_int_64_min_max(
          build, nparams, ra, arg, args, arg3, nresults, pcpos, false,
        )
      } else {
        no_builtin()
      }
    }
    LBF::LBF_INTEGER_NEG => {
      if fflag::LuauCodegenInteger3.get() {
        translate_builtin_int_64_neg(build, nparams, ra, arg, nresults, pcpos)
      } else {
        no_builtin()
      }
    }
    LBF::LBF_INTEGER_CLAMP => {
      if fflag::LuauCodegenInteger3.get() {
        translate_builtin_int_64_clamp(build, nparams, ra, arg, args, arg3, nresults, pcpos)
      } else {
        no_builtin()
      }
    }
    LBF::LBF_INTEGER_LT => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::Less,
    ),
    LBF::LBF_INTEGER_LE => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::LessEqual,
    ),
    LBF::LBF_INTEGER_GT => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::Greater,
    ),
    LBF::LBF_INTEGER_GE => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::GreaterEqual,
    ),
    LBF::LBF_INTEGER_ULT => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::UnsignedLess,
    ),
    LBF::LBF_INTEGER_ULE => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::UnsignedLessEqual,
    ),
    LBF::LBF_INTEGER_UGT => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::UnsignedGreater,
    ),
    LBF::LBF_INTEGER_UGE => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::UnsignedGreaterEqual,
    ),
    LBF::LBF_INTEGER_BAND => int2_multiarg(build, IrCmd::BitandInt64, false, -1, bargs),
    LBF::LBF_INTEGER_BOR => int2_multiarg(build, IrCmd::BitorInt64, false, 0, bargs),
    LBF::LBF_INTEGER_BXOR => int2_multiarg(build, IrCmd::BitxorInt64, false, 0, bargs),
    LBF::LBF_INTEGER_BNOT => {
      if fflag::LuauCodegenInteger3.get() {
        translate_builtin_int_64_bnot(build, nparams, ra, arg, nresults, pcpos)
      } else {
        no_builtin()
      }
    }
    LBF::LBF_INTEGER_BTEST => int2_multiarg(build, IrCmd::BitandInt64, true, -1, bargs),
    LBF::LBF_INTEGER_LSHIFT => int2_shift(
      build,
      IrCmd::BitlshiftInt64,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_INTEGER_RSHIFT => int2_shift(
      build,
      IrCmd::BitrshiftInt64,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_INTEGER_ARSHIFT => int2_shift(
      build,
      IrCmd::BitarshiftInt64,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_INTEGER_LROTATE => int2_rotate(
      build,
      IrCmd::BitlrotateInt64,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_INTEGER_RROTATE => int2_rotate(
      build,
      IrCmd::BitrrotateInt64,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    LBF::LBF_INTEGER_COUNTLZ => int2_unary(
      build,
      IrCmd::BitcountlzInt64,
      nparams,
      ra,
      arg,
      nresults,
      pcpos,
    ),
    LBF::LBF_INTEGER_COUNTRZ => int2_unary(
      build,
      IrCmd::BitcountrzInt64,
      nparams,
      ra,
      arg,
      nresults,
      pcpos,
    ),
    LBF::LBF_INTEGER_BSWAP => int2_unary(
      build,
      IrCmd::ByteswapInt64,
      nparams,
      ra,
      arg,
      nresults,
      pcpos,
    ),
    LBF::LBF_INTEGER_EXTRACT => {
      if fflag::LuauCodegenInteger3.get() {
        translate_builtin_int_64_extract(build, nparams, ra, arg, args, arg3, nresults, pcpos)
      } else {
        no_builtin()
      }
    }
    _ => no_builtin(),
  }
}

fn int2_binary(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
  op: Int64Binary,
) -> BuiltinImplResult {
  if fflag::LuauCodegenInteger3.get() {
    translate_builtin_int_64_binary(build, nparams, ra, arg, args, nresults, pcpos, op)
  } else {
    no_builtin()
  }
}

fn int2_compare(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
  cond: IrCondition,
) -> BuiltinImplResult {
  if fflag::LuauCodegenInteger3.get() {
    translate_builtin_int_64_compare(build, nparams, ra, arg, args, nresults, pcpos, cond)
  } else {
    no_builtin()
  }
}

fn int2_multiarg(
  build: &mut IrBuilder,
  cmd: IrCmd,
  btest: bool,
  identity: i64,
  bargs: BuiltinArgs,
) -> BuiltinImplResult {
  if fflag::LuauCodegenInteger3.get() {
    translate_builtin_int_64_multiarg_op(build, cmd, btest, identity, bargs)
  } else {
    no_builtin()
  }
}

fn int2_shift(
  build: &mut IrBuilder,
  cmd: IrCmd,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if fflag::LuauCodegenInteger3.get() {
    translate_builtin_int_64_shift(build, cmd, nparams, ra, arg, args, nresults, pcpos)
  } else {
    no_builtin()
  }
}

fn int2_rotate(
  build: &mut IrBuilder,
  cmd: IrCmd,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if fflag::LuauCodegenInteger3.get() {
    translate_builtin_int_64_rotate(build, cmd, nparams, ra, arg, args, nresults, pcpos)
  } else {
    no_builtin()
  }
}

fn int2_unary(
  build: &mut IrBuilder,
  cmd: IrCmd,
  nparams: i32,
  ra: i32,
  arg: i32,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if fflag::LuauCodegenInteger3.get() {
    translate_builtin_int_64_unary(build, cmd, nparams, ra, arg, nresults, pcpos)
  } else {
    no_builtin()
  }
}
