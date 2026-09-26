use core::{
  mem::size_of,
  ptr::{addr_of, addr_of_mut, eq},
  slice::from_raw_parts_mut,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    arrayornewkey::arrayornewkey, lua_m_free::lua_m_free, lua_m_realloc::lua_m_realloc_,
    newkey::newkey, runerror::runerror, setarrayvector::setarrayvector,
    setnodevector::setnodevector,
  },
  macros::{
    dummynode::dummynode, err_table_overflow::ERR_TABLE_OVERFLOW, getnodekey::getnodekey,
    gkey::gval, maxbits::MAXSIZE, setnvalue::setnvalue, setobjt_2_t::setobjt2t,
  },
  records::{lua_node::LuaNode, lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn resize(l: *mut LuaState, t: *mut LuaTable, nasize: i32, nhsize: i32) {
  unsafe {
    if nasize > MAXSIZE || nhsize > MAXSIZE {
      runerror(l, ERR_TABLE_OVERFLOW.as_ptr().cast());
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

      // 收缩掉的数组尾段重哈希：对照 cpp ltable.cpp:622-643，每轮重新读取
      // **活的** `t->array`——`newkey` 可重入 rehash→resize→setarrayvector
      // 重分配数组（搬块后旧基址悬垂），绝不复用 `newkey` 之前算出的槽位指针。
      for i in nasize..oldasize {
        let e = (*t).array.add(i as usize);
        if !(*e).is_nil() {
          let mut ok = TValue::default();
          setnvalue!(addr_of_mut!(ok), (i + 1) as f64);
          let dest = newkey(l, t, addr_of!(ok));
          // newkey 返回后重新解引用活数组取源值（与 cpp 参数求值语义一致）
          let e = (*t).array.add(i as usize);
          setobjt2t!(l, dest, &*e);
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
        if !(*gval!(old)).is_nil() {
          let mut ok = TValue::default();
          getnodekey!(l, addr_of_mut!(ok), old);
          setobjt2t!(l, arrayornewkey(l, t, addr_of!(ok)), gval!(old));
        }
      }

      lua_m_free(
        l,
        nold as *mut u8,
        oldhsize_slots as usize * size_of::<LuaNode>(),
        (*t).memcat,
      );
    }
  }
}
