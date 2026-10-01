use crate::{
  functions::getluaproto::get_lua_proto,
  records::{call_info::CallInfo, lua_state::LuaState, proto::Proto},
};

/// 判定调用栈第 `level` 层是否有原生代码（`luaG_hasnative`）。`l` 以引用传入（存活由
/// 类型保证）；`level` 越界（含负值，`as u32` 后即大于栈深）统一返回 0，`ci`/`base_ci`
/// 为同一 CallInfo 数组内的合法指针是 `lua_State` 结构不变量。只读、不抛错、不分配。
pub fn lua_g_hasnative(l: &LuaState, level: i32) -> i32 {
  // SAFETY: 块内 `ci` 偏移已由上方 `level` 界比较收窄到 base_ci..ci 内；
  // `get_lua_proto` 只读帧函数槽，`(*proto).execdata` 为 Proto 字段读。
  unsafe {
    if (level as u32) >= (l.ci).offset_from(l.base_ci) as u32 {
      return 0;
    }

    let ci: *mut CallInfo = l.ci.offset(-(level as isize));
    let proto: *mut Proto = get_lua_proto(ci);
    if proto.is_null() {
      return 0;
    }

    ((*proto).execdata).is_null() as i32 ^ 1
  }
}
