

/// 对应 C++ 原生 `static int print(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:2069`）。
use alloc::string::String;
use ulua_vm::{functions::lua_l_tolstring::lua_l_tolstring_ref};
use crate::{functions::get_type_function_runtime::get_type_function_runtime};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn print(l: &mut LuaState) -> i32 {
  unsafe {
    let mut result = String::new();

    let n = l.get_top();
    for i in 1..=n {
      // Safety: `lua_l_tolstring_ref` 把参数串化并压栈，返回全字节切片（内嵌 `\0`
      // 不截断）；`None`（抛错发散前的兜底形态）与旧 null 指针同为空串。
      let s = lua_l_tolstring_ref(l.as_mut_ptr(), i).unwrap_or_default();
      if i > 1 {
        result.push('\t');
      }

      result.push_str(&String::from_utf8_lossy(s));
      l.pop(1);
    }

    let ctx = get_type_function_runtime(&mut *l);
    (*ctx).messages.push(result);

    0
  }
}
