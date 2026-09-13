use core::mem::transmute;

use ulua_common::{
  FFlag,
  enums::luau_builtin_function::{LuauBuiltinFunction, LuauBuiltinFunction as LBF},
};
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

  if FFlag::LuauCodegenInteger2.get()
    && (args.kind() == IrOpKind::Constant || arg3.kind() == IrOpKind::Constant)
  {
    match bfid {
      x if x == LBF::LBF_MATH_MIN as i32
        || x == LBF::LBF_MATH_MAX as i32
        || x == LBF::LBF_MATH_POW as i32
        || x == LBF::LBF_MATH_FMOD as i32
        || x == LBF::LBF_MATH_ATAN2 as i32
        || x == LBF::LBF_MATH_LDEXP as i32
        || x == LBF::LBF_MATH_LERP as i32
        || x == LBF::LBF_MATH_CLAMP as i32
        || x == LBF::LBF_BIT32_BAND as i32
        || x == LBF::LBF_BIT32_BOR as i32
        || x == LBF::LBF_BIT32_BXOR as i32
        || x == LBF::LBF_BIT32_BTEST as i32
        || x == LBF::LBF_BIT32_LSHIFT as i32
        || x == LBF::LBF_BIT32_RSHIFT as i32
        || x == LBF::LBF_BIT32_ARSHIFT as i32
        || x == LBF::LBF_BIT32_LROTATE as i32
        || x == LBF::LBF_BIT32_RROTATE as i32
        || x == LBF::LBF_BIT32_EXTRACT as i32
        || x == LBF::LBF_BIT32_EXTRACTK as i32
        || x == LBF::LBF_BIT32_REPLACE as i32
        || x == LBF::LBF_VECTOR as i32
        || x == LBF::LBF_TABLE_INSERT as i32
        || x == LBF::LBF_BUFFER_READI8 as i32
        || x == LBF::LBF_BUFFER_READU8 as i32
        || x == LBF::LBF_BUFFER_WRITEU8 as i32
        || x == LBF::LBF_BUFFER_READI16 as i32
        || x == LBF::LBF_BUFFER_READU16 as i32
        || x == LBF::LBF_BUFFER_WRITEU16 as i32
        || x == LBF::LBF_BUFFER_READI32 as i32
        || x == LBF::LBF_BUFFER_READU32 as i32
        || x == LBF::LBF_BUFFER_WRITEU32 as i32
        || x == LBF::LBF_BUFFER_READF32 as i32
        || x == LBF::LBF_BUFFER_WRITEF32 as i32
        || x == LBF::LBF_BUFFER_READF64 as i32
        || x == LBF::LBF_BUFFER_WRITEF64 as i32
        || x == LBF::LBF_BUFFER_READINTEGER as i32 =>
      {
        if !is_compatible_constant(build, args, IrConstKind::Double) {
          return no_builtin();
        }
        if !is_compatible_constant(build, arg3, IrConstKind::Double) {
          return no_builtin();
        }
      }
      x if x == LBF::LBF_BUFFER_WRITEINTEGER as i32 => {
        if !is_compatible_constant(build, args, IrConstKind::Double) {
          return no_builtin();
        }
        if !is_compatible_constant(build, arg3, IrConstKind::Int64) {
          return no_builtin();
        }
      }
      x if x == LBF::LBF_INTEGER_ADD as i32
        || x == LBF::LBF_INTEGER_SUB as i32
        || x == LBF::LBF_INTEGER_MUL as i32
        || x == LBF::LBF_INTEGER_DIV as i32
        || x == LBF::LBF_INTEGER_IDIV as i32
        || x == LBF::LBF_INTEGER_UDIV as i32
        || x == LBF::LBF_INTEGER_REM as i32
        || x == LBF::LBF_INTEGER_UREM as i32
        || x == LBF::LBF_INTEGER_MOD as i32
        || x == LBF::LBF_INTEGER_MIN as i32
        || x == LBF::LBF_INTEGER_MAX as i32
        || x == LBF::LBF_INTEGER_CLAMP as i32
        || x == LBF::LBF_INTEGER_LT as i32
        || x == LBF::LBF_INTEGER_LE as i32
        || x == LBF::LBF_INTEGER_GT as i32
        || x == LBF::LBF_INTEGER_GE as i32
        || x == LBF::LBF_INTEGER_ULT as i32
        || x == LBF::LBF_INTEGER_ULE as i32
        || x == LBF::LBF_INTEGER_UGT as i32
        || x == LBF::LBF_INTEGER_UGE as i32
        || x == LBF::LBF_INTEGER_BAND as i32
        || x == LBF::LBF_INTEGER_BOR as i32
        || x == LBF::LBF_INTEGER_BXOR as i32
        || x == LBF::LBF_INTEGER_BNOT as i32
        || x == LBF::LBF_INTEGER_BTEST as i32
        || x == LBF::LBF_INTEGER_LSHIFT as i32
        || x == LBF::LBF_INTEGER_RSHIFT as i32
        || x == LBF::LBF_INTEGER_ARSHIFT as i32
        || x == LBF::LBF_INTEGER_LROTATE as i32
        || x == LBF::LBF_INTEGER_RROTATE as i32
        || x == LBF::LBF_INTEGER_EXTRACT as i32 =>
      {
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

  match bfid {
    x if x == LBF::LBF_ASSERT as i32 => {
      translate_builtin_assert(build, nparams, ra, arg, args, nresults, pcpos)
    }
    x if x == LBF::LBF_MATH_DEG as i32 => translate_builtin_math_deg_rad(
      build,
      IrCmd::DivNum,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_MATH_RAD as i32 => translate_builtin_math_deg_rad(
      build,
      IrCmd::MulNum,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_MATH_LOG as i32 => {
      translate_builtin_math_log(build, nparams, ra, arg, args, nresults, pcpos)
    }
    x if x == LBF::LBF_MATH_MIN as i32 => translate_builtin_math_min_max(
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
    x if x == LBF::LBF_MATH_MAX as i32 => translate_builtin_math_min_max(
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
    x if x == LBF::LBF_MATH_CLAMP as i32 => translate_builtin_math_clamp(
      build, nparams, ra, arg, args, arg3, nresults, fallback, pcpos,
    ),
    x if x == LBF::LBF_MATH_FLOOR as i32 => {
      translate_builtin_math_unary(build, IrCmd::FloorNum, nparams, ra, arg, nresults, pcpos)
    }
    x if x == LBF::LBF_MATH_CEIL as i32 => {
      translate_builtin_math_unary(build, IrCmd::CeilNum, nparams, ra, arg, nresults, pcpos)
    }
    x if x == LBF::LBF_MATH_SQRT as i32 => {
      translate_builtin_math_unary(build, IrCmd::SqrtNum, nparams, ra, arg, nresults, pcpos)
    }
    x if x == LBF::LBF_MATH_ABS as i32 => {
      translate_builtin_math_unary(build, IrCmd::AbsNum, nparams, ra, arg, nresults, pcpos)
    }
    x if x == LBF::LBF_MATH_ROUND as i32 => {
      translate_builtin_math_unary(build, IrCmd::RoundNum, nparams, ra, arg, nresults, pcpos)
    }
    x if x == LBF::LBF_MATH_EXP as i32
      || x == LBF::LBF_MATH_ASIN as i32
      || x == LBF::LBF_MATH_SIN as i32
      || x == LBF::LBF_MATH_SINH as i32
      || x == LBF::LBF_MATH_ACOS as i32
      || x == LBF::LBF_MATH_COS as i32
      || x == LBF::LBF_MATH_COSH as i32
      || x == LBF::LBF_MATH_ATAN as i32
      || x == LBF::LBF_MATH_TAN as i32
      || x == LBF::LBF_MATH_TANH as i32
      || x == LBF::LBF_MATH_LOG10 as i32 =>
    {
      translate_builtin_number_to_number_libm(
        build,
        unsafe { transmute::<u8, LuauBuiltinFunction>(bfid as u8) },
        nparams,
        ra,
        arg,
        nresults,
        pcpos,
      )
    }
    x if x == LBF::LBF_MATH_SIGN as i32 => {
      translate_builtin_math_unary(build, IrCmd::SignNum, nparams, ra, arg, nresults, pcpos)
    }
    x if x == LBF::LBF_MATH_POW as i32
      || x == LBF::LBF_MATH_FMOD as i32
      || x == LBF::LBF_MATH_ATAN2 as i32
      || x == LBF::LBF_MATH_LDEXP as i32 =>
    {
      translate_builtin_2_number_to_number_libm(
        build,
        unsafe { transmute::<u8, LuauBuiltinFunction>(bfid as u8) },
        nparams,
        ra,
        arg,
        args,
        nresults,
        pcpos,
      )
    }
    x if x == LBF::LBF_MATH_FREXP as i32 || x == LBF::LBF_MATH_MODF as i32 => {
      translate_builtin_number_to_2_number(
        build,
        unsafe { transmute::<u8, LuauBuiltinFunction>(bfid as u8) },
        nparams,
        ra,
        arg,
        args,
        nresults,
        pcpos,
      )
    }
    x if x == LBF::LBF_BIT32_BAND as i32 => {
      translate_builtin_bit_32_multiarg_op(build, IrCmd::BitandUint, false, bargs)
    }
    x if x == LBF::LBF_BIT32_BOR as i32 => {
      translate_builtin_bit_32_multiarg_op(build, IrCmd::BitorUint, false, bargs)
    }
    x if x == LBF::LBF_BIT32_BXOR as i32 => {
      translate_builtin_bit_32_multiarg_op(build, IrCmd::BitxorUint, false, bargs)
    }
    x if x == LBF::LBF_BIT32_BTEST as i32 => {
      translate_builtin_bit_32_multiarg_op(build, IrCmd::BitandUint, true, bargs)
    }
    x if x == LBF::LBF_BIT32_BNOT as i32 => {
      translate_builtin_bit_32_bnot(build, nparams, ra, arg, args, nresults, pcpos)
    }
    x if x == LBF::LBF_BIT32_LSHIFT as i32 => translate_builtin_bit_32_shift(
      build,
      IrCmd::BitlshiftUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_BIT32_RSHIFT as i32 => translate_builtin_bit_32_shift(
      build,
      IrCmd::BitrshiftUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_BIT32_ARSHIFT as i32 => translate_builtin_bit_32_shift(
      build,
      IrCmd::BitarshiftUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_BIT32_LROTATE as i32 => translate_builtin_bit_32_rotate(
      build,
      IrCmd::BitlrotateUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_BIT32_RROTATE as i32 => translate_builtin_bit_32_rotate(
      build,
      IrCmd::BitrrotateUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_BIT32_EXTRACT as i32 => {
      translate_builtin_bit_32_extract(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    x if x == LBF::LBF_BIT32_EXTRACTK as i32 => {
      translate_builtin_bit_32_extract_k(build, nparams, ra, arg, args, nresults, pcpos)
    }
    x if x == LBF::LBF_BIT32_COUNTLZ as i32 => translate_builtin_bit_32_unary(
      build,
      IrCmd::BitcountlzUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_BIT32_COUNTRZ as i32 => translate_builtin_bit_32_unary(
      build,
      IrCmd::BitcountrzUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_BIT32_REPLACE as i32 => {
      translate_builtin_bit_32_replace(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    x if x == LBF::LBF_TYPE as i32 => {
      translate_builtin_type(build, nparams, ra, arg, args, nresults)
    }
    x if x == LBF::LBF_TYPEOF as i32 => {
      translate_builtin_typeof(build, nparams, ra, arg, args, nresults)
    }
    x if x == LBF::LBF_VECTOR as i32 => {
      translate_builtin_vector(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    x if x == LBF::LBF_TABLE_INSERT as i32 => {
      translate_builtin_table_insert(build, nparams, ra, arg, args, nresults, pcpos)
    }
    x if x == LBF::LBF_STRING_LEN as i32 => {
      translate_builtin_string_len(build, nparams, ra, arg, args, nresults, pcpos)
    }
    x if x == LBF::LBF_BIT32_BYTESWAP as i32 => translate_builtin_bit_32_unary(
      build,
      IrCmd::ByteswapUint,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_BUFFER_READI8 as i32 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadi8,
      1,
      IrCmd::IntToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    x if x == LBF::LBF_BUFFER_READU8 as i32 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadu8,
      1,
      IrCmd::IntToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    x if x == LBF::LBF_BUFFER_WRITEU8 as i32 => translate_builtin_buffer_write(
      build,
      bargs,
      IrCmd::BufferWritei8,
      1,
      IrCmd::NumToUint,
      false,
    ),
    x if x == LBF::LBF_BUFFER_READI16 as i32 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadi16,
      2,
      IrCmd::IntToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    x if x == LBF::LBF_BUFFER_READU16 as i32 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadu16,
      2,
      IrCmd::IntToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    x if x == LBF::LBF_BUFFER_WRITEU16 as i32 => translate_builtin_buffer_write(
      build,
      bargs,
      IrCmd::BufferWritei16,
      2,
      IrCmd::NumToUint,
      false,
    ),
    x if x == LBF::LBF_BUFFER_READI32 as i32 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadi32,
      4,
      IrCmd::IntToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    x if x == LBF::LBF_BUFFER_READU32 as i32 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadi32,
      4,
      IrCmd::UintToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    x if x == LBF::LBF_BUFFER_WRITEU32 as i32 => translate_builtin_buffer_write(
      build,
      bargs,
      IrCmd::BufferWritei32,
      4,
      IrCmd::NumToUint,
      false,
    ),
    x if x == LBF::LBF_BUFFER_READF32 as i32 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadf32,
      4,
      IrCmd::FloatToNum,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    x if x == LBF::LBF_BUFFER_WRITEF32 as i32 => translate_builtin_buffer_write(
      build,
      bargs,
      IrCmd::BufferWritef32,
      4,
      IrCmd::NumToFloat,
      false,
    ),
    x if x == LBF::LBF_BUFFER_READF64 as i32 => translate_builtin_buffer_read(
      build,
      bargs,
      IrCmd::BufferReadf64,
      8,
      IrCmd::NOP,
      IrCmd::StoreDouble,
      LuaType::Number as u8,
    ),
    x if x == LBF::LBF_BUFFER_WRITEF64 as i32 => {
      translate_builtin_buffer_write(build, bargs, IrCmd::BufferWritef64, 8, IrCmd::NOP, false)
    }
    x if x == LBF::LBF_BUFFER_READINTEGER as i32 => {
      if FFlag::LuauCodegenBufferInteger.get() {
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
    x if x == LBF::LBF_BUFFER_WRITEINTEGER as i32 => {
      if FFlag::LuauCodegenBufferInteger.get() {
        translate_builtin_buffer_write(build, bargs, IrCmd::BufferWritei64, 8, IrCmd::NOP, true)
      } else {
        no_builtin()
      }
    }
    x if x == LBF::LBF_VECTOR_MAGNITUDE as i32 => {
      translate_builtin_vector_magnitude(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    x if x == LBF::LBF_VECTOR_NORMALIZE as i32 => {
      translate_builtin_vector_normalize(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    x if x == LBF::LBF_VECTOR_CROSS as i32 => {
      translate_builtin_vector_cross(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    x if x == LBF::LBF_VECTOR_DOT as i32 => {
      translate_builtin_vector_dot(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    x if x == LBF::LBF_VECTOR_FLOOR as i32 => translate_builtin_vector_map_1_x_4(
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
    x if x == LBF::LBF_VECTOR_CEIL as i32 => translate_builtin_vector_map_1_x_4(
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
    x if x == LBF::LBF_VECTOR_ABS as i32 => translate_builtin_vector_map_1_x_4(
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
    x if x == LBF::LBF_VECTOR_SIGN as i32 => translate_builtin_vector_map_1(
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
    x if x == LBF::LBF_VECTOR_CLAMP as i32 => translate_builtin_vector_clamp(
      build, nparams, ra, arg, args, arg3, nresults, fallback, pcpos,
    ),
    x if x == LBF::LBF_VECTOR_MIN as i32 => translate_builtin_vector_min_max(
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
    x if x == LBF::LBF_VECTOR_MAX as i32 => translate_builtin_vector_min_max(
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
    x if x == LBF::LBF_VECTOR_LERP as i32 => {
      translate_builtin_vector_lerp(build, nparams, ra, arg, args, arg3, nresults, pcpos)
    }
    x if x == LBF::LBF_MATH_LERP as i32 => translate_builtin_math_lerp(
      build, nparams, ra, arg, args, arg3, nresults, fallback, pcpos,
    ),
    x if x == LBF::LBF_MATH_ISNAN as i32 => {
      translate_builtin_math_is_nan(build, nparams, ra, arg, args, nresults, pcpos)
    }
    x if x == LBF::LBF_INTEGER_CREATE as i32 => {
      if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_create(build, nparams, ra, arg, nresults, pcpos)
      } else {
        no_builtin()
      }
    }
    x if x == LBF::LBF_INTEGER_TONUMBER as i32 => {
      if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_to_number(build, nparams, ra, arg, nresults, pcpos)
      } else {
        no_builtin()
      }
    }
    x if x == LBF::LBF_INTEGER_ADD as i32 => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Add,
    ),
    x if x == LBF::LBF_INTEGER_SUB as i32 => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Sub,
    ),
    x if x == LBF::LBF_INTEGER_MUL as i32 => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Mul,
    ),
    x if x == LBF::LBF_INTEGER_DIV as i32 => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Div,
    ),
    x if x == LBF::LBF_INTEGER_IDIV as i32 => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Idiv,
    ),
    x if x == LBF::LBF_INTEGER_UDIV as i32 => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Udiv,
    ),
    x if x == LBF::LBF_INTEGER_REM as i32 => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Rem,
    ),
    x if x == LBF::LBF_INTEGER_UREM as i32 => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Urem,
    ),
    x if x == LBF::LBF_INTEGER_MOD as i32 => int2_binary(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      Int64Binary::Mod,
    ),
    x if x == LBF::LBF_INTEGER_MIN as i32 => {
      if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_min_max(build, nparams, ra, arg, args, arg3, nresults, pcpos, true)
      } else {
        no_builtin()
      }
    }
    x if x == LBF::LBF_INTEGER_MAX as i32 => {
      if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_min_max(
          build, nparams, ra, arg, args, arg3, nresults, pcpos, false,
        )
      } else {
        no_builtin()
      }
    }
    x if x == LBF::LBF_INTEGER_NEG as i32 => {
      if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_neg(build, nparams, ra, arg, nresults, pcpos)
      } else {
        no_builtin()
      }
    }
    x if x == LBF::LBF_INTEGER_CLAMP as i32 => {
      if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_clamp(build, nparams, ra, arg, args, arg3, nresults, pcpos)
      } else {
        no_builtin()
      }
    }
    x if x == LBF::LBF_INTEGER_LT as i32 => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::Less,
    ),
    x if x == LBF::LBF_INTEGER_LE as i32 => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::LessEqual,
    ),
    x if x == LBF::LBF_INTEGER_GT as i32 => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::Greater,
    ),
    x if x == LBF::LBF_INTEGER_GE as i32 => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::GreaterEqual,
    ),
    x if x == LBF::LBF_INTEGER_ULT as i32 => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::UnsignedLess,
    ),
    x if x == LBF::LBF_INTEGER_ULE as i32 => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::UnsignedLessEqual,
    ),
    x if x == LBF::LBF_INTEGER_UGT as i32 => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::UnsignedGreater,
    ),
    x if x == LBF::LBF_INTEGER_UGE as i32 => int2_compare(
      build,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
      IrCondition::UnsignedGreaterEqual,
    ),
    x if x == LBF::LBF_INTEGER_BAND as i32 => {
      int2_multiarg(build, IrCmd::BitandInt64, false, -1, bargs)
    }
    x if x == LBF::LBF_INTEGER_BOR as i32 => {
      int2_multiarg(build, IrCmd::BitorInt64, false, 0, bargs)
    }
    x if x == LBF::LBF_INTEGER_BXOR as i32 => {
      int2_multiarg(build, IrCmd::BitxorInt64, false, 0, bargs)
    }
    x if x == LBF::LBF_INTEGER_BNOT as i32 => {
      if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_bnot(build, nparams, ra, arg, nresults, pcpos)
      } else {
        no_builtin()
      }
    }
    x if x == LBF::LBF_INTEGER_BTEST as i32 => {
      int2_multiarg(build, IrCmd::BitandInt64, true, -1, bargs)
    }
    x if x == LBF::LBF_INTEGER_LSHIFT as i32 => int2_shift(
      build,
      IrCmd::BitlshiftInt64,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_INTEGER_RSHIFT as i32 => int2_shift(
      build,
      IrCmd::BitrshiftInt64,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_INTEGER_ARSHIFT as i32 => int2_shift(
      build,
      IrCmd::BitarshiftInt64,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_INTEGER_LROTATE as i32 => int2_rotate(
      build,
      IrCmd::BitlrotateInt64,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_INTEGER_RROTATE as i32 => int2_rotate(
      build,
      IrCmd::BitrrotateInt64,
      nparams,
      ra,
      arg,
      args,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_INTEGER_COUNTLZ as i32 => int2_unary(
      build,
      IrCmd::BitcountlzInt64,
      nparams,
      ra,
      arg,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_INTEGER_COUNTRZ as i32 => int2_unary(
      build,
      IrCmd::BitcountrzInt64,
      nparams,
      ra,
      arg,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_INTEGER_BSWAP as i32 => int2_unary(
      build,
      IrCmd::ByteswapInt64,
      nparams,
      ra,
      arg,
      nresults,
      pcpos,
    ),
    x if x == LBF::LBF_INTEGER_EXTRACT as i32 => {
      if FFlag::LuauCodegenInteger2.get() {
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
  if FFlag::LuauCodegenInteger2.get() {
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
  if FFlag::LuauCodegenInteger2.get() {
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
  if FFlag::LuauCodegenInteger2.get() {
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
  if FFlag::LuauCodegenInteger2.get() {
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
  if FFlag::LuauCodegenInteger2.get() {
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
  if FFlag::LuauCodegenInteger2.get() {
    translate_builtin_int_64_unary(build, cmd, nparams, ra, arg, nresults, pcpos)
  } else {
    no_builtin()
  }
}
