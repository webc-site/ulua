//! Source: `VM/include/lualib.h` (lualib.h:86-98, hand-ported)

use core::ptr::null_mut;

use crate::records::{lua_state::LuaState, t_string::tstring};

// luaconf.h:96
pub const LUA_BUFFERSIZE: usize = 512;

/// `luaL_Buffer` 的字符串累加缓冲。生命周期不变量（由 `luaL_buffinit`/`luaL_addlstring`/
/// `luaL_pushresult` 一族维护，见 functions/lua_l_buffinit.rs 等）：
/// - 初始化后 `p..end` 恒为同一分配区内的可写游标：未溢出时指向 `buffer` 内联区，
///   溢出后经 `extendstrbuf` 换入 `storage` 持有的 GC 字符串区；`p == end` 表示写满；
/// - `l`/`storage` 在 init 前为 null（cpp 同构），使用期内非空；`storage` 仅在已换入
///   GC 缓冲时指向该 TString，属借用（所有权在栈/串表），不入 GC 图句柄改造范围。
///
/// `#[repr(C)]` 是对外契约：C 宿主可自带 `luaL_Buffer` 内存经 `ulua-capi` 传入，布局须与 C 头一致
/// （`u8` 与 C `char` 同尺寸同对齐，布局逐字节一致）。
#[repr(C)]
#[derive(Debug)]
pub struct LuaLStrbuf {
  /// DELIBERATE DEVIATION（§2 判定=规则 2「指针算式基址」，保留 `*mut`）：`p` 是写游标，
  /// 所有操作都以它做裸地址算术——`b.end.offset_from(b.p)` 求剩余空间、`b.p = b.p.add(len)`
  /// 推进、`copy_nonoverlapping(s, b.p, len)` 落字节（见 `lua_l_addlstring.rs:18,24` 等）。
  /// cpp `lualib.h:88` 里 `p` 由 `luaL_buffinit` 立即写为 `buffer` 首址，使用期恒非空，null
  /// 只在 `new`/`Default` 的「未 init 占位形态」。改 `Option<NonNull<u8>>` 会把每次压字节的
  /// `add`/`offset_from` 热路径逼成 unwrap，且 `Option` 无法参与这些地址算式（§9.4）。
  pub p: *mut u8,
  /// DELIBERATE DEVIATION（§2 判定=规则 2）：`end` 是当前缓冲区写终点（开区间界），仅作
  /// `b.end.offset_from(b.p)` 的被减数与 `b.end = b.p.wrapping_add(LUA_BUFFERSIZE)` 的赋值目标
  /// （`lua_l_buffinit.rs:14`、`extendstrbuf.rs`）。cpp `lualib.h:89` 由 init 立即赋为合法界址，
  /// 使用期非空，null 只在 `new` 占位。理由同 [`Self::p`]：相减/加界址算式无法对 `Option` 表达，
  /// 改型会给每次余量判定加 unwrap（§9.4）。
  pub end: *mut u8,
  /// 借用中的 Lua 状态句柄（cpp `lualib.h:90` `lua_State* L`，所有权在调用方）。§2 判定=规则 1
  /// 「未接线缺席」——理想形态是 `Option<NonNull<LuaState>>`，但本次改动被限定在本定义文件：`l`
  /// 的写点在 `lua_l_buffinit.rs:16`（`b.l = l`）、读点遍布 `extendstrbuf`/`lua_l_addlstring`/
  /// `lua_l_pushresult` 等并行会话持有的文件（裸指针传参、非 `.is_none()` 判定），就地改型会破坏
  /// 那些文件编译。故本轮保留 `*mut LuaState` 并记录此待协调改点，非「照抄 cpp」的机械保留。
  pub l: *mut LuaState,
  /// GC 溢出缓冲句柄：null = 未换入动态串缓冲（仍写内联 `buffer`）。§2 判定=规则 1「缺席」——
  /// 理想形态 `Option<NonNull<tstring>>`（复用 null niche，零尺寸开销）。但本定义文件外，`storage`
  /// 的判空/读写在 `extendstrbuf.rs`（`(*b).storage.is_null()`、`(*b).storage = new_storage`、
  /// `(*(*b).storage).data...`）、`lua_l_pushresult.rs`、`lua_l_buffinit.rs:18`（`b.storage = null_mut()`），
  /// 均属并行会话文件；就地改型令其编译失败。故本轮保留 `*mut tstring` 并记录待协调改点。另注：本
  /// struct 是 `#[repr(C)]` 且经 `ulua-capi` 接收 C 宿主自带的 `luaL_Buffer` 内存（见类型文档），裸
  /// 指针字段也是该对外布局契约的一部分。
  pub storage: *mut tstring,
  /// 内联小缓冲；cpp `alignas(LUAI_MAXALIGN)`，此处由 repr(C) 保证紧随其前字段布局。
  pub buffer: [u8; LUA_BUFFERSIZE],
}

impl LuaLStrbuf {
  /// 空缓冲：字段由 `luaL_buffinit` / `luaL_buffinitsize` 填充。
  pub fn new() -> Self {
    Self {
      // 既有约定（review.md §2）：c-API `#[repr(C)]` 缓冲结构 POD 字段初值——游标 p/end、宿主态 l、动态串缓冲句柄 storage 均以 null 占位（storage=null 即未换入动态缓冲、仍写内联 buffer），裸指针为对外布局契约一部分，勿改 Option；storage 语义详见上方字段 doc
      p: null_mut(),
      end: null_mut(),
      l: null_mut(),
      storage: null_mut(),
      buffer: [0; LUA_BUFFERSIZE],
    }
  }
}

// `new` + `Default` 并存是 clippy(new_without_default) 的要求，非死代码。
impl Default for LuaLStrbuf {
  fn default() -> Self {
    Self::new()
  }
}
