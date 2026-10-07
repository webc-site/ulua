use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{get_type_user_data::get_type_user_data, throw_type_error::throw_type_error},
  macros::lua_check_args,
};
pub(crate) fn is_equal_to_type(l: &mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 运行时约定传入并全程存活（经 `c_thunk!` 蹦床重建为独占 `&mut`）。
  // 参数个数不符时 `throw_type_error` 返回 `!` 不返回；`self_ty`/`arg` 来自 `get_type_user_data`，
  // 该函数要么返回指向存活 tagged userdata 的非空 TypeFunctionTypeId、要么抛类型错误（返回 `!`）。
  // 故 `*self_ty == *arg` 解引用合法，二者在本次调用内存活。单线程串行，无并发别名。
  unsafe {
    lua_check_args!(l, != 2, "expected 2 arguments, but got {}");

    let self_ty = get_type_user_data(l, 1);
    let arg = get_type_user_data(l, 2);

    l.push_boolean(*self_ty == *arg);
    1
  }
}
