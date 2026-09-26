use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::luai_veceq::luai_veceq,
  macros::{
    gcvalue::gcvalue, iscollectable::iscollectable, lightuserdatatag::lightuserdatatag,
    luai_inteq::luai_inteq, luai_numeq::luai_numeq, lvalue::lvalue, pvalue::pvalue, ttype::ttype,
  },
  type_aliases::t_value::TValue,
};

/// # Safety
/// `t1`/`t2` 须为可读、正确对齐的 `TValue`（cpp lobject.cpp:36）：仅读 tag 与 payload 标量/
/// 指针位模式做相等判断，不沿 GC 对象递归；Vector tag 时按 `value+extra` 连续布局读
/// `LUA_VECTOR_SIZE` 个分量，故两操作数必须完整在界。
pub unsafe fn lua_o_rawequal_obj(t1: *const TValue, t2: *const TValue) -> i32 {
  unsafe {
    let tag = ttype!(t1);
    if tag != ttype!(t2) {
      return 0;
    }
    // tag 判别经 `LuaType::from_c_int`（const fn）取判别式，与键轴孪生
    // `lua_o_rawequal_key`（TKeyView match）同形：原 `t if t == LuaType::X as u32` 魔法数
    // guard 链消除，collectable 兜底（String/Table/…/DeadKey 与未知 tag）仍落 `_` 臂，
    // 指针同一性比较逐位不变
    match LuaType::from_c_int(tag as i32) {
      Some(LuaType::Nil) => 1,
      Some(LuaType::Number) => luai_numeq((*t1).as_number(), (*t2).as_number()) as i32,
      Some(LuaType::Integer) => {
        // lvalue 为 i64，luai_inteq 按 int64 精确比较（cpp lnumutils.h:18）
        luai_inteq(lvalue!(t1), lvalue!(t2)) as i32
      }
      Some(LuaType::Vector) => {
        // as_vector_ref 返回 &[f32; 3]，luai_veceq 取裸指针逐分量比较
        luai_veceq(
          (*t1).as_vector_ref().as_ptr(),
          (*t2).as_vector_ref().as_ptr(),
        ) as i32
      }
      Some(LuaType::Boolean) => ((*t1).as_boolean_raw() == (*t2).as_boolean_raw()) as i32,
      Some(LuaType::LightUserData) => {
        // cpp lobject.h:73 pvalue(o)：指针相等 + lightuserdata tag 相等
        (pvalue!(t1) == pvalue!(t2) && lightuserdatatag!(t1) == lightuserdatatag!(t2)) as i32
      }
      _ => {
        LUAU_ASSERT!(iscollectable!(t1));
        (gcvalue!(t1) == gcvalue!(t2)) as i32
      }
    }
  }
}
