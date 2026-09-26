use crate::records::lua_state::LuaState;

/// # Safety
/// `l` 须为存活 `LuaState` 且调用帧处于一致态：`(*l).ci`、`(*l).base_ci` 均指向其 `base_ci[0..size_ci]` 数组内
/// 且 `ci ≥ base_ci`（差值即当前 C/Lua 调用深度，须非负）。纯指针差值，不解引用帧内容、不分配、不抛错。
/// cpp VM/src/ldebug.cpp:215
pub unsafe fn lua_stackdepth(l: *mut LuaState) -> i32 {
  unsafe { (*l).ci.offset_from((*l).base_ci) as i32 }
}
