use alloc::string::String;

use ulua_common::functions::c_str::cstr_cow;
use ulua_vm::{
  macros::{lua_getglobal::lua_getglobal, lua_pop::lua_pop, lua_tostring::lua_tostring},
  records::lua_state::LuaState,
};

/// 全局变量名 `capturedoutput`（NUL 结尾字节串，`lua_getglobal` 的 `*const c_char` 契约用）。
const GLOBAL_CAPTURED_OUTPUT: &[u8] = b"capturedoutput\0";

/// 读取全局 `capturedoutput`（对齐 cpp `getCapturedOutput`，
/// ReplFixture/ReplWithPathFixture 公共实现）。
///
/// # Safety
/// `l` 必须指向已初始化且定义了 `capturedoutput` 全局的 `LuaState`。
pub(crate) unsafe fn captured_output(l: *mut LuaState) -> String {
  // Safety: l 为契约保证的存活 state；`GLOBAL_CAPTURED_OUTPUT` 常量编译期即 NUL 结尾，
  // 指针恒有效；`lua_tostring!`（即 `lua_tolstring` 传 null 长度出参，C-API 约定
  // 表示不需要长度）对栈上非字符串值返回 null（全局未定义时），下一行判空后再
  // `cstr_cow`；串在被 `lua_pop` 弹出前由栈槽保活，其间无任何改栈操作。
  unsafe {
    lua_getglobal(l, GLOBAL_CAPTURED_OUTPUT.as_ptr().cast());
    let str_ptr = lua_tostring!(l, -1);
    let result = if str_ptr.is_null() {
      String::new()
    } else {
      cstr_cow(str_ptr).into_owned()
    };
    lua_pop(l, 1);
    result
  }
}
