use crate::{
  functions::{
    lua_createtable::lua_createtable, lua_pushvalue::lua_pushvalue, lua_setfield::lua_setfield,
    lua_setmetatable::lua_setmetatable,
  },
  macros::{lua_pop::lua_pop, lua_pushliteral::lua_pushliteral},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn createmetatable_mut(l: *mut lua_State) {
  unsafe {
    lua_createtable(l, 0, 1); // create metatable for strings

    lua_pushliteral(l, c"".as_ptr()); // dummy string

    lua_pushvalue(l, -2);

    lua_setmetatable(l, -2); // set string metatable

    lua_pop(l, 1); // pop dummy string

    lua_pushvalue(l, -2); // string library...

    lua_setfield(l, -2, c"__index".as_ptr()); // ...is the __index metamethod

    lua_pop(l, 1); // pop metatable
  }
}
