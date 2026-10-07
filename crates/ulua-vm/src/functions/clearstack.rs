use crate::{
  functions::c_slice_mut, macros::setnilvalue::setnilvalue, records::lua_state::LuaState,
};

/// 栈顶 `top..stack+stacksize` 补空区间逐格置 nil（w6e 收形：`l` 折为 `&mut` 引用形参，
/// 安全 fn；真实裸指针算术下沉到唯一的逐句窄 `unsafe` 块）。
///
/// 调用序契约（正确性，非内存安全）：`l` 的存活与 `top` 落在
/// `stack..stack+stacksize` 界内由 `LuaState` 自持不变量保证（栈数组一致时恒成立），
/// 窄块内仅按该界派生窗口并写栈槽，不越出栈数组。
pub(crate) fn clearstack(l: &mut LuaState) {
  let stack_end = l.stack.wrapping_add(l.stacksize as usize);
  // SAFETY: `stack`/`stacksize`/`top` 为 `LuaState` 自持不变量字段（上方调用序契约），
  // `top` 与 `stack_end` 同属一块栈数组故 `offset_from` 合法；fill 由两界之差非负定界，
  // top..top+fill 覆盖 top 之后的全部槽位且不越出数组。栈窗口 top..stack_end 补空：
  // 原 `while o < stack_end { setnilvalue; o += 1 }` 逐格指针走查收为一次 c_slice_mut
  // 填充（同 lua_settop 的窗口惯例）
  unsafe {
    let fill = stack_end.offset_from(l.top).max(0) as usize;
    for slot in c_slice_mut(l.top, fill) {
      setnilvalue!(slot);
    }
  }
}
