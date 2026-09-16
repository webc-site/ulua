use alloc::string::String;

/// C++ `setPrintLine` 的回调签名（Analysis/src/Luau.cpp）
pub type PrintLineProc = Option<extern "C-unwind" fn(line: &String)>;

pub(crate) static mut LUAU_PRINT_LINE: PrintLineProc = None;

pub fn set_print_line(pl: PrintLineProc) {
  unsafe {
    LUAU_PRINT_LINE = pl;
  }
}
