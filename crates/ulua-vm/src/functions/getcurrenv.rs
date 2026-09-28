use crate::{
  macros::curr_func::curr_func,
  records::{lua_state::LuaState, lua_table::LuaTable},
};

/// # Safety
/// `l` 须为存活 LuaState 且 `ci`/`base_ci` 指向同一 CallInfo 数组（`ci == base_ci` 时取 `(*l).gt` 全局表，
/// 否则 `curr_func!` 解引用当前帧闭包并读其 `env`）。返回的 LuaTable 指针仅在对应环境对象存活期间有效。
/// cpp/VM/src/lapi.cpp:81 getcurrenv。
pub(crate) unsafe fn getcurrenv(l: *mut LuaState) -> *mut LuaTable {
  unsafe {
    if (*l).ci == (*l).base_ci {
      (*l).gt
    } else {
      (*curr_func!(l)).env
    }
  }
}
