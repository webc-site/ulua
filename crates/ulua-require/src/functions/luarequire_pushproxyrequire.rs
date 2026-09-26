use core::ffi::c_void;

use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{
    lua_proxyrequire::lua_proxyrequire, pushrequireclosureinternal::pushrequireclosureinternal,
  },
  records::luarequire_configuration::LuarequireConfigurationInit,
};

/// # Safety
/// `l` 必须指向存活的 `LuaState`（宿主 Lua/C API 句柄，本 crate 归 ulua-vm
/// C-API 真边界裁定）；`config_init` 为宿主提供的 `extern "C-unwind"` 配置
/// 初始化回调（null 即报错发散，字段须满足 `validate_config` 校验）；`ctx`
/// 为宿主不透明指针，仅转手存为 lightuserdata upvalue，本 crate 不解引用。
pub unsafe fn luarequire_pushproxyrequire(
  l: *mut LuaState,
  config_init: LuarequireConfigurationInit,
  ctx: *mut c_void,
) -> i32 {
  // Safety: l 为宿主提供的有效 LuaState；config_init/ctx 仅转交 pushrequireclosureinternal 存储（ctx 作 lightuserdata，不 deref），Some(lua_proxyrequire) 是本 crate 静态 C 闭包指针——参数生命周期均覆盖调用，无别名/悬挂窗口。
  unsafe {
    pushrequireclosureinternal(
      l,
      config_init,
      ctx,
      Some(lua_proxyrequire),
      b"proxyrequire\0",
    )
  }
}
