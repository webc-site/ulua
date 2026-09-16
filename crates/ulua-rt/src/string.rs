//! The [`LuaString`] handle. Mirrors `mlua::String`.

use core::{
  cmp::Ordering,
  ffi::c_void,
  fmt::{self, Debug, Display, Formatter},
  hash::{Hash, Hasher},
  slice::from_raw_parts,
  str::from_utf8,
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

  /// 借用字符串字节执行 `f`（零拷贝）。字符串对象由注册表引用与本栈槽位双重
  /// 保活；Luau 的 GC 不移动对象，借用仅覆盖 `f` 的执行期（`f` 不重入 VM）。
  /// 全部字节访问路径（比较 / 哈希 / 调试 / 转换）共用此封装。
  pub(crate) fn with_bytes<R>(&self, f: impl FnOnce(&[u8]) -> R) -> R {
    let state = self.reference.state();
    unsafe {
      self.reference.push();
      let mut len = 0usize;
      let p = lua_tolstring(state, -1, &mut len);
      let bytes: &[u8] = if p.is_null() {
        &[]
      } else {
        from_raw_parts(p as *const u8, len)
      };
      let out = f(bytes);
      lua_pop(state, 1);
      out
    }
  }

  /// Get the raw bytes of the string (a copy).
  ///
  /// Mirrors `mlua::String::as_bytes` (we return an owned `Vec<u8>` rather
  /// than a borrowed guard — a deliberate simplification).
  pub fn as_bytes(&self) -> Vec<u8> {
    self.with_bytes(<[u8]>::to_vec)
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
  /// terminated). Mirrors `mlua::String::as_bytes_with_nul`.
  pub fn as_bytes_with_nul(&self) -> Vec<u8> {
    self.with_bytes(|bytes| {
      let mut v = Vec::with_capacity(bytes.len() + 1);
      v.extend_from_slice(bytes);
      v.push(0);
      v
    })
  }

  /// A raw pointer identifying the interned string (for identity
  /// comparison). Mirrors `mlua::String::to_pointer`.
  pub fn to_pointer(&self) -> *const c_void {
    self.reference.to_pointer()
  }

  /// A `Display`-able view that renders the bytes lossily as UTF-8.
  /// Mirrors `mlua::String::display`.
  pub fn display(&self) -> LuaStringDisplay {
    LuaStringDisplay(self.to_string_lossy())
  }
}

/// `Display` adapter returned by [`LuaString::display`].
pub struct LuaStringDisplay(String);

impl Display for LuaStringDisplay {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(&self.0)
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
    lua_pushlstring(state, bytes.as_ptr() as *const c_char, bytes.len());
    LuaString::from_ref(lua.pop_ref())
  }
}
