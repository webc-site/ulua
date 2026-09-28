use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::luai_veceq::luai_veceq,
  macros::{
    gcvalue::gcvalue, iscollectable::iscollectable, lightuserdatatag::lightuserdatatag,
    luai_inteq::luai_inteq, luai_numeq::luai_numeq, lvalue::lvalue, pvalue::pvalue, ttype::ttype,
  },
  records::t_key::TKey,
  type_aliases::t_value::TValue,
};

/// 纯值比较（cpp lobject.cpp:61），指针形参收为 `&TKey`/`&TValue`（非空/对齐由引用保证）。
pub(crate) fn lua_o_rawequal_key(t1: &TKey, t2: &TValue) -> i32 {
  // SAFETY: 引用接收者保证非空与对齐；tag 相等后仅读对应 payload 分量
  unsafe {
    let tag = ttype!(t1);
    if tag != ttype!(t2) {
      0
    } else {
      match LuaType::from_c_int(tag as i32) {
        Some(LuaType::Nil) => 1,
        Some(LuaType::Number) => luai_numeq(t1.as_number(), t2.as_number()) as i32,
        Some(LuaType::Integer) => luai_inteq(lvalue!(t1), lvalue!(t2)) as i32,
        Some(LuaType::Vector) => {
          luai_veceq(t1.as_vector_ref().as_ptr(), t2.as_vector_ref().as_ptr()) as i32
        }
        Some(LuaType::Boolean) => (t1.as_boolean_raw() == t2.as_boolean_raw()) as i32,
        Some(LuaType::LightUserData) => {
          (pvalue!(t1) == pvalue!(t2) && lightuserdatatag!(t1) == lightuserdatatag!(t2)) as i32
        }
        _ => {
          LUAU_ASSERT!(iscollectable!(t1));
          (gcvalue!(t1) == gcvalue!(t2)) as i32
        }
      }
    }
  }
}
