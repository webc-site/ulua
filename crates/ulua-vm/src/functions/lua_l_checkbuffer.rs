use core::ffi::c_void;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tobuffer::lua_tobuffer_bytes_ref, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// Rust 内部核心（r11 R-C T1 出参收口，形制对齐 `lua_l_checklstring_ref`）：
/// cpp `luaL_checkbuffer`（`laux.cpp:150`）的切片形态——栈槽 `narg` 处为 buffer 时
/// 返回其数据块的可变借用切片，否则按 cpp 抛 "buffer expected"（`tag_error` 发散）。
///
/// C 形 `size_t* len` 出参收口为切片长度；出参折算由垫片 [`lua_l_checkbuffer`] 独家
/// 承接，Rust 窗口消费方一律直接用本函数。
///
/// # Safety
/// `l` 须为正在执行的 C 函数帧的存活 `LuaState` 且处于可抛错受保护帧，`narg` 为其
/// 合法栈索引。返回借用指向栈槽 buffer userdata 的内联数据块，在本次调用期间有效
/// （buffer 定长不 resize、GC 不移动对象、栈槽引用钉住存活期——契约三要素见
/// [`lua_tobuffer_bytes_ref`]，与 cpp「`luaL_checkbuffer` 结果在本次调用内可读写」同契）。
pub fn lua_l_checkbuffer_ref<'a>(l: &mut LuaState, narg: i32) -> &'a mut [u8] {
  unsafe {
    // SAFETY: 转发同契约的 `lua_tobuffer_bytes_ref`；借出寿命 `'a` 由栈槽引用钉住
    match lua_tobuffer_bytes_ref(l, narg) {
      Some(bytes) => bytes,
      // `None` 即 cpp 的 NULL 失败路径：按 `luaL_checkbuffer` 语义抛错、不返回
      None => tag_error(l, narg, LuaType::Buffer as i32),
    }
  }
}

/// C-ABI 镜像垫片：把 [`lua_l_checkbuffer_ref`] 的切片折回 laux 约定的
/// `(void*, size_t* len)` 出参形——成功路径写出数据块长度，`len` 可为 null。
/// 仅供跨 crate / 测试门面等既有 C 形消费点使用；T9 收口时随簇 B 消费方迁移删除。
///
/// # Safety
/// C-ABI 镜像垫片（裸指针出参/入参为 C 约定面，按 review.md §2 保留 unsafe 形）：
/// `l` 与 `narg` 契约同 [`lua_l_checkbuffer_ref`]；槽位不是 buffer userdata 时经
/// `tag_error` 抛 Lua 错误、不返回（不触碰 `*len`）；`len` 必须为可写 `usize` 槽或
/// null。返回指针指向该栈槽 userdata 的数据块，长度 `*len` 字节，在该 userdata 保持
/// 为栈槽值期间存活。cpp laux.cpp:150。
pub unsafe fn lua_l_checkbuffer(l: &mut LuaState, narg: i32, len: *mut usize) -> *mut c_void {
  unsafe {
    let bytes = lua_l_checkbuffer_ref(l, narg);

    // SAFETY: 契约保证 `len` 非空即可写 usize 槽；成功路径写入长度，观察序与旧派生一致
    if !len.is_null() {
      *len = bytes.len();
    }
    bytes.as_mut_ptr().cast::<c_void>()
  }
}

// lualib.h name
