//! `extern "C" executeScript`（cpp `CLI/src/Web.cpp:184-208`）的集成测试。

use core::{ffi::CStr, ptr::null};
use std::ffi::CString;

use ulua_web::functions::execute_script::execute_script;

/// 成功脚本返回 null（结果为空）。
#[test]
fn clean_source_returns_null() {
  // SAFETY: 字面量 C 字符串。
  unsafe {
    assert!(execute_script(c"local x = 1".as_ptr()).is_null());
  }
}

/// 错误脚本返回缓存的错误信息 C 指针。
#[test]
fn error_source_returns_cached_message() {
  // SAFETY: 字面量 C 字符串。
  unsafe {
    let ptr = execute_script(c"error('boom')".as_ptr());
    assert!(!ptr.is_null());
    let msg = CStr::from_ptr(ptr).to_string_lossy();
    assert!(msg.contains("boom"), "got: {msg}");
  }
}

/// 返回指针指向调用方持有的缓存：下一次调用前内容保持稳定。
#[test]
fn returned_pointer_owns_a_copy_of_the_source_bytes() {
  // SAFETY: 自建 C 字符串在调用期间存活；返回指针指向线程本地缓存。
  unsafe {
    let source = CString::new("error('first')").unwrap();
    let first = execute_script(source.as_ptr());
    assert!(!first.is_null());
    // 释放调用方缓冲后仍可读，证明指针不借用输入。
    drop(source);
    assert!(CStr::from_ptr(first).to_string_lossy().contains("first"));
  }
}

/// null 输入按空脚本处理，返回 null。
#[test]
fn null_source_returns_null() {
  // SAFETY: null 是契约允许的输入。
  unsafe {
    assert!(execute_script(null()).is_null());
  }
}
