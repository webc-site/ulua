use core::ptr::null;

use crate::{
  enums::{t_key_view::TKeyView, value_view::ValueView},
  functions::{
    arrayindex::arrayindex, lua_g_runerror_l::lua_g_runerror_l,
    lua_o_rawequal_key::lua_o_rawequal_key, mainposition::mainposition, walk_nodes::walk_nodes,
  },
  macros::{gcvalue::gcvalue, gnode::gnode, iscollectable::iscollectable},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 LuaState 且处于可抛错受保护帧（找不到 key 时 `lua_g_runerror_l` 抛 "invalid key to 'next'"）；
/// `t` 须指向存活 LuaTable，`sizearray` 与数组区、`node` 哈希区（`mainposition`/`gnode` 及 `key.next` 链）自洽；
/// `key` 须指向存活 TValue（可能已是 dead key，但按 C++ 语义仍可与 `gkey` 指针比较）。cpp/VM/src/ltable.cpp:351。
pub(crate) unsafe fn findindex(l: *mut LuaState, t: *mut LuaTable, key: StkId) -> i32 {
  unsafe {
    // 查询键（TValue 轴，B1 ValueView）：nil → 首迭代；数值 → 数组槽候选；其余进哈希链
    let i = match ValueView::from_tvalue(&*key) {
      ValueView::Nil => return -1, // first iteration
      ValueView::Number(n) => arrayindex(n),
      _ => -1,
    };

    if i > 0 && i <= (*t).sizearray {
      i - 1 // yes; that's the index (corrected to C)
    } else {
      // check whether `key' is somewhere in the chain
      // key may be dead already, but it is ok to use it in `next'
      walk_nodes(mainposition(t, key as *const _), |n| -> Option<i32> {
        if lua_o_rawequal_key(&(*n).key, &*key) != 0
          || matches!(TKeyView::from_tkey(&(*n).key),
            TKeyView::DeadKey(kgc) if iscollectable!(key) && gcvalue!(key) == kgc)
        {
          // hash elements are numbered after array ones
          Some((n.offset_from(gnode!(t, 0)) as i32) + (*t).sizearray)
        } else {
          None
        }
      })
      .unwrap_or_else(|| lua_g_runerror_l(l, null(), format_args!("invalid key to 'next'")))
    }
  }
}
