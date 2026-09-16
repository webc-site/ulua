use core::ffi::c_void;

use ulua_vm::{
  enums::tms::TMS,
  functions::{
    lua_c_barrierback::lua_c_barrierback_export,
    lua_c_barrierf::lua_c_barrierf_export,
    lua_c_barriertable::lua_c_barriertable_export,
    lua_c_step::lua_c_step_export,
    lua_f_close::lua_f_close_export,
    lua_f_findupval::lua_f_findupval_export,
    lua_f_new_lclosure::lua_f_new_lclosure_export,
    lua_h_clone::lua_h_clone_export,
    lua_h_getn::lua_h_getn_export,
    lua_h_new::lua_h_new_export,
    lua_h_resizearray::lua_h_resizearray_export,
    lua_h_setnum::lua_h_setnum_export,
    lua_t_gettm::lua_t_gettm_export,
    lua_t_objtypenamestr::lua_t_objtypenamestr_export,
    lua_v_concat::lua_v_concat_export,
    lua_v_doarithimpl::{
      lua_v_doarithimpl_tm_add, lua_v_doarithimpl_tm_div, lua_v_doarithimpl_tm_idiv,
      lua_v_doarithimpl_tm_mod, lua_v_doarithimpl_tm_mul, lua_v_doarithimpl_tm_pow,
      lua_v_doarithimpl_tm_sub, lua_v_doarithimpl_tm_unm,
    },
    lua_v_dolen::lua_v_dolen_export,
    lua_v_equalval::lua_v_equalval_export,
    lua_v_gettable::lua_v_gettable_export,
    lua_v_lessequal::lua_v_lessequal_export,
    lua_v_lessthan::lua_v_lessthan_export,
    lua_v_settable::lua_v_settable_export,
  },
  records::{closure::Closure, lua_table::LuaTable, udata::Udata},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  functions::{
    call_epilog_c::call_epilog_c_export, call_fallback::call_fallback_export,
    call_prolog::call_prolog_export, execute_dupclosure::execute_dupclosure_export,
    execute_forgprep::execute_forgprep_export, execute_getglobal::execute_getglobal_export,
    execute_gettableks::execute_gettableks_export,
    execute_getvarargs_const::execute_getvarargsconst,
    execute_getvarargs_mult_ret::execute_getvarargsmult_ret,
    execute_namecall::execute_namecall_export, execute_prepvarargs::execute_prepvarargs_export,
    execute_setglobal::execute_setglobal_export, execute_setlist::execute_setlist_export,
    execute_settableks::execute_settableks_export, forg_loop_node_iter::forg_loop_node_iter_export,
    forg_loop_non_table_fallback::forg_loop_non_table_fallback_export,
    forg_loop_non_table_fallback_deprecated::forg_loop_non_table_fallback_deprecated_export,
    forg_loop_table_iter::forg_loop_table_iter_export,
    forg_prep_xnext_fallback::forg_prep_xnext_fallback_export, get_import::get_import_export,
    new_userdata::new_userdata_export,
  },
  records::c_math,
  type_aliases::{instruction_ir_builder::Instruction, lua_state::lua_State},
};

// Function pointer types for NativeContext
pub type NativeCompareFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, lhs: *const TValue, rhs: *const TValue) -> i32;

pub type NativeArithFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, ra: StkId, rb: *const TValue, rc: *const TValue);

pub type NativeDolenFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, ra: StkId, rb: *const TValue);

pub type NativeTableAccessFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, t: *const TValue, key: *mut TValue, val: StkId);

pub type NativeConcatFn = unsafe extern "C-unwind" fn(l: *mut lua_State, total: i32, last: i32);

pub type NativeHGetnFn = unsafe extern "C-unwind" fn(t: *mut c_void) -> i32;

pub type NativeHNewFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, narray: i32, lnhash: i32) -> *mut c_void;

pub type NativeHCloneFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, tt: *mut c_void) -> *mut c_void;

pub type NativeHResizearrayFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, t: *mut c_void, nasize: i32);

pub type NativeHSetnumFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, t: *mut c_void, key: i32) -> *mut TValue;

pub type NativeCBarriertableFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, t: *mut c_void, v: *mut c_void);

pub type NativeCBarrierfFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, o: *mut c_void, v: *mut c_void);

pub type NativeCBarrierbackFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, o: *mut c_void, gclist: *mut *mut c_void);

pub type NativeCStepFn = unsafe extern "C-unwind" fn(l: *mut lua_State, assist: bool) -> usize;

pub type NativeFCloseFn = unsafe extern "C-unwind" fn(l: *mut lua_State, level: StkId);

pub type NativeFFindupvalFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, level: StkId) -> *mut c_void;

pub type NativeFNewLclosureFn = unsafe extern "C-unwind" fn(
  l: *mut lua_State,
  nelems: i32,
  e: *mut c_void,
  p: *mut c_void,
) -> *mut c_void;

pub type NativeTGettmFn =
  unsafe extern "C-unwind" fn(events: *mut c_void, event: TMS, ename: *mut c_void) -> *const TValue;

pub type NativeTObjtypenamestrFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, o: *const TValue) -> *const c_void;

pub type NativeMathUnaryFn = unsafe extern "C-unwind" fn(f64) -> f64;

pub type NativeMathBinaryFn = unsafe extern "C-unwind" fn(f64, f64) -> f64;

pub type NativeMathLdexpFn = unsafe extern "C-unwind" fn(f64, i32) -> f64;

pub type NativeMathFrexpFn = unsafe extern "C-unwind" fn(f64, *mut i32) -> f64;

pub type NativeMathModfFn = unsafe extern "C-unwind" fn(f64, *mut f64) -> f64;

pub type NativeForgLoopIterFn = unsafe extern "C-unwind" fn(
  l: *mut lua_State,
  h: *mut LuaTable,
  index: i32,
  ra: *mut TValue,
) -> bool;

pub type NativeForgLoopFallbackFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, insn_a: i32, aux: i32) -> i32;

pub type NativeForgLoopFallbackDeprecatedFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, insn_a: i32, aux: i32) -> bool;

pub type NativeForgPrepXnextFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, ra: *mut TValue, pc: i32);

pub type NativeCallPrologFn = unsafe extern "C-unwind" fn(
  l: *mut lua_State,
  ra: *mut TValue,
  argtop: StkId,
  nresults: i32,
) -> *mut Closure;

pub type NativeCallEpilogCFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, nresults: i32, n: i32);

pub type NativeNewUserdataFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, s: usize, tag: i32) -> *mut Udata;

pub type NativeGetImportFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, res: StkId, id: u32, pc: u32);

pub type NativeCallFallbackFn = unsafe extern "C-unwind" fn(
  l: *mut lua_State,
  ra: StkId,
  argtop: StkId,
  nresults: i32,
) -> *mut Closure;

pub type NativeExecuteOpcodeFn = unsafe extern "C-unwind" fn(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction;

pub type NativeExecuteGetvarargsMultRetFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, pc: *const Instruction, base: StkId, rai: i32);

pub type NativeExecuteGetvarargsConstFn =
  unsafe extern "C-unwind" fn(l: *mut lua_State, base: StkId, rai: i32, b: i32);

// Rust 原生函数指针常量（无需 extern 声明块，直接引用仓库内符号）
pub const LUA_V_LESSTHAN: NativeCompareFn = lua_v_lessthan_export;
pub const LUA_V_LESSEQUAL: NativeCompareFn = lua_v_lessequal_export;
pub const LUA_V_EQUALVAL: NativeCompareFn = lua_v_equalval_export;

pub const LUA_V_DOARITH_ADD: NativeArithFn = lua_v_doarithimpl_tm_add;
pub const LUA_V_DOARITH_SUB: NativeArithFn = lua_v_doarithimpl_tm_sub;
pub const LUA_V_DOARITH_MUL: NativeArithFn = lua_v_doarithimpl_tm_mul;
pub const LUA_V_DOARITH_DIV: NativeArithFn = lua_v_doarithimpl_tm_div;
pub const LUA_V_DOARITH_IDIV: NativeArithFn = lua_v_doarithimpl_tm_idiv;
pub const LUA_V_DOARITH_MOD: NativeArithFn = lua_v_doarithimpl_tm_mod;
pub const LUA_V_DOARITH_POW: NativeArithFn = lua_v_doarithimpl_tm_pow;
pub const LUA_V_DOARITH_UNM: NativeArithFn = lua_v_doarithimpl_tm_unm;

pub const LUA_V_DOLEN: NativeDolenFn = lua_v_dolen_export;
pub const LUA_V_GETTABLE: NativeTableAccessFn = lua_v_gettable_export;
pub const LUA_V_SETTABLE: NativeTableAccessFn = lua_v_settable_export;
pub const LUA_V_CONCAT: NativeConcatFn = lua_v_concat_export;

pub const LUA_H_GETN: NativeHGetnFn = lua_h_getn_export;
pub const LUA_H_NEW: NativeHNewFn = lua_h_new_export;
pub const LUA_H_CLONE: NativeHCloneFn = lua_h_clone_export;
pub const LUA_H_RESIZEARRAY: NativeHResizearrayFn = lua_h_resizearray_export;
pub const LUA_H_SETNUM: NativeHSetnumFn = lua_h_setnum_export;

pub const LUA_C_BARRIERTABLE: NativeCBarriertableFn = lua_c_barriertable_export;
pub const LUA_C_BARRIERF: NativeCBarrierfFn = lua_c_barrierf_export;
pub const LUA_C_BARRIERBACK: NativeCBarrierbackFn = lua_c_barrierback_export;
pub const LUA_C_STEP: NativeCStepFn = lua_c_step_export;

pub const LUA_F_CLOSE: NativeFCloseFn = lua_f_close_export;
pub const LUA_F_FINDUPVAL: NativeFFindupvalFn = lua_f_findupval_export;
pub const LUA_F_NEW_LCLOSURE: NativeFNewLclosureFn = lua_f_new_lclosure_export;

pub const LUA_T_GETTM: NativeTGettmFn = lua_t_gettm_export;
pub const LUA_T_OBJTYPENAMESTR: NativeTObjtypenamestrFn = lua_t_objtypenamestr_export;

pub const LIBM_EXP: NativeMathUnaryFn = c_math::exp;
pub const LIBM_POW: NativeMathBinaryFn = c_math::pow;
pub const LIBM_FMOD: NativeMathBinaryFn = c_math::fmod;
pub const LIBM_LOG: NativeMathUnaryFn = c_math::log;
pub const LIBM_LOG2: NativeMathUnaryFn = c_math::log2;
pub const LIBM_LOG10: NativeMathUnaryFn = c_math::log10;
pub const LIBM_LDEXP: NativeMathLdexpFn = c_math::ldexp;
pub const LIBM_ROUND: NativeMathUnaryFn = c_math::round;
pub const LIBM_FREXP: NativeMathFrexpFn = c_math::frexp;
pub const LIBM_MODF: NativeMathModfFn = c_math::modf;

pub const LIBM_ASIN: NativeMathUnaryFn = c_math::asin;
pub const LIBM_SIN: NativeMathUnaryFn = c_math::sin;
pub const LIBM_SINH: NativeMathUnaryFn = c_math::sinh;
pub const LIBM_ACOS: NativeMathUnaryFn = c_math::acos;
pub const LIBM_COS: NativeMathUnaryFn = c_math::cos;
pub const LIBM_COSH: NativeMathUnaryFn = c_math::cosh;
pub const LIBM_ATAN: NativeMathUnaryFn = c_math::atan;
pub const LIBM_ATAN2: NativeMathBinaryFn = c_math::atan2;
pub const LIBM_TAN: NativeMathUnaryFn = c_math::tan;
pub const LIBM_TANH: NativeMathUnaryFn = c_math::tanh;

pub const FORG_LOOP_TABLE_ITER: NativeForgLoopIterFn = forg_loop_table_iter_export;
pub const FORG_LOOP_NODE_ITER: NativeForgLoopIterFn = forg_loop_node_iter_export;
pub const FORG_LOOP_NON_TABLE_FALLBACK: NativeForgLoopFallbackFn =
  forg_loop_non_table_fallback_export;
pub const FORG_LOOP_NON_TABLE_FALLBACK_DEPRECATED: NativeForgLoopFallbackDeprecatedFn =
  forg_loop_non_table_fallback_deprecated_export;
pub const FORG_PREP_XNEXT_FALLBACK: NativeForgPrepXnextFn = forg_prep_xnext_fallback_export;
pub const CALL_PROLOG: NativeCallPrologFn = call_prolog_export;
pub const CALL_EPILOG_C: NativeCallEpilogCFn = call_epilog_c_export;
pub const NEW_USERDATA: NativeNewUserdataFn = new_userdata_export;
pub const GET_IMPORT: NativeGetImportFn = get_import_export;
pub const CALL_FALLBACK: NativeCallFallbackFn = call_fallback_export;

pub const EXECUTE_GETGLOBAL: NativeExecuteOpcodeFn = execute_getglobal_export;
pub const EXECUTE_SETGLOBAL: NativeExecuteOpcodeFn = execute_setglobal_export;
pub const EXECUTE_GETTABLEKS: NativeExecuteOpcodeFn = execute_gettableks_export;
pub const EXECUTE_SETTABLEKS: NativeExecuteOpcodeFn = execute_settableks_export;
pub const EXECUTE_NAMECALL: NativeExecuteOpcodeFn = execute_namecall_export;
pub const EXECUTE_FORGPREP: NativeExecuteOpcodeFn = execute_forgprep_export;
pub const EXECUTE_GETVARARGSMULT_RET: NativeExecuteGetvarargsMultRetFn = execute_getvarargsmult_ret;
pub const EXECUTE_GETVARARGSCONST: NativeExecuteGetvarargsConstFn = execute_getvarargsconst;
pub const EXECUTE_DUPCLOSURE: NativeExecuteOpcodeFn = execute_dupclosure_export;
pub const EXECUTE_PREPVARARGS: NativeExecuteOpcodeFn = execute_prepvarargs_export;
pub const EXECUTE_SETLIST: NativeExecuteOpcodeFn = execute_setlist_export;
