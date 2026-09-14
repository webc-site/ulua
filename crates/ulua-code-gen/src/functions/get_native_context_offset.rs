use core::mem::offset_of;

use ulua_common::enums::luau_builtin_function as lbf;

use crate::records::native_context::NativeContext;

// 模式匹配需要 i32 常量,别名枚举判别值为本地 const
const LBF_MATH_ACOS: i32 = lbf::LBF_MATH_ACOS as i32;
const LBF_MATH_ASIN: i32 = lbf::LBF_MATH_ASIN as i32;
const LBF_MATH_ATAN2: i32 = lbf::LBF_MATH_ATAN2 as i32;
const LBF_MATH_ATAN: i32 = lbf::LBF_MATH_ATAN as i32;
const LBF_MATH_COSH: i32 = lbf::LBF_MATH_COSH as i32;
const LBF_MATH_COS: i32 = lbf::LBF_MATH_COS as i32;
const LBF_MATH_EXP: i32 = lbf::LBF_MATH_EXP as i32;
const LBF_MATH_LOG10: i32 = lbf::LBF_MATH_LOG10 as i32;
const LBF_MATH_LOG: i32 = lbf::LBF_MATH_LOG as i32;
const LBF_MATH_SINH: i32 = lbf::LBF_MATH_SINH as i32;
const LBF_MATH_SIN: i32 = lbf::LBF_MATH_SIN as i32;
const LBF_MATH_TANH: i32 = lbf::LBF_MATH_TANH as i32;
const LBF_MATH_TAN: i32 = lbf::LBF_MATH_TAN as i32;
const LBF_MATH_FMOD: i32 = lbf::LBF_MATH_FMOD as i32;
const LBF_MATH_POW: i32 = lbf::LBF_MATH_POW as i32;
// CodeGen 内部虚拟 builtin id(见 cpp/CodeGen/include/Luau/IrData.h,超出 VM 枚举范围)
const LBF_IR_MATH_LOG2: i32 = 256;
const LBF_MATH_LDEXP: i32 = lbf::LBF_MATH_LDEXP as i32;

pub fn get_native_context_offset(bfid: i32) -> u32 {
  match bfid {
    LBF_MATH_ACOS => offset_of!(NativeContext, libm_acos) as u32,
    LBF_MATH_ASIN => offset_of!(NativeContext, libm_asin) as u32,
    LBF_MATH_ATAN2 => offset_of!(NativeContext, libm_atan2) as u32,
    LBF_MATH_ATAN => offset_of!(NativeContext, libm_atan) as u32,
    LBF_MATH_COSH => offset_of!(NativeContext, libm_cosh) as u32,
    LBF_MATH_COS => offset_of!(NativeContext, libm_cos) as u32,
    LBF_MATH_EXP => offset_of!(NativeContext, libm_exp) as u32,
    LBF_MATH_LOG10 => offset_of!(NativeContext, libm_log10) as u32,
    LBF_MATH_LOG => offset_of!(NativeContext, libm_log) as u32,
    LBF_MATH_SINH => offset_of!(NativeContext, libm_sinh) as u32,
    LBF_MATH_SIN => offset_of!(NativeContext, libm_sin) as u32,
    LBF_MATH_TANH => offset_of!(NativeContext, libm_tanh) as u32,
    LBF_MATH_TAN => offset_of!(NativeContext, libm_tan) as u32,
    LBF_MATH_FMOD => offset_of!(NativeContext, libm_fmod) as u32,
    LBF_MATH_POW => offset_of!(NativeContext, libm_pow) as u32,
    LBF_IR_MATH_LOG2 => offset_of!(NativeContext, libm_log2) as u32,
    LBF_MATH_LDEXP => offset_of!(NativeContext, libm_ldexp) as u32,
    _ => {
      debug_assert!(false, "Unsupported bfid");
      0
    }
  }
}
