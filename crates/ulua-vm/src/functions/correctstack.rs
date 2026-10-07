use crate::{
  functions::c_slice_mut,
  records::{lua_state::LuaState, up_val::UpVal},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// `oldstack` 为栈搬迁前的旧基址，仅取地址参与偏移换算、从不解引用（realloc 后旧区可能已释放，
/// 故保持裸指针形态而非 `&TValue`），只读即可。
pub(crate) unsafe fn correctstack(l: *mut LuaState, oldstack: *const TValue) {
  unsafe {
    let stack_bytes = (*l).stack as *mut u8;
    let oldstack_addr = oldstack as isize;

    // 栈搬迁偏移：把旧栈上的指针重映射到新 `stack` 基址；具名闭包消除多处重复裸偏移式。
    let remap =
      |p: *mut TValue| stack_bytes.wrapping_offset(p as isize - oldstack_addr) as *mut TValue;

    (*l).reanchor_top(remap((*l).top));

    // 开上值经 threadnext 串成链表（非连续数组），§3 仅对连续数组走迭代器，此处保留指针追逐。
    let mut up: *mut UpVal = (*l).openupval;
    while !up.is_null() {
      (*up).v = remap((*up).v);
      up = (*up).u.open.threadnext;
    }

    // base_ci..=ci 是连续 CallInfo 数组：切片迭代取代 `while ci <= ci` 手工裸偏移自增。
    let frames = c_slice_mut((*l).base_ci, (*l).ci.offset_from((*l).base_ci) as usize + 1);
    for frame in frames {
      // r12-w7a1 定性保留：`frame.top/base/func` 为 CallInfo 裸字段重排（栈搬迁
      // remap 本体）——records/slot.rs 边界红线明载「CallInfo 裸字段与帧内算术
      // 不落句柄」；本函数即 realloc 后的指针重读修正协作面，任何预绑定槽窗都会
      // 跨 realloc 悬窗，按 w6d 口径钉死定性、不强收。
      frame.top = remap(frame.top);
      frame.base = remap(frame.base);
      frame.func = remap(frame.func);
    }

    (*l).base = remap((*l).base);
  }
}
