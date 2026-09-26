use core::{ffi::c_uint, ptr::eq};

use crate::{
  enums::t_key_view::TKeyView,
  functions::{hashnum::hashnum, lua_a_toobject::LUA_O_NILOBJECT, walk_nodes::walk_nodes},
  macros::{dummynode::dummynode, gkey::gval, luai_numeq::luai_numeq},
  records::lua_table::LuaTable,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `t` 须指向存活 LuaTable，`array` 覆盖 `sizearray` 项、`node` 哈希区经 `hashnum`/`key.next` 链可寻址
/// （`node` 为 dummynode 时跳过哈希分支）；命中桶的 key 数值经 `TKeyView::Number`（B2a 键轴读链）
/// 判定后读取。返回 `array`/桶内指针
/// 或 `luaO_nilobject`，指针仅在表未被重哈希/回收前有效。不抛错/不分配。cpp/VM/src/ltable.cpp:1058 luaH_getnum。
pub(crate) unsafe fn lua_h_getnum(t: *mut LuaTable, key: i32) -> *const TValue {
  unsafe {
    // (1 <= key && key <= t->sizearray)
    if (key as c_uint).wrapping_sub(1) < (*t).sizearray as c_uint {
      (*t).array.add((key - 1) as usize)
    } else if !eq((*t).node, dummynode) {
      let nk = key as f64;

      walk_nodes(hashnum(t, nk), |n| -> Option<*const TValue> {
        // check whether `key' is somewhere in the chain
        if matches!(TKeyView::from_tkey(&(*n).key),
          TKeyView::Number(k) if luai_numeq(k, nk))
        {
          return Some(gval!(n)); // that's it
        }
        None
      })
      .unwrap_or(LUA_O_NILOBJECT)
    } else {
      LUA_O_NILOBJECT
    }
  }
}
