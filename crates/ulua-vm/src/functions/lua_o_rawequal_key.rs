use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::t_key_view::TKeyView,
  functions::luai_veceq::luai_veceq,
  macros::{
    gcvalue::gcvalue, iscollectable::iscollectable, lightuserdatatag::lightuserdatatag,
    luai_inteq::luai_inteq, luai_numeq::luai_numeq, lvalue::lvalue, pvalue::pvalue, ttype::ttype,
  },
  records::t_key::TKey,
  type_aliases::t_value::TValue,
};

/// 纯值比较（cpp lobject.cpp:61），指针形参收为 `&TKey`/`&TValue`（非空/对齐由引用保证）。
///
/// tag 判等前置后，键轴分发收敛为 [`TKeyView`] 变体 match（B2a，§11 路线图）——原
/// `t if t == LuaType::X as u32` 魔法数字 guard 链即其形状；对侧 payload 仍经读取宏取
/// （tag 已判等，分发结果与逐侧 tag 一致），collectable 兜底分支保持 `gcvalue!` 指针
/// 同一性比较，DeadKey 等无专用比较逻辑的 tag 走同一兜底，行为逐字节不变。
pub fn lua_o_rawequal_key(t1: &TKey, t2: &TValue) -> i32 {
  // Safety: 视图构造与读值宏均为 unsafe trait 形状调用，引用接收者不解裸指针；union
  // 字段仅在双方 tag 相等的分支下按活跃变体读取，Vector 分支按 value+extra 连续布局
  // 读满 LUA_VECTOR_SIZE 分量
  unsafe {
    let tag = ttype!(t1);
    if tag != ttype!(t2) {
      0
    } else {
      match TKeyView::from_tkey(t1) {
        TKeyView::Nil => 1,
        TKeyView::Number(a) => luai_numeq(a, t2.as_number()) as i32,
        TKeyView::Integer(a) => luai_inteq(a, lvalue!(t2)) as i32,
        TKeyView::Vector(a) => luai_veceq(a.as_ptr(), t2.as_vector_ref().as_ptr()) as i32,
        TKeyView::Boolean(a) => (a == t2.as_boolean_raw()) as i32,
        TKeyView::LightUserdata { pointer, tag: ktag } => {
          (pointer == pvalue!(t2) && ktag == lightuserdatatag!(t2)) as i32
        }
        _ => {
          LUAU_ASSERT!(iscollectable!(t1));
          (gcvalue!(t1) == gcvalue!(t2)) as i32
        }
      }
    }
  }
}
