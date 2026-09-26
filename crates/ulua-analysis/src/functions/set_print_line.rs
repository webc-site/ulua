use parking_lot::Mutex;

use crate::functions::default_luau_print_line::default_luau_print_line;

/// C++ `PrintLineProc`（TypeInfer.h:504，`void (*)(const std::string&)`）；
/// 此处用普通（非 FFI）fn 指针 + `&str` 借用承载——全仓无 C 侧消费者，
/// 假 `extern "C-unwind"` 只会逼出 usize→fn 的 transmute。
pub type PrintLineProc = fn(line: &str);

/// cpp 的全局 `extern PrintLineProc luauPrintLine`（TypeInfer.h:506）。
/// Mutex 包裹保持 `static` 免 `static mut` 的数据竞争 UB；parking_lot 无中毒
/// 语义，天然对齐 cpp std::function（见 r1-analysis item 11 的策略）。
static LUAU_PRINT_LINE: Mutex<Option<PrintLineProc>> = Mutex::new(Some(default_luau_print_line));

/// cpp `setPrintLine`（TypeInfer.h:509，unit test hook）。
pub fn set_print_line(pl: Option<PrintLineProc>) {
  *LUAU_PRINT_LINE.lock() = pl;
}

/// 当前 print 回调；`None` 表示宿主清空了槽位。
pub fn luau_print_line() -> Option<PrintLineProc> {
  *LUAU_PRINT_LINE.lock()
}
