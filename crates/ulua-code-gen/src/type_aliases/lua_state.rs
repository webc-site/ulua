//! CodeGen 视角下的 `LuaState`——VM 自己的类型，并非不透明替身
//! （CodeGen 直接对真实 VM 状态执行操作）。

pub use ulua_vm::records::lua_state::LuaState;
