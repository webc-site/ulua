use core::mem::zeroed;

use crate::{
  records::native_fn::{
    NativeArithFn, NativeCBarrierbackFn, NativeCBarrierfFn, NativeCBarriertableFn, NativeCStepFn,
    NativeCallEpilogCFn, NativeCallFallbackFn, NativeCallPrologFn, NativeCompareFn, NativeConcatFn,
    NativeDolenFn, NativeExecuteGetvarargsConstFn, NativeExecuteGetvarargsMultRetFn,
    NativeExecuteOpcodeFn, NativeFCloseFn, NativeFFindupvalFn, NativeFNewLclosureFn,
    NativeForgLoopFallbackDeprecatedFn, NativeForgLoopFallbackFn, NativeForgLoopIterFn,
    NativeForgPrepXnextFn, NativeGetImportFn, NativeHCloneFn, NativeHGetnFn, NativeHNewFn,
    NativeHResizearrayFn, NativeHSetnumFn, NativeMathBinaryFn, NativeMathFrexpFn,
    NativeMathLdexpFn, NativeMathModfFn, NativeMathUnaryFn, NativeNewUserdataFn, NativeTGettmFn,
    NativeTObjtypenamestrFn, NativeTableAccessFn,
  },
  type_aliases::luau_fast_function::LuauFastFunction,
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct NativeContext {
  pub gate_entry: *mut u8,
  pub gate_exit: *mut u8,

  pub lua_v_lessthan: Option<NativeCompareFn>,
  pub lua_v_lessequal: Option<NativeCompareFn>,
  pub lua_v_equalval: Option<NativeCompareFn>,
  pub lua_v_doarithadd: Option<NativeArithFn>,
  pub lua_v_doarithsub: Option<NativeArithFn>,
  pub lua_v_doarithmul: Option<NativeArithFn>,
  pub lua_v_doarithdiv: Option<NativeArithFn>,
  pub lua_v_doarithidiv: Option<NativeArithFn>,
  pub lua_v_doarithmod: Option<NativeArithFn>,
  pub lua_v_doarithpow: Option<NativeArithFn>,
  pub lua_v_doarithunm: Option<NativeArithFn>,
  pub lua_v_dolen: Option<NativeDolenFn>,
  pub lua_v_gettable: Option<NativeTableAccessFn>,
  pub lua_v_settable: Option<NativeTableAccessFn>,
  pub lua_v_concat: Option<NativeConcatFn>,

  pub lua_h_getn: Option<NativeHGetnFn>,
  pub lua_h_new: Option<NativeHNewFn>,
  pub lua_h_clone: Option<NativeHCloneFn>,
  pub lua_h_resizearray: Option<NativeHResizearrayFn>,
  pub lua_h_setnum: Option<NativeHSetnumFn>,

  pub lua_c_barriertable: Option<NativeCBarriertableFn>,
  pub lua_c_barrierf: Option<NativeCBarrierfFn>,
  pub lua_c_barrierback: Option<NativeCBarrierbackFn>,
  pub lua_c_step: Option<NativeCStepFn>,

  pub lua_f_close: Option<NativeFCloseFn>,
  pub lua_f_findupval: Option<NativeFFindupvalFn>,
  pub lua_f_new_lclosure: Option<NativeFNewLclosureFn>,

  pub lua_t_gettm: Option<NativeTGettmFn>,
  pub lua_t_objtypenamestr: Option<NativeTObjtypenamestrFn>,

  pub libm_exp: Option<NativeMathUnaryFn>,
  pub libm_pow: Option<NativeMathBinaryFn>,
  pub libm_fmod: Option<NativeMathBinaryFn>,
  pub libm_asin: Option<NativeMathUnaryFn>,
  pub libm_sin: Option<NativeMathUnaryFn>,
  pub libm_sinh: Option<NativeMathUnaryFn>,
  pub libm_acos: Option<NativeMathUnaryFn>,
  pub libm_cos: Option<NativeMathUnaryFn>,
  pub libm_cosh: Option<NativeMathUnaryFn>,
  pub libm_atan: Option<NativeMathUnaryFn>,
  pub libm_atan2: Option<NativeMathBinaryFn>,
  pub libm_tan: Option<NativeMathUnaryFn>,
  pub libm_tanh: Option<NativeMathUnaryFn>,
  pub libm_log: Option<NativeMathUnaryFn>,
  pub libm_log2: Option<NativeMathUnaryFn>,
  pub libm_log10: Option<NativeMathUnaryFn>,
  pub libm_ldexp: Option<NativeMathLdexpFn>,
  pub libm_round: Option<NativeMathUnaryFn>,
  pub libm_frexp: Option<NativeMathFrexpFn>,
  pub libm_modf: Option<NativeMathModfFn>,

  pub forg_loop_table_iter: Option<NativeForgLoopIterFn>,
  pub forg_loop_node_iter: Option<NativeForgLoopIterFn>,
  pub forg_loop_non_table_fallback: Option<NativeForgLoopFallbackFn>,
  pub forg_loop_non_table_fallback_deprecated: Option<NativeForgLoopFallbackDeprecatedFn>,
  pub forg_prep_xnext_fallback: Option<NativeForgPrepXnextFn>,
  pub call_prolog: Option<NativeCallPrologFn>,
  pub call_epilog_c: Option<NativeCallEpilogCFn>,
  pub new_userdata: Option<NativeNewUserdataFn>,
  pub get_import: Option<NativeGetImportFn>,

  pub call_fallback: Option<NativeCallFallbackFn>,

  pub execute_getglobal: Option<NativeExecuteOpcodeFn>,
  pub execute_setglobal: Option<NativeExecuteOpcodeFn>,
  pub execute_gettableks: Option<NativeExecuteOpcodeFn>,
  pub execute_settableks: Option<NativeExecuteOpcodeFn>,
  pub execute_namecall: Option<NativeExecuteOpcodeFn>,
  pub execute_setlist: Option<NativeExecuteOpcodeFn>,
  pub execute_forgprep: Option<NativeExecuteOpcodeFn>,
  pub execute_getvarargsmult_ret: Option<NativeExecuteGetvarargsMultRetFn>,
  pub execute_getvarargsconst: Option<NativeExecuteGetvarargsConstFn>,
  pub execute_dupclosure: Option<NativeExecuteOpcodeFn>,
  pub execute_prepvarargs: Option<NativeExecuteOpcodeFn>,

  pub luau_f_table: [LuauFastFunction; 256],
}

impl Default for NativeContext {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}
