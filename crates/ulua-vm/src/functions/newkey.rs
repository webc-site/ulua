use core::ptr::eq;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  checkliveness,
  functions::{
    arrayornewkey::arrayornewkey,
    getfreepos::getfreepos,
    mainposition::{mainposition, mainposition_tkey},
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
/// 表键与值槽占用判据直接经 `key.is_number() + key.as_number()` 与 `is_nil()` 处理，
/// 零开销匹配、无中间 enum 构造。
///
/// # Safety
///
/// `l` 必须指向存活 `LuaState`（表满时 `rehash` 可经它 OOM 报错）；`t` 必须指向存活
/// `LuaTable`；`key` 必须为存活 `TValue` 的共享只读借用。本函数对 `key` 全程只读，体内
/// `rehash`/`getfreepos`/`luaC_barriert` 均为表侧/GC 侧分配，不搬移 Lua 栈，故 `key`
/// 源自栈槽时借用仍安全。返回值指向 t 哈希部分新落位的值槽，
/// 其有效性随 t 直至下一次结构性写表。
/// 壳强制保持函数边界：字符串 intern 表等高频调用方（`lua_h_setstr` 等）以恰一次
/// call 进入，与拆分前单函数 `newkey` 形态逐位同形（patterns 复测归因：壳可内联时
/// 调用方膨胀 + 多层 call，+2% 级回归）。
#[inline(never)]
pub(crate) unsafe fn newkey(l: *mut LuaState, t: *mut LuaTable, key: &TValue) -> *mut TValue {
  // SAFETY: 契约同 [`newkey_with_mp`]；mp 由同一契约的 `mainposition` 现算，
  // 与调用方自行预算的桶逐位同源（同 key 同表状态下 `mainposition` 结果唯一）
  unsafe {
    let mp = mainposition(t, key);
    newkey_with_mp(l, t, key, mp)
  }
}

/// [`newkey`] 的预定位变体：调用方已在写前探测中算出 `key` 的主位桶 `mp` 时直传，
/// 免去 `newkey` 壳内的重算（同 key 同表状态下 `mainposition` 结果唯一，行为逐位
/// 同构）。前提：自 `mp` 算出至本函数调用之间，`t` 的哈希部分未发生 rehash
/// （`lua_v_settable` 写前探测只读表，满足）。
///
/// # Safety
/// 契约与 [`newkey`] 完全一致；额外要求 `mp` 为 `mainposition(t, key)` 在当前表
/// 状态下的返回值（或哨兵 `dummynode`）。
///
/// `#[inline(always)]`：主体展开回 [`newkey`] 壳内即恢复拆分前单函数形态；在
/// [`crate::functions::lua_v_settable::settable_num_fastpath`]（自身
/// `#[inline(never)]`，膨胀被隔离）的 miss 支里同样展开，不外溢。
#[inline(always)]
pub(crate) unsafe fn newkey_with_mp(
  l: *mut LuaState,
  t: *mut LuaTable,
  key: &TValue,
  mut mp: *mut LuaNode,
) -> *mut TValue {
  // SAFETY: 契约保证 l/t 存活、key 为存活 TValue 只读借用；节点搬运的 offset/next 均落在 t->node 的 sizenode 数组内（ltable 不变式）
  unsafe {
    // 键恰为数组尾 +1：整段数组扩容即可，无需哈希槽（cpp `nvalue(key) == t->sizearray + 1`）
    if key.is_number() && key.as_number() == ((*t).sizearray + 1) as f64 {
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
    // r12-w6d 逐点复核定性：本文件 4 处 gnode 字样均为裁决文档口径，体内无宏调用
    // 代码点位（`mp` 经 `mainposition` 取回，源头判据见 hashint/hashnum/hashpointer
    // 各票注），next 链 offset/offset_from 改写属指针判据保留面，无收编面。
    // `mp` 来自调用方预算或 [`newkey`] 壳内 `mainposition`，语义与 cpp 原位重算
    // 逐位同源（同 key 同表状态结果唯一），不再重复计算。
    if !(*gval!(mp)).is_nil() || eq(mp, dummynode) {
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

      let mut othern = mainposition_tkey(t, &(*mp).key);

      // next 链改写（gnode! 裁决口径，保留裸形）：`othern.offset(next)` 单步链进与
      // `set_next(.. offset_from ..)` 基址差写依赖裸 *mut 同一性——窗形仅能化为
      // `window[idx]` 同址往返，unsafe 总量不减（E1 主控对 gnode! 的收编否决即此口径）。
      // 窗等价契约：`n.offset_from(othern)` ⇔ 实向量 node 窗内下标差；搬运次序
      // （找链尾 → 改 prev-next → `*n = *mp` 搬节点 → 断链清 nil）与 oracle
      // cpp/VM/src/ltable.cpp:922 `luaH_newkey` 逐位一致，不作窗化重排。
      if othern != mp {
        loop {
          let next_n = othern.offset((*othern).key.next() as isize);
          if next_n == mp {
            break;
          }
          othern = next_n;
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
    LUAU_ASSERT!((*gval!(mp)).is_nil());
    gval!(mp)
  }
}
