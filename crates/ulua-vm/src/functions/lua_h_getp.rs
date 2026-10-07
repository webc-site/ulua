use core::ffi::c_void;

use crate::{
  enums::t_key_view::TKeyView,
  functions::{hashpointer::hashpointer, lua_a_toobject::LUA_O_NILOBJECT, walk_nodes::walk_nodes},
  records::lua_table::LuaTable,
  type_aliases::t_value::TValue,
};

/// `luaH_getp`（cpp ltable.cpp:1101）：按 light 指针键的位模式查表值槽，命中返回其
/// 只读指针，缺省返回 `LUA_O_NILOBJECT` 哨兵。
///
/// r16-v4c 步骤 1 safe 化：`t` 前移 `&LuaTable` 引用形——本函数真解引用全源自 `t`
/// （node 区游走与节点键/值读），其存活与 node 区和 `sizenode` 自洽改由
/// `LuaTable` 类型不变式承载；`key` 全函数体纯位模式用（`==` 比较、入 `hashpointer`
/// 哈希、经 `setpvalue`→`set_pvalue` 入载荷），保持 `*mut c_void` 裸形、全程不
/// 解引用。体内唯一真实裸触点为 `hashpointer`/`walk_nodes` 原语调用与节点解引用
/// （两者签名原语形不动），收进一个窄 unsafe 窗。返回槽指针的有效期随表直至下
/// 一次结构性写表（cpp 同契约）。
pub(crate) fn lua_h_getp(t: &LuaTable, key: *mut c_void, tag: i32) -> *const TValue {
  // SAFETY: `t` 为存活 LuaTable 引用（类型承载），其 node 区与 `lsizenode` 自洽
  // （LuaTable 不变式，⇔ cpp ltable.cpp:1101 前提）：`hashpointer` 按 `key` 哈希值
  // 定位、`walk_nodes` 沿 next 链游走仅限本表节点数组界内；`key` 只参与哈希与位
  // 模式比较、`tag` 为值型比较，均无内存前提。
  unsafe {
    walk_nodes(
      hashpointer(t as *const LuaTable, key),
      |n| -> Option<*const TValue> {
        // 键轴读链收敛为 B2a TKeyView match：pointer+tag 两段比较与原
        // `ttislightuserdata!(nk) && pvalue!(nk) == key && lightuserdatatag!(nk) == tag` 逐位等价
        if matches!(TKeyView::from_tkey(&(*n).key),
        TKeyView::LightUserdata { pointer, tag: ktag }
          if pointer == key && ktag == tag)
        {
          return Some(&(*n).val);
        }
        None
      },
    )
    .unwrap_or(LUA_O_NILOBJECT)
  }
}
