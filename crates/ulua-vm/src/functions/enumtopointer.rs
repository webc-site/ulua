use core::ffi::c_void;

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
    // 地址仅作回调身份键透传：`*mut` 形状由 C 回调签名固定，本函数不据此解写
    gco as *const GCObject as *mut c_void
  }
}
