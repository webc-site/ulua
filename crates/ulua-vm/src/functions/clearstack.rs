use crate::{
  functions::c_slice_mut, macros::setnilvalue::setnilvalue, records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `top` 落在 `stack..stack+stacksize` 界内（栈数组
/// 一致时恒成立）：`top..stack_end` 窗口逐格置 nil，不越出栈数组。
pub(crate) unsafe fn clearstack(l: *mut LuaState) {
  unsafe {
    let stack_end = (*l).stack.wrapping_add((*l).stacksize as usize);
    // 栈窗口 top..stack_end 补空：原 `while o < stack_end { setnilvalue; o += 1 }`
    // 逐格指针走查收为一次 c_slice_mut 填充（同 lua_settop 的窗口惯例）
    let fill = stack_end.offset_from((*l).top).max(0) as usize;
    // Safety: fill 由栈数组内两界之差定界，top..top+fill 覆盖 top 之后的全部槽位
    for slot in c_slice_mut((*l).top, fill) {
      setnilvalue!(slot);
    }
  }
}
