//! Source: `VM/src/lbaselib.cpp:438-489` (hand-ported)

use crate::{
  functions::{
    auxopen::auxopen, cstr, lua_b_assert::lua_b_assert_arm, lua_b_error::lua_b_error_arm,
    lua_b_gcinfo::lua_b_gcinfo_arm, lua_b_getfenv::lua_b_getfenv_arm,
    lua_b_getmetatable::lua_b_getmetatable_arm, lua_b_inext::lua_b_inext_arm,
    lua_b_ipairs::lua_b_ipairs_arm, lua_b_newproxy::lua_b_newproxy_arm, lua_b_next::lua_b_next_arm,
    lua_b_pairs::lua_b_pairs_arm, lua_b_pcallcont::lua_b_pcallcont_arm,
    lua_b_pcally::lua_b_pcally_arm, lua_b_print::lua_b_print_arm,
    lua_b_rawequal::lua_b_rawequal_arm, lua_b_rawget::lua_b_rawget_arm,
    lua_b_rawlen::lua_b_rawlen_arm, lua_b_rawset::lua_b_rawset_arm, lua_b_select::lua_b_select_arm,
    lua_b_setfenv::lua_b_setfenv_arm, lua_b_setmetatable::lua_b_setmetatable_arm,
    lua_b_tonumber::lua_b_tonumber_arm, lua_b_tostring::lua_b_tostring_arm,
    lua_b_type::lua_b_type_arm, lua_b_typeof::lua_b_typeof_arm,
    lua_b_xpcallcont::lua_b_xpcallcont_arm, lua_b_xpcally::lua_b_xpcally_arm,
    lua_l_register::lua_l_register_bytes, lua_pushcclosurek::lua_pushcclosurek,
    lua_pushlstring::lua_pushlstring_bytes,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_lib_fn::lua_lib_fn},
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

static BASE_FUNCS: [LuaLReg; 19] = [
  LuaLReg::new(b"assert", lua_b_assert_arm),
  LuaLReg::new(b"error", lua_b_error_arm),
  LuaLReg::new(b"gcinfo", lua_b_gcinfo_arm),
  LuaLReg::new(b"getfenv", lua_b_getfenv_arm),
  LuaLReg::new(b"getmetatable", lua_b_getmetatable_arm),
  LuaLReg::new(b"next", lua_b_next_arm),
  LuaLReg::new(b"newproxy", lua_b_newproxy_arm),
  LuaLReg::new(b"print", lua_b_print_arm),
  LuaLReg::new(b"rawequal", lua_b_rawequal_arm),
  LuaLReg::new(b"rawget", lua_b_rawget_arm),
  LuaLReg::new(b"rawset", lua_b_rawset_arm),
  LuaLReg::new(b"rawlen", lua_b_rawlen_arm),
  LuaLReg::new(b"select", lua_b_select_arm),
  LuaLReg::new(b"setfenv", lua_b_setfenv_arm),
  LuaLReg::new(b"setmetatable", lua_b_setmetatable_arm),
  LuaLReg::new(b"tonumber", lua_b_tonumber_arm),
  LuaLReg::new(b"tostring", lua_b_tostring_arm),
  LuaLReg::new(b"type", lua_b_type_arm),
  LuaLReg::new(b"typeof", lua_b_typeof_arm),
];

const G_NAME: &[u8] = b"_G";
const VERSION_NAME: &[u8] = b"_VERSION";

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// push_value/set_global_bytes/set_field_bytes 皆安全门面直调，注册建表与压串所经
/// `lua_l_register_bytes`/`lua_pushlstring_bytes` 亦已降为安全 `fn`（r12-w6d，裸操作屏障下沉
/// 被调内部），仅 `auxopen` 与 `lua_pushcclosurek` 两处 C 形态被调各自落窄块（被调方自身保留
/// `# Safety`：裸 C 函数指针与 `lua_s_new` 转手未清零），故本体维持安全 `fn`）：`l` 须为可分配、可抛错的受保护帧且栈顶之上
/// 留足空槽；`auxopen`/`lua_pushcclosurek` 的 debugname 为静态 NUL 结尾字面量，臂为本文件静态表
/// 同款合法 `unsafe extern "C-unwind"` 函数。cpp/VM/src/lbaselib.cpp:438-489 luaopen_base。
pub fn luaopen_base(l: &mut LuaState) -> i32 {
  l.push_value(LUA_GLOBALSINDEX);
  l.set_global_bytes(G_NAME);

  // BASE_FUNCS 为本文件同卫生域生成的合法 `unsafe extern "C-unwind"` 臂静态表，名字为
  // 不含尾部 `\0` 的静态字节切片，满足 `lua_l_register_bytes` 切片契约（被调已降为安全
  // `fn`，r12-w6d）
  lua_l_register_bytes(l, Some(G_NAME), &BASE_FUNCS);

  // b"Luau" 为本文件自有的界内静态切片；切片核心已降为安全 `fn`（r12-w6d）
  lua_pushlstring_bytes(l, b"Luau");
  l.set_global_bytes(VERSION_NAME);

  // SAFETY: `l.as_mut_ptr()` 为当前独占借用重建的裸句柄，借用窗止于本次调用；
  // `cstr(b"ipairs\0")` 指向静态 NUL 结尾字面量，两臂为合法 `unsafe extern "C-unwind"`
  // 静态表同款函数，满足 `auxopen` 的存活/有效前提
  unsafe {
    auxopen(
      l.as_mut_ptr(),
      cstr(b"ipairs\0"),
      Some(lua_b_ipairs_arm),
      Some(lua_b_inext_arm),
    );
  }
  // SAFETY: 同上（`cstr(b"pairs\0")` 静态 NUL 字面量，pairs/next 两臂合法）
  unsafe {
    auxopen(
      l.as_mut_ptr(),
      cstr(b"pairs\0"),
      Some(lua_b_pairs_arm),
      Some(lua_b_next_arm),
    );
  }

  // SAFETY: `l.as_mut_ptr()` 重建自当前独占借用、借用窗止于本次调用；pcally/pcallcont
  // 为合法 C 臂且遵循 Lua C 函数约定，debugname 为静态 NUL 字面量，nup=0 无待捕获上值
  unsafe {
    lua_pushcclosurek(
      l.as_mut_ptr(),
      Some(lua_b_pcally_arm),
      cstr(b"pcall\0"),
      0,
      Some(lua_b_pcallcont_arm),
    );
  }
  l.set_field_bytes(-2, b"pcall");

  // SAFETY: 同上（xpcally/xpcallcont 为合法 C 臂，debugname 为静态 NUL 字面量，nup=0）
  unsafe {
    lua_pushcclosurek(
      l.as_mut_ptr(),
      Some(lua_b_xpcally_arm),
      cstr(b"xpcall\0"),
      0,
      Some(lua_b_xpcallcont_arm),
    );
  }
  l.set_field_bytes(-2, b"xpcall");

  1
}

lua_lib_fn!(pub fn luaopen_base @ref, luaopen_base_arm);
