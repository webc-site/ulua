//! Source: `VM/src/lgc.cpp`

/// Returns non-zero if the GC object `o` has been cleared (collected).
///
/// Strings are treated as values and are never considered cleared: their white
/// bits are reset so they will not be swept, and 0 is returned immediately.
/// For all other collectable types the function returns the `iswhite` result —
/// non-zero means the object's white bits are still set, i.e. it was not
/// reached during the mark phase and has been (or will be) collected.
///
/// C++ original: `static int isobjcleared(GCObject* o)` in VM/src/lgc.cpp:608
use crate::{
  macros::{iswhite::iswhite, stringmark::stringmark},
  records::{gc_object::GCObject, t_string::tstring},
};

/// # Safety
/// `o` 必须指向头字段仍可读、尚未交还给分配器的 GCObject（sweep 阶段 `freeargs` 之后的协议窗口内）；
/// String 分支会写其 mark 位（`stringmark`）。对已释放内存调用将读写悬垂对象、返回虚假的"已回收"判定。
/// cpp lgc.cpp:657。
#[inline]
pub(crate) unsafe fn isobjcleared(o: *mut GCObject) -> i32 {
  // Safety: 契约保证 `o` 头字段可读，String 分支仅重写自身 mark 位，其余分支只读颜色位
  unsafe {
    if let Some(ts) = (*o).as_string_mut() {
      // strings are 'values', so they are never weak — stringmark(&o->ts)
      stringmark!(ts as *mut tstring);
      0
    } else {
      // iswhite(o)：白色位仍置位 ⇒ 标记阶段未到达，将被清扫
      iswhite!(o) as i32
    }
  }
}
