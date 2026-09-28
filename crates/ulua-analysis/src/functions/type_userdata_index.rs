use ulua_vm::{macros::lua_upvalueindex::lua_upvalueindex, records::lua_state};

use crate::{
  functions::{get_tag::get_tag, get_type_user_data::get_type_user_data, push_string::push_string},
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int typeUserdataIndex(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1962`）。
pub unsafe fn type_userdata_index(l: *mut LuaState) -> i32 {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let self_ty = get_type_user_data(l, 1);
    let field = (*vm_l).check_bytes(2);

    if field == b"tag" {
      let tag = get_tag(l, self_ty);
      // `tag` is an owned Rust String with no trailing NUL. Pushing it through
      // the C-string lua_pushstring scanned past its bytes into adjacent memory,
      // yielding garbage like "table\u{7f}" (nondeterministic — passed locally,
      // failed in CI; type_function_user_tag_field). Use the length-aware
      // push_string（长度版）按 tag.len() 精确拷贝。
      push_string(l, tag);
      1
    } else {
      (*vm_l).push_value(lua_upvalueindex(1));
      (*vm_l).get_field_bytes(-1, field);
      1
    }
  }
}
