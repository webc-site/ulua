use core::ptr::{addr_of_mut, eq};

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
  // SAFETY: 契约保证两指针读写合法，value/tt 赋值与 1 元素 extra 拷贝均在各对象界内
  unsafe {
    (*obj).value = (*node).key.value;
    (*obj).extra = (*node).key.extra;
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
/// `node` 必须指向可写 LuaNode（含 key.extra 槽），`obj` 必须为存活可读 `TValue` 的共享
/// 只读借用，两对象存活且互不重叠；
/// `g` 为当时存活 `global_State`（仅 debug 存活断言读取，⇔ cpp `L->global`）。
#[inline]
unsafe fn setnodekey_direct(node: *mut LuaNode, obj: &TValue, g: *mut global_State) {
  // SAFETY: 契约保证两指针读写合法，key 三分量拷贝均在节点与 TValue 界内
  unsafe {
    (*node).key.value = obj.value;
    (*node).key.extra = obj.extra;
    (*node).key.set_tt(obj.tt);
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
/// `LuaTable`；`key` 必须为存活 `TValue` 的共享只读借用。本函数对 `key` 全程只读，体内
/// `rehash`/`getfreepos`/`luaC_barriert` 均为表侧/GC 侧分配，不搬移 Lua 栈，故 `key`
/// 源自栈槽时借用仍安全。返回值指向 t 哈希部分新落位的值槽，
/// 其有效性随 t 直至下一次结构性写表。
pub(crate) unsafe fn newkey(l: *mut LuaState, t: *mut LuaTable, key: &TValue) -> *mut TValue {
  // SAFETY: 契约保证 l/t 存活、key 为存活 TValue 只读借用；节点搬运的 offset/next 均落在 t->node 的 sizenode 数组内（ltable 不变式）
  unsafe {
    // 键恰为数组尾 +1：整段数组扩容即可，无需哈希槽（cpp `nvalue(key) == t->sizearray + 1`）
    if let ValueView::Number(k) = ValueView::from_tvalue(key)
      && k == ((*t).sizearray + 1) as f64
    {
      rehash(l, t, key);
      return arrayornewkey(l, t, key);
    }

    // 哈希部分占用（gnode! 裁决口径，全程保留裸形）：`mp` 由 `mainposition`→`gnode!`
    // 取回裸 *mut LuaNode，其 provenance 依 mainposition 契约挂在表裸指针下；
    // 哨兵判据 `eq(mp, dummynode)` 是指针相等而非桶内容——这正对应 E1
    // `node_window_mut()` 对哨兵表返回空窗的裁决口径（写哨兵表必须先经
    // `is_hash_dummy()` 级判据换发实向量，本函数的换发路径即该判据命中的
    // rehash→arrayornewkey 支），若窗化后以 len()==0 判空会破坏此契约。
    // 窗等价契约：下文所有 `offset(next)`/`offset_from` 节点算术 ⇔ 实向量窗内
    // 下标的加减（`n.offset_from(othern)` ⇔ 窗下标差），界外由 UB 降 panic 一
    // 事在 E1 访问器本基线缺位（见票单），暂以 LUA_ASSERT! 与上游不变式兜底。
    let mut mp = mainposition(t, key);
    if !matches!(ValueView::from_tvalue(&*gval!(mp)), ValueView::Nil) || eq(mp, dummynode) {
      // cpp `LuaNode* n = getfreepos(t); if (n == NULL)`：`None` 即「哈希部分无空槽」，
      // 走 rehash 扩容后转 `arrayornewkey`。空槽缺席是 cpp 的正常控制流分支，不是
      // 不变式违例，故此处用 `let-else` 消费 `Option`，无 unwrap 也无断言被吞。
      let Some(free) = getfreepos(t) else {
        rehash(l, t, key);
        return arrayornewkey(l, t, key);
      };

      // cpp `LUAU_ASSERT(n != dummynode)`：断言语义原样保留，且可论证恒成立（§6 允许
      // 的 100% 安全处）—— `getfreepos` 只可能返回 `gnode!(t, i)`（`i < lastfree`）落点，
      // 而哨兵表由 `luaH_new`/`setnodevector(size=0)` 建立 `node == DUMMYNODE` 与
      // `lastfree == 0` 的配对不变式（哨兵判据 `node == DUMMYNODE`，cpp `luaH_isdummy`），`lastfree == 0` 使
      // 扫描循环一次都不执行，故 `Some` 永不携带哨兵地址；非哨兵表的 `node` 是
      // `setnodevector` 经 `luaM_newarray` 分配的恰 `sizenode` 个桶的实向量，`as_ptr` 与
      // 后续节点搬运（`*n = *mp`、`offset_from` 链改写）均落在该界内。
      let n = free.as_ptr();
      LUAU_ASSERT!(!eq(n, dummynode));

      let mut mk = TValue::default();
      getnodekey_direct(addr_of_mut!(mk), mp, (*l).global);
      let mut othern = mainposition(t, &mk);

      // next 链改写（gnode! 裁决口径，保留裸形）：`othern.offset(next)` 单步链进与
      // `set_next(.. offset_from ..)` 基址差写依赖裸 *mut 同一性——窗形仅能化为
      // `window[idx]` 同址往返，unsafe 总量不减（E1 主控对 gnode! 的收编否决即此口径）。
      // 窗等价契约：`n.offset_from(othern)` ⇔ 实向量 node 窗内下标差；搬运次序
      // （找链尾 → 改 prev-next → `*n = *mp` 搬节点 → 断链清 nil）与 oracle
      // cpp/VM/src/ltable.cpp:922 `luaH_newkey` 逐位一致，不作窗化重排。
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
