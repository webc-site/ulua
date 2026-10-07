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

/// 纯值比较（cpp lobject.cpp:36）：tag 判别 + payload 标量/指针位模式比较，不沿 GC
/// 对象递归、不触发 `__eq`。r16-v5 收口：`pub unsafe fn` → `pub fn`，对齐键轴孪生
/// [`lua_o_rawequal_key`](crate::functions::lua_o_rawequal_key) 引用形（非空/对齐由
/// 引用承载；体内宏组 ttype!/lvalue!/pvalue!/gcvalue!/iscollectable!/lightuserdatatag!
/// 引用侧兼容性由孪生逐点背书）。
/// # Safety
/// 调用序契约（正确性，非内存安全；safe fn 文档断言，由调用方承载）：Vector 臂按
/// `value+extra` 连续布局读 `LUA_VECTOR_SIZE` 个分量，故两操作数必须完整在界。
pub fn lua_o_rawequal_obj(t1: &TValue, t2: &TValue) -> i32 {
  // SAFETY: 引用接收者保证非空与对齐；tag 相等后仅读对应 payload 分量
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
      Some(LuaType::Number) => luai_numeq(t1.as_number(), t2.as_number()) as i32,
      Some(LuaType::Integer) => {
        // lvalue 为 i64，luai_inteq 按 int64 精确比较（cpp lnumutils.h:18）
        luai_inteq(lvalue!(t1), lvalue!(t2)) as i32
      }
      Some(LuaType::Vector) => {
        // as_vector_ref 返回 &[f32; 3]，luai_veceq 直接按切片逐分量比较
        luai_veceq(t1.as_vector_ref(), t2.as_vector_ref()) as i32
      }
      Some(LuaType::Boolean) => (t1.as_boolean_raw() == t2.as_boolean_raw()) as i32,
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
