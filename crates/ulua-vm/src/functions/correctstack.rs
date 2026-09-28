use crate::{
  functions::c_slice_mut,
  records::{lua_state::LuaState, up_val::UpVal},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn correctstack(l: *mut LuaState, oldstack: *mut TValue) {
  unsafe {
    let stack_bytes = (*l).stack as *mut u8;
    let oldstack_addr = oldstack as isize;

    // 栈搬迁偏移：把旧栈上的指针重映射到新 `stack` 基址；具名闭包消除多处重复裸偏移式。
    let remap =
      |p: *mut TValue| stack_bytes.wrapping_offset(p as isize - oldstack_addr) as *mut TValue;

    (*l).top = remap((*l).top);

    // 开上值经 threadnext 串成链表（非连续数组），§3 仅对连续数组走迭代器，此处保留指针追逐。
    let mut up: *mut UpVal = (*l).openupval;
    while !up.is_null() {
      (*up).v = remap((*up).v);
      up = (*up).u.open.threadnext;
    }

    // base_ci..=ci 是连续 CallInfo 数组：切片迭代取代 `while ci <= ci` 手工裸偏移自增。
    let frames = c_slice_mut((*l).base_ci, (*l).ci.offset_from((*l).base_ci) as usize + 1);
    for frame in frames {
      frame.top = remap(frame.top);
      frame.base = remap(frame.base);
      frame.func = remap(frame.func);
    }

    (*l).base = remap((*l).base);
  }
}
