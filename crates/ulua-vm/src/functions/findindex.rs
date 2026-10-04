use core::ptr::null;

use crate::{
  enums::{t_key_view::TKeyView, value_view::ValueView},
  functions::{
    arrayindex::arrayindex, lua_g_runerror_l::lua_g_runerror_l,
    lua_o_rawequal_key::lua_o_rawequal_key, mainposition::mainposition, walk_nodes::walk_nodes,
  },
  macros::{gcvalue::gcvalue, gnode::gnode, iscollectable::iscollectable},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// `next` 迭代键 → 元素下标定位（w6e 入侧收形 + 诚实降级：`l: *mut LuaState →
/// &mut LuaState`，去 `&mut *l`/裸转手往返；三个形参全引用形后，体内真实裸操作
/// 只剩哈希链游走一处，收为 `else` 分支的窄 `unsafe` 块）。
///
/// 调用序契约（正确性，非内存安全——`l`/`t`/`key` 的存活与借用形态已由类型承载）：
/// `l` 须处于可抛错受保护帧（找不到 key 时 `lua_g_runerror_l` 抛 "invalid key to
/// 'next'" 发散，不返回）；`t` 的 `sizearray` 与数组区、`node` 哈希区
/// （`mainposition`/`gnode` 及 `key.next` 链）自洽；`key` 为只读查询键（可能已是
/// dead key，但按 C++ 语义仍可与 `gkey` 指针比较）。
/// cpp/VM/src/ltable.cpp:351。
pub(crate) fn findindex(l: &mut LuaState, t: &LuaTable, key: &TValue) -> i32 {
  // 查询键（TValue 轴，B1 ValueView）：nil → 首迭代；数值 → 数组槽候选；其余进哈希链
  let i = match ValueView::from_tvalue(key) {
    ValueView::Nil => return -1, // first iteration
    ValueView::Number(n) => arrayindex(n),
    _ => -1,
  };

  // 数组段命中分支：`i > 0 && i <= t.sizearray` 界判 ⇔ E1 `array_window()` 契约
  // （窗长 = sizearray.max(0)，`i-1` 即窗内下标，越界口径由 UB 降 panic 在本形下
  // 已由界判先行达成）；本分支只回下标不取指针，无裸 `.add()` 可消，保持原形。
  if i > 0 && i <= t.sizearray {
    i - 1 // yes; that's the index (corrected to C)
  } else {
    // check whether `key' is somewhere in the chain
    // key may be dead already, but it is ok to use it in `next'
    // 哈希链遍历（gnode! 裁决口径，保留裸形）：`walk_nodes` 沿 `key.next` 单链游走，
    // 与 cpp `findindex` 的链序逐位一致；改窗全扫会破坏命中序与 O(链长) 行为，
    // 且返回值依赖 `gnode!(t, 0)` 基址的 `offset_from` 算术（E1 裁决明载豁免）。
    // r12-w6d 逐点复核定性：本文件唯一代码点位即该基址算术类，宏结果不作取值/键槽
    // 消费，非纯下标读写，维持保留——w6e 降级后该保留面即本块，块外全部安全形。
    // 窗等价契约：命中节点 `n.offset_from(gnode!(t, 0))` ⇔ 该桶在 `node_window()`
    // 内的下标；哨兵表下本链起点即 dummy 单格（窗长恒 1、键值皆 nil，rawequal/
    // DeadKey 判据必不命中），落到 runerror 分支，与 E1「判空不得用
    // node_window().len()==0、须以桶内容为准」契约口径一致——本函数无判空点位。
    // SAFETY: 上注即本块契约：`mainposition`/`walk_nodes`/`gnode!` 全程落 `t` 自身
    // node 数组界内（表借用的不变量面），`lua_g_runerror_l` 在 `l` 的可抛错帧上发散。
    unsafe {
      walk_nodes(mainposition(t, key), |n| -> Option<i32> {
        if lua_o_rawequal_key(&(*n).key, key) != 0
          || matches!(TKeyView::from_tkey(&(*n).key),
            TKeyView::DeadKey(kgc) if iscollectable!(key) && gcvalue!(key) == kgc)
        {
          // hash elements are numbered after array ones
          Some((n.offset_from(gnode!(t, 0)) as i32) + t.sizearray)
        } else {
          None
        }
      })
      .unwrap_or_else(|| lua_g_runerror_l(l, null(), format_args!("invalid key to 'next'")))
    }
  }
}
