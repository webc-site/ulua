use core::{ffi::c_void, slice::from_raw_parts_mut};

use crate::{
  enums::value_view::ValueView,
  functions::index_2_addr::index_2_addr,
  records::{lua_state::LuaState, luau_buffer::LuauBuffer},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// 数据窗唯一的裸构造 unsafe 点：存活 `LuauBuffer` 句柄 → 内联数据块可变借用切片。
/// [`lua_tobuffer_bytes_ref`]（栈索引形）与 [`lua_tobuffer_bytes_from_value`]（FASTCALL
/// 值形）都经此派生——`from_raw_parts_mut` 在 buffer 家族只出现这一处（T10 前 FASTCALL
/// 侧的 `data.as_ptr().add(..)` 手工裸窗即本点的旁路，已收编）。
///
/// # Safety
/// `b` 指向存活 `LuauBuffer`（契约三要素同 [`lua_tobuffer_bytes_ref`]：定长不 resize、
/// GC 不移动、引用槽钉住寿命 `'a`）；柔性数组首址恒非空可解引用。
#[inline]
unsafe fn buffer_bytes_from_handle<'a>(b: *mut LuauBuffer) -> &'a mut [u8] {
  // SAFETY: 契约保证 `b` 指向存活 LuauBuffer，其 `data` 内联块起 `len` 字节可读写、
  // 地址稳定（要素 1/2），存活期由引用槽钉住（要素 3）；柔性数组首址恒非空可解引用。
  unsafe { from_raw_parts_mut((*b).data.as_mut_ptr().cast::<u8>(), (*b).len as usize) }
}

/// buffer 全家族唯一的窗口派生 unsafe 点（r11 R-C T1 窄腰核心）：`idx` 槽为 buffer
/// userdata 时返回其内联数据块的可变借用切片，非 buffer 返回 `None`。
///
/// C 形 `size_t* len` 出参收口为切片长度本身；出参形态的折回由 [`lua_tobuffer`]
/// 垫片独家承接。Rust 侧一切数据窗借用都自本函数（或其值形对偶
/// [`lua_tobuffer_bytes_from_value`]）派生，裸构造收口在
/// [`buffer_bytes_from_handle`] 一处（review.md §2：unsafe 关进有契约的最小边界，
/// 不得渗透到业务逻辑）。
///
/// # Safety
/// 契约三要素（借出上界的论证与 ulua-rt/src/buffer.rs 的 `as_slice` 同源）：
/// 1. buffer 定长不 resize——`data`/`len` 在 `lua_b_newbuffer` 一次性分配后恒定，
///    窗口路径不存在重分配或失效面；
/// 2. GC 不移动对象——收集器只标记/清扫，存活 buffer 的内联数据块地址稳定；
/// 3. 栈槽/注册表引用钉住存活期——借用寿命 `'a` 与 `l` 的借用解耦，要求 `a` 有效期内
///    该 buffer 值始终作为存活栈槽值（或注册表引用）被钉住，否则悬垂借用。
///
/// 另 `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId。
pub unsafe fn lua_tobuffer_bytes_ref<'a>(l: &mut LuaState, idx: i32) -> Option<&'a mut [u8]> {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    match ValueView::from_tvalue(&*o) {
      // SAFETY: 契约与上文逐字同（栈槽 `o` 即钉住寿命的引用槽）
      ValueView::Buffer(b) => Some(buffer_bytes_from_handle(b)),
      _ => None,
    }
  }
}

/// [`lua_tobuffer_bytes_ref`] 的 FASTCALL 值形对偶：直接以实参槽 `TValue` 指针取窗，
/// buffer userdata → 数据块可变借用切片，非 buffer → `None`；全程不抛错——快速调用
/// 判据失败的统一回退（cpp `luauF_*` 返回 -1 转慢路径）由调用方按返回的 `Option`
/// 收口，抛错序留在慢路径库函数本体。
///
/// # Safety
/// `tv` 须指向当前快速调用帧传入的可读 TValue 槽；借出寿命 `'a` 由该槽钉住
/// （[`lua_tobuffer_bytes_ref`] 契约三要素同款——buffer 值在借用有效期内须为存活
/// 栈槽值）。
pub unsafe fn lua_tobuffer_bytes_from_value<'a>(tv: *const TValue) -> Option<&'a mut [u8]> {
  unsafe {
    match ValueView::from_tvalue(&*tv) {
      // SAFETY: 契约保证槽可读且 buffer 值被该槽钉住，与栈索引形唯一差别在取槽方式
      ValueView::Buffer(b) => Some(buffer_bytes_from_handle(b)),
      _ => None,
    }
  }
}

/// C-ABI 镜像垫片：把 [`lua_tobuffer_bytes_ref`] 的切片折回 cpp `lua_tobuffer`
/// （`VM/src/lapi.cpp`）的 `(void*, size_t* len)` 出参形——`idx` 槽为 buffer 时返回其
/// 数据块首字节可变引用并把数据长度写进 `len`（`len` 可为 null，此时仅取址不写长度，
/// 与 cpp 传 `nullptr` 一致）；非 buffer 返回 `None` 且不触碰 `*len`。
///
/// 仅供跨 crate（ulua-rt）与测试门面的既有 C 形消费点使用；T9 收口时随消费方迁移删除。
///
/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId；`len` 须为可写
/// `usize` 槽或 null。返回引用指向 buffer 自有内存，在该 buffer 存活期间有效
/// （[`lua_tobuffer_bytes_ref`] 契约三要素）。
pub unsafe fn lua_tobuffer<'a>(
  l: *mut LuaState,
  idx: i32,
  len: *mut usize,
) -> Option<&'a mut c_void> {
  unsafe {
    let bytes = lua_tobuffer_bytes_ref(&mut *l, idx)?;

    // SAFETY: 契约保证 `len` 非空即指向可写 usize 槽；仅成功路径写入（与旧派生可观察逐点一致）
    if !len.is_null() {
      *len = bytes.len();
    }
    // SAFETY: 内联数据块首址恒非空且对齐 1；借用寿命 `'a` 由栈槽钉住
    // （[`lua_tobuffer_bytes_ref`] 契约），折回裸引用与旧 `as_ptr().as_mut()` 同形。
    Some(&mut *bytes.as_mut_ptr().cast::<c_void>())
  }
}
