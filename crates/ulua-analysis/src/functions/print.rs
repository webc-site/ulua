use alloc::string::String;
use core::{ffi::c_int, slice::from_raw_parts};

use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_l_tolstring::lua_l_tolstring},
  macros::lua_pop::lua_pop,
  records::lua_state,
};

use crate::{
  functions::get_type_function_runtime::get_type_function_runtime,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn print(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let mut result = String::new();

    let n = lua_gettop(vm_l);
    for i in 1..=n {
      let mut len = 0usize;
      let s = lua_l_tolstring(vm_l, i, &mut len as *mut usize);
      if i > 1 {
        result.push('\t');
      }

      let bytes = if s.is_null() || len == 0 {
        &[]
      } else {
        from_raw_parts(s as *const u8, len)
      };
      result.push_str(&String::from_utf8_lossy(bytes));
      lua_pop(vm_l, 1);
    }

    let ctx = get_type_function_runtime(l);
    (*ctx).messages.push(result);

    0
  }
}
