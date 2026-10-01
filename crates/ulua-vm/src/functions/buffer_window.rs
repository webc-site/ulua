//! buffer 库的共享访问窗口：cpp `lbuflib.cpp` 里 `luaL_checkbuffer` → 偏移 →
//! `checkRead`/`checkWrite` → 按字节 memcpy 的同形骨架。
//!
//! 12 个 `buffer_*` C 函数都是这一形状（只差数值语义），故把「栈窗口取数据界」「界校验
//! 后定位」「定宽标量按字节读写」各收为单点；调用点只保留自己的实参顺序与数值处理。
//!
//! R-C 切片分层（r11 T1）：窄腰为下方 `*_ref` 核族——窗口派生收口于
//! `lua_tobuffer_bytes_ref` 的全家族唯一 unsafe 点（栈槽 userdata → `&'a mut [u8]`），
//! 界检定位在 [`buffer_at_ref`]（`isoutofbounds` const 检后切片索引，越界自 UB 级
//! `add` 降为 panic 级），定宽标量读写为全 safe 的 [`load_scalar_ref`]/[`store_scalar_ref`]
//! （from_le_bytes/to_le_bytes + `SwapBe` 单点翻转）。旧裸指针形中 [`buffer_data`] 与
//! [`buffer_read_window`] 降为委托垫片；[`buffer_at`]/[`load_scalar`]/[`store_scalar`]
//! 因裸指针入参与 `T: SwapBe` 泛型界无法折回切片核（委托会引入第二窗口派生点、或把
//! 更强泛型界传染进 T2/T3 消费方文件），本票保留原体窄 unsafe，T9 一并清零。

use core::{
  mem::size_of,
  ptr::{read_unaligned, write_unaligned},
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    buffer_errors::{buffer_bitcount_error, buffer_oob_error},
    buffer_swapbe::SwapBe,
    lua_tobuffer::lua_tobuffer_bytes_ref,
    tag_error::tag_error,
  },
  macros::isoutofbounds::isoutofbounds,
  records::lua_state::LuaState,
};

// ---------------------------------------------------------------------------
// ref 核族（R-C T1 窄腰）
// ---------------------------------------------------------------------------

/// 栈窗口取 buffer 实参（ref 核）：第 `narg` 号槽 userdata 数据块的全部字节窗口。
///
/// C 形 `size_t* len` 出参在此收口为切片长度；非 buffer 实参仍由 `luaL_checkbuffer`
/// 经 `luaL_typeerror` 抛错、不返回（与原调用点同位置、同类别）。窗口派生 unsafe
/// 关在 [`lua_tobuffer_bytes_ref`] 单点（契约三要素与借用纪律见彼处 `# Safety`）。
/// cpp laux.cpp:150。
///
/// # Safety
/// `l` 须为正在执行的 buffer 库 C 函数帧的存活 `LuaState`，`narg` 为其合法栈索引；
/// 返回切片在本次 C 调用期间有效（同 cpp `luaL_checkbuffer` 结果契约）。
#[inline]
pub(crate) fn buffer_data_ref<'a>(l: &mut LuaState, narg: i32) -> &'a mut [u8] {
  // SAFETY: 契约由调用方文档承载；转发 `lua_tobuffer_bytes_ref`，`None` 即 cpp 的
  // NULL 失败路径，按 `luaL_checkbuffer` 语义抛 "buffer expected"（tag_error 发散不返回）
  unsafe {
    lua_tobuffer_bytes_ref(l, narg).unwrap_or_else(|| tag_error(l, narg, LuaType::Buffer as i32))
  }
}

/// 界校验并定位（ref 核）：`[offset, offset + size)` 完整落在数据界内时返回其子窗口，
/// 否则抛 "buffer access out of bounds"（cpp `checkRead`/`checkWrite` 的单点收口）。
///
/// 负偏移按 cpp 一致方式回绕成大 `u32`，必然命中越界分支；校验通过后以
/// `offset as u32 as usize` 做切片索引——派生契约违约时从旧 [`buffer_at`] 的 UB 级
/// `add` 降为 panic 级索引，是严格改善面。
///
/// # Safety
/// `l` 处于可捕获错误的受保护帧（抛错路径）；`buf` 须为同一调用内为对应实参派生
/// 的窗口（同一 userdata 自洽的数据界）。
#[inline]
pub(crate) fn buffer_at_ref<'a>(
  l: &mut LuaState,
  buf: &'a mut [u8],
  offset: i32,
  size: usize,
) -> &'a mut [u8] {
  // 纯界校验（安全 const 算术）留在 unsafe 外；仅抛错调用收进窄块。
  if isoutofbounds(offset, buf.len(), size) {
    // SAFETY: 契约保证 `l` 为存活调用帧，buffer_oob_error 抛错不返回
    unsafe { buffer_oob_error(l) };
  }

  // 界检通过才可达：`offset as u32` 非负且 `+ size ≤ buf.len()`，索引落在数据界内
  &mut buf[offset as u32 as usize..][..size]
}

/// 栈窗口一步到位（ref 核）：`buffer_data_ref(l, 1)` + `check_integer(2)` 偏移定位。
///
/// 仅适用于「先取 #1 buffer、再取 #2 偏移、随即界校验」的读取形（cpp `buffer_read*`）；
/// 写入形需在偏移与校验之间取第 3 号实参，故仍分两步调用 [`buffer_data_ref`] +
/// [`buffer_at_ref`]。
///
/// # Safety
/// `l` 为存活 C 函数帧，索引 1 为 buffer、索引 2 为读取偏移。
#[inline]
pub(crate) fn buffer_read_window_ref<'a>(l: &mut LuaState, size: usize) -> &'a mut [u8] {
  let buf = buffer_data_ref(l, 1);
  let offset = l.check_integer(2);

  buffer_at_ref(l, buf, offset, size)
}

/// 定宽标量 ↔ 小端字节数组换算（ref 标量核的本地 trait）。
///
/// `SwapBe` 只承载字节序翻转、不带数组换算（`from_le_bytes` 需具体 `[u8; N]`，
/// 无法由 `T: SwapBe` 泛型推导），且 `buffer_swapbe.rs` 不在本票改动面，故收口于此；
/// 清单与 `buffer_swapbe` 的密封列表逐字一致，T9 收口时可考虑上提合并。
pub trait ScalarLe: SwapBe + private_scalar_le::Sealed {
  /// 小端装载（LE cfg 下 cpp `memcpy(&val, p, sizeof(T))` 的等价）
  fn load_le(src: &[u8]) -> Self;
  /// 小端回写（[`load_le`](ScalarLe::load_le) 的对偶）
  fn store_le(self, dst: &mut [u8]);
}

mod private_scalar_le {
  /// 密封父 trait：buffer 标量只能是 [`ScalarLe`](super::ScalarLe) 列出的定宽整数
  pub trait Sealed {}
}

macro_rules! impl_scalar_le {
  ($($t:ty),* $(,)?) => {
    $(
      impl private_scalar_le::Sealed for $t {}

      impl ScalarLe for $t {
        #[inline(always)]
        fn load_le(src: &[u8]) -> Self {
          // 契约：src 宽度须覆盖 size_of::<Self>()（由 buffer_at_ref 界检保证），
          // 违约降为 panic 级索引/长度失配，对照旧裸形的 UB 级越界读
          let mut bytes = [0u8; size_of::<Self>()];
          bytes.copy_from_slice(&src[..size_of::<Self>()]);
          Self::from_le_bytes(bytes)
        }

        #[inline(always)]
        fn store_le(self, dst: &mut [u8]) {
          dst[..size_of::<Self>()].copy_from_slice(&self.to_le_bytes());
        }
      }
    )*
  };
}

impl_scalar_le!(i8, u8, i16, u16, i32, u32, i64, u64);

/// 定宽标量的按字节装载（ref 核，全 safe）：小端换算后仅在大端 cfg 下经 `SwapBe`
/// 单点翻转——与旧裸指针 [`load_scalar`] 的 `read_unaligned` 逐字节等价（buffer 数据
/// 块无对齐承诺）。
///
/// `src` 长度须覆盖 `size_of::<T>()`（由 [`buffer_at_ref`] 的界校验保证）。
#[inline]
pub fn load_scalar_ref<T: ScalarLe>(src: &[u8]) -> T {
  let mut val = T::load_le(src);

  if LUAU_BIG_ENDIAN {
    val = val.swap_be();
  }

  val
}

/// 定宽标量的按字节回写（ref 核，全 safe；[`load_scalar_ref`] 的对偶：大端 cfg 下
/// 先翻转字节序再 `to_le_bytes`）。
///
/// `dst` 长度须覆盖 `size_of::<T>()`（由 [`buffer_at_ref`] 的界校验保证）。
#[inline]
pub fn store_scalar_ref<T: ScalarLe>(dst: &mut [u8], mut val: T) {
  if LUAU_BIG_ENDIAN {
    val = val.swap_be();
  }

  val.store_le(dst);
}

// ---------------------------------------------------------------------------
// 旧裸指针形镜像垫片（T2–T8 迁移消费方后，T9 清零）
// ---------------------------------------------------------------------------

/// C 形垫片：把 [`buffer_data_ref`] 的切片折回 `(*mut u8, usize)` 元组，
/// 供尚未迁移的旧消费方（T2–T8 票面）零改动编译。
///
/// # Safety
/// 同 [`buffer_data_ref`]：`l` 为存活 C 函数帧，`narg` 为其合法栈索引。
#[inline]
pub(crate) unsafe fn buffer_data(l: *mut LuaState, narg: i32) -> (*mut u8, usize) {
  // SAFETY: 契约保证 `l`/`narg` 满足 `luaL_checkbuffer` 的帧与索引约定（转发 ref 核）
  let buf = unsafe { buffer_data_ref(&mut *l, narg) };
  (buf.as_mut_ptr(), buf.len())
}

/// 界校验并定位：`[offset, offset + size)` 完整落在数据界内时返回 `buf + offset`，
/// 否则抛 "buffer access out of bounds"（cpp `checkRead`/`checkWrite` 的单点收口）。
///
/// 负偏移按 cpp 一致方式回绕成大 `u32`，必然命中越界分支，故 `add` 只在界内到达。
///
/// 不委托 [`buffer_at_ref`]：入参是裸 `(ptr, len)`，折回切片须再造一次
/// `from_raw_parts_mut`（违「窗口派生单点」窄腰约束），故暂留原体窄 unsafe；
/// T3/T4/T7 消费方迁往 ref 核后本形随 T9 清零。
///
/// # Safety
/// `buf`/`len` 须取自 [`buffer_data`]（同一 userdata 自洽的数据界）；抛错路径要求
/// `l` 处于可捕获错误的受保护帧。
#[inline]
pub(crate) unsafe fn buffer_at(
  l: *mut LuaState,
  buf: *mut u8,
  len: usize,
  offset: i32,
  size: usize,
) -> *mut u8 {
  // 纯界校验（安全 const 算术）留在 unsafe 外；仅抛错调用与地址算术收进窄块。
  if isoutofbounds(offset, len, size) {
    // SAFETY: 契约保证 `l` 为存活调用帧，buffer_oob_error 抛错不返回
    unsafe { buffer_oob_error(l) };
  }

  // SAFETY: 上方校验已排除越界，`offset as usize` 令 `add` 落在 buf..buf+len 界内
  unsafe { buf.add(offset as usize) }
}

/// 栈窗口一步到位：[`buffer_read_window_ref`] 的裸指针形垫片（折 `as_mut_ptr`），
/// 供尚未迁移的读取簇消费方（T2 票面）零改动编译。
///
/// # Safety
/// 同 [`buffer_read_window_ref`]：`l` 为存活 C 函数帧，索引 1 为 buffer、索引 2 为偏移。
#[inline]
pub(crate) unsafe fn buffer_read_window(l: *mut LuaState, size: usize) -> *mut u8 {
  // SAFETY: 契约保证索引 1 为 buffer、索引 2 为读取偏移（转发 ref 核）
  unsafe { buffer_read_window_ref(&mut *l, size) }.as_mut_ptr()
}

/// 定宽标量的按字节装载（cpp `memcpy(&val, p, sizeof(T))`；大端配置下再翻转字节序）。
/// `read_unaligned` 与原「zeroed + copy_nonoverlapping(size_of::<T>())」逐字节等价
/// （buffer 数据块无对齐承诺）。
///
/// 不委托 [`load_scalar_ref`]：ref 核需 `T: ScalarLe` 换算界，本形消费方
/// （`buffer_readinteger` 等）仅带 `T: SwapBe` 界，折委托须改 T2 票面文件的
/// 泛型界（越出本票 3 文件范围）；T2 迁 ref 核后本形随 T9 清零。
///
/// # Safety
/// `src` 须指向 `size_of::<T>()` 个可读字节（由 [`buffer_at`] 的界校验保证）。
#[inline]
pub(crate) unsafe fn load_scalar<T: SwapBe>(src: *const u8) -> T {
  // SAFETY: 契约保证 src 起 size_of::<T>() 字节可读；T 为 POD（SwapBe: Copy）
  let mut val = unsafe { read_unaligned(src.cast::<T>()) };

  if LUAU_BIG_ENDIAN {
    val = val.swap_be();
  }

  val
}

/// 定宽标量的按字节回写（[`load_scalar`] 的对偶；大端配置下先翻转字节序）。
///
/// 不委托 [`store_scalar_ref`] 的理由同 [`load_scalar`]（泛型界传染 T3 票面）。
///
/// # Safety
/// `dst` 须指向 `size_of::<T>()` 个可写字节（由 [`buffer_at`] 的界校验保证）。
#[inline]
pub(crate) unsafe fn store_scalar<T: SwapBe>(dst: *mut u8, mut val: T) {
  if LUAU_BIG_ENDIAN {
    val = val.swap_be();
  }

  // SAFETY: 契约保证 dst 起 size_of::<T>() 字节可写；T 为 POD（SwapBe: Copy）
  unsafe { write_unaligned(dst.cast::<T>(), val) };
}

/// cpp `static_assert(sizeof(T) == sizeof(StorageType))`：const 块在单态化期求值，
/// 尺寸失配在编译期报错，不留运行期 panic 点（浮点按存储宽度重排的前置）。
pub(crate) fn assert_same_width<T, StorageType>() {
  const {
    assert!(
      size_of::<T>() == size_of::<StorageType>(),
      "T 与 StorageType 尺寸必须一致方可按字节重排"
    );
  }
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
/// # Safety
/// `l` 为存活帧，`len` 取自同一 buffer 的 [`buffer_data`]。
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
