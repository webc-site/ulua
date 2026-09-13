use core::ptr::null;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::tms::TMS,
  functions::{
    call_bin_tm::call_bin_tm, lua_g_aritherror::luaG_aritherror, lua_v_tonumber::lua_v_tonumber,
    luai_numidiv::luai_numidiv, luai_nummod::luai_nummod,
  },
  macros::{
    cast_to::cast_to, luai_numadd::luai_numadd, luai_numdiv::luai_numdiv, luai_nummul::luai_nummul,
    luai_numpow::luai_numpow, luai_numsub::luai_numsub, luai_numunm::luai_numunm, nvalue::nvalue,
    setnvalue::setnvalue, setvvalue::setvvalue, ttisnumber::ttisnumber, ttisvector::ttisvector,
    vvalue::vvalue,
  },
  records::lua_state::lua_State,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_v_doarithimpl(
  l: *mut lua_State,
  ra: StkId,
  rb: *const TValue,
  rc: *const TValue,
  op: TMS,
) {
  unsafe {
    let mut tempb = TValue::default();
    let mut tempc = TValue::default();
    let _b: *const TValue;
    let _c: *const TValue;

    let vb = if ttisvector!(rb) {
      vvalue!(rb).as_ptr()
    } else {
      null()
    };
    let vc = if ttisvector!(rc) {
      vvalue!(rc).as_ptr()
    } else {
      null()
    };

    if !vb.is_null() && !vc.is_null() {
      match op {
        TMS::TmAdd => {
          setvvalue!(
            ra,
            *vb.add(0) + *vc.add(0),
            *vb.add(1) + *vc.add(1),
            *vb.add(2) + *vc.add(2),
            *vb.add(3) + *vc.add(3)
          );
          return;
        }
        TMS::TmSub => {
          setvvalue!(
            ra,
            *vb.add(0) - *vc.add(0),
            *vb.add(1) - *vc.add(1),
            *vb.add(2) - *vc.add(2),
            *vb.add(3) - *vc.add(3)
          );
          return;
        }
        TMS::TmMul => {
          setvvalue!(
            ra,
            *vb.add(0) * *vc.add(0),
            *vb.add(1) * *vc.add(1),
            *vb.add(2) * *vc.add(2),
            *vb.add(3) * *vc.add(3)
          );
          return;
        }
        TMS::TmDiv => {
          setvvalue!(
            ra,
            *vb.add(0) / *vc.add(0),
            *vb.add(1) / *vc.add(1),
            *vb.add(2) / *vc.add(2),
            *vb.add(3) / *vc.add(3)
          );
          return;
        }
        TMS::TmIDiv => {
          setvvalue!(
            ra,
            luai_numidiv(*vb.add(0) as f64, *vc.add(0) as f64) as f32,
            luai_numidiv(*vb.add(1) as f64, *vc.add(1) as f64) as f32,
            luai_numidiv(*vb.add(2) as f64, *vc.add(2) as f64) as f32,
            luai_numidiv(*vb.add(3) as f64, *vc.add(3) as f64) as f32
          );
          return;
        }
        TMS::TmUnm => {
          setvvalue!(ra, -*vb.add(0), -*vb.add(1), -*vb.add(2), -*vb.add(3));
          return;
        }
        _ => {}
      }
    } else if !vb.is_null() {
      let c_ptr = if ttisnumber!(rc) {
        rc
      } else {
        lua_v_tonumber(rc, &mut tempc)
      };
      if !c_ptr.is_null() {
        let nc = cast_to!(f32, nvalue!(c_ptr));
        match op {
          TMS::TmMul => {
            setvvalue!(
              ra,
              *vb.add(0) * nc,
              *vb.add(1) * nc,
              *vb.add(2) * nc,
              *vb.add(3) * nc
            );
            return;
          }
          TMS::TmDiv => {
            setvvalue!(
              ra,
              *vb.add(0) / nc,
              *vb.add(1) / nc,
              *vb.add(2) / nc,
              *vb.add(3) / nc
            );
            return;
          }
          TMS::TmIDiv => {
            setvvalue!(
              ra,
              luai_numidiv(*vb.add(0) as f64, nc as f64) as f32,
              luai_numidiv(*vb.add(1) as f64, nc as f64) as f32,
              luai_numidiv(*vb.add(2) as f64, nc as f64) as f32,
              luai_numidiv(*vb.add(3) as f64, nc as f64) as f32
            );
            return;
          }
          _ => {}
        }
      }
    } else if !vc.is_null() {
      let b_ptr = if ttisnumber!(rb) {
        rb
      } else {
        lua_v_tonumber(rb, &mut tempb)
      };
      if !b_ptr.is_null() {
        let nb = cast_to!(f32, nvalue!(b_ptr));
        match op {
          TMS::TmMul => {
            setvvalue!(
              ra,
              nb * *vc.add(0),
              nb * *vc.add(1),
              nb * *vc.add(2),
              nb * *vc.add(3)
            );
            return;
          }
          TMS::TmDiv => {
            setvvalue!(
              ra,
              nb / *vc.add(0),
              nb / *vc.add(1),
              nb / *vc.add(2),
              nb / *vc.add(3)
            );
            return;
          }
          TMS::TmIDiv => {
            setvvalue!(
              ra,
              luai_numidiv(nb as f64, *vc.add(0) as f64) as f32,
              luai_numidiv(nb as f64, *vc.add(1) as f64) as f32,
              luai_numidiv(nb as f64, *vc.add(2) as f64) as f32,
              luai_numidiv(nb as f64, *vc.add(3) as f64) as f32
            );
            return;
          }
          _ => {}
        }
      }
    }

    let b_res = lua_v_tonumber(rb, &mut tempb);
    let c_res = lua_v_tonumber(rc, &mut tempc);
    if !b_res.is_null() && !c_res.is_null() {
      let nb = nvalue!(b_res);
      let nc = nvalue!(c_res);
      match op {
        TMS::TmAdd => setnvalue!(ra, luai_numadd(nb, nc)),
        TMS::TmSub => setnvalue!(ra, luai_numsub(nb, nc)),
        TMS::TmMul => setnvalue!(ra, luai_nummul(nb, nc)),
        TMS::TmDiv => setnvalue!(ra, luai_numdiv(nb, nc)),
        TMS::TmIDiv => setnvalue!(ra, luai_numidiv(nb, nc)),
        TMS::TmMod => setnvalue!(ra, luai_nummod(nb, nc)),
        TMS::TmPow => setnvalue!(ra, luai_numpow(nb, nc)),
        TMS::TmUnm => setnvalue!(ra, luai_numunm(nb)),
        _ => LUAU_ASSERT!(false),
      }
    } else if call_bin_tm(l, rb, rc, ra, op) == 0 {
      luaG_aritherror(l, rb, rc, op);
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaV_doarithimpl_TM_ADD")]
pub unsafe extern "C-unwind" fn lua_v_doarithimpl_tm_add(
  l: *mut lua_State,
  ra: StkId,
  rb: *const TValue,
  rc: *const TValue,
) {
  unsafe {
    lua_v_doarithimpl(l, ra, rb, rc, TMS::TmAdd);
  }
}

#[unsafe(export_name = "ulua_luaV_doarithimpl_TM_SUB")]
/// # Safety
/// 传入的指针必须有效且指向存活对象。
pub unsafe extern "C-unwind" fn lua_v_doarithimpl_tm_sub(
  l: *mut lua_State,
  ra: StkId,
  rb: *const TValue,
  rc: *const TValue,
) {
  unsafe {
    lua_v_doarithimpl(l, ra, rb, rc, TMS::TmSub);
  }
}

#[unsafe(export_name = "ulua_luaV_doarithimpl_TM_MUL")]
/// # Safety
/// 传入的指针必须有效且指向存活对象。
pub unsafe extern "C-unwind" fn lua_v_doarithimpl_tm_mul(
  l: *mut lua_State,
  ra: StkId,
  rb: *const TValue,
  rc: *const TValue,
) {
  unsafe {
    lua_v_doarithimpl(l, ra, rb, rc, TMS::TmMul);
  }
}

#[unsafe(export_name = "ulua_luaV_doarithimpl_TM_DIV")]
/// # Safety
/// 传入的指针必须有效且指向存活对象。
pub unsafe extern "C-unwind" fn lua_v_doarithimpl_tm_div(
  l: *mut lua_State,
  ra: StkId,
  rb: *const TValue,
  rc: *const TValue,
) {
  unsafe {
    lua_v_doarithimpl(l, ra, rb, rc, TMS::TmDiv);
  }
}

#[unsafe(export_name = "ulua_luaV_doarithimpl_TM_IDIV")]
/// # Safety
/// 传入的指针必须有效且指向存活对象。
pub unsafe extern "C-unwind" fn lua_v_doarithimpl_tm_idiv(
  l: *mut lua_State,
  ra: StkId,
  rb: *const TValue,
  rc: *const TValue,
) {
  unsafe {
    lua_v_doarithimpl(l, ra, rb, rc, TMS::TmIDiv);
  }
}

#[unsafe(export_name = "ulua_luaV_doarithimpl_TM_MOD")]
/// # Safety
/// 传入的指针必须有效且指向存活对象。
pub unsafe extern "C-unwind" fn lua_v_doarithimpl_tm_mod(
  l: *mut lua_State,
  ra: StkId,
  rb: *const TValue,
  rc: *const TValue,
) {
  unsafe {
    lua_v_doarithimpl(l, ra, rb, rc, TMS::TmMod);
  }
}

#[unsafe(export_name = "ulua_luaV_doarithimpl_TM_POW")]
/// # Safety
/// 传入的指针必须有效且指向存活对象。
pub unsafe extern "C-unwind" fn lua_v_doarithimpl_tm_pow(
  l: *mut lua_State,
  ra: StkId,
  rb: *const TValue,
  rc: *const TValue,
) {
  unsafe {
    lua_v_doarithimpl(l, ra, rb, rc, TMS::TmPow);
  }
}

#[unsafe(export_name = "ulua_luaV_doarithimpl_TM_UNM")]
/// # Safety
/// 传入的指针必须有效且指向存活对象。
pub unsafe extern "C-unwind" fn lua_v_doarithimpl_tm_unm(
  l: *mut lua_State,
  ra: StkId,
  rb: *const TValue,
  rc: *const TValue,
) {
  unsafe {
    lua_v_doarithimpl(l, ra, rb, rc, TMS::TmUnm);
  }
}
