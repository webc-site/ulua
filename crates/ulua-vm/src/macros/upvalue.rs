#[macro_export]
macro_rules! upvalue {
  ($o:expr) => {
    // C 的 `&(o)->value.gc->uv`（lobject.h:97）只是一个裸指针：UpVal 会被 GC 与
    // 闭包/线程链表并发改写，返回 `&mut` 会违反借用栈的唯一性。
    $crate::macros::check_exp::check_exp!(
      (*$o).is_upval(),
      core::ptr::addr_of_mut!((*(*$o).value.gc).uv) as *mut $crate::records::up_val::UpVal
    )
  };
}

pub use upvalue;
