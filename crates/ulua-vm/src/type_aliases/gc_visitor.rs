use core::ffi::c_void;

use crate::records::{gc_object::GCObject, lua_page::lua_Page};

/// Heap-walk callback, mirroring cpp `bool (*)(void* context, lua_Page* page, GCObject* gco)`
/// (`cpp/VM/src/lmem.h:37`).
///
/// Rust ABI rather than `extern "C"`: every in-tree visitor is a Rust `unsafe fn`.
///
/// # Safety
///
/// 实现者：`page` 须指向存活 `lua_Page`、`gco` 须指向该页内存活的 `GCObject`、
/// `context` 须与调用方约定的实际类型一致，返回 `false` 依 cpp 约定中止遍历。
/// 调用方：仅在同一线程、对象已初始化时经 GC 遍历回调之。
pub type GcVisitor =
  unsafe fn(context: *mut c_void, page: *mut lua_Page, gco: *mut GCObject) -> bool;
