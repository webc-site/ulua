use core::ffi::c_void;

use crate::records::gc_object::GCObject;

#[inline]
/// # Safety
/// `gco` 须为存活 GCObject；若类型为 UserData，则 `u.data` 必须是创建时登记的
/// 有效载荷指针。返回值仅作序列化地址、本函数不解引用。违反则读出悬垂指针值。cpp lgcdebug.cpp:754。
pub(crate) unsafe fn enumtopointer(gco: *mut GCObject) -> *mut c_void {
  // Safety: 契约保证 `gco` 存活；UserData 分支仅取其 data 字段值，不解引用
  unsafe {
    if let Some(u) = (*gco).as_udata_mut() {
      u.data.as_mut_ptr() as *mut c_void
    } else {
      gco as *mut c_void
    }
  }
}
