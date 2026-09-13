use core::ffi::{CStr, c_char, c_int};

use ulua_vm::{
  functions::{
    lua_getfield::lua_getfield, lua_pushlstring::lua_pushlstring, lua_pushvalue::lua_pushvalue,
  },
  macros::{lua_l_checkstring::luaL_checkstring, lua_upvalueindex::lua_upvalueindex},
  records::lua_state,
};

use crate::{
  functions::{get_tag::get_tag, get_type_user_data::get_type_user_data},
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn type_userdata_index(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let self_ty = get_type_user_data(l, 1);
    let field_ptr = luaL_checkstring!(vm_l, 2);
    let field = CStr::from_ptr(field_ptr).to_bytes();

    if field == b"tag" {
      let tag = get_tag(l, self_ty);
      // `tag` is an owned Rust String with no trailing NUL. Pushing it through
      // the C-string lua_pushstring scanned past its bytes into adjacent memory,
      // yielding garbage like "table\u{7f}" (nondeterministic — passed locally,
      // failed in CI; type_function_user_tag_field). Use the length-aware
      // lua_pushlstring to copy exactly tag.len() bytes.
      lua_pushlstring(vm_l, tag.as_ptr() as *const c_char, tag.len());
      1
    } else {
      lua_pushvalue(vm_l, lua_upvalueindex(1));
      lua_getfield(vm_l, -1, field_ptr as *const c_char);
      1
    }
  }
}
