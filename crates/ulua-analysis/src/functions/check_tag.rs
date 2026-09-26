use ulua_common::functions::c_str::cstr_cow;
use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_pushboolean::lua_pushboolean},
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state,
};

use crate::{
  functions::{
    get_tag::get_tag, get_type_user_data::get_type_user_data, throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int checkTag(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1820`）。
pub unsafe fn check_tag(l: *mut LuaState) -> i32 {
  // Safety: l 是 Lua VM 调注册闭包时传入的存活 lua_State，索引 1/2 即其栈上实参；
  // get_type_user_data 对非 type 用户数据先经 luaL_typeerror 抛出、返回的 arena 指针有效；
  // luaL_checkstring 非字符串即抛错，成功时返回 NUL 结尾、本次调用内被 GC 保活的字符串，
  // cstr_cow 读取合法；错误经 throw_type_error 收口；其余调用仅操作该栈。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_check_args!(vm_l, != 2, "type.is: expected 2 arguments, but got {}");

    let self_ = get_type_user_data(l, 1);
    let arg = luaL_checkstring!(vm_l, 2);
    let arg_str = cstr_cow(arg);

    let tag = get_tag(l, self_);
    lua_pushboolean(vm_l, i32::from(tag.as_bytes() == arg_str.as_bytes()));
    1
  }
}
