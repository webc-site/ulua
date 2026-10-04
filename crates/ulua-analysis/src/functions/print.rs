/// 对应 C++ 原生 `static int print(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:2069`）。
use alloc::string::String;

use ulua_vm::{functions::lua_l_tolstring::lua_l_tolstring_ref, records::lua_state::LuaState};

use crate::functions::get_type_function_runtime::get_type_function_runtime;
pub(crate) fn print(l: &mut LuaState) -> i32 {
  let mut result = String::new();

  let n = l.get_top();
  for i in 1..=n {
    // Safety: `l.as_mut_ptr()` 是 `&mut l` 同一对象的镜像透传；`lua_l_tolstring_ref` 把参数
    // 串化并压栈，返回全字节切片（内嵌 `\0` 不截断）；`None`（抛错发散前的兜底形态）与旧
    // null 指针同为空串。切片在下一次 VM 重入（本循环的 `l.pop`）前即被 lossy 拷入 `result`。
    let s = unsafe { lua_l_tolstring_ref(l.as_mut_ptr(), i) }.unwrap_or_default();
    if i > 1 {
      result.push('\t');
    }

    result.push_str(&String::from_utf8_lossy(s));
    l.pop(1);
  }

  let ctx = get_type_function_runtime(l).expect("runtime 于注册阶段挂载，会话内恒非空");
  ctx.get_mut().messages.push(result);

  0
}
