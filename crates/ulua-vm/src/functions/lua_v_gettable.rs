//! Node: `cxx:Function:Luau.VM:VM/src/lvmutils.cpp:102:luaV_gettable`
//! Source: `VM/src/lvmutils.cpp:102-180` (hand-ported)

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::tms::TMS,
  functions::{
    call_t_mres::call_t_mres, lua_g_indexerror::luaG_indexerror,
    lua_g_missingmembererror::luaG_missingmembererror, lua_h_get::lua_h_get,
    lua_t_gettmbyobj::lua_t_gettmbyobj,
  },
  macros::{
    classvalue::classvalue, fasttm::fasttm, gval_2_slot::gval2slot, hvalue::hvalue,
    lua_g_runerror::lua_g_runerror, lua_o_nilobject::luaO_nilobject,
    lua_r_lookupmemberatoffset::luaR_lookupmemberatoffset, maxtagloop::MAXTAGLOOP, nvalue::nvalue,
    objectvalue::objectvalue, setobj_2_s::setobj2s, ttisclass::ttisclass,
    ttisfunction::ttisfunction, ttisnil::ttisnil, ttisnumber::ttisnumber, ttisobject::ttisobject,
    ttistable::ttistable,
  },
  records::{luau_class::LuauClass, luau_object::LuauObject},
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_v_gettable(
  l: *mut lua_State,
  mut t: *const TValue,
  key: *mut TValue,
  val: StkId,
) {
  unsafe {
    let mut loop_ = 0;
    while loop_ < MAXTAGLOOP {
      let tm: *const TValue;
      if ttistable!(t) {
        let h = hvalue!(t);

        let res = lua_h_get(h, key as *const TValue);

        if res != luaO_nilobject {
          (*l).cachedslot = gval2slot!(h, res);
        }

        if !ttisnil!(res) {
          setobj2s!(l, val, res);
          return;
        }

        tm = fasttm(l, (*h).metatable, TMS::TmIndex as i32);
        if tm.is_null() {
          setobj2s!(l, val, res);
          return;
        }
      } else if FFlag::DebugLuauUserDefinedClassesRuntime.get() && ttisobject!(t) {
        let inst = &mut **objectvalue!(t) as *mut LuauObject;
        let offsettval = lua_h_get((*(*inst).lclass).memberstooffset, key as *const TValue);

        if ttisnil!(offsettval) {
          luaG_missingmembererror(l, t, key as *const TValue);
        }

        LUAU_ASSERT!(ttisnumber!(offsettval));
        let offset = nvalue!(offsettval) as i32;
        setobj2s!(l, val, luaR_lookupmemberatoffset!(inst, offset));
        return;
      } else if FFlag::DebugLuauUserDefinedClassesRuntime.get() && ttisclass!(t) {
        let lco = &mut **classvalue!(t) as *mut LuauClass;
        let res = lua_h_get((*lco).memberstooffset, key as *const TValue);

        if ttisnil!(res) {
          luaG_missingmembererror(l, t, key as *const TValue);
        }

        LUAU_ASSERT!(ttisnumber!(res));
        let offset = nvalue!(res) as i32;
        LUAU_ASSERT!(offset >= 0 && offset < (*lco).numberofallmembers);

        if offset < (*lco).numberofinstancemembers {
          luaG_missingmembererror(l, t, key as *const TValue);
        }

        let static_idx = (offset - (*lco).numberofinstancemembers) as usize;
        let static_slot = (*lco).staticmembers.add(static_idx);

        setobj2s!(l, val, static_slot);
        return;
      } else {
        tm = lua_t_gettmbyobj(l, t, TMS::TmIndex);
        if ttisnil!(tm) {
          luaG_indexerror(l, t, key as *const TValue);
        }
      }

      if ttisfunction!(tm) {
        call_t_mres(l, val, tm, t, key as *const TValue);
        return;
      }
      t = tm;
      loop_ += 1;
    }
    lua_g_runerror!(l, "'__index' chain too long; possible loop");
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaV_gettable")]
pub unsafe extern "C-unwind" fn lua_v_gettable_export(
  l: *mut lua_State,
  t: *const TValue,
  key: *mut TValue,
  val: StkId,
) {
  unsafe {
    lua_v_gettable(l, t, key, val);
  }
}

pub use lua_v_gettable as luaV_gettable;
