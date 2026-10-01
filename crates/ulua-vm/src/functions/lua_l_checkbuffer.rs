use core::ffi::c_void;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tobuffer::lua_tobuffer_bytes_ref, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// C-ABI 镜像垫片（cpp laux.cpp:150；T9 按裁决收敛）：本家族 Rust 侧一律直接消费
/// [`lua_tobuffer_bytes_ref`]（切片窗口）或 `buffer_data_ref`（非 buffer 即抛错形），
/// 本形仅供 laux 边界直传 `size_t*` 出参处使用。内部调 ref 核心、把窗口长度折回
/// 出参——逐字复刻 `lua_l_checklstring` 的「ref 核心 + 垫片」形制。
///
/// # Safety
/// C-ABI 镜像垫片（裸指针出参/入参为 C 约定面，按 review.md §2 保留 unsafe 形）：
///
/// `l` 必须是正在执行的 C 函数帧的存活 `LuaState`，`narg` 为其合法栈索引；槽位不是
/// buffer userdata 时经 `tag_error` 抛 Lua 错误、不返回。`len` 必须为可写 `usize` 槽
/// 或 null（仅成功路径写入数据块长度，抛错路径不触碰）；返回指针指向该栈槽
/// userdata 的数据块，长度 `*len` 字节，在该 userdata 保持为栈槽值期间存活
/// （buffer 定长不 resize、GC 不移动对象，契约详见 [`lua_tobuffer_bytes_ref`]）。
/// cpp laux.cpp:150。
pub unsafe fn lua_l_checkbuffer(l: &mut LuaState, narg: i32, len: *mut usize) -> *mut c_void {
  // SAFETY: 契约保证 l 为存活调用帧、narg 合法栈索引、len 可写或 null；
  // 非 buffer 槽位经 tag_error 抛错、发散不返回
  unsafe {
    let Some(bytes) = lua_tobuffer_bytes_ref(l, narg) else {
      // SAFETY: 契约保证 l 为存活调用帧；非 buffer 槽位经 typeerror 抛错、发散不返回。
      tag_error(l, narg, LuaType::Buffer as i32);
    };

    if !len.is_null() {
      *len = bytes.len();
    }

    bytes.as_mut_ptr().cast::<c_void>()
  }
}

// lualib.h name
