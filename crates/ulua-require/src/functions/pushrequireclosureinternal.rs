use core::{
  ffi::c_void,
  mem::{size_of, zeroed},
  ptr::{NonNull, write},
};

use ulua_vm::{
  functions::lua_pushcclosurek::lua_pushcclosurek,
  macros::{
    lua_l_error::luaL_error, lua_newuserdata::lua_newuserdata,
    lua_pushlightuserdata::lua_pushlightuserdata,
  },
  records::lua_state::LuaState,
  type_aliases::lua_c_function::LuaCFunction,
};

use crate::{
  functions::{lua_requirecont::lua_requirecont, validate_config::validate_config},
  records::luarequire_configuration::{LuarequireConfigurationInit, luarequire_Configuration},
};

/// # Safety
/// `l` 必须指向存活的 `LuaState`；`config_init` 为宿主 `extern "C-unwind"`
/// 初始化回调（null 即报错发散），须在 `luarequire_Configuration` 契约内填充
/// 本函数刚分配的 userdata（校验交 `validate_config`）；`ctx` 为宿主不透明
/// lightuserdata 指针，仅转手存储、本 crate 不解引用；`requirelikefunc` 须为
/// 静态存活的 `LuaCFunction` 闭包体；`debugname` 须为静态 NUL 结尾字节串
/// （调用点 `b"..\0"`）。
pub(crate) unsafe fn pushrequireclosureinternal(
  l: *mut LuaState,
  config_init: LuarequireConfigurationInit,
  ctx: *mut c_void,
  requirelikefunc: LuaCFunction,
  // NUL 结尾静态字节串（调用点 `b"..\0"`）。
  debugname: &'static [u8],
) -> i32 {
  // Safety: l 为宿主开启库时存活的 LuaState；lua_newuserdata 的返回以 `NonNull`
  // 承接——null（内存耗尽）即经 luaL_error 发散，故其后 `as_ptr()` 写入的块大小必为
  // `size_of::<luarequire_Configuration>()`（与分配同一表达式）、块内无其它引用，
  // zeroed() 初始化合法。
  let config = unsafe {
    let Some(config) = NonNull::new(
      lua_newuserdata(l, size_of::<luarequire_Configuration>()).cast::<luarequire_Configuration>(),
    ) else {
      luaL_error!(l, "failed to allocate memory for require configuration");
    };

    write(config.as_ptr(), zeroed());
    config
  };

  // Safety: config_init 按其契约填字段，validate_config 校验全部回调非空后以
  // `as_ref()` 只读借用（userdata 随后被闭包 upvalue 持有，与闭包同寿命）。
  unsafe {
    let Some(config_init) = config_init else {
      luaL_error!(
        l,
        "require configuration is missing required initializer function"
      );
    };

    config_init(config.as_ptr());
    validate_config(l, config.as_ref());
  }

  // Safety: ctx 是转手存储的 lightuserdata（本 crate 不解引用）；debugname 为静态
  // 生存期字节串（调用点 `b"..\0"`），as_ptr().cast() 交 lua_pushcclosurek 登记；
  // lua_requirecont 为静态存活函数指针。
  unsafe {
    lua_pushlightuserdata(l, ctx);
    lua_pushcclosurek(
      l,
      requirelikefunc,
      debugname.as_ptr().cast(),
      2,
      Some(lua_requirecont),
    );
  }

  1
}
