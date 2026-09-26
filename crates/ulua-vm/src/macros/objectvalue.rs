#[macro_export]
macro_rules! objectvalue {
  ($o:expr) => {
    // C 的 `&(o)->value.gc->lobject`（lobject.h:99）只是一个裸指针。
    // 刻意不返回 `&mut`：GC 堆对象会被 GC 与栈上其它 TValue 槽经裸指针并发读写，
    // 造出独占引用会违反 Stacked Borrows 的唯一性（真 UB）。与 hvalue! 同形。
    $crate::macros::check_exp::check_exp!(
      (*$o).is_object(),
      core::ptr::addr_of_mut!((*(*$o).value.gc).lobject) as *mut $crate::records::luau_object::LuauObject
    )
  };
}

pub use objectvalue;
