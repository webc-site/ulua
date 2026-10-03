use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_a_pushvalue::lua_a_pushvalue, lua_h_clone::lua_h_clone,
    lua_l_getmetafield::lua_l_getmetafield_bytes,
  },
  macros::{lua_lib_fn::lua_lib_fn, sethvalue::sethvalue, tm_metatable::TM_METATABLE},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tclone(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);

    // 拆两语句（tfreeze r16-v3 #60 同判例）：被调收形为 `&mut` 实参形后与 `arg_check`
    // 独占接收者借用冲突（E0499）；求值序本即先查元方法后落 arg_check，句间无场写，拆句逐位等价。
    let no_meta = lua_l_getmetafield_bytes(&mut *l, 1, TM_METATABLE) == 0;

    (*l).arg_check(no_meta, 1, "table has a protected metatable");

    let tt = lua_h_clone(l, (*(*l).base).as_table_ptr());

    let mut v = TValue::default();
    sethvalue!(l, &mut v, tt);
    lua_a_pushvalue(&mut *l, &v);

    1
  }
}

lua_lib_fn!(pub fn tclone, tclone_arm);
