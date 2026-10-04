//! Source: `VM/include/lua.h:488-502` 的激活记录，review.md §10 收形为原生 Rust 形态。
//!
//! cpp 的 `const char* name/what/source/short_src` + `char short_src[LUAI_MAXSHORTSRC]`
//! 在此端口曾是「指针字段 + 裸 `ssbuf` 存储」的混合体，跨 crate 进出全靠 NUL 扫描门面
//! 折转。现在：
//! - `name/what/source` 为 `Option<&'static [u8]>`（不含终止 NUL 的串体字节），
//!   `None` 即原 null 哨兵；
//! - `short_src` 为定长 [`ShortSrc`]（`[u8; N]` + 长度），NUL 截断语义收敛在写端一处；
//! - 本结构不再是 C ABI 镜像（无 `#[repr(C)]`）：仓内没有真实 C 消费者读取该布局，
//!   `LuaHook` 只传递指针，Rust 读写两端类型一致。
//!
//! 存活契约（与原裸指针同级）：`name/source` 借用的串体经闭包→Proto 可达（函数值在
//! 栈/帧上即存活、GC 不移动），`what` 为 VM 静态字面量；`ar` 的读取窗须在写它的
//! `lua_getinfo` 帧与所属 state 存活期内完成——`'static` 是这一既有契约的类型表达，
//! fabrication 收口在 `auxgetinfo`/`getfuncname` 的单点 unsafe（见 `tstr_bytes`）。

use core::ffi::c_void;

use crate::macros::lua_idsize::LUA_IDSIZE;

/// `short_src` 定长缓冲容量：cpp `LUAI_MAXSHORTSRC`（luaconf.h:71 `LUA_IDSIZE` = 256）。
pub const SHORT_SRC_CAP: usize = LUA_IDSIZE as usize;

/// (s) 短源名载体：定长 `[u8; SHORT_SRC_CAP]` + 有效长度。
///
/// 读面 [`ShortSrc::bytes`] 返回 NUL 前的有效字节；写端只经
/// [`crate::functions::auxgetinfo::auxgetinfo`] 的截断核心
/// （[`crate::functions::lua_o_chunkid::lua_o_chunkid_ref`]）落数据，
/// 长度恒不超过缓冲，故读面无需再判 NUL/越界。
#[derive(Clone, Copy, Debug, Default)]
pub struct ShortSrc {
  /// 定长存储；`buf[len..]` 恒未观察
  buf: [u8; SHORT_SRC_CAP],
  /// 有效字节数（不含 NUL 终止符语义位——本载体以长度取代终止符）
  len: usize,
}

impl ShortSrc {
  /// 读面：有效短源名字节（截断语义已由写端保证，这里恒为完整观察值）。
  pub fn bytes(&self) -> &[u8] {
    &self.buf[..self.len]
  }

  /// 写端单点：落定有效字节。`n` 由截断核心保证 `<= SHORT_SRC_CAP`，
  /// 此处仅做区间校验（`min` 收敛，防写端回归时越界）。
  pub(crate) fn set(&mut self, bytes: &[u8]) {
    let n = bytes.len().min(SHORT_SRC_CAP);
    self.buf[..n].copy_from_slice(&bytes[..n]);
    self.len = n;
  }

  /// 截断核心所需的可写 scratch（与 `buf` 同一存储，避免二次拷贝）。
  pub(crate) fn scratch(&mut self) -> &mut [u8] {
    &mut self.buf
  }

  /// 结果已就地写入 [`Self::scratch`] 时，仅提交长度。
  pub(crate) fn commit_len(&mut self, n: usize) {
    self.len = n.min(SHORT_SRC_CAP);
  }
}

/// C++ `struct lua_Debug` — activation record。
#[derive(Clone, Copy, Debug, Default)]
pub struct LuaDebug {
  /// (n) 函数名（`None` = 未知，原 null 哨兵）
  pub name: Option<&'static [u8]>,
  /// (s) 帧类型模板字节：`b"C"` / `b"Lua"`（§10：比较一律用字节切片，无终止 NUL）
  pub what: Option<&'static [u8]>,
  /// (s) chunk 源名（原样串体，含 `=`/`@` sigil）
  pub source: Option<&'static [u8]>,
  /// (s) 截断后的短源名
  pub short_src: ShortSrc,
  /// (s) 定义行
  pub linedefined: i32,
  /// (l) 当前行
  pub currentline: i32,
  /// (p) VM 内全局唯一的 proto id；C 函数为 0
  pub protoid: i32,
  /// (p) proto 在自身字节码模块内的下标；C 函数为 -1
  pub bytecodeid: i32,
  /// (u) 上值数量
  pub nupvals: u8,
  /// (a) 参数数量
  pub nparams: u8,
  /// (a) 是否变参（0/1，原 `c_char` 位——§10 收为原生 `u8`）
  pub isvararg: u8,
  /// only valid in luau_callhook：hook 透传的用户数据 POD 裸柄（`None` 语义 = null，
  /// 从不解引用；lua.h `void*` 面的合法镜像位，§10 豁免台账见 `ulua-rt/src/sys.rs`）
  pub userdata: *mut c_void,
}
