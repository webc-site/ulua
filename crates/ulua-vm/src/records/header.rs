use core::ptr::null_mut;

use crate::records::lua_state::LuaState;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Header {
  /// cpp `lstrlib.cpp:1175` `Header.lua_State* l`：strlib pack/unpack 的存活状态借用句柄，
  /// 仅作为报错通道（`luaL_error!(h.l, ..)`、`(*h.l).arg_error(..)`），非指针算式基址。
  /// §2 判定=规则 1「未接线缺席」——理想形态 `Option<NonNull<LuaState>>`；但本次改动被限定在本
  /// 定义文件：`l` 的写点在 `initheader.rs:10`（`(*h).l = l`，cpp 同点位由 `Header::default()` 的
  /// null 占位后接线），读点遍布 `getnum.rs`/`getoption.rs`/`getdetails.rs`/`getnumlimit.rs` 等并行
  /// 会话文件，且都经 `luaL_error!`/`arg_error` 以裸 `*mut LuaState` 传参。就地改型会破坏这些文件
  /// 编译，故本轮保留 `*mut LuaState`，null 只出现在 `Default` 占位形态（接线后即非空）。
  pub(crate) l: *mut LuaState,
  pub(crate) islittle: i32,
  pub(crate) maxalign: i32,
}

impl Default for Header {
  fn default() -> Self {
    Self {
      l: null_mut(),
      islittle: 0,
      maxalign: 0,
    }
  }
}
