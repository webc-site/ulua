use core::ffi::c_void;

use crate::records::gc_object::GCObject;

/// GC 枚举的对象地址序列化（cpp `VM/src/lgcdebug.cpp:754` `enumtopointer`）。
///
/// B 档契约前移（参照 `abs_index`/`isyielded` 先例）：原 `*mut GCObject` 存活契约
/// 改由 `&mut` 接收者的引用有效性规则在调用点承载。UserData 分支经安全门面
/// `as_udata_mut`（tag 匹配后才触碰 union 分支）只取创建时登记的载荷指针值，
/// 其余类型返回自身地址；全程不解引用返回值——它仅作序列化地址上报。
#[inline]
pub(crate) fn enumtopointer(gco: &mut GCObject) -> *mut c_void {
  if let Some(u) = gco.as_udata_mut() {
    u.data.as_mut_ptr() as *mut c_void
  } else {
    gco as *mut GCObject as *mut c_void
  }
}
