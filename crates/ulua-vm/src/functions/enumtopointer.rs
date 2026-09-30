use core::{ffi::c_void, ptr::from_ref};

use crate::records::gc_object::GCObject;

/// GC 枚举的对象地址序列化（cpp `VM/src/lgcdebug.cpp:754` `enumtopointer`）。
///
/// 只读门面（review §2）：入参由 `&mut GCObject` 降为 `&GCObject`——枚举期间对象正被共享遍历，
/// 索取可变借用会谎称排他。UserData 分支经安全门面 `as_udata`（tag 匹配后才触碰 union 分支）
/// 只取创建时登记的载荷地址，其余类型返回自身地址。返回值仅作 node/edge 回调的不透明身份键
/// （C ABI 形状要求 `*mut c_void`，回调侧按地址配对、从不解引用），故全程 safe。
#[inline]
pub(crate) fn enumtopointer(gco: &GCObject) -> *mut c_void {
  if let Some(u) = gco.as_udata() {
    u.data.as_ptr() as *mut c_void
  } else {
    from_ref(gco).cast::<c_void>()
  }
}

/// 空节点名（cpp `nullptr`）：枚举族各叶子在无命名信息时统一交此哨兵，
/// 由 [`crate::functions::enumnode::enumnode`] 的 `objname` 参数承载可空性。
pub(crate) const NO_OBJNAME: *const core::ffi::c_char = null();
