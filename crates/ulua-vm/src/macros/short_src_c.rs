//! C 函数短源名 "[C]"（cpp getinfo 的 short_src 占位）。
//!
//! review.md §10 收形：观察面为原生 `&[u8]` 窗（`ShortSrc::set` / `c_file_write_bytes`
//! 消费），无终止 NUL——旧 `*const c_char` 契约的扫描读等值截断已收敛到写端单点。

/// C 函数短源名字节（无终止 NUL）。
pub const SHORT_SRC_C: &[u8] = b"[C]";
