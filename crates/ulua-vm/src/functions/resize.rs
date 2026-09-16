use core::{
  mem::size_of,
  ptr::{addr_of, addr_of_mut, eq},
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    arrayornewkey::arrayornewkey, lua_m_free::luaM_free_, lua_m_realloc::lua_m_realloc_,
    newkey::newkey, runerror::runerror, setarrayvector::setarrayvector,
    setnodevector::setnodevector,
  },
  macros::{
    cast_num::cast_num, dummynode::dummynode, getnodekey::getnodekey, gkey::gval,
    setnvalue::setnvalue, setobjt_2_t::setobjt2t, ttisnil::ttisnil,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

const MAXSIZE: i32 = 1 << 26;

pub(crate) unsafe fn resize(l: *mut lua_State, t: *mut LuaTable, nasize: i32, nhsize: i32) {
  unsafe {
    if nasize > MAXSIZE || nhsize > MAXSIZE {
      runerror(l, c"table overflow".as_ptr());
    }

    let oldasize = (*t).sizearray;
    let oldhsize = (*t).lsizenode;
    let nold = (*t).node;

    if nasize > oldasize {
      setarrayvector(l, t, nasize);
    }

    setnodevector(l, t, nhsize);
    let nnew = (*t).node;

    if nasize < oldasize {
      (*t).sizearray = nasize;

      for i in nasize..oldasize {
        let e = (*t).array.add(i as usize);
        if !ttisnil!(e) {
          let mut ok = TValue::default();
          setnvalue!(addr_of_mut!(ok), cast_num!(i + 1));
          setobjt2t!(l, newkey(l, t, addr_of!(ok)), e);
        }
      }

      let newarray = lua_m_realloc_(
        l,
        (*t).array as *mut u8,
        oldasize as usize * size_of::<TValue>(),
        nasize as usize * size_of::<TValue>(),
        (*t).memcat,
      ) as *mut TValue;
      (*t).array = newarray;
    }

    let anew = (*t).array;

    let oldhsize_slots = 1i32 << oldhsize;
    for i in (0..oldhsize_slots).rev() {
      let old = nold.add(i as usize);
      if !ttisnil!(gval!(old)) {
        let mut ok = TValue::default();
        getnodekey!(l, addr_of_mut!(ok), old);
        setobjt2t!(l, arrayornewkey(l, t, addr_of!(ok)), gval!(old));
      }
    }

    LUAU_ASSERT!(nnew == (*t).node);
    LUAU_ASSERT!(anew == (*t).array);

    if !eq(nold, dummynode) {
      luaM_free_(
        l,
        nold as *mut u8,
        oldhsize_slots as usize * size_of::<LuaNode>(),
        (*t).memcat,
      );
    }
  }
}
