use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{c_slice, lua_l_checktype::lua_l_checktype, lua_pushnumber::lua_pushnumber},
  macros::{
    hvalue::hvalue, nvalue::nvalue, sizenode::sizenode, ttisnil::ttisnil, ttisnumber::ttisnumber,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn maxn(l: *mut lua_State) -> c_int {
  let mut max: f64 = 0.0;
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);

    let t = hvalue!((*l).base);

    // 数组尾部：最后一个非 nil 元素决定 max，rposition 从后往前提前终止
    let arr = c_slice((*t).array, (*t).sizearray as usize);
    if let Some(i) = arr.iter().rposition(|v| !ttisnil!(v)) {
      max = (i + 1) as f64;
    }

    let node_size = sizenode!(t);
    // 节点数组按切片迭代：消除逐次 add 的索引写法，宏只读取不写回
    for n in c_slice((*t).node, node_size as usize) {
      if !ttisnil!(&n.val) && ttisnumber!(&n.key) {
        let v = nvalue!(&n.key);

        if v > max {
          max = v;
        }
      }
    }

    lua_pushnumber(l, max);
  }
  1
}
