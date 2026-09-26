use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping, eq};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  checkliveness,
  enums::value_view::ValueView,
  functions::{
    arrayornewkey::arrayornewkey, getfreepos::getfreepos, mainposition::mainposition,
    rehash::rehash,
  },
  macros::{
    dummynode::dummynode, gkey::gval, lua_c_barriert::luaC_barriert, setnilvalue::setnilvalue,
  },
  records::{
    global_state::global_State, lua_node::LuaNode, lua_state::LuaState, lua_table::LuaTable,
  },
  type_aliases::t_value::TValue,
};

/// §11 pass C3：`getnodekey` 宏（`macros/getnodekey.rs`，cpp `lobject.h:521-529`）的
/// 本地直写变体——写序 value → extra → tt、尾部 `checkliveness!(g, obj)` 与宏/cpp 逐位一致；
/// tt 写经 C1 收口方法 `TValue::set_tt`（cpp 裸赋值 `i_o->tt =` 的 Rust 对应），不再直写字段。
///
/// # Safety
///
/// `obj` 必须指向可写 TValue（含 extra 槽），`node` 必须指向可读 LuaNode，两对象存活且互不重叠；
/// `g` 为当时存活 `global_State`（仅 debug 存活断言读取，⇔ cpp `L->global`）。
#[inline]
unsafe fn getnodekey_direct(obj: *mut TValue, node: *const LuaNode, g: *mut global_State) {
  // Safety: 契约保证两指针读写合法，value/tt 赋值与 1 元素 extra 拷贝均在各对象界内
  unsafe {
    (*obj).value = (*node).key.value;
    copy_nonoverlapping((*node).key.extra.as_ptr(), (*obj).extra.as_mut_ptr(), 1);
    (*obj).set_tt((*node).key.tt());
    checkliveness!(g, obj);
  }
}

/// §11 pass C3：cpp `setnodekey` 宏（`lobject.h:511-519`）的本地直写变体——写序
/// value → extra → tt、尾部 `checkliveness!(g, obj)`（源键侧）与 cpp 逐位一致；tt 写经
/// `TKey::set_tt` 位域访问器（低 4 位落 tag、保留 next 位），cpp 位域 `key.tt =` 同义。
///
/// # Safety
///
/// `node` 必须指向可写 LuaNode（含 key.extra 槽），`obj` 必须指向可读 TValue，两对象存活且互不重叠；
/// `g` 为当时存活 `global_State`（仅 debug 存活断言读取，⇔ cpp `L->global`）。
#[inline]
unsafe fn setnodekey_direct(node: *mut LuaNode, obj: *const TValue, g: *mut global_State) {
  // Safety: 契约保证两指针读写合法，key 三分量拷贝均在节点与 TValue 界内
  unsafe {
    (*node).key.value = (*obj).value;
    copy_nonoverlapping((*obj).extra.as_ptr(), (*node).key.extra.as_mut_ptr(), 1);
    (*node).key.set_tt((*obj).tt);
    checkliveness!(g, obj);
  }
}

/// cpp `newkey_DEPRECATED`（`VM/src/ltable.cpp:922`）：为 `t` 的哈希部分占用一个新节点
/// 并返回其值槽。
///
/// §11 pass B（表键簇）：键的 `ttisnumber! + nvalue!`「恰好接在数组尾」判据、值槽的
/// `ttisnil!` 占用判据一律经 [`ValueView`] 变体（Number/Nil 即对应 tag），不再有独立
/// 的 tag 判定 + payload 读链。
///
/// # Safety
///
/// `l` 必须指向存活 `LuaState`（表满时 `rehash` 可经它 OOM 报错）；`t` 必须指向存活
/// `LuaTable`；`key` 必须指向存活可读 `TValue`。返回值指向 t 哈希部分新落位的值槽，
/// 其有效性随 t 直至下一次结构性写表。
pub(crate) unsafe fn newkey(l: *mut LuaState, t: *mut LuaTable, key: *const TValue) -> *mut TValue {
  // Safety: 契约保证 l/t/key 存活；节点搬运的 offset/next 均落在 t->node 的 sizenode 数组内（ltable 不变式）
  unsafe {
    // 键恰为数组尾 +1：整段数组扩容即可，无需哈希槽（cpp `nvalue(key) == t->sizearray + 1`）
    if let ValueView::Number(k) = ValueView::from_tvalue(&*key)
      && k == ((*t).sizearray + 1) as f64
    {
      rehash(l, t, key);
      return arrayornewkey(l, t, key);
    }

    let mut mp = mainposition(t, key);
    if !matches!(ValueView::from_tvalue(&*gval!(mp)), ValueView::Nil) || eq(mp, dummynode) {
      let n = getfreepos(t);
      if n.is_null() {
        rehash(l, t, key);
        return arrayornewkey(l, t, key);
      }

      LUAU_ASSERT!(!eq(n, dummynode));

      let mut mk = TValue::default();
      getnodekey_direct(addr_of_mut!(mk), mp, (*l).global);
      let mut othern = mainposition(t, addr_of!(mk));

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

    setnodekey_direct(mp, key, (*l).global);
    luaC_barriert!(l, t, key);
    LUAU_ASSERT!(matches!(
      ValueView::from_tvalue(&*gval!(mp)),
      ValueView::Nil
    ));
    gval!(mp)
  }
}
