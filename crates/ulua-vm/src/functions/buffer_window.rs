//! buffer 库的共享访问窗口：cpp `lbuflib.cpp` 里 `luaL_checkbuffer` → 偏移 →
//! `checkRead`/`checkWrite` → 按字节 memcpy 的同形骨架。
//!
//! 12 个 `buffer_*` C 函数都是这一形状（只差数值语义），故把「栈窗口取数据界」「界校验
//! 后定位」「定宽标量按字节读写」各收为单点；调用点只保留自己的实参顺序与数值处理。

use core::{
  mem::{size_of, zeroed},
  ptr::{addr_of, addr_of_mut, copy_nonoverlapping},
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_errors::{buffer_bitcount_error, buffer_oob_error},
    buffer_swapbe::SwapBe,
    lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger,
  },
  macros::isoutofbounds::isoutofbounds,
  records::lua_state::LuaState,
};

/// 栈窗口取 buffer 实参：第 `narg` 号槽的 userdata 数据块首字节与其长度。
///
/// C 形 `size_t* len` 出参在此收口为元组；非 buffer 实参仍由 `luaL_checkbuffer`
/// 经 `luaL_typeerror` 抛错、不返回（与原调用点同位置、同类别）。cpp laux.cpp:150。
///
/// # Safety
/// `l` 须为正在执行的 buffer 库 C 函数帧的存活 `LuaState`，`narg` 为其合法栈索引。
#[inline]
pub(crate) unsafe fn buffer_data(l: *mut LuaState, narg: i32) -> (*mut u8, usize) {
  let mut len: usize = 0;
  // Safety: 契约保证 `l`/`narg` 满足 `luaL_checkbuffer` 的帧与索引约定，`len` 为本帧可写 usize 槽
  let buf = unsafe { lua_l_checkbuffer(l, narg, &mut len) };
  (buf.cast::<u8>(), len)
}

/// 界校验并定位：`[offset, offset + size)` 完整落在数据界内时返回 `buf + offset`，
/// 否则抛 "buffer access out of bounds"（cpp `checkRead`/`checkWrite` 的单点收口）。
///
/// 负偏移按 cpp 一致方式回绕成大 `u32`，必然命中越界分支，故 `add` 只在界内到达。
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
  // Safety: 契约保证 buf/len 为同块 buffer 的数据界，越界即经 buffer_oob_error 抛错不返回
  unsafe {
    if isoutofbounds(offset, len, size) {
      buffer_oob_error(l);
    }

    buf.add(offset as usize)
  }
}

/// 栈窗口一步到位：`buffer_data(l, 1)` + `buffer_at(.., lua_l_checkinteger(l, 2), size)`。
///
/// 仅适用于「先取 #1 buffer、再取 #2 偏移、随即界校验」的读取形（cpp `buffer_read*`）；
/// 写入形需在偏移与校验之间取第 3 号实参，故仍分两步调用。
///
/// # Safety
/// 同 [`buffer_data`] 与 [`buffer_at`]：`l` 为存活 C 函数帧，索引 1 为 buffer、索引 2 为偏移。
#[inline]
pub(crate) unsafe fn buffer_read_window(l: *mut LuaState, size: usize) -> *mut u8 {
  // Safety: 契约保证索引 1 为 buffer（其数据界自洽）、索引 2 为读取偏移
  unsafe {
    let (buf, len) = buffer_data(l, 1);
    let offset = lua_l_checkinteger(l, 2);

    buffer_at(l, buf, len, offset, size)
  }
}

/// 定宽标量的按字节装载（cpp `memcpy(&val, p, sizeof(T))`；大端配置下再翻转字节序）。
///
/// # Safety
/// `src` 须指向 `size_of::<T>()` 个可读字节（由 [`buffer_at`] 的界校验保证）。
#[inline]
pub(crate) unsafe fn load_scalar<T: SwapBe>(src: *const u8) -> T {
  // Safety: 契约保证 src 起 size_of::<T>() 字节可读；val 为本帧可写局部
  unsafe {
    let mut val: T = zeroed();
    copy_nonoverlapping(src, addr_of_mut!(val).cast::<u8>(), size_of::<T>());

    if LUAU_BIG_ENDIAN {
      val = val.swap_be();
    }

    val
  }
}

/// 定宽标量的按字节回写（[`load_scalar`] 的对偶；大端配置下先翻转字节序）。
///
/// # Safety
/// `dst` 须指向 `size_of::<T>()` 个可写字节（由 [`buffer_at`] 的界校验保证）。
#[inline]
pub(crate) unsafe fn store_scalar<T: SwapBe>(dst: *mut u8, mut val: T) {
  // Safety: 契约保证 dst 起 size_of::<T>() 字节可写；val 为本帧局部
  unsafe {
    if LUAU_BIG_ENDIAN {
      val = val.swap_be();
    }

    copy_nonoverlapping(addr_of!(val).cast::<u8>(), dst, size_of::<T>());
  }
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
  // Safety: 校验失败即抛错不返回，通过的区间必落在数据界内且 ≤ 8 字节
  unsafe {
    if bitoffset < 0 {
      buffer_oob_error(l);
    }
    if (bitcount as u32) > 32 {
      buffer_bitcount_error(l);
    }
    if bitoffset as u64 + bitcount as u64 > len as u64 * 8 {
      buffer_oob_error(l);
    }
    (
      (bitoffset / 8) as usize,
      ((bitoffset + bitcount as i64 + 7) / 8) as usize,
    )
  }
}
