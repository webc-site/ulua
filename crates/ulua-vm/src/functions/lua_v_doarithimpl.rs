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
        TMS::TmAdd => return set_vec_binop(ra, vb, vc, |a, b| a + b),
        TMS::TmSub => return set_vec_binop(ra, vb, vc, |a, b| a - b),
        TMS::TmMul => return set_vec_binop(ra, vb, vc, |a, b| a * b),
        TMS::TmDiv => return set_vec_binop(ra, vb, vc, |a, b| a / b),
        TMS::TmIDiv => {
          return set_vec_binop(ra, vb, vc, |a, b| luai_numidiv(a as f64, b as f64) as f32);
        }
        // 一元取负：第二个通道指针不会被 `f` 读取，复用 vb
        TMS::TmUnm => return set_vec_binop(ra, vb, vb, |a, _| -a),
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
        // 标量广播到 4 通道
        let ncs = [nc; 4];
        match op {
          TMS::TmMul => return set_vec_binop(ra, vb, ncs.as_ptr(), |a, b| a * b),
          TMS::TmDiv => return set_vec_binop(ra, vb, ncs.as_ptr(), |a, b| a / b),
          TMS::TmIDiv => {
            return set_vec_binop(ra, vb, ncs.as_ptr(), |a, b| {
              luai_numidiv(a as f64, b as f64) as f32
            });
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
        // 标量广播到 4 通道
        let nbs = [nb; 4];
        match op {
          TMS::TmMul => return set_vec_binop(ra, nbs.as_ptr(), vc, |a, b| a * b),
          TMS::TmDiv => return set_vec_binop(ra, nbs.as_ptr(), vc, |a, b| a / b),
          TMS::TmIDiv => {
            return set_vec_binop(ra, nbs.as_ptr(), vc, |a, b| {
              luai_numidiv(a as f64, b as f64) as f32
            });
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

/// 向量逐通道二元运算辅助：对 4 通道按分量执行 `f` 并写入 `ra`。
/// `f` 经内联后与逐通道手写展开等价；一元运算（TmUnm）可对两个参数传同一指针。
#[inline]
unsafe fn set_vec_binop(ra: StkId, vb: *const f32, vc: *const f32, f: impl Fn(f32, f32) -> f32) {
  unsafe {
    setvvalue!(
      ra,
      f(*vb.add(0), *vc.add(0)),
      f(*vb.add(1), *vc.add(1)),
      f(*vb.add(2), *vc.add(2)),
      f(*vb.add(3), *vc.add(3))
    );
  }
}

/// 8 个算术 tag-method 导出入口（C ABI 注册表用），各自转发到
/// `lua_v_doarithimpl` 并传入对应 `TMS`。
macro_rules! tm_exports {
  ($(($variant:ident, $snake:ident)),+ $(,)?) => {
    $(
      /// # Safety
      /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
      #[cfg_attr(
        feature = "capi",
        unsafe(export_name = concat!("ulua_luaV_doarithimpl_", stringify!($variant)))
      )]
      pub unsafe extern "C-unwind" fn $snake(
        l: *mut lua_State,
        ra: StkId,
        rb: *const TValue,
        rc: *const TValue,
      ) {
        unsafe { lua_v_doarithimpl(l, ra, rb, rc, TMS::$variant) }
      }
    )+
  };
}

tm_exports! {
  (TmAdd, lua_v_doarithimpl_tm_add),
  (TmSub, lua_v_doarithimpl_tm_sub),
  (TmMul, lua_v_doarithimpl_tm_mul),
  (TmDiv, lua_v_doarithimpl_tm_div),
  (TmIDiv, lua_v_doarithimpl_tm_idiv),
  (TmMod, lua_v_doarithimpl_tm_mod),
  (TmPow, lua_v_doarithimpl_tm_pow),
  (TmUnm, lua_v_doarithimpl_tm_unm),
}
