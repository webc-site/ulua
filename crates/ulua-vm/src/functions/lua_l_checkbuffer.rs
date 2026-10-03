use core::ffi::c_void;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tobuffer::lua_tobuffer_bytes_ref, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// Rust 内部核心（r11 R-C T1 出参收口，形制对齐 `lua_l_checklstring_ref`；r16-p28
/// 锚定形）：cpp `luaL_checkbuffer`（`laux.cpp:150`）的切片形态——栈槽 `narg` 处为
/// buffer 时返回其数据块的可变借用切片，否则按 cpp 抛 "buffer expected"（`tag_error` 发散）。
///
/// C 形 `size_t* len` 出参收口为切片长度；出参折算由垫片 [`lua_l_checkbuffer`] 独家
/// 承接，Rust 窗口消费方一律直接用本函数。
///
/// 调用序契约（正确性，非内存安全）——r16-p28 起返回切片锚定 `l` 的 `&mut` 借用
/// （生命周期省略，未受约束 `'a` 已消灭）：持窗期间 `l` 被独占借用钉住，不得再经 `l`
/// 读参、压栈或触发分配/GC（编译器拒绝）；窗口稳定三要素（buffer 定长不 resize、GC 不
/// 移动对象、栈槽引用钉住存活期，见 [`lua_tobuffer_bytes_ref`]）由该借用承载，存续上界
/// 即借用结束点。纯安全代码自此无法把窗口实例化为 `'static` 取走。`l` 仍须为正在执行
/// 的 C 函数帧的存活 `LuaState` 且处于可抛错受保护帧，`narg` 为其合法栈索引。
pub fn lua_l_checkbuffer_ref(l: &mut LuaState, narg: i32) -> &mut [u8] {
  unsafe {
    // SAFETY: 转发 `lua_tobuffer_bytes_ref`；其裸窗借出在本门面收窄为不长于 `l` 借用
    // （只此一向收窄，栈槽引用钉住存续上界，见函数文档契约）
    match lua_tobuffer_bytes_ref(l, narg) {
      Some(bytes) => bytes,
      // `None` 即 cpp 的 NULL 失败路径：按 `luaL_checkbuffer` 语义抛错、不返回
      None => tag_error(l, narg, LuaType::Buffer as i32),
    }
  }
}

/// C-ABI 镜像垫片：把 [`lua_l_checkbuffer_ref`] 的切片折回 laux 约定的
/// `(void*, size_t* len)` 出参形——成功路径写出数据块长度，`len` 可为 null。
///
/// r12 T9 裁决保留：ulua-vm 内消费面实测为零（簇 B 已全迁 [`lua_l_checkbuffer_ref`]），
/// ulua-capi 亦无引用；唯一真实消费方是 ulua-conformance 测试门面（safe_api.rs 的
/// `l_checkbuffer_ptr`/`l_checkbuffer_len`，api.rs 据此断言与 `lua_tobuffer` 的指针
/// 同一性）——即 cpp `luaL_checkbuffer`（laux.cpp:150）镜像契约的验证面。函数体保持
/// 对 ref 核心的一行委托 + 出参折回（折回即本垫片存在的全部理由）。
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
