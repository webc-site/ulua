//! Node: `cxx:Function:Luau.VM:VM/src/lvmutils.cpp:182:luaV_settable`
//! Source: `VM/src/lvmutils.cpp:182-240` (hand-ported)

use core::{mem::zeroed, ptr::null};

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::tms::TMS,
  functions::{
    call_tm::call_tm, lua_g_indexerror::luaG_indexerror,
    lua_g_missingmembererror::luaG_missingmembererror, lua_g_readonlyerror::luaG_readonlyerror,
    lua_h_get::lua_h_get, lua_t_gettmbyobj::lua_t_gettmbyobj,
  },
  macros::{
    fasttm::fasttm, gval_2_slot::gval2slot, hvalue::hvalue, lua_c_barrier::luaC_barrier,
    lua_c_barriert::luaC_barriert, lua_g_runerror::lua_g_runerror, lua_h_setslot::luaH_setslot,
    maxtagloop::MAXTAGLOOP, nvalue::nvalue, objectvalue::objectvalue, setobj::setobj,
    setobj_2_class::setobj2class, setobj_2_t::setobj2t, ttisfunction::ttisfunction,
    ttisnil::ttisnil, ttisobject::ttisobject, ttistable::ttistable,
  },
  records::luau_object::LuauObject,
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_v_settable(
  l: *mut lua_State,
  mut t: *const TValue,
  key: *mut TValue,
  val: StkId,
) {
  unsafe {
    let mut temp: TValue = zeroed();
    let mut loop_ = 0;
    while loop_ < MAXTAGLOOP {
      let mut tm: *const TValue = null();
      if ttistable!(t) {
        let h = hvalue!(t);

        let oldval = lua_h_get(h, key as *const TValue);

        if ttisnil!(oldval) {
          tm = fasttm(l, (*h).metatable, TMS::TmNewIndex as i32);
        }

        if !ttisnil!(oldval) || tm.is_null() {
          if (*h).readonly != 0 {
            luaG_readonlyerror(l);
          }

          let newval = luaH_setslot!(l, h, oldval, key as *const TValue);

          (*l).cachedslot = gval2slot!(h, newval as *const TValue);

          setobj2t!(l, newval, val as *const TValue);
          luaC_barriert!(l, h, val as *const TValue);
          return;
        }
      } else if FFlag::DebugLuauUserDefinedClassesRuntime.get() && ttisobject!(t) {
        let inst = &mut **objectvalue!(t) as *mut LuauObject;
        let offset = lua_h_get((*(*inst).lclass).memberstooffset, key as *const TValue);
        if ttisnil!(offset) {
          luaG_missingmembererror(l, t, key as *const TValue);
        }
        let offsetnum = nvalue!(offset) as i32;
        LUAU_ASSERT!(offsetnum >= 0 && offsetnum < (*(*inst).lclass).numberofallmembers);
        if offsetnum >= (*(*inst).lclass).numberofinstancemembers {
          luaG_indexerror(l, t, key as *const TValue);
        }
        setobj2class!(
          l,
          (*inst).members.add(offsetnum as usize),
          val as *const TValue
        );
        luaC_barrier!(l, inst, val as *const TValue);
        return;
      } else {
        tm = lua_t_gettmbyobj(l, t, TMS::TmNewIndex);
        if ttisnil!(tm) {
          luaG_indexerror(l, t, key as *const TValue);
        }
      }

      if ttisfunction!(tm) {
        call_tm(l, tm, t, key as *const TValue, val as *const TValue);
        return;
      }
      setobj!(l, &mut temp as *mut TValue, tm);
      t = &temp as *const TValue;
      loop_ += 1;
    }
    lua_g_runerror!(l, "'__newindex' chain too long; possible loop");
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaV_settable")]
pub unsafe extern "C-unwind" fn lua_v_settable_export(
  l: *mut lua_State,
  t: *const TValue,
  key: *mut TValue,
  val: StkId,
) {
  unsafe {
    lua_v_settable(l, t, key, val);
  }
}

pub use lua_v_settable as luaV_settable;
