use core::ffi::c_int;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::luai_veceq::luai_veceq,
  macros::{
    bvalue::bvalue, gcvalue::gcvalue, iscollectable::iscollectable,
    lightuserdatatag::lightuserdatatag, luai_inteq::luai_inteq, luai_numeq::luai_numeq,
    lvalue::lvalue, nvalue::nvalue, ttype::ttype, vvalue::vvalue,
  },
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_o_rawequal_obj(t1: *const TValue, t2: *const TValue) -> c_int {
  unsafe {
    if ttype!(t1) != ttype!(t2) {
      0
    } else {
      let tag = ttype!(t1);
      if tag == LuaType::Nil as i32 {
        1
      } else if tag == LuaType::Number as i32 {
        luai_numeq(nvalue!(t1), nvalue!(t2)) as c_int
      } else if tag == LuaType::Integer as i32 {
        // luai_inteq expects f64 in the provided API card, so we cast the i64 lvalue
        luai_inteq(lvalue!(t1) as f64, lvalue!(t2) as f64) as c_int
      } else if tag == LuaType::Vector as i32 {
        // vvalue returns the array [f32; 2], luai_veceq expects *const f32
        luai_veceq(vvalue!(t1).as_ptr(), vvalue!(t2).as_ptr()) as c_int
      } else if tag == LuaType::Boolean as i32 {
        (bvalue!(t1) == bvalue!(t2)) as c_int
      } else if tag == LuaType::LightUserData as i32 {
        // pvalue is a unit-type constant in the provided API card, so we access the field directly
        ((*t1).value.p == (*t2).value.p && lightuserdatatag!(t1) == lightuserdatatag!(t2)) as c_int
      } else {
        LUAU_ASSERT!(iscollectable!(t1));
        (gcvalue!(t1) == gcvalue!(t2)) as c_int
      }
    }
  }
}

pub use lua_o_rawequal_obj as luaO_rawequalObj;
