//! The [`Buffer`] handle (Luau `buffer` type). Mirrors `mlua::Buffer`.
//!
//! A buffer is a fixed-size, mutable, GC-managed byte array — Luau's answer to
//! a `Vec<u8>` / `ArrayBuffer`. Like the other reference-typed handles
//! ([`Table`](crate::Table), [`Function`](crate::Function)) it holds a registry
//! reference ([`LuaRef`]) that keeps the underlying object alive and lets us
//! re-push it onto the stack on demand.
//!
//! The data lives in VM-managed memory; [`Buffer::as_slice`] / [`as_slice_mut`]
//! borrow the raw bytes directly through ulua's `lua_tobuffer`, so reads and
//! writes go straight to the buffer with no copy.

use core::{
  ffi::c_void,
  fmt::{self, Debug, Formatter},
  ptr::{copy, copy_nonoverlapping},
  slice::{from_raw_parts, from_raw_parts_mut},
};
use std::io::{self, Error, ErrorKind, Read, Seek, SeekFrom, Write};

use crate::{
  error::Result,
  registry::RegHandle,
  state::{Lua, LuaRef, ensure_stack, ensure_stack_or_panic},
  sync::{NOT_SYNC, NotSync, XRc},
  sys::*,
  table::number_at,
};

/// A Luau buffer type.
///
/// See the buffer [documentation] for more information.
///
/// Mirrors `mlua::Buffer`. Holds a registry reference keeping the buffer alive.
///
/// Under the `send` feature it is `Send` but never `Sync` — see
/// `crate::sync::NotSync`.
///
/// [documentation]: https://luau.org/library#buffer-library
#[derive(Clone)]
pub struct Buffer {
  pub(crate) reference: XRc<LuaRef>,
  pub(crate) _not_sync: NotSync,
}

impl Buffer {
  pub(crate) fn from_ref(reference: LuaRef) -> Buffer {
    Buffer {
      reference: XRc::new(reference),
      _not_sync: NOT_SYNC,
    }
  }

  /// The owning [`Lua`].
  pub(crate) fn lua(&self) -> Lua {
    self.reference.lua()
  }

  /// Copies the buffer data into a new `Vec<u8>`.
  ///
  /// Mirrors `mlua::Buffer::to_vec`.
  pub fn to_vec(&self) -> Vec<u8> {
    self.as_slice().to_vec()
  }

  /// Returns the length of the buffer.
  ///
  /// Mirrors `mlua::Buffer::len`.
  pub fn len(&self) -> usize {
    self.as_slice().len()
  }

  /// Returns `true` if the buffer is empty.
  ///
  /// Mirrors `mlua::Buffer::is_empty`.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Reads given number of bytes from the buffer at the given offset.
  ///
  /// Offset is 0-based.
  ///
  /// Mirrors `mlua::Buffer::read_bytes`.
  #[track_caller]
  pub fn read_bytes<const N: usize>(&self, offset: usize) -> [u8; N] {
    let data = self.as_slice();
    let mut bytes = [0u8; N];
    bytes.copy_from_slice(&data[offset..offset + N]);
    bytes
  }

  /// Writes given bytes to the buffer at the given offset.
  ///
  /// Offset is 0-based.
  ///
  /// Mirrors `mlua::Buffer::write_bytes`. 与 mlua 的 `&self` + 内部可变性不同，
  /// 这里按 `&mut` 纪律收口（与 `Buffer::as_slice_mut` 一致）：`Buffer` 是
  /// `Clone` 的句柄，克隆体共享同一块 VM 内存，从 `&self` 造 `&mut` 会让别的
  /// 句柄正在使用的 `&[u8]` 失效；`b.write_bytes(off, b.as_slice())` 这种自别名
  /// 写法在借用检查器处即被拒。
  ///
  /// # Panics
  ///
  /// Panics if `offset + bytes.len()` is greater than the buffer length.
  #[track_caller]
  pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) {
    // `as_raw_parts` 契约要求预留 push/pop 的一层。
    ensure_stack_or_panic(self.reference.state(), 1);
    // Safety: 紧邻上一行的 `ensure_stack_or_panic` 满足 `as_raw_parts` 的
    // 栈头寸契约；owning VM 存活且由当前线程驱动（句柄 `XRc<LuaRef>` 持有
    // inner，move-not-share 的 `send` 约定）。
    let (buf, size) = unsafe { self.as_raw_parts() };
    let len = bytes.len();
    assert!(
      offset <= size && len <= size - offset,
      "range end index {} out of range for slice of length {size}",
      offset + len
    );
    // Safety: 上方 assert 保证 `offset <= size`，`buf.add(offset)` 停在对象
    // 末尾的合法 one-past 位（不单独解引用）；`buf` 由 `as_raw_parts` 断言非空
    // 且指向存活 buffer 对象的字节区。
    let dst = unsafe { buf.add(offset) };
    // 源区间可能就是同一块 VM 内存（另一枚克隆句柄借出的 `&[u8]`），
    // 重叠时 `copy_nonoverlapping` 是 UB，改用 `copy`（memmove 语义）。
    let src = bytes.as_ptr();
    let (src_int, dst_int) = (src as usize, dst as usize);
    let overlaps = src_int < dst_int + len && dst_int < src_int + len;
    // Safety: 两操作数都是各自 `&[u8]` 的完整区间（assert 保证
    // `offset + len <= size`，dst 在界内；src 来自 `bytes`），u8 对齐平凡。
    // `copy_nonoverlapping` 仅在地址区间判明不重叠的分支使用；重叠分支走
    // `copy`（memmove 语义，允许重叠）。len==0 时两者对空区间均合法。
    unsafe {
      if overlaps {
        copy(src, dst, len);
      } else {
        copy_nonoverlapping(src, dst, len);
      }
    }
  }

  /// Returns an adaptor implementing [`Read`], [`Write`] and
  /// [`Seek`] over the buffer.
  ///
  /// Buffer operations are infallible, none of the read/write functions will
  /// return an `Err`.
  ///
  /// Mirrors `mlua::Buffer::cursor`.
  pub fn cursor(self) -> impl Read + Write + Seek {
    BufferCursor(self, 0)
  }

  /// A raw pointer identifying this buffer (for identity comparison).
  /// Mirrors `Value::Buffer(_).to_pointer()`.
  pub(crate) fn to_pointer(&self) -> *const c_void {
    self.reference.to_pointer()
  }

  /// Borrow the buffer's bytes directly (no copy).
  ///
  /// 只在 crate 内可见，且有一条纪律：借出的 `&[u8]` 指向 **VM 堆内存**，任何
  /// 可能写同一 buffer 的代码（用户回调、另一枚克隆句柄的
  /// [`Buffer::write_bytes`]）都不得在该引用活跃期间运行——需要跨回调交付字节
  /// 的调用点（serde）必须先 [`Buffer::to_vec`] 拷贝。
  pub(crate) fn as_slice(&self) -> &[u8] {
    // `as_raw_parts` 契约要求预留 push/pop 的一层。
    ensure_stack_or_panic(self.reference.state(), 1);
    // Safety: VM 存活且由当前线程驱动（`XRc<LuaRef>` 持有 inner），头寸已
    // 预留；`as_raw_parts` 返回的 `(ptr, size)` 指向 buffer 对象内联字节区，
    // 指针非空由其内部 assert 保证，对象被注册表引用钉住且 Luau GC 不移动
    // 对象，故切片在 `&self` 借用期内有效（文档纪律：借出期间不得有并发写）。
    unsafe {
      let (buf, size) = self.as_raw_parts();
      from_raw_parts(buf, size)
    }
  }

  /// Mutably borrow the buffer's bytes directly (no copy).
  pub(crate) fn as_slice_mut(&mut self) -> &mut [u8] {
    // `as_raw_parts` 契约要求预留 push/pop 的一层。
    ensure_stack_or_panic(self.reference.state(), 1);
    // Safety: 同 `as_slice`；可变性纪律由 `&mut self` 接收者承担——本 crate
    // 内所有取字节路径都经本方法或 `as_slice`，`&mut` 借用期间不存在其他
    // 活跃 Rust 借用指向同一句柄；跨克隆句柄的并发借用由 `as_slice` 文档
    // 明示为禁止（mlua `Buffer::as_slice_mut` 同型契约）。
    unsafe {
      let (buf, size) = self.as_raw_parts();
      from_raw_parts_mut(buf, size)
    }
  }

  /// The raw `(ptr, len)` of the underlying buffer object via ulua's
  /// `lua_tobuffer`. Pushes the buffer, reads the parts, then pops — the
  /// pointer remains valid because the registry ref keeps the object alive.
  ///
  /// # Safety
  /// owning VM 必须存活且由当前线程驱动，且 main state 上已预留至少 1 个
  /// 栈空位（内部 push/pop 一层）。返回的裸指针未编码生命周期：仅当 `self`
  /// 的注册表引用仍然钉住该 buffer 对象时有效（即调用点不得让它在 `&self`
  /// 之后被使用，也不得在另一线程并发驱动同一 VM 触发 GC 期间解引用）。
  unsafe fn as_raw_parts(&self) -> (*mut u8, usize) {
    let state = self.reference.state();
    // Safety: `state` 存活；注册表引用指向登记时的 buffer 对象，
    // `reference.push()`（`lua_rawgeti`）取回原对象压到已预留的空位上。
    // `lua_tobuffer(-1)` 对刚压入的 buffer 值返回其内联数据指针与真实长度
    // （非 buffer 才返回 null，由 assert 收口为 panic 而非 UB）；`lua_pop` 后
    // 指针仍有效——对象由注册表引用钉住、GC 不移动，栈槽只是视图。
    unsafe {
      self.reference.push();
      let mut size = 0usize;
      let buf = lua_tobuffer(state, -1, &mut size);
      lua_pop(state, 1);
      // `Option` 已消 null 哨兵：非 buffer 槽位为 `None`，收口为 panic 而非 UB。
      let buf = buf.expect("invalid Luau buffer") as *mut c_void;
      (buf.cast::<u8>(), size)
    }
  }
}

impl RegHandle for Buffer {
  fn reference(&self) -> &XRc<LuaRef> {
    &self.reference
  }
}

impl Debug for Buffer {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    // Mirror mlua: a buffer renders as its byte contents (a byte slice).
    write!(f, "Buffer({:?})", self.as_slice())
  }
}

impl PartialEq for Buffer {
  fn eq(&self, other: &Self) -> bool {
    // Reference (pointer) identity, matching mlua: two handles are equal iff
    // they point at the *same* buffer object (NOT byte-wise content).
    self.to_pointer() == other.to_pointer()
  }
}

/// Cursor adapter returned by [`Buffer::cursor`]. The `usize` is the current
/// 0-based offset into the buffer. Mirrors mlua's `BufferCursor`.
struct BufferCursor(Buffer, usize);

impl Read for BufferCursor {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    let data = self.0.as_slice();
    if self.1 == data.len() {
      return Ok(0);
    }
    let len = buf.len().min(data.len() - self.1);
    buf[..len].copy_from_slice(&data[self.1..self.1 + len]);
    self.1 += len;
    Ok(len)
  }
}

impl Write for BufferCursor {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    let data = self.0.as_slice_mut();
    if self.1 == data.len() {
      return Ok(0);
    }
    let len = buf.len().min(data.len() - self.1);
    data[self.1..self.1 + len].copy_from_slice(&buf[..len]);
    self.1 += len;
    Ok(len)
  }

  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }
}

impl Seek for BufferCursor {
  fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
    let data = self.0.as_slice();
    let new_offset = match pos {
      SeekFrom::Start(offset) => offset as i64,
      SeekFrom::End(offset) => data.len() as i64 + offset,
      SeekFrom::Current(offset) => self.1 as i64 + offset,
    };
    if new_offset < 0 {
      return Err(Error::new(
        ErrorKind::InvalidInput,
        "invalid seek to a negative position",
      ));
    }
    if new_offset as usize > data.len() {
      return Err(Error::new(
        ErrorKind::InvalidInput,
        "invalid seek to a position beyond the end of the buffer",
      ));
    }
    self.1 = new_offset as usize;
    Ok(self.1 as u64)
  }
}

// ---------------------------------------------------------------------------
// Buffer creation
//
// `lua_newbuffer` allocates a GC buffer and pushes it. When `size` exceeds the
// VM's `MAX_BUFFER_SIZE` (1GB) the underlying `luaM_toobig` *raises* a Lua
// error (longjmp) rather than returning. Unwinding that across Rust frames is
// UB, so we run `lua_newbuffer` inside `lua_pcall` via a small C trampoline:
// the requested size is passed as a number argument, and a raising allocation
// is reported as an ordinary non-zero status with the error on the stack.
// ---------------------------------------------------------------------------

/// `c_newbuffer` 闭包的调试名：静态 NUL 结尾字节串，交给 `lua_pushcclosurek` 的
/// `*const c_char` 收口点（会被闭包长期持有，`'static` 永不失效）。
const NEWBUFFER_NAME: &[u8] = b"ulua-rt-newbuffer\0";

/// C trampoline: stack is `[size]` (a number). Allocates a buffer of that many
/// bytes via `lua_newbuffer`, leaving the buffer object on top.
///
/// # Safety
/// 仅由 `lua_pcall` 在被调闭包帧内调用：`state` 存活且带 `LUA_MINSTACK` 头寸，
/// 栈槽 1 是 `create_buffer_with_capacity` 压入的 number 实参。
unsafe extern "C-unwind" fn c_newbuffer(state: *mut LuaState) -> c_int {
  // Safety: `state` 由 `lua_pcall` 在被调闭包内提供（存活，且帧带
  // `LUA_MINSTACK` 头寸）。栈槽 1 由创建方 `create_buffer_with_capacity`
  // 压入的 number 实参占据（契约"栈为 [size]"），经 `number_at` 只读消费；
  // 理论上的非数值形态回落到 0，与旧形 `lua_tonumberx(.., NULL)` 对非数值
  // 返 0.0 的取值逐位一致，`f64 as usize` 饱和转换是 Rust 定义行为。
  // `lua_settop(state, 0)` 丢弃实参后 `lua_newbuffer` 在空帧上分配并压回
  // 恰好一个值（其 toobig 错误经外层受保护调用转成非零 status，不跨帧
  // unwind），返回 1 与之相符。
  unsafe {
    let size = number_at(state, 1).unwrap_or(0.0) as usize;
    lua_settop(state, 0);
    lua_newbuffer(state, size);
    1
  }
}

/// Create a buffer of `size` zero-initialized bytes, catching an over-limit
/// allocation as an `Err` rather than letting the VM longjmp.
pub(crate) fn create_buffer_with_capacity(lua: &Lua, size: usize) -> Result<Buffer> {
  let state = lua.state();
  // 栈峰值：trampoline 闭包 + size 实参两层（pcall 弹出并压回结果）。
  ensure_stack(state, 2)?;
  // Safety: `state` 存活且上一行已预留 2 层头寸，恰覆盖 push closure + push number；
  // `NEWBUFFER_NAME` 是静态 NUL 结尾字节串（会被闭包长期持有，'static 永不失效），
  // `c_newbuffer` 是与 `LuaCFunction` C-ABI 兼容的 `unsafe extern "C-unwind"` 入口，
  // `nupvalue=0` 与栈上无 upvalue 一致；`size as f64` 的饱和转换在 VM 侧再还原。
  unsafe {
    lua_pushcclosurek(
      state,
      Some(c_newbuffer),
      NEWBUFFER_NAME.as_ptr().cast(),
      0,
      None,
    );
    lua_pushnumber(state, size as f64);
  }
  // Safety: 闭包与 size 实参已由上块压栈，`pcall(1 实参, 1 返回值)` 与
  // `c_newbuffer` 的栈约定严格配对；超上限分配在 `c_newbuffer` 内 raise，经此
  // 受保护调用转成非零 status（不跨帧 unwind），栈头寸由预留的 2 层覆盖。
  let status = unsafe { lua_pcall(state, 1, 1, 0) };
  if status != 0 {
    // 失败路径 `pop_error`（safe fn）消费 VM 留下的错误消息，栈平衡唯一。
    return Err(lua.pop_error(status));
  }
  // 成功路径栈顶恒为新 buffer 对象，`pop_ref`（safe fn）消费之并登记注册表引用。
  Ok(Buffer::from_ref(lua.pop_ref()))
}

// §8：测 `pub(crate) Buffer::as_slice` 借出的重叠区间写（memmove 语义）；
// pub API 只给 `to_vec` 堆拷贝，无从构造重叠，迁 tests/ 需泄 pub，保留 src。
#[cfg(test)]
mod tests {
  use crate::state::Lua;

  /// 源区间落在同一块 VM 内存里（另一枚克隆句柄借出的切片）时必须走 memmove
  /// 语义：旧实现用 `copy_from_slice`（→ `copy_nonoverlapping`），区间重叠即 UB。
  #[test]
  fn overlapping_write_moves_within_buffer() {
    let lua = Lua::new();
    let source = lua.create_buffer(b"abcdefg").unwrap();
    let mut sink = source.clone();
    let window = source.as_slice();
    // 目标 [2..5) 与源 [0..3) 重叠：memmove 语义下等价「先读出 abc 再写入」，
    // 得到 a b a b c f g；memcpy 前向拷贝会在读到 'c' 之前就把它覆盖成 'a'。
    sink.write_bytes(2, &window[..3]);
    assert_eq!(sink.to_vec(), b"ababcfg");
  }
}
