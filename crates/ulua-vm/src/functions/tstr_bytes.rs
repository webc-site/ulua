//! `tstring` 串体的原生字节读取单点（review.md §10/§3：C 串 → `&[u8]`）。
//!
//! cpp 侧 `getstr(ts)` 交出 NUL 结尾串首址、各消费点再 `strlen`/透传；本端口
//! TString 布局自带 `len`，串体长度无需扫描即可确定。debug/info 面
//! （`auxgetinfo`/`getfuncname`/`getcoverage`/`getcounters`）一律经本 helper 拿
//! 内容字节（不含终止 NUL），不再借道 NUL 扫描门面。

use core::slice::from_raw_parts;

use memchr::memchr;

use crate::{macros::getstr::getstr, records::t_string::tstring};

/// 取 `tstring` 的 `len` 个内容字节（不含 cpp 观察用的终止 NUL）。
///
/// # Safety
/// `ts` 必须指向存活 `tstring`（`len` 自洽、`data` 柔性数组覆盖 `len + 1` 字节，
/// 既有 TString 布局不变量）；返回借用窗的存活期由调用方按对象真实存活期实例化
/// （借出窗内不得有 GC/清扫令该字符串失效——与原 `getstr` 裸指针契约逐字同一）。
#[inline]
pub(crate) unsafe fn tstr_bytes<'a>(ts: *const tstring) -> &'a [u8] {
  // SAFETY: 契约保证 `ts` 存活；`from_raw_parts` 的界内读窗 `len` 字节由 TString
  // 布局不变量覆盖（`data` 柔性数组 `len + 1`，多出的 NUL 不入窗）。
  unsafe { from_raw_parts(getstr(ts).cast::<u8>(), (*ts).len as usize) }
}

/// C 串观察等值点（review.md §10）：旧消费面对 `*const c_char` 窗按 NUL 扫描读
/// （`strlen`/`cstr_bytes`），即首个 NUL 前的前缀；切片面在写点同位截断后，
/// 每个读取结果与旧扫描读逐字节相等（cpp `lua_Debug` 的 C 串字段面正是该语义）。
#[inline]
pub(crate) fn cut_at_nul(bytes: &[u8]) -> &[u8] {
  let end = memchr(0, bytes).unwrap_or(bytes.len());
  &bytes[..end]
}
