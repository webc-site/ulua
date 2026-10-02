//! Source: `VM/include/lualib.h` (lualib.h:86-98, hand-ported)

use core::ptr::{null_mut, NonNull};

use crate::records::{lua_state::LuaState, t_string::tstring};

// luaconf.h:96
pub(crate) const LUA_BUFFERSIZE: usize = 512;

/// `luaL_Buffer` 的字符串累加缓冲。生命周期不变量（由 `luaL_buffinit`/`luaL_addlstring`/
/// `luaL_pushresult` 一族维护，见 functions/lua_l_buffinit.rs 等）：
/// - 初始化后 `p..end` 恒为同一分配区内的可写游标：未溢出时指向 `buffer` 内联区，
///   溢出后经 `extendstrbuf` 换入 `storage` 持有的 GC 字符串区；`p == end` 表示写满；
/// - `l`/`storage` 在 init 前为 `None`（cpp 里裸指针的 null 占位），使用期内 `l` 恒为
///   `Some`；`storage` 仅在已换入 GC 缓冲时为 `Some` 指向该 TString，属借用（所有权在栈/串表）。
///
/// `#[repr(C)]` 是对外契约：C 宿主可自带 `luaL_Buffer` 内存经 `ulua-capi` 传入，布局须与 C 头一致
/// （`u8` 与 C `char` 同尺寸同对齐；`l`/`storage` 用 `Option<NonNull>`，其 null niche 与裸指针等宽
/// 等对齐，C 侧 NULL ↔ Rust `None`，故逐字节布局不变；`ulua-capi` 仅透传 `*mut LuaLStrbuf`、不读字段）。
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
  /// Lua 状态句柄（cpp `lualib.h:90` `lua_State* L`，所有权在调用方）。§2 判定=规则 1「缺席」：
  /// 形态为 `Option<NonNull<LuaState>>`——init 前 `None`（`new()`/C 宿主零初始化均对应 cpp 里
  /// 未接线的 NULL 占位），`luaL_buffinit` 接线后为 `Some(NonNull)`，使用期非空（写点见
  /// `lua_l_buffinit.rs`，读点在 `extendstrbuf`/`lua_l_pushresult`/`lua_l_addvalue` 等，均已收敛）。
  /// `Option<NonNull>` 复用 null niche、与裸指针等宽等对齐，故 `#[repr(C)]` 对外布局逐字节不变。
  pub l: Option<NonNull<LuaState>>,
  /// GC 溢出缓冲句柄：`None` = 未换入动态串缓冲（仍写内联 `buffer`，cpp `B->storage == nullptr`），
  /// 换入后为 `Some(NonNull)` 仅在已换入时指向该 TString，属借用（所有权在栈/串表）。§2 判定=规则 1
  /// 「缺席」：`Option<NonNull<tstring>>` 复用 null niche、与裸指针等宽，`#[repr(C)]` 对外布局不变；
  /// 判空/读写消费点已收敛至 `extendstrbuf.rs`/`lua_l_pushresult.rs`/`lua_l_buffinit.rs`，故本轮完成改型。
  pub storage: Option<NonNull<tstring>>,
  /// 内联小缓冲；cpp `alignas(LUAI_MAXALIGN)`，此处由 repr(C) 保证紧随其前字段布局。
  pub buffer: [u8; LUA_BUFFERSIZE],
}

impl LuaLStrbuf {
  /// 空缓冲：字段由 `luaL_buffinit` / `luaL_buffinitsize` 填充。
  pub fn new() -> Self {
    Self {
      // review.md §2：c-API `#[repr(C)]` 缓冲结构字段初值——游标 p/end 保留裸指针（地址算式基址，
      // 见各字段 doc）；宿主态 l 与动态串缓冲句柄 storage 用 `Option<NonNull>` 句柄形态，未 init 时
      // None 即 cpp 里的 null 占位（storage=None 即未换入动态缓冲、仍写内联 buffer）。null niche
      // 保证 `Option<NonNull<T>>` 与裸指针等宽等对齐，故 `#[repr(C)]` 对外布局逐字节不变。
      p: null_mut(),
      end: null_mut(),
      l: None,
      storage: None,
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
