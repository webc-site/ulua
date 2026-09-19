use core::ffi::c_void;

use crate::records::{gc_object::GCObject, lua_page::lua_Page};

/// Heap-walk callback, mirroring cpp `bool (*)(void* context, lua_Page* page, GCObject* gco)`
/// (`cpp/VM/src/lmem.h:37`).
///
/// Rust ABI rather than `extern "C"`: every in-tree visitor is a Rust `unsafe fn`.
pub type GcVisitor =
  unsafe fn(context: *mut c_void, page: *mut lua_Page, gco: *mut GCObject) -> bool;
