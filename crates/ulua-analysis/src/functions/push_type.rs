// kTypeUserdataTag is a constant used for Luau Type Function userdata.

use core::mem::size_of;

use ulua_vm::{
  functions::{lua_l_checkstack::lua_l_checkstack, lua_newuserdatatagged::lua_newuserdatatagged},
  records::lua_state::LuaState,
};

use crate::{functions::lua_names::TYPE, type_aliases::type_function_type_id::TypeFunctionTypeId};
const K_TYPE_USERDATA_TAG: i32 = 42;

/// 把 [`TypeFunctionTypeId`] 包成类型 userdata 净压栈顶（`pushType` 收口点）。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本函数
/// 体内不安全只剩 `lua_newuserdatatagged` 一处 C 形态被调而落窄块，故本体为安全 `fn`）：
/// `l` 须为可分配、可抛错的受保护帧（前置由首行 `lua_l_checkstack` 保证栈可增 2 槽）；
/// `TYPE` 为 NUL 结尾字节串，元表缺失时 `set_metatable` 是无操作。
pub fn push_type(l: &mut LuaState, r#type: TypeFunctionTypeId) {
  lua_l_checkstack(l, 2, "allocating type");

  // Safety: `l.as_mut_ptr()` 是 `&mut l` 同一对象的镜像透传（存活与独占由引用承载）；
  // `lua_newuserdatatagged` 是 vm 侧 C 形态门面，分配失败走 VM 错误路径，成功返回按最大
  // 对齐的 `K_TYPE_USERDATA_TAG` 用户数据体，容量恰为 `size_of::<TypeFunctionTypeId>()`。
  let ptr = unsafe {
    lua_newuserdatatagged(
      l.as_mut_ptr(),
      size_of::<TypeFunctionTypeId>(),
      K_TYPE_USERDATA_TAG,
    )
  } as *mut TypeFunctionTypeId;

  // Safety: 上方返回体容量恰为一个 `TypeFunctionTypeId`，写入界内且存活。
  unsafe { *ptr = r#type };

  l.get_metatable_by_bytes(TYPE);
  l.set_metatable(-2);
}
