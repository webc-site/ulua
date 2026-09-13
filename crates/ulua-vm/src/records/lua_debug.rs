//! Node: `cxx:Record:Luau.VM:VM/include/lua.h:488:lua_debug`
//! Source: `VM/include/lua.h:488-502` (hand-ported; was a `_private: ()`
//! placeholder — the lying-record class)

/// C++ `struct lua_Debug` — activation record. `LUA_IDSIZE` = 256
/// (luaconf.h:71).
use core::ffi::c_char;
use core::ffi::{c_int, c_void};
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LuaDebug {
  pub name: *const c_char,      // (n)
  pub what: *const c_char,      // (s) `Lua', `C', `main', `tail'
  pub source: *const c_char,    // (s)
  pub short_src: *const c_char, // (s)
  pub linedefined: c_int,       // (s)
  pub currentline: c_int,       // (l)
  pub nupvals: u8,              // (u) number of upvalues
  pub nparams: u8,              // (a) number of parameters
  pub isvararg: c_char,         // (a)
  /// only valid in luau_callhook
  pub userdata: *mut c_void,

  pub ssbuf: [c_char; 256],
}
