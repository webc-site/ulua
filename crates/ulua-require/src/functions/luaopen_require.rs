use core::ffi::c_void;

use ulua_vm::{macros::lua_setglobal::lua_setglobal, records::lua_state::LuaState};

use crate::{
  functions::luarequire_pushrequire::luarequire_pushrequire,
  records::luarequire_configuration::LuarequireConfigurationInit,
};

/// 全局表注册名（NUL 结尾字节串，仅 `lua_setglobal` 收口点转 C 指针）。
const REQUIRE_GLOBAL: &[u8] = b"require\0";

/// # Safety
/// `l` 必须指向存活的 `LuaState`（宿主 Lua/C API 句柄，本 crate 归 ulua-vm
/// C-API 真边界裁定）；`config_init`/`ctx` 前提同 `luarequire_pushrequire`
/// （初始化回调按契约填表、null 即报错发散；ctx 仅转手存储不解引用）。
pub unsafe fn luaopen_require(
  l: *mut LuaState,
  config_init: LuarequireConfigurationInit,
  ctx: *mut c_void,
) {
  // Safety: l 为宿主开启 require 库时提供的存活 LuaState；config_init/ctx
  // 原样转交 `luarequire_pushrequire`（内部再构造 pushrequireclosureinternal
  // 闭包），由其按配置初始化契约分配并校验；REQUIRE_GLOBAL 为静态 NUL 结尾字节串。
  unsafe {
    luarequire_pushrequire(l, config_init, ctx);
    lua_setglobal(l, REQUIRE_GLOBAL.as_ptr().cast());
  }
}
