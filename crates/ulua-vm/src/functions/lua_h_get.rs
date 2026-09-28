use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_h_getnum::lua_h_getnum, lua_h_getstr::lua_h_getstr, lua_o_rawequal_key::lua_o_rawequal_key,
    mainposition::mainposition, walk_nodes::walk_nodes,
  },
  macros::{gkey::gval, lua_o_nilobject::LUA_O_NILOBJECT, luai_numeq::luai_numeq, ttype::ttype},
  records::lua_table::LuaTable,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `t` 须指向存活 `LuaTable`（`array/sizearray/node` 元数据一致），`key` 须为可读对齐
/// `TValue`（cpp ltable.cpp:1119）：hash 慢路径沿 next 链游走全程限定在该表节点数组界内，
/// 返回 nil 时给出全局 `LUA_O_NILOBJECT` 哨兵。
pub unsafe fn lua_h_get(t: *mut LuaTable, key: *const TValue) -> *const TValue {
  unsafe {
    let tt = ttype!(key);
    match tt {
      // tag 判别与 mainposition 同写法：走 LuaType 常量，不用裸字面量
      x if x == LuaType::Nil as u32 => return LUA_O_NILOBJECT,
      x if x == LuaType::String as u32 => return lua_h_getstr(t, (*key).as_string_ptr() as *mut _),
      x if x == LuaType::Number as u32 => {
        let n = (*key).as_number();
        let k = n as i32;
        if luai_numeq(k as f64, n) {
          return lua_h_getnum(t, k);
        }
        // 非整数数值键落到下方 hash 慢路径（cpp `goto hash`）
      }
      _ => {}
    }

    // hash 慢路径：mainposition 起沿 next 链探测（cpp luaH_get 的 `hash:` 标签）
    walk_nodes(mainposition(t, key), |n| -> Option<*const TValue> {
      if lua_o_rawequal_key(&(*n).key, &*key) != 0 {
        Some(gval!(n))
      } else {
        None
      }
    })
    .unwrap_or(LUA_O_NILOBJECT)
  }
}
