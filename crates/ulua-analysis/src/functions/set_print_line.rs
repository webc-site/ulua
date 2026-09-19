use alloc::string::String;
use core::{
  mem::transmute,
  sync::atomic::{AtomicUsize, Ordering},
};

/// C++ `setPrintLine` 的回调签名（Analysis/src/Luau.cpp）
pub type PrintLineProc = Option<extern "C-unwind" fn(line: &String)>;

/// cpp 的全局 `printLine` 槽位；`0` 表示未挂回调。以 `AtomicUsize` 承载函数
/// 指针，避免 `static mut` 在并发读写下的 UB（同 `assert_handler` 的做法）。
static LUAU_PRINT_LINE: AtomicUsize = AtomicUsize::new(0);

pub fn set_print_line(pl: PrintLineProc) {
  LUAU_PRINT_LINE.store(pl.map(|f| f as usize).unwrap_or(0), Ordering::Relaxed);
}

/// 当前 print 回调；`None` 即槽位为空。
pub fn luau_print_line() -> PrintLineProc {
  let v = LUAU_PRINT_LINE.load(Ordering::Relaxed);
  if v == 0 {
    None
  } else {
    // SAFETY: `v` 由 `set_print_line` 从同型函数指针存入，宽度一致。
    Some(unsafe { transmute::<usize, extern "C-unwind" fn(line: &String)>(v) })
  }
}
