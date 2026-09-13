use core::ptr::{copy_nonoverlapping, eq};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    arrayornewkey::arrayornewkey, getfreepos::getfreepos, mainposition::mainposition,
    rehash::rehash,
  },
  macros::{
    dummynode::dummynode, gkey::gval, lua_c_barriert::luaC_barriert, nvalue::nvalue,
    setnilvalue::setnilvalue, ttisnil::ttisnil, ttisnumber::ttisnumber,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

#[inline]
unsafe fn getnodekey_direct(obj: *mut TValue, node: *const LuaNode) {
  unsafe {
    (*obj).value = (*node).key.value;
    copy_nonoverlapping((*node).key.extra.as_ptr(), (*obj).extra.as_mut_ptr(), 1);
    (*obj).tt = (*node).key.tt();
  }
}

#[inline]
unsafe fn setnodekey_direct(node: *mut LuaNode, obj: *const TValue) {
  unsafe {
    (*node).key.value = (*obj).value;
    copy_nonoverlapping((*obj).extra.as_ptr(), (*node).key.extra.as_mut_ptr(), 1);
    (*node).key.set_tt((*obj).tt);
  }
}

pub(crate) unsafe fn newkey(
  l: *mut lua_State,
  t: *mut LuaTable,
  key: *const TValue,
) -> *mut TValue {
  unsafe {
    if ttisnumber!(key) && nvalue!(key) == ((*t).sizearray + 1) as f64 {
      rehash(l, t, key);
      return arrayornewkey(l, t, key);
    }

    let mut mp = mainposition(t, key);
    if !ttisnil!(gval!(mp)) || eq(mp, dummynode) {
      let n = getfreepos(t);
      if n.is_null() {
        rehash(l, t, key);
        return arrayornewkey(l, t, key);
      }

      LUAU_ASSERT!(!eq(n, dummynode));

      let mut mk = TValue::default();
      getnodekey_direct(core::ptr::addr_of_mut!(mk), mp);
      let mut othern = mainposition(t, core::ptr::addr_of!(mk));

      if othern != mp {
        while othern.offset((*othern).key.next() as isize) != mp {
          othern = othern.offset((*othern).key.next() as isize);
        }

        (*othern).key.set_next(n.offset_from(othern) as i32);
        *n = *mp;

        if (*mp).key.next() != 0 {
          (*n)
            .key
            .set_next((*n).key.next() + mp.offset_from(n) as i32);
          (*mp).key.set_next(0);
        }

        setnilvalue!(gval!(mp));
      } else {
        if (*mp).key.next() != 0 {
          (*n)
            .key
            .set_next(mp.offset((*mp).key.next() as isize).offset_from(n) as i32);
        } else {
          LUAU_ASSERT!((*n).key.next() == 0);
        }

        (*mp).key.set_next(n.offset_from(mp) as i32);
        mp = n;
      }
    }

    setnodekey_direct(mp, key);
    luaC_barriert!(l, t, key);
    LUAU_ASSERT!(ttisnil!(gval!(mp)));
    gval!(mp)
  }
}
