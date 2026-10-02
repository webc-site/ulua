//! buffer 库的共享访问窗口：cpp `lbuflib.cpp` 里 `luaL_checkbuffer` → 偏移 →
//! `checkRead`/`checkWrite` → 按字节 memcpy 的同形骨架。
//!
//! r11 R-C T1 窄腰 + r12 T9 读写侧提前收口：全部真实逻辑落在切片核心
//! （[`buffer_data_ref`] / [`buffer_at_ref`] / [`buffer_read_window_ref`] 与签名安全的
//! [`load_scalar_ref`] / [`store_scalar_ref`]）——T2–T5/T8 消费方迁移完毕后，旧裸指针
//! 委托垫片已全部清零。窗口派生的唯一 unsafe 点在
//! `lua_tobuffer_bytes_ref`（lua_tobuffer.rs），本模块不再出现任何数据窗裸构造。

use core::{
  mem::size_of,
  ptr::{read_unaligned, write_unaligned},
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_errors::{buffer_bitcount_error, buffer_oob_error},
    buffer_swapbe::SwapBe,
    lua_l_checkbuffer::lua_l_checkbuffer_ref,
  },
  macros::isoutofbounds::isoutofbounds,
  records::lua_state::LuaState,
};

/// 栈窗口取 buffer 实参（切片核心）：第 `narg` 号槽 userdata 数据块的可变借用切片，
/// 数据界即切片界——旧 `(*mut u8, usize)` 元组的出参收口形态（review.md §3）。
/// 非 buffer 实参经 `lua_l_checkbuffer_ref` 抛 "buffer expected"、不返回。cpp laux.cpp:150。
///
/// # Safety
/// `l` 须为正在执行的 buffer 库 C 函数帧的存活 `LuaState`，`narg` 为其合法栈索引；
/// 借出寿命 `'a` 由栈槽引用钉住（契约三要素见 `lua_tobuffer_bytes_ref`）。
#[inline]
pub(crate) unsafe fn buffer_data_ref<'a>(l: *mut LuaState, narg: i32) -> &'a mut [u8] {
  // SAFETY: 契约与 `lua_l_checkbuffer_ref` 同（存活帧 + 合法栈索引），`&mut *l` 一次性
  // 重借用即其期望的接收者形
  unsafe { lua_l_checkbuffer_ref(&mut *l, narg) }
}

/// 界校验并定位（切片核心）：`[offset, offset + size)` 完整落在数据界内时返回子切片，
/// 否则抛 "buffer access out of bounds"（cpp `checkRead`/`checkWrite` 的单点收口）。
///
/// 负偏移按 cpp 一致方式回绕成大 `u32`，必然命中越界分支；校验通过后切片索引必然
/// 在界内——契约违约（绕过校验）暴露为 panic 级索引，是相对旧形 UB 级 `add` 的严格改善面。
///
/// # Safety
/// `buf` 的数据界自洽（通常取自 [`buffer_data_ref`]）；抛错路径要求 `l` 处于可捕获
/// 错误的受保护帧。
#[inline]
pub(crate) unsafe fn buffer_at_ref(
  l: *mut LuaState,
  buf: &mut [u8],
  offset: i32,
  size: usize,
) -> &mut [u8] {
  // 纯界校验（安全 const 算术）留在 unsafe 外；仅抛错调用收进窄块。
  if isoutofbounds(offset, buf.len(), size) {
    // SAFETY: 契约保证 `l` 为存活调用帧，buffer_oob_error 抛错不返回
    unsafe { buffer_oob_error(l) };
  }

  &mut buf[offset as u32 as usize..][..size]
}

/// 栈窗口一步到位（切片核心）：[`buffer_data_ref`]`(l, 1)` +
/// [`buffer_at_ref`]`(.., l.check_integer(2), size)`。
///
/// 仅适用于「先取 #1 buffer、再取 #2 偏移、随即界校验」的读取形（cpp `buffer_read*`）；
/// 写入形需在偏移与校验之间取第 3 号实参，故仍分两步调用。
///
/// # Safety
/// `l` 为存活 C 函数帧，索引 1 为 buffer、索引 2 为偏移（契约同 [`buffer_data_ref`]
/// 与 [`buffer_at_ref`]）。
#[inline]
pub(crate) unsafe fn buffer_read_window_ref<'a>(l: *mut LuaState, size: usize) -> &'a mut [u8] {
  unsafe {
    let buf = buffer_data_ref(l, 1);
    let offset = (*l).check_integer(2);

    buffer_at_ref(l, buf, offset, size)
  }
}

/// 定宽标量的按字节装载（切片核心，签名安全）：cpp `memcpy(&val, p, sizeof(T))` +
/// 大端配置下 `SwapBe` 单点翻转的等价收口——buffer 规范字节布局为小端，逐字节解释与
/// `from_le_bytes` 全域一致（`read_unaligned` 覆盖数据块的对齐不确定性，与旧形逐位同义）。
///
/// 入参窗口短于 `size_of::<T>()` 时以 `assert!` panic 收口——相对旧裸形契约违约的
/// UB 是严格改善面。彻底改走 `from_le_bytes` 需为 `SwapBe` 增补关联字节数组
/// （buffer_swapbe.rs，出 T1 文件范围），登记至 T9。
#[inline]
pub(crate) fn load_scalar_ref<T: SwapBe>(src: &[u8]) -> T {
  assert!(
    src.len() >= size_of::<T>(),
    "load_scalar_ref: 窗口短于标量宽度"
  );

  // SAFETY: 上方断言保证 `src` 起 size_of::<T>() 字节可读；SwapBe 密封闭集全为定宽
  // POD（整型与浮点 f32/f64 经 to_bits 位模式，任意位模式均合法），read_unaligned 无对齐承诺
  let mut val = unsafe { read_unaligned(src.as_ptr().cast::<T>()) };

  if LUAU_BIG_ENDIAN {
    val = val.swap_be();
  }

  val
}

/// 定宽标量的按字节回写（[`load_scalar_ref`] 的对偶，签名安全）：小端规范布局，
/// 大端配置下经 `SwapBe` 单点翻转；窗口短于 `size_of::<T>()` 时 `assert!` panic 收口。
#[inline]
pub(crate) fn store_scalar_ref<T: SwapBe>(dst: &mut [u8], mut val: T) {
  assert!(
    dst.len() >= size_of::<T>(),
    "store_scalar_ref: 窗口短于标量宽度"
  );

  if LUAU_BIG_ENDIAN {
    val = val.swap_be();
  }

  // SAFETY: 上方断言保证 `dst` 起 size_of::<T>() 字节可写；T 为 POD（SwapBe: Copy）
  unsafe { write_unaligned(dst.as_mut_ptr().cast::<T>(), val) };
}

/// 每字节位数：buffer 位窗口 bit↔byte 换算的单点真相
/// （cpp `lbuflib.cpp:303/306/307/318` 的 `* 8`、`/ 8`、`+ 7`、`& 0x7`）。
/// 建议后续上提到 ulua-common 供全仓复用。
pub(crate) const BITS_PER_BYTE: u32 = 8;

/// readbits/writebits 支持的最大位宽（cpp `lbuflib.cpp:300` `unsigned(bitcount) > 32`）。
pub(crate) const MAX_BITCOUNT: u32 = 32;

/// 位数向上取整为字节数：`(x + BITS_PER_BYTE - 1) / BITS_PER_BYTE` 的单点真相
/// （cpp `lbuflib.cpp:307` `(bitoffset + bitcount + 7) / 8`；校验通过后 x ≥ 0，
/// 截断除法与 cpp 逐位一致）。
pub(crate) const fn align_up_bits_to_bytes(bits: i64) -> i64 {
  (bits + BITS_PER_BYTE as i64 - 1) / BITS_PER_BYTE as i64
}

const _: () = assert!(
  BITS_PER_BYTE == 8
    && MAX_BITCOUNT == 32
    && align_up_bits_to_bytes(0) == 0
    && align_up_bits_to_bytes(1) == 1
    && align_up_bits_to_bytes(8) == 1
    && align_up_bits_to_bytes(9) == 2
    && align_up_bits_to_bytes(MAX_BITCOUNT as i64) == 4
);

/// readbits/writebits 共用的位窗口界校验（cpp lbuflib.cpp:300/316 两处同形检查段），
/// 返回承载 `[bitoffset, bitoffset+bitcount)` 的字节区间；oob→bitcount→oob 的抛错顺序
/// 保持原样（`unsigned(bitcount) > 32` 令负数位宽也命中），故实参读取留在调用点。
///
/// 消费方（簇 C：`buffer_readbits`/`buffer_writebits`）已迁移到切片形——`len` 取自
/// [`buffer_data_ref`] 切片的 `buf.len()`；签名保持裸 `usize`（纯界校验参数，无借用）。
///
/// # Safety
/// `l` 为存活帧，`len` 为同一 buffer 经 [`buffer_data_ref`] 取得的数据界长度（即
/// `buf.len()`），与后续用返回区间索引该切片时自洽。
#[inline]
pub(crate) unsafe fn buffer_bit_bounds(
  l: *mut LuaState,
  len: usize,
  bitoffset: i64,
  bitcount: i32,
) -> (usize, usize) {
  // 纯校验/算术留在 unsafe 外；各抛错调用收进窄块。
  if bitoffset < 0 {
    // SAFETY: `l` 为存活调用帧，buffer_oob_error 抛错不返回
    unsafe { buffer_oob_error(l) };
  }
  if (bitcount as u32) > MAX_BITCOUNT {
    // SAFETY: 同上，buffer_bitcount_error 抛错不返回
    unsafe { buffer_bitcount_error(l) };
  }
  if bitoffset as u64 + bitcount as u64 > len as u64 * BITS_PER_BYTE as u64 {
    // SAFETY: 同上
    unsafe { buffer_oob_error(l) };
  }

  // 校验通过：区间必落在数据界内且 ≤ 8 字节
  (
    (bitoffset / BITS_PER_BYTE as i64) as usize,
    align_up_bits_to_bytes(bitoffset + bitcount as i64) as usize,
  )
}
