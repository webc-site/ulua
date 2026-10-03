

/// 对应 C++ 原生 `static int typeUserdataIndex(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1962`）。
use ulua_vm::{macros::lua_upvalueindex::lua_upvalueindex};
use crate::{functions::{get_tag::get_tag, get_type_user_data::get_type_user_data, push_string::push_string}};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn type_userdata_index(l: &mut LuaState) -> i32 {
  unsafe {
    let self_ty = get_type_user_data(&mut *l, 1);
    let field = l.check_bytes(2);

    if field == b"tag" {
      let tag = get_tag(&mut *l, self_ty);
      // `tag` is an owned Rust String with no trailing NUL. Pushing it through
      // the C-string lua_pushstring scanned past its bytes into adjacent memory,
      // yielding garbage like "table\u{7f}" (nondeterministic — passed locally,
      // failed in CI; type_function_user_tag_field). Use the length-aware
      // push_string（长度版）按 tag.len() 精确拷贝。
      push_string(&mut *l, tag);
      1
    } else {
      l.push_value(lua_upvalueindex(1));
      l.get_field_bytes(-1, field);
      1
    }
  }
}
