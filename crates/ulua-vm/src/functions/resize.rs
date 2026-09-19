use core::{
  mem::size_of,
  ptr::{addr_of, addr_of_mut, eq},
  slice::from_raw_parts_mut,
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

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
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

      // 收缩掉的数组尾段重哈希：切片化遍历替代逐下标指针推进
      let tail = from_raw_parts_mut(
        (*t).array.add(nasize as usize),
        (oldasize - nasize) as usize,
      );
      for (i, e) in tail.iter_mut().enumerate() {
        if !ttisnil!(&*e) {
          let mut ok = TValue::default();
          setnvalue!(addr_of_mut!(ok), cast_num!(nasize + i as i32 + 1));
          setobjt2t!(l, newkey(l, t, addr_of!(ok)), &*e);
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

    LUAU_ASSERT!(nnew == (*t).node);
    LUAU_ASSERT!(anew == (*t).array);

    let oldhsize_slots = 1i32 << oldhsize;

    if !eq(nold, dummynode) {
      // 旧 node 全量搬到新表（倒序保持 cpp 搬移次序）；dummy 静态节点无内容可搬
      let oldnodes = from_raw_parts_mut(nold, oldhsize_slots as usize);
      for old in oldnodes.iter_mut().rev() {
        if !ttisnil!(gval!(old)) {
          let mut ok = TValue::default();
          getnodekey!(l, addr_of_mut!(ok), old);
          setobjt2t!(l, arrayornewkey(l, t, addr_of!(ok)), gval!(old));
        }
      }

      luaM_free_(
        l,
        nold as *mut u8,
        oldhsize_slots as usize * size_of::<LuaNode>(),
        (*t).memcat,
      );
    }
  }
}
