//! r12-w5s T9 切片化：截断/格式化真实逻辑全部落在切片核心 [`lua_o_chunkid_ref`]
//! （safe fn：切片读写 + memchr3 扫描，无裸指针）。C-ABI 镜像垫片
//! [`lua_o_chunkid`] 保留 cpp `luaO_chunkid` 的指针契约面——返回值须指向
//! `buf` 或 `source + 1` 两种存储之一（`lua_Debug.short_src` C 结构字段由
//! `auxgetinfo` 直写，指针面不可收口），窗口派生只发生在垫片，核心零裸窗。
//! cpp `lobject.cpp:157`。

use core::{
  ffi::c_char,
  slice::{from_raw_parts, from_raw_parts_mut},
};

use memchr::memchr3;

// 重复字面量提取为常量，逻辑不变（截断省略号；`[string "` 前缀与 `"]` 后缀各
// 只出现一次，留字面）。
const DOTS: &[u8] = b"...";

/// chunkid 落点（对应 cpp `luaO_chunkid` 的两类 return）：`=`/`@` 前缀源名
/// 未超长时结果留在源名本体（`return source + 1`），否则写回输出缓冲
/// （`return buf`）。不携带借用——C-ABI 垫片按自身裸参取点，指针即全部消费面。
#[derive(Clone, Copy)]
enum ChunkIdSite {
  /// cpp `return source + 1`：载荷自源名第 2 字节起（跳过 `=`/`@` 前缀）
  Source,
  /// cpp `return buf`：截断/包裹结果已写回输出缓冲（含 `buflen == 0` 退化守卫）
  Buf,
}

/// 截断格式化核心（切片形，真实逻辑，safe）：cpp `luaO_chunkid`（lobject.cpp:157）
/// 三分支同形——`=` 前缀就地截尾、`@` 前缀 `"..." + 尾段` 截断、其余包
/// `[string "…"]` 并止于首个 `\n`/`\r`。
///
/// cstr 读模型：`source` 恰覆盖 `srclen` 字节 payload（不含终止 NUL）；cpp 在
/// 串尾点位读到的恒是 Lua 串/TString 布局保证的终止 NUL，本核心以 `first()`/
/// `get()` 越界归一为 0 同点同判（空 payload 即 cpp 读首 NUL 走 else 分支）。
/// 各分支写入区含终止 NUL 均落在 `buf` 界内。
///
/// 前置条件（由垫片 [`lua_o_chunkid`] 的窗派生承接）：`buf.len() >= 16`
/// （cpp `sizeof("[string \"...\"]")` 扣减面；仓内消费点全数传 `LUA_IDSIZE` 或
/// `lua_Debug.ssbuf.len()`），`@` 截断分支另需 `>= 4`。过小缓冲在 cpp 为写越界
/// UB，此处以切片界检 panic 收口（严格改善面，仓内不可达）。
fn lua_o_chunkid_ref(buf: &mut [u8], source: &[u8]) -> ChunkIdSite {
  let srclen = source.len();
  let buflen = buf.len();

  if source.first() == Some(&b'=') {
    if srclen <= buflen {
      return ChunkIdSite::Source;
    }
    // `buflen == 0` 退化守卫保留原形（cpp 此处 `memcpy(buf, source + 1, -1)`
    // 为 size_t 回绕 UB，既有收口不改动）
    if buflen == 0 {
      return ChunkIdSite::Buf;
    }
    // truncate the part after '='：cpp `memcpy(buf, source + 1, buflen - 1)`
    // + `buf[buflen - 1] = '\0'`；`srclen > buflen` 保证读窗 `1..buflen` 界内
    let n = buflen - 1;
    buf[..n].copy_from_slice(&source[1..1 + n]);
    buf[n] = 0;
    ChunkIdSite::Buf
  } else if source.first() == Some(&b'@') {
    if srclen <= buflen {
      return ChunkIdSite::Source;
    }
    if buflen == 0 {
      return ChunkIdSite::Buf;
    }
    // truncate the part after '@'：cpp `memcpy(buf, "...", 3)` +
    // `memcpy(buf + 3, source + srclen - (buflen - 4), buflen - 4)` +
    // `buf[buflen - 1] = '\0'`；`srclen > buflen >= 4` 保证尾段起点 ≥ 1、
    // 写点 `3 + tail_len == buflen - 1` 界内
    buf[..DOTS.len()].copy_from_slice(DOTS);
    let tail_len = buflen - DOTS.len() - 1; // cpp `buflen - 4`
    buf[DOTS.len()..DOTS.len() + tail_len].copy_from_slice(&source[srclen - tail_len..]);
    buf[buflen - 1] = 0;
    ChunkIdSite::Buf
  } else {
    // buf = [string "string"]

    // C++ `strcspn(source, "\n\r")`：在 srclen 范围内找首个 '\n' / '\r' / NUL
    // （TString 布局保证 srclen 处有终止 NUL，故与 strcspn 语义一致）。
    // memchr3 走 SIMD，扫描上界受 payload 切片约束不越界。
    let len = memchr3(b'\n', b'\r', 0, source).unwrap_or(srclen);

    // buflen -= sizeof("[string \"...\"]")；
    // cpp 的 sizeof 含 NUL 终止符，即减去 16
    let inner_buflen = buflen.saturating_sub(16);
    let len = len.min(inner_buflen);

    // strcpy(buf, "[string \"")
    let prefix_bytes = b"[string \"";
    buf[..prefix_bytes.len()].copy_from_slice(prefix_bytes);

    let mut end = prefix_bytes.len();
    // strncat(buf, source, len)
    if len > 0 {
      buf[end..end + len].copy_from_slice(&source[..len]);
      end += len;
    }
    // cpp `if (source[len] != '\0')`：len == srclen 点位即串尾终止 NUL
    // （payload 越界归一 0），命中则必须截断
    if source.get(len).copied().unwrap_or(0) != 0 {
      // must truncate? strcat(buf, "...")
      buf[end..end + DOTS.len()].copy_from_slice(DOTS);
      end += DOTS.len();
    }
    // strcat(buf, "\"]") + 终止符（end 至多半径 buflen - 1，界内见前置条件注）
    let suffix = b"\"]";
    buf[end..end + suffix.len()].copy_from_slice(suffix);
    end += suffix.len();
    buf[end] = 0;
    ChunkIdSite::Buf
  }
}

/// C-ABI 镜像垫片（窗口派生唯一发生地，真实逻辑见 [`lua_o_chunkid_ref`]）：
/// 把切片核心的落点折回 cpp `luaO_chunkid` 的裸指针 return——`buf` 或
/// `source + 1`。消费面实测：`auxgetinfo` 需指针直写 `lua_Debug.short_src`
/// C 结构字段，`pusherror`/`lua_l_where`/`loadsafe` 经 `cstr_cow`/`cstr_bytes`
/// 门面读回指针串，指针形不可收口，故垫片保留。
///
/// # Safety
/// `buf`/缓冲指针必须存活且 `buf[..buflen]` 可写，输出缓冲满足注释所述最小长度
/// （如 LUA_DEBUG_SOURCEINFO、32+ 字节）；`source[..srclen]` 可读且第 `srclen`
/// 位为终止 NUL（TString 布局/`ChunkName` 构造保证，与 cpp 读终止符的点位
/// 同构）；返回值寿命随 `buf` 或 `source`，与入参共用。
pub(crate) unsafe fn lua_o_chunkid(
  buf: *mut c_char,
  buflen: usize,
  source: *const c_char,
  srclen: usize,
) -> *mut c_char {
  debug_assert!(!buf.is_null());
  debug_assert!(!source.is_null());

  // SAFETY: 契约保证 `buf` 起 `buflen` 字节可写（窗长即 cpp `buflen` 本尊，
  // 不增不减界）；`buf` 与 `source` 两窗不重叠（写侧/读侧分离，cpp memcpy
  // 同源前置）
  let buf_win = unsafe { from_raw_parts_mut(buf.cast::<u8>(), buflen) };
  // SAFETY: 契约保证 `source` 起 `srclen` 字节可读；终止 NUL 点位由核心
  // `first()`/`get()` 越界归一承接（见 [`lua_o_chunkid_ref`] cstr 读模型注）
  let src_win = unsafe { from_raw_parts(source.cast::<u8>(), srclen) };

  match lua_o_chunkid_ref(buf_win, src_win) {
    // cpp `return source + 1`：`=`/`@` 未超长落点在源名本体（const→mut
    // 折返保留原形，调用方按只读 C 串消费）
    ChunkIdSite::Source => unsafe { source.add(1) as *mut c_char },
    // cpp `return buf`
    ChunkIdSite::Buf => buf,
  }
}
