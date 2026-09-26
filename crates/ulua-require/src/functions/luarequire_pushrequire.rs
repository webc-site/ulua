use core::ffi::c_void;

use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{lua_require::lua_require, pushrequireclosureinternal::pushrequireclosureinternal},
  records::luarequire_configuration::LuarequireConfigurationInit,
};

/// # Safety
/// `l` 必须指向存活的 `LuaState`（宿主 Lua/C API 句柄，本 crate 归 ulua-vm
/// C-API 真边界裁定）；`config_init` 为宿主提供的 `extern "C-unwind"` 配置
/// 初始化回调（null 即报错发散），非空时须能按 `luarequire_Configuration`
/// 契约填充整表且满足 `validate_config` 的必填回调校验；`ctx` 为宿主不透明
/// 指针，仅转手存为闭包 lightuserdata upvalue 并在运行期原样交还宿主回调，
/// 本 crate 全程不解引用。
pub(crate) unsafe fn luarequire_pushrequire(
  l: *mut LuaState,
  config_init: LuarequireConfigurationInit,
  ctx: *mut c_void,
) -> i32 {
  // Safety: l 是宿主按 Lua/C API 提供的有效 LuaState；config_init 是宿主实现、签名匹配配置初始化回调的函数指针，ctx 为不透明宿主指针——二者仅被原样转交 pushrequireclosureinternal（在 l 栈 userdata 内构造配置、把 ctx 存为 lightuserdata upvalue，全程不解引用 ctx），故本壳无指针有效性之外的额外前提，无别名/悬挂窗口。
  unsafe { pushrequireclosureinternal(l, config_init, ctx, Some(lua_require), b"require\0") }
}
