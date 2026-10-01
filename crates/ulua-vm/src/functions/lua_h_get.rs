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
/// `t` 须指向存活 `LuaTable`（`array/sizearray/node` 元数据一致），`key` 须为存活
/// `TValue` 的共享只读借用（cpp ltable.cpp:1119）：本函数全程只读——tag 判别、
/// `mainposition` 起沿 next 链游走均限定在该表节点数组界内，体内无元方法调用、无栈
/// 扩容，不存在跨栈重定位使用 `key` 的路径；返回 nil 时给出全局 `LUA_O_NILOBJECT` 哨兵。
pub(crate) unsafe fn lua_h_get(t: *mut LuaTable, key: &TValue) -> *const TValue {
  unsafe {
    let tt = ttype!(key);
    match tt {
      // tag 判别与 mainposition 同写法：走 LuaType 常量，不用裸字面量
      x if x == LuaType::Nil as u32 => return LUA_O_NILOBJECT,
      x if x == LuaType::String as u32 => {
        // B2-2a 任务B：getstr 折叠 Option<Slot> 后，本函数裸 *const 返回在边界还原
        // 哨兵原形（下游 lua_v_gettable/lua_h_set 消费链跨调用持槽，不强行句柄化）
        return lua_h_getstr(&*t, key.as_string_ptr() as *mut _)
          .map_or(LUA_O_NILOBJECT, |s| s.as_const_ptr());
      }
      x if x == LuaType::Number as u32 => {
        let n = key.as_number();
        let k = n as i32;
        if luai_numeq(k as f64, n) {
          return lua_h_getnum(&*t, k);
        }
        // 非整数数值键落到下方 hash 慢路径（cpp `goto hash`）
      }
      _ => {}
    }

    // hash 慢路径：mainposition 起沿 next 链探测（cpp luaH_get 的 `hash:` 标签）
    walk_nodes(mainposition(t, key), |n| -> Option<*const TValue> {
      if lua_o_rawequal_key(&(*n).key, key) != 0 {
        Some(gval!(n))
      } else {
        None
      }
    })
    .unwrap_or(LUA_O_NILOBJECT)
  }
}
