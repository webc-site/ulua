#[macro_export]
macro_rules! classvalue {
  ($o:expr) => {
    // C 的 `&(o)->value.gc->lclass`（lobject.h:98）只是一个裸指针；`&mut` 会与
    // GC / 其它栈槽的并发裸指针访问冲突（Stacked Borrows 唯一性），故用裸指针。
    $crate::macros::check_exp::check_exp!(
      (*$o).is_class(),
      core::ptr::addr_of_mut!((*(*$o).value.gc).lclass) as *mut $crate::records::luau_class::LuauClass
    )
  };
}

pub use classvalue;
