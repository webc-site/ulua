//! 本文件对应 `ulua_luai_num2str` 导出符号（源：ulua-vm/src/functions/luai_num_2_str.rs）。
//! vm 侧已收口为 safe 缓冲形态 `luai_num2str_buf(&mut [u8], f64) -> usize`，其指针镜像声明
//! 随之删除；本壳无法再由 `functions/shells.rs` 中透传裸指针的共用宏 `capi_shell!` 直呼
//! （宏体语义不得改，同形宏壳零行为变化），故从宏模板退役、写显式 `extern "C-unwind"`
//! 调用，与 `lua_stackdepth.rs`/`lua_g_getline.rs` 等表示适配显式壳统形：唯一差异是在本帧
//! 把 `buf` 重建为契约上界的可写切片后转调，`unsafe` 只留在本 FFI 边界。
use core::{ffi::c_char, slice};

use ulua_vm::{
  functions::luai_num_2_str::luai_num2str_buf, macros::luai_maxnum_2_str::LUAI_MAXNUM2STR,
};

/// # Safety
/// C ABI 导出壳（符号 `ulua_luai_num2str`），除把 `buf` 在本帧重建为 `LUAI_MAXNUM2STR`
/// 字节的可写切片外，仅透传至
/// `ulua_vm::functions::luai_num_2_str::luai_num2str_buf(buf_slice, n)`，零业务逻辑。
/// 调用方须保证：
/// - `buf`（`*mut c_char`）：指向可写至少 `LUAI_MAXNUM2STR`（cpp lnumutils.h:127）字节的
///   缓冲，非空、对齐，整个调用期间存活，且调用期间无其它别名读写（写出内容即本缓冲）；
/// - `n`：值型参数（待转串的 `f64`），无指针前提；
/// - 返回值（`*mut c_char`）：指向 `buf` 内写出内容末尾（`buf + 写出长度`），由调用方按此
///   定长使用；指向调用方自有缓冲内，不新分配内存；
/// - 其余安全前置条件与被调函数的契约一致。
#[unsafe(export_name = "ulua_luai_num2str")]
pub unsafe extern "C-unwind" fn luai_num2str(buf: *mut c_char, n: f64) -> *mut c_char {
  // Safety: 契约声明 `buf` 可写 LUAI_MAXNUM2STR 字节，切片按同一上界重建（`c_char` 与
  // `u8` 宽度均为 1，不改地址与可写性），借用仅在本表达式内存在，被调写入受该上界约束。
  let len = unsafe {
    luai_num2str_buf(
      slice::from_raw_parts_mut(buf.cast::<u8>(), LUAI_MAXNUM2STR as usize),
      n,
    )
  };
  // Safety: 被调写出长度 `len <= LUAI_MAXNUM2STR`（缓冲上界即其契约），故 `buf.add(len)`
  // 仍落在调用方声明的可写区内，指针偏移不越界。
  unsafe { buf.add(len) }
}
