use core::{ffi::c_void, slice::from_raw_parts_mut};

use crate::{
  enums::value_view::ValueView, functions::index_2_addr::index_2_addr,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// Rust 内部核心（R-C T1 窄腰，全家族唯一窗口派生 unsafe 点）：cpp `lua_tobuffer`
/// （`VM/src/lapi.cpp`）的切片形态——`idx` 槽为 buffer 时返回其数据块的可变字节
/// 窗口 `&'a mut [u8]`，非 buffer 返回 `None`。
///
/// C 形 `size_t* len` 出参与「指针 + 长度」二元组收口为切片长度；失败路径不触碰
/// 出参，出参写入由 C-ABI 垫片 [`lua_tobuffer`] 与 [`lua_l_checkbuffer`] 独家承接
/// （同 `lua_tolstring_ref` 的「ref 核心 + 垫片」形制）。
///
/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId。
///
/// 本函数内的 `from_raw_parts_mut` 是全 buffer 家族唯一的窗口派生 unsafe 点，依赖
/// 契约三要素（与 `crates/ulua-rt/src/buffer.rs` `as_slice`/`as_slice_mut` 门面
/// :160-176 的已论证文本同源）：
/// 1. buffer 定长不 resize——`lua_b_newbuffer` 按 `sizebuffer(len)` 单块分配，数据块
///    无独立所有权、无 realloc 路径；
/// 2. GC 不移动对象——数据区随对象体内联，回收仅经 `freeobj` → `lua_b_freebuffer`
///    整块释放，永不搬迁；
/// 3. 栈槽/注册表引用钉住存活期——槽值仍为该 buffer 期间切片有效；借用寿命 `'a` 与
///    槽解耦（同 [`lua_tobuffer`] 既有返回形），调用方保证不跨可致其失效的 VM 操作
///    持有。
///
/// 借用纪律：窗口路径禁 Box/Vec 化与 `mem::forget`（无独立所有权，双放防护是纪律性
/// 约束）；再入面——可能执行 Lua 代码的取参（如 `__tostring` 元方法）之后不得仍
/// 持有本窗口再重派生重叠借用，消费侧统一「后置派生」。
#[inline]
pub unsafe fn lua_tobuffer_bytes_ref<'a>(l: &mut LuaState, idx: i32) -> Option<&'a mut [u8]> {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    match ValueView::from_tvalue(&*o) {
      ValueView::Buffer(b) => {
        // SAFETY: 上方契约三要素成立——`b` 指向存活 buffer 对象，`data` 柔性数组区
        // 自洽含 `len` 个可写字节，借用期内对象不移动、不回收（栈槽/注册表引用钉住）。
        let buf = &mut *b;
        Some(from_raw_parts_mut(
          buf.data.as_mut_ptr().cast::<u8>(),
          buf.len as usize,
        ))
      }
      _ => None,
    }
  }
}

/// C-ABI 适配垫片（T9 按裁决收敛）：把 [`lua_tobuffer_bytes_ref`] 的切片折算回
/// cpp 约定的「`void*` + `size_t* len` 出参」形态，仅供真实 C ABI 边界与既有
/// `Option<&mut c_void>` 消费面（`ulua-rt`）使用；Rust 调用方一律直接消费 ref 核心。
///
/// # Safety
/// 契约自 [`lua_tobuffer_bytes_ref`] 传递（`l` 存活、`idx` 合法栈索引）；此外 `len`
/// 须为可写 `usize` 槽或 null——仅 `Some` 路径写入数据长度，`None` 时不触碰 `*len`，
/// 与 cpp 传 `nullptr` 的既有行为逐位一致。
pub unsafe fn lua_tobuffer<'a>(
  l: *mut LuaState,
  idx: i32,
  len: *mut usize,
) -> Option<&'a mut c_void> {
  // SAFETY: 契约保证 `l` 存活、`idx` 为合法栈索引（转发 `lua_tobuffer_bytes_ref`）；
  // `len` 非空即指向可写 `usize` 槽（C 镜像出参折算，同 `c_str_out_param` 形制）
  unsafe {
    let bytes = lua_tobuffer_bytes_ref(&mut *l, idx)?;

    if !len.is_null() {
      *len = bytes.len();
    }

    bytes.as_mut_ptr().cast::<c_void>().as_mut()
  }
}
