//! cfg 门控的分配计数包装器（`--features count-alloc` 启用）。
//!
//! 口径：只统计 Rust 侧 `GlobalAlloc` 请求（`alloc`/`alloc_zeroed`/`realloc` 各计
//! 一次，字节数按新布局大小）；mlua vendored Luau C++ 直接走 C `malloc`，不经
//! Rust 全局分配器，故分配计数只报 ulua 侧。默认（feature 关闭）不编译本模块，
//! 全局分配器保持裸 `MiMalloc`，既有对比行为与耗时完全不变。

#![cfg(feature = "count-alloc")]

use std::{
  alloc::{GlobalAlloc, Layout},
  sync::atomic::{AtomicU64, Ordering},
};

use mimalloc::MiMalloc;

pub static CALLS: AtomicU64 = AtomicU64::new(0);
pub static BYTES: AtomicU64 = AtomicU64::new(0);

pub struct Counting;

unsafe impl GlobalAlloc for Counting {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    CALLS.fetch_add(1, Ordering::Relaxed);
    BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
    unsafe { MiMalloc.alloc(layout) }
  }

  unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
    CALLS.fetch_add(1, Ordering::Relaxed);
    BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
    unsafe { MiMalloc.alloc_zeroed(layout) }
  }

  unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    CALLS.fetch_add(1, Ordering::Relaxed);
    BYTES.fetch_add(new_size as u64, Ordering::Relaxed);
    unsafe { MiMalloc.realloc(ptr, layout, new_size) }
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    unsafe { MiMalloc.dealloc(ptr, layout) }
  }
}

/// 当前累计 (次数, 字节)。
pub fn snapshot() -> (u64, u64) {
  (CALLS.load(Ordering::Relaxed), BYTES.load(Ordering::Relaxed))
}
