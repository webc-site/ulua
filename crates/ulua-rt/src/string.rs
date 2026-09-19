//! The [`LuaString`] handle. Mirrors `mlua::String`.

use core::{
  char::REPLACEMENT_CHARACTER,
  cmp::Ordering,
  ffi::c_void,
  fmt::{self, Debug, Display, Formatter},
  hash::{Hash, Hasher},
  slice::from_raw_parts,
  str::{from_utf8, from_utf8_unchecked},
};
use std::{borrow::Cow, fmt::Write as _};

use crate::{
  error::{Error, Result},
  state::{Lua, LuaRef},
  sync::{NOT_SYNC, NotSync, XRc},
  sys::*,
};

/// A garbage-collected Lua string.
///
/// Mirrors `mlua::String`. Holds a registry reference to the underlying Lua
/// string so the bytes stay alive for the handle's lifetime.
///
/// Under the `send` feature it is `Send` but never `Sync` — see
/// [`crate::sync::NotSync`].
#[derive(Clone)]
pub struct LuaString {
  pub(crate) reference: XRc<LuaRef>,
  pub(crate) _not_sync: NotSync,
}

impl LuaString {
  pub(crate) fn from_ref(reference: LuaRef) -> LuaString {
    LuaString {
      reference: XRc::new(reference),
      _not_sync: NOT_SYNC,
    }
  }

  /// Push this string onto the owning state's stack.
  pub(crate) unsafe fn push_to_stack(&self) {
    self.reference.push();
  }

  pub(crate) fn with_bytes<R>(&self, f: impl FnOnce(&[u8]) -> R) -> R {
    f(self.as_bytes())
  }

  /// Borrow the raw bytes of the string (zero-copy).
  ///
  /// Mirrors `mlua::String::as_bytes`, but returns a borrowed slice: the
  /// string object is pinned by the registry ref, Luau's GC never moves
  /// objects, and the `XRc<LuaRef>` chain keeps the whole VM alive — so the
  /// pointer stays valid for as long as `&self` does.
  pub fn as_bytes(&self) -> &[u8] {
    let state = self.reference.state();
    unsafe {
      self.reference.push();
      // `lua_tolstring` 会把数字就地转成字符串，而转换产物不受注册表引用
      // 保护；`LuaString` 只从真实字符串构造，这里加一道类型闸门防御，
      // 保证指针指向的必是注册表钉住的原对象。
      let bytes = if lua_type(state, -1) == LuaType::String as c_int {
        let mut len = 0usize;
        let p = lua_tolstring(state, -1, &mut len);
        if p.is_null() {
          &[]
        } else {
          from_raw_parts(p.cast::<u8>(), len)
        }
      } else {
        &[]
      };
      lua_pop(state, 1);
      bytes
    }
  }

  /// Get the string as a UTF-8 `&str`, erroring if it is not valid UTF-8.
  ///
  /// Mirrors `mlua::String::to_str` (returns an owned `String` here).
  pub fn to_str(&self) -> Result<String> {
    self.with_bytes(|bytes| {
      str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|e| Error::FromLuaConversionError {
          from: "string",
          to: "String".to_string(),
          message: Some(format!("invalid utf-8: {e}")),
        })
    })
  }

  /// Get the string lossily as a Rust `String` (invalid UTF-8 replaced).
  ///
  /// Mirrors `mlua::String::to_string_lossy`.
  pub fn to_string_lossy(&self) -> String {
    self.with_bytes(|bytes| String::from_utf8_lossy(bytes).into_owned())
  }

  /// The raw bytes with a trailing NUL appended (Lua strings are NUL
  /// terminated). Mirrors `mlua::String::as_bytes_with_nul`, borrowing like
  /// [`LuaString::as_bytes`]: Luau stores the terminator inline at
  /// `data[len]` (VM `lstring.cpp` "ending 0"), so it can be included
  /// without copying.
  pub fn as_bytes_with_nul(&self) -> &[u8] {
    let bytes = self.as_bytes();
    if bytes.is_empty() {
      // 类型闸门未命中的空切片无 NUL 可读；真实空串的终止符内容同为 0，
      // 回退到等价的常量切片。
      b"\0"
    } else {
      unsafe { from_raw_parts(bytes.as_ptr(), bytes.len() + 1) }
    }
  }

  /// A raw pointer identifying the interned string (for identity
  /// comparison). Mirrors `mlua::String::to_pointer`.
  pub fn to_pointer(&self) -> *const c_void {
    self.reference.to_pointer()
  }

  /// A `Display`-able view that renders the bytes lossily as UTF-8.
  /// Mirrors `mlua::String::display`.
  pub fn display(&self) -> LuaStringDisplay<'_> {
    LuaStringDisplay { s: self }
  }
}

/// `Display` adapter returned by [`LuaString::display`]。
///
/// 只持有字符串句柄，`fmt` 时才流式做 lossy UTF-8 输出——避免为整份字节
/// 预先分配 + 重编码（Display 可能只被格式化一小部分甚至不调用）。
pub struct LuaStringDisplay<'a> {
  s: &'a LuaString,
}

impl Display for LuaStringDisplay<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    use core::fmt::Write as _;

    self.s.with_bytes(|bytes| {
      // 与 `String::from_utf8_lossy` 逐字节一致的语义：合法段直写，每个
      // 无效字节写一个 U+FFFD（截断的序列只算一个）。
      let mut rest = bytes;
      loop {
        match from_utf8(rest) {
          Ok(valid) => {
            f.write_str(valid)?;
            return Ok(());
          }
          Err(e) => {
            let (valid, after) = rest.split_at(e.valid_up_to());
            // SAFETY：valid_up_to 之前的段必为合法 UTF-8。
            f.write_str(unsafe { from_utf8_unchecked(valid) })?;
            f.write_char(REPLACEMENT_CHARACTER)?;
            let skip = e.error_len().unwrap_or(after.len());
            rest = &after[skip..];
          }
        }
      }
    })
  }
}

impl Debug for LuaString {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    // Mirror mlua: valid utf-8 prints as a normal Rust string literal,
    // otherwise as a byte-string literal `b"..."`.
    self.with_bytes(|bytes| match from_utf8(bytes) {
      Ok(s) => write!(f, "{s:?}"),
      Err(_) => {
        f.write_str("b\"")?;
        for &b in bytes {
          match b {
            b'\0' => f.write_str("\\0")?,
            b'\r' => f.write_str("\\r")?,
            b'\n' => f.write_str("\\n")?,
            b'\t' => f.write_str("\\t")?,
            b'\\' => f.write_str("\\\\")?,
            b'"' => f.write_str("\\\"")?,
            0x20..=0x7e => f.write_char(b as char)?,
            _ => write!(f, "\\x{b:02x}")?,
          }
        }
        f.write_str("\"")
      }
    })
  }
}

impl PartialEq for LuaString {
  fn eq(&self, other: &Self) -> bool {
    self.with_bytes(|a| other.with_bytes(|b| a == b))
  }
}

impl Eq for LuaString {}

impl PartialOrd for LuaString {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for LuaString {
  fn cmp(&self, other: &Self) -> Ordering {
    self.with_bytes(|a| other.with_bytes(|b| a.cmp(b)))
  }
}

impl Hash for LuaString {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.with_bytes(|bytes| bytes.hash(state));
  }
}

// --- Comparisons against common Rust byte/str types ------------------------

macro_rules! impl_str_eq {
  ($($ty:ty => $conv:expr),* $(,)?) => {$(
    impl PartialEq<$ty> for LuaString {
      fn eq(&self, other: &$ty) -> bool {
        let f: fn(&$ty) -> &[u8] = $conv;
        self.with_bytes(|a| a == f(other))
      }
    }
    impl PartialOrd<$ty> for LuaString {
      fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
        let f: fn(&$ty) -> &[u8] = $conv;
        Some(self.with_bytes(|a| a.cmp(f(other))))
      }
    }
  )*};
}

impl_str_eq! {
    str => |s| s.as_bytes(),
    String => |s| s.as_bytes(),
    [u8] => |s| s,
    Vec<u8> => |s| s.as_slice(),
}

// `&str`（引用套引用）与 `Cow`（带生命周期参数）无法进入上面的宏：宏体内的
// `fn(&$ty) -> &[u8]` 函数指针类型对二者会产生匿名生命周期歧义，故保持手写。
impl PartialEq<&str> for LuaString {
  fn eq(&self, other: &&str) -> bool {
    self.with_bytes(|a| a == other.as_bytes())
  }
}
impl PartialOrd<&str> for LuaString {
  fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
    Some(self.with_bytes(|a| a.cmp(other.as_bytes())))
  }
}

impl PartialEq<Cow<'_, [u8]>> for LuaString {
  fn eq(&self, other: &Cow<'_, [u8]>) -> bool {
    self.with_bytes(|a| a == other.as_ref())
  }
}

impl<const N: usize> PartialEq<&[u8; N]> for LuaString {
  fn eq(&self, other: &&[u8; N]) -> bool {
    self.with_bytes(|a| a == other.as_slice())
  }
}
impl<const N: usize> PartialOrd<&[u8; N]> for LuaString {
  fn partial_cmp(&self, other: &&[u8; N]) -> Option<Ordering> {
    Some(self.with_bytes(|a| a.cmp(other.as_slice())))
  }
}

/// Helper to create a fresh Lua string from bytes on a given state, returning a
/// handle. Used by [`Lua::create_string`].
pub(crate) fn create_string(lua: &Lua, bytes: &[u8]) -> LuaString {
  let state = lua.state();
  unsafe {
    lua_pushlstring(state, bytes.as_ptr().cast::<c_char>(), bytes.len());
    LuaString::from_ref(lua.pop_ref())
  }
}

#[cfg(test)]
mod tests {
  use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

  use crate::{state::Lua, sys::lua_gettop};

  #[test]
  fn bytes_callback_restores_stack_before_unwind() {
    let lua = Lua::new();
    let string = lua.create_string(b"a\0\xff");
    let top = unsafe { lua_gettop(lua.state()) };
    let result = catch_unwind(AssertUnwindSafe(|| {
      string.with_bytes(|bytes| {
        assert_eq!(bytes, b"a\0\xff");
        assert_eq!(unsafe { lua_gettop(lua.state()) }, top);
        resume_unwind(Box::new(()));
      });
    }));
    assert!(result.is_err());
    assert_eq!(unsafe { lua_gettop(lua.state()) }, top);
    assert_eq!(string.as_bytes(), b"a\0\xff");
  }
}
