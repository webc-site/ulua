use core::ptr::{eq, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{lua_type::LuaType, tms::TMS},
  functions::{call_t_mres::call_t_mres, get_comp_tm::get_comp_tm, luai_veceq::luai_veceq},
  macros::{
    bvalue::bvalue, classvalue::classvalue, gcvalue::gcvalue, hvalue::hvalue, l_isfalse::l_isfalse,
    lightuserdatatag::lightuserdatatag, luai_inteq::luai_inteq, luai_numeq::luai_numeq,
    lvalue::lvalue, nvalue::nvalue, objectvalue::objectvalue, ttype::ttype, uvalue::uvalue,
    vvalue::vvalue,
  },
  type_aliases::{lua_state::LuaState, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_v_equalval(l: *mut LuaState, t1: *const TValue, t2: *const TValue) -> i32 {
  unsafe {
    let tm: *const TValue;
    LUAU_ASSERT!(ttype!(t1) == ttype!(t2));

    match ttype!(t1) {
      t if t == LuaType::Nil as i32 => return 1,
      t if t == LuaType::Number as i32 => {
        return if luai_numeq(nvalue!(t1), nvalue!(t2)) {
          1
        } else {
          0
        };
      }
      t if t == LuaType::Integer as i32 => {
        return if luai_inteq(lvalue!(t1) as f64, lvalue!(t2) as f64) {
          1
        } else {
          0
        };
      }
      t if t == LuaType::Vector as i32 => {
        return if luai_veceq(vvalue!(t1).as_ptr(), vvalue!(t2).as_ptr()) {
          1
        } else {
          0
        };
      }
      t if t == LuaType::Boolean as i32 => {
        return if bvalue!(t1) == bvalue!(t2) { 1 } else { 0 };
      }
      t if t == LuaType::LightUserData as i32 => {
        return if ((*t1).value.p == (*t2).value.p)
          && (lightuserdatatag!(t1) == lightuserdatatag!(t2))
        {
          1
        } else {
          0
        };
      }
      t if t == LuaType::UserData as i32 => {
        let u1 = uvalue!(t1);
        let u2 = uvalue!(t2);
        tm = get_comp_tm(l, u1.metatable, u2.metatable, TMS::TmEq);
        if tm.is_null() {
          return if eq(u1, u2) { 1 } else { 0 };
        }
      }
      t if t == LuaType::Class as i32 => {
        return if eq(classvalue!(t1), classvalue!(t2)) {
          1
        } else {
          0
        };
      }
      t if t == LuaType::Object as i32 => {
        let t1inst = objectvalue!(t1);
        let t2inst = objectvalue!(t2);
        let mt1 = if t1inst.lclass.is_null() {
          null_mut()
        } else {
          (*t1inst.lclass).instancemetatable
        };
        let mt2 = if t2inst.lclass.is_null() {
          null_mut()
        } else {
          (*t2inst.lclass).instancemetatable
        };
        tm = get_comp_tm(l, mt1, mt2, TMS::TmEq);
        if tm.is_null() {
          return if eq(t1inst, t2inst) { 1 } else { 0 };
        }
      }
      t if t == LuaType::Table as i32 => {
        let h1 = hvalue!(t1);
        let h2 = hvalue!(t2);
        tm = get_comp_tm(l, (*h1).metatable, (*h2).metatable, TMS::TmEq);
        if tm.is_null() {
          return if eq(h1, h2) { 1 } else { 0 };
        }
      }
      _ => {
        return if eq(gcvalue!(t1), gcvalue!(t2)) { 1 } else { 0 };
      }
    }

    call_t_mres(l, (*l).top, tm, t1, t2);
    if !l_isfalse!((*l).top) { 1 } else { 0 }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaV_equalval")]
pub unsafe extern "C-unwind" fn lua_v_equalval_export(
  l: *mut LuaState,
  t1: *const TValue,
  t2: *const TValue,
) -> i32 {
  unsafe { lua_v_equalval(l, t1, t2) }
}
