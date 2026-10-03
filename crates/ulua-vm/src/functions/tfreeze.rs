use crate::{
  enums::lua_type::LuaType,
  functions::{lua_getreadonly::lua_getreadonly, lua_l_getmetafield::lua_l_getmetafield_bytes},
  macros::{lua_lib_fn::lua_lib_fn, tm_metatable::TM_METATABLE},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tfreeze(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);

    // r16-v3 #60 拆两语句：`lua_getreadonly` 前移 `&LuaState` 只读形后与
    // `arg_check` 独占接收者借用冲突；原位现读（求值序本即先读标志后落 arg_check，
    // 句间无场写），拆句逐位等价。
    let not_frozen = lua_getreadonly(&*l, 1) == 0;

    (*l).arg_check(not_frozen, 1, "table is already frozen");

    // 拆两语句（同上方 r16-v3 #60 判例）：被调收形为 `&mut` 实参形后与 `arg_check`
    // 独占接收者借用冲突；先查元方法后落 arg_check 的求值序不变，句间无场写，拆句逐位等价。
    let no_meta = lua_l_getmetafield_bytes(&mut *l, 1, TM_METATABLE) == 0;

    (*l).arg_check(no_meta, 1, "table has a protected metatable");

    (*l).set_readonly(1, true);

    (*l).push_value(1);
    1
  }
}

lua_lib_fn!(pub fn tfreeze, tfreeze_arm);
