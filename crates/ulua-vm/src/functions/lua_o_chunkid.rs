//! r12-w5s T9 切片化 + w6c §10 收形：截断/格式化真实逻辑全部落在切片核心
//! [`lua_o_chunkid_ref`]（safe fn：切片读写 + memchr3 扫描，无裸指针）。
//! C-ABI 镜像垫片已删除——`LuaDebug::short_src` 收形为 [`crate::records::lua_debug::ShortSrc`]
//! （定长 `[u8; N]` + 长度）后，落点由 [`ChunkId`] 枚举以「源窗下标 / 写窗长度」表达，
//! 指针面消费点（`auxgetinfo`/`pusherror`/`lua_l_where`/`loadsafe`）全部改为切片读，
//! 全仓 `lua_o_chunkid` 的 `*mut c_char` return（cpp `buf` 或 `source + 1`）不再有消费者。
//! cpp `lobject.cpp:157`。

use memchr::memchr3;

// 重复字面量提取为常量，逻辑不变（截断省略号；`[string "` 前缀与 `"]` 后缀各
// 只出现一次，留字面）。
const DOTS: &[u8] = b"...";

/// 包裹分支的 cpp 骨架预留量：`sizeof("[string \"...\"]")`。cpp `sizeof` 一个字符串
/// 字面量含终止 NUL，即 14 + 1 = 15。
///
/// DELIBERATE DEVIATION（review.md §0，修正回齐 oracle）：迁移前的本端口写作 `16`，
/// 注释误称「sizeof 含 NUL 即 16」；字面量 `[string "..."]` 实长 14，`sizeof` 为 15。
/// 本常量以表达式重算并与 cpp 取值一致；行为差异仅在「源正文长恰为 inner_buflen+1」
/// 的边界窗（旧写作 `N-1 + "..."`，oracle 为保留整段 N 不截尾），见报告「截断词面变化」。
const WRAP_RESERVE: usize = b"[string \"...\"]".len() + 1;

/// chunkid 落点（对应 cpp `luaO_chunkid` 的两类 return，§10 收形为长度表达）：
/// `=`/`@` 前缀源名未超长时结果留在源名本体（cpp `return source + 1`），否则写回
/// 输出缓冲（cpp `return buf`）。
#[derive(Clone, Copy)]
pub(crate) enum ChunkId {
  /// cpp `return source + 1`：载荷 = 源窗 `source[1..]`（跳过 `=`/`@` 前缀；旧读面
  /// 为 NUL 扫描，源窗即恰覆盖 payload，`[1..]` 与原 strlen 读取逐字节相等）
  Source,
  /// cpp `return buf`：结果已就地写入输出缓冲，携带有效字节数（不含终止 NUL——
  /// 旧核心写 `buf[n] = 0` 后消费面按 NUL 截读，`[..n]` 与其逐字节相等）
  Buf(usize),
}

/// 把 [`ChunkId`] 落点折成结果字节窗（两臂寿命统一为入参公共寿命，零 unsafe）。
pub(crate) fn chunkid_slice<'a>(buf: &'a [u8], source: &'a [u8], site: ChunkId) -> &'a [u8] {
  match site {
    ChunkId::Source => &source[1..],
    ChunkId::Buf(n) => &buf[..n],
  }
}

/// 截断格式化核心（切片形，真实逻辑，safe）：cpp `luaO_chunkid`（lobject.cpp:157）
/// 三分支同形——`=` 前缀就地截尾、`@` 前缀 `"..." + 尾段` 截断、其余包
/// `[string "…"]` 并止于首个 `\n`/`\r`。
///
/// cstr 读模型：`source` 恰覆盖 `srclen` 字节 payload（不含终止 NUL）；cpp 在
/// 串尾点位读到的恒是 Lua 串/TString 布局保证的终止 NUL，本核心以 `first()`/
/// `get()` 越界归一为 0 同点同判（空 payload 即 cpp 读首 NUL 走 else 分支）。
/// `Buf(n)` 臂写入区（含 `buf[n] = 0` 的终止位）均落在 `buf` 界内。
///
/// 前置条件（旧垫片窗派生的承接，消费点全数传 `LUA_IDSIZE` 或 `ShortSrc` scratch 面）：
/// `buf.len() >= 16`（包裹分支骨架扣减面），`@` 截断分支另需 `>= 4`。过小缓冲在
/// cpp 为写越界 UB，此处以切片界检 panic 收口（严格改善面，仓内不可达）。
pub(crate) fn lua_o_chunkid_ref(buf: &mut [u8], source: &[u8]) -> ChunkId {
  let srclen = source.len();
  let buflen = buf.len();

  if source.first() == Some(&b'=') {
    if srclen <= buflen {
      return ChunkId::Source;
    }
    // `buflen == 0` 退化守卫保留原形（cpp 此处 `memcpy(buf, source + 1, -1)`
    // 为 size_t 回绕 UB，既有收口不改动）
    if buflen == 0 {
      return ChunkId::Buf(0);
    }
    // truncate the part after '='：cpp `memcpy(buf, source + 1, buflen - 1)`
    // + `buf[buflen - 1] = '\0'`；`srclen > buflen` 保证读窗 `1..buflen` 界内
    let n = buflen - 1;
    buf[..n].copy_from_slice(&source[1..1 + n]);
    buf[n] = 0;
    ChunkId::Buf(n)
  } else if source.first() == Some(&b'@') {
    if srclen <= buflen {
      return ChunkId::Source;
    }
    if buflen == 0 {
      return ChunkId::Buf(0);
    }
    // truncate the part after '@'：cpp `memcpy(buf, "...", 3)` +
    // `memcpy(buf + 3, source + srclen - (buflen - 4), buflen - 4)` +
    // `buf[buflen - 1] = '\0'`；`srclen > buflen >= 4` 保证尾段起点 ≥ 1、
    // 写点 `3 + tail_len == buflen - 1` 界内
    buf[..DOTS.len()].copy_from_slice(DOTS);
    let tail_len = buflen - DOTS.len() - 1; // cpp `buflen - 4`
    buf[DOTS.len()..DOTS.len() + tail_len].copy_from_slice(&source[srclen - tail_len..]);
    buf[buflen - 1] = 0;
    ChunkId::Buf(buflen - 1)
  } else {
    // buf = [string "string"]

    // C++ `strcspn(source, "\n\r")`：在 srclen 范围内找首个 '\n' / '\r' / NUL
    // （TString 布局保证 srclen 处有终止 NUL，故与 strcspn 语义一致）。
    // memchr3 走 SIMD，扫描上界受 payload 切片约束不越界。
    let len = memchr3(b'\n', b'\r', 0, source).unwrap_or(srclen);

    // cpp `buflen -= sizeof("[string \"...\"]")`（含终止 NUL 的 sizeof，见
    // [`WRAP_RESERVE`] 注）
    let inner_buflen = buflen.saturating_sub(WRAP_RESERVE);
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
    ChunkId::Buf(end)
  }
}
