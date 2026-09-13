use ulua_vm::macros::luau_f_table::LUAU_F_TABLE;

use crate::records::{
  native_context::NativeContext,
  native_fn::{
    CALL_EPILOG_C, CALL_FALLBACK, CALL_PROLOG, EXECUTE_DUPCLOSURE, EXECUTE_FORGPREP,
    EXECUTE_GETGLOBAL, EXECUTE_GETTABLEKS, EXECUTE_GETVARARGSCONST, EXECUTE_GETVARARGSMULT_RET,
    EXECUTE_NAMECALL, EXECUTE_PREPVARARGS, EXECUTE_SETGLOBAL, EXECUTE_SETLIST, EXECUTE_SETTABLEKS,
    FORG_LOOP_NODE_ITER, FORG_LOOP_NON_TABLE_FALLBACK, FORG_LOOP_NON_TABLE_FALLBACK_DEPRECATED,
    FORG_LOOP_TABLE_ITER, FORG_PREP_XNEXT_FALLBACK, GET_IMPORT, LIBM_ACOS, LIBM_ASIN, LIBM_ATAN,
    LIBM_ATAN2, LIBM_COS, LIBM_COSH, LIBM_EXP, LIBM_FMOD, LIBM_FREXP, LIBM_LDEXP, LIBM_LOG,
    LIBM_LOG2, LIBM_LOG10, LIBM_MODF, LIBM_POW, LIBM_ROUND, LIBM_SIN, LIBM_SINH, LIBM_TAN,
    LIBM_TANH, LUA_C_BARRIERBACK, LUA_C_BARRIERF, LUA_C_BARRIERTABLE, LUA_C_STEP, LUA_F_CLOSE,
    LUA_F_FINDUPVAL, LUA_F_NEW_LCLOSURE, LUA_H_CLONE, LUA_H_GETN, LUA_H_NEW, LUA_H_RESIZEARRAY,
    LUA_H_SETNUM, LUA_T_GETTM, LUA_T_OBJTYPENAMESTR, LUA_V_CONCAT, LUA_V_DOARITH_ADD,
    LUA_V_DOARITH_DIV, LUA_V_DOARITH_IDIV, LUA_V_DOARITH_MOD, LUA_V_DOARITH_MUL, LUA_V_DOARITH_POW,
    LUA_V_DOARITH_SUB, LUA_V_DOARITH_UNM, LUA_V_DOLEN, LUA_V_EQUALVAL, LUA_V_GETTABLE,
    LUA_V_LESSEQUAL, LUA_V_LESSTHAN, LUA_V_SETTABLE, NEW_USERDATA,
  },
};

pub fn init_functions(context: &mut NativeContext) {
  context.luau_f_table = LUAU_F_TABLE;

  context.lua_v_lessthan = Some(LUA_V_LESSTHAN);
  context.lua_v_lessequal = Some(LUA_V_LESSEQUAL);
  context.lua_v_equalval = Some(LUA_V_EQUALVAL);

  context.lua_v_doarithadd = Some(LUA_V_DOARITH_ADD);
  context.lua_v_doarithsub = Some(LUA_V_DOARITH_SUB);
  context.lua_v_doarithmul = Some(LUA_V_DOARITH_MUL);
  context.lua_v_doarithdiv = Some(LUA_V_DOARITH_DIV);
  context.lua_v_doarithidiv = Some(LUA_V_DOARITH_IDIV);
  context.lua_v_doarithmod = Some(LUA_V_DOARITH_MOD);
  context.lua_v_doarithpow = Some(LUA_V_DOARITH_POW);
  context.lua_v_doarithunm = Some(LUA_V_DOARITH_UNM);

  context.lua_v_dolen = Some(LUA_V_DOLEN);
  context.lua_v_gettable = Some(LUA_V_GETTABLE);
  context.lua_v_settable = Some(LUA_V_SETTABLE);
  context.lua_v_concat = Some(LUA_V_CONCAT);

  context.lua_h_getn = Some(LUA_H_GETN);
  context.lua_h_new = Some(LUA_H_NEW);
  context.lua_h_clone = Some(LUA_H_CLONE);
  context.lua_h_resizearray = Some(LUA_H_RESIZEARRAY);
  context.lua_h_setnum = Some(LUA_H_SETNUM);

  context.lua_c_barriertable = Some(LUA_C_BARRIERTABLE);
  context.lua_c_barrierf = Some(LUA_C_BARRIERF);
  context.lua_c_barrierback = Some(LUA_C_BARRIERBACK);
  context.lua_c_step = Some(LUA_C_STEP);

  context.lua_f_close = Some(LUA_F_CLOSE);
  context.lua_f_findupval = Some(LUA_F_FINDUPVAL);
  context.lua_f_new_lclosure = Some(LUA_F_NEW_LCLOSURE);

  context.lua_t_gettm = Some(LUA_T_GETTM);
  context.lua_t_objtypenamestr = Some(LUA_T_OBJTYPENAMESTR);

  context.libm_exp = Some(LIBM_EXP);
  context.libm_pow = Some(LIBM_POW);
  context.libm_fmod = Some(LIBM_FMOD);
  context.libm_log = Some(LIBM_LOG);
  context.libm_log2 = Some(LIBM_LOG2);
  context.libm_log10 = Some(LIBM_LOG10);
  context.libm_ldexp = Some(LIBM_LDEXP);
  context.libm_round = Some(LIBM_ROUND);
  context.libm_frexp = Some(LIBM_FREXP);
  context.libm_modf = Some(LIBM_MODF);

  context.libm_asin = Some(LIBM_ASIN);
  context.libm_sin = Some(LIBM_SIN);
  context.libm_sinh = Some(LIBM_SINH);
  context.libm_acos = Some(LIBM_ACOS);
  context.libm_cos = Some(LIBM_COS);
  context.libm_cosh = Some(LIBM_COSH);
  context.libm_atan = Some(LIBM_ATAN);
  context.libm_atan2 = Some(LIBM_ATAN2);
  context.libm_tan = Some(LIBM_TAN);
  context.libm_tanh = Some(LIBM_TANH);

  context.forg_loop_table_iter = Some(FORG_LOOP_TABLE_ITER);
  context.forg_loop_node_iter = Some(FORG_LOOP_NODE_ITER);
  context.forg_loop_non_table_fallback = Some(FORG_LOOP_NON_TABLE_FALLBACK);
  context.forg_loop_non_table_fallback_deprecated = Some(FORG_LOOP_NON_TABLE_FALLBACK_DEPRECATED);
  context.forg_prep_xnext_fallback = Some(FORG_PREP_XNEXT_FALLBACK);
  context.call_prolog = Some(CALL_PROLOG);
  context.call_epilog_c = Some(CALL_EPILOG_C);
  context.new_userdata = Some(NEW_USERDATA);
  context.get_import = Some(GET_IMPORT);

  context.call_fallback = Some(CALL_FALLBACK);

  context.execute_getglobal = Some(EXECUTE_GETGLOBAL);
  context.execute_setglobal = Some(EXECUTE_SETGLOBAL);
  context.execute_gettableks = Some(EXECUTE_GETTABLEKS);
  context.execute_settableks = Some(EXECUTE_SETTABLEKS);

  context.execute_namecall = Some(EXECUTE_NAMECALL);
  context.execute_forgprep = Some(EXECUTE_FORGPREP);
  context.execute_getvarargsmult_ret = Some(EXECUTE_GETVARARGSMULT_RET);
  context.execute_getvarargsconst = Some(EXECUTE_GETVARARGSCONST);
  context.execute_dupclosure = Some(EXECUTE_DUPCLOSURE);
  context.execute_prepvarargs = Some(EXECUTE_PREPVARARGS);
  context.execute_setlist = Some(EXECUTE_SETLIST);
}
