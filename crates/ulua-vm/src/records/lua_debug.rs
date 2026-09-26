//! Source: `VM/include/lua.h:488-502` (hand-ported; was a `_private: ()`
//! placeholder — the lying-record class)

/// C++ `struct lua_Debug` — activation record. `LUA_IDSIZE` = 256
/// (luaconf.h:71).
use core::ffi::c_char;
use core::ffi::c_void;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LuaDebug {
  pub name: *const c_char,      // (n)
  pub what: *const c_char,      // (s) `Lua', `C', `main', `tail'
  pub source: *const c_char,    // (s)
  pub short_src: *const c_char, // (s)
  pub linedefined: i32,         // (s)
  pub currentline: i32,         // (l)
  /// (p) VM 内全局唯一的 proto id；C 函数为 0
  pub protoid: i32,
  /// (p) proto 在自身字节码模块内的下标；C 函数为 -1
  pub bytecodeid: i32,
  pub nupvals: u8,      // (u) number of upvalues
  pub nparams: u8,      // (a) number of parameters
  pub isvararg: c_char, // (a)
  /// only valid in luau_callhook
  pub userdata: *mut c_void,

  pub ssbuf: [c_char; 256],
}
