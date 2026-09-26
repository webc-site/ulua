use alloc::string::String;

use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_l_tolstring::lua_l_tolstring_ref},
  macros::lua_pop::lua_pop,
  records::lua_state,
};

use crate::{
  functions::get_type_function_runtime::get_type_function_runtime,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int print(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:2069`）。
pub unsafe fn print(l: *mut LuaState) -> i32 {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let mut result = String::new();

    let n = lua_gettop(vm_l);
    for i in 1..=n {
      // Safety: `lua_l_tolstring_ref` 把参数串化并压栈，返回全字节切片（内嵌 `\0`
      // 不截断）；`None`（抛错发散前的兜底形态）与旧 null 指针同为空串。
      let s = lua_l_tolstring_ref(vm_l, i).unwrap_or_default();
      if i > 1 {
        result.push('\t');
      }

      result.push_str(&String::from_utf8_lossy(s));
      lua_pop(vm_l, 1);
    }

    let ctx = get_type_function_runtime(l);
    (*ctx).messages.push(result);

    0
  }
}
