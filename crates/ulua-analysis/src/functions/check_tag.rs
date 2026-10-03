

/// 对应 C++ 原生 `static int checkTag(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1820`）。
use crate::{functions::{get_tag::get_tag, get_type_user_data::get_type_user_data, throw_type_error::throw_type_error}, macros::lua_check_args};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn check_tag(l: &mut LuaState) -> i32 {
  // Safety: l 是 Lua VM 调注册闭包时传入的存活 lua_State，索引 1/2 即其栈上实参；
  // get_type_user_data 对非 type 用户数据先经 luaL_typeerror 抛出、返回的 arena 指针有效；
  // luaL_checkstring 非字符串即抛错，成功时返回 NUL 结尾、本次调用内被 GC 保活的字符串，
  // cstr_cow 读取合法；错误经 throw_type_error 收口；其余调用仅操作该栈。
  unsafe {
    lua_check_args!(l, != 2, "type.is: expected 2 arguments, but got {}");

    let self_ = get_type_user_data(&mut *l, 1);
    // cpp `std::string arg = luaL_checkstring(L, 2)` 本就落地拥有值；同样先取字节再
    // 继续，既对齐 oracle，也满足 `check_bytes` 的「持窗期间不得再动 state」契约，
    // 使后续 get_tag/push_boolean 的可变借用互不重叠。
    let arg = l.check_bytes(2).to_vec();

    let tag = get_tag(&mut *l, self_);
    l.push_boolean(tag.as_bytes() == arg);
    1
  }
}
