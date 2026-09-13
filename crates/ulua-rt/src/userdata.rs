//! [`UserData`] / [`UserDataMethods`] / [`UserDataFields`] and the
//! [`AnyUserData`] handle. Mirrors `mlua::UserData` / `mlua::UserDataMethods` /
//! `mlua::UserDataFields` / `mlua::AnyUserData`.
//!
//! ## Implementation
//!
//! A `T: UserData` value is boxed into a Lua userdata as a typed wrapper
//! [`DataCell<T>`] = `{ key: CellKey, RefCell<Option<T>> }` (via
//! [`lua_newuserdatadtor`], whose destructor drops the cell). The leading
//! `CellKey` (a `TypeId` for ordinary userdata, a unique marker for scope-
//! created userdata) makes Rust-side typed read-back **sound**: every accessor
//! ([`AnyUserData::borrow`], [`borrow_mut`](AnyUserData::borrow_mut),
//! [`take`](AnyUserData::take), [`is`](AnyUserData::is)) reads the stored
//! `TypeId` from the userdata pointer and compares it with `TypeId::of::<T>()`
//! before downcasting — a mismatch is an [`Error::UserDataTypeMismatch`].
//! `take` replaces the `Option<T>` with `None`; subsequent access reports
//! [`Error::UserDataDestructed`].
//!
//! Each registered method/field is compiled into a Rust closure wired into a
//! per-instance metatable:
//!   - ordinary methods go into a method table,
//!   - field getters/setters are dispatched by an `__index`/`__newindex`
//!     function (only when fields are registered),
//!   - meta-methods (e.g. `__add`) go directly on the metatable.

use core::{
  fmt::{self, Debug, Display, Formatter},
  mem::size_of,
  ops::{Deref, DerefMut},
  ptr::{drop_in_place, write},
};
use std::{
  any::TypeId,
  cell::{Ref, RefCell, RefMut},
  marker::PhantomData,
  sync::atomic::{AtomicU64, Ordering},
};

use crate::{
  callback::{BoxedCallback, create_callback_function},
  error::{Error, Result},
  function::Function,
  state::{Lua, LuaRef},
  sync::{MaybeSend, MaybeSync, NOT_SYNC, NotSync, XRc},
  sys::*,
  table::Table,
  traits::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti},
  value::Value,
};

/// A Rust type that can be exposed to Lua as userdata.
///
/// Mirrors `mlua::UserData`. Implement [`UserData::add_methods`] and/or
/// [`UserData::add_fields`] to register the surface visible from Lua.
pub trait UserData: Sized {
  /// Register fields (getters/setters). Default: none.
  fn add_fields<F: UserDataFields<Self>>(_fields: &mut F) {}

  /// Register methods and meta-methods. Default: none.
  fn add_methods<M: UserDataMethods<Self>>(_methods: &mut M) {}
}

/// Registrar passed to [`UserData::add_methods`].
///
/// Mirrors `mlua::UserDataMethods`.
pub trait UserDataMethods<T> {
  /// Register a method callable as `obj:name(...)`; receives `&T`.
  fn add_method<M, A, R>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &T, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti;

  /// Register a method callable as `obj:name(...)`; receives `&mut T`.
  fn add_method_mut<M, A, R>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &mut T, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti;

  /// Register a plain function in the userdata namespace (no `self`).
  fn add_function<F, A, R>(&mut self, name: impl Into<String>, function: F)
  where
    F: Fn(&Lua, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti;

  /// Register a meta-method (e.g. `MetaMethod::Add`, `"__tostring"`);
  /// receives `&T`.
  fn add_meta_method<M, A, R>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &T, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti;

  /// Register a meta-method receiving `&mut T`.
  fn add_meta_method_mut<M, A, R>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &mut T, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti;
}

/// Registrar passed to [`UserData::add_fields`].
///
/// Mirrors `mlua::UserDataFields`. Field getters/setters are dispatched by the
/// userdata's `__index`/`__newindex`.
pub trait UserDataFields<T> {
  /// Register a constant field value (read-only).
  fn add_field<V>(&mut self, name: impl Into<String>, value: V)
  where
    V: IntoLua + Clone + MaybeSend + 'static;

  /// Register a field whose getter receives `&T`.
  fn add_field_method_get<M, R>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &T) -> Result<R> + MaybeSend + 'static,
    R: IntoLua;

  /// Register a field whose setter receives `&mut T` and the assigned value.
  fn add_field_method_set<M, A>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &mut T, A) -> Result<()> + MaybeSend + 'static,
    A: FromLua;

  /// Register a field whose getter receives the [`AnyUserData`] handle.
  fn add_field_function_get<F, R>(&mut self, name: impl Into<String>, function: F)
  where
    F: Fn(&Lua, AnyUserData) -> Result<R> + MaybeSend + 'static,
    R: IntoLua;

  /// Register a field whose setter receives the [`AnyUserData`] handle and
  /// the assigned value.
  fn add_field_function_set<F, A>(&mut self, name: impl Into<String>, function: F)
  where
    F: Fn(&Lua, AnyUserData, A) -> Result<()> + MaybeSend + 'static,
    A: FromLua;
}

/// A handle to an arbitrary Lua userdata value.
///
/// Mirrors `mlua::AnyUserData`. Supports construction, use-from-Lua, and typed
/// Rust-side borrowing ([`borrow`](AnyUserData::borrow) /
/// [`borrow_mut`](AnyUserData::borrow_mut) / [`take`](AnyUserData::take) /
/// [`is`](AnyUserData::is)).
#[derive(Clone)]
pub struct AnyUserData {
  pub(crate) reference: XRc<LuaRef>,
  pub(crate) _not_sync: NotSync,
}

impl AnyUserData {
  pub(crate) fn from_ref(reference: LuaRef) -> AnyUserData {
    AnyUserData {
      reference: XRc::new(reference),
      _not_sync: NOT_SYNC,
    }
  }

  pub(crate) unsafe fn push_to_stack(&self) {
    self.reference.push();
  }

  /// The owning [`Lua`].
  pub fn lua(&self) -> Lua {
    self.reference.lua()
  }

  /// A raw pointer identifying this userdata. Mirrors
  /// `mlua::AnyUserData::to_pointer`.
  pub fn to_pointer(&self) -> *const c_void {
    let state = self.reference.state();
    unsafe {
      self.reference.push();
      let p = lua_topointer(state, -1);
      lua_pop(state, 1);
      p
    }
  }

  /// Compare for equality honoring an `__eq` metamethod.
  /// Mirrors `mlua::AnyUserData::equals`.
  pub fn equals(&self, other: &AnyUserData) -> Result<bool> {
    let lua = self.lua();
    let state = lua.state();
    unsafe {
      self.reference.push();
      other.reference.push();
      let eq = lua_equal(state, -2, -1);
      lua_pop(state, 2);
      Ok(eq != 0)
    }
  }

  /// Recover a `&DataCell<T>` from the userdata storage, checking the
  /// embedded `CellKey`. Returns `UserDataTypeMismatch` if the concrete type
  /// differs.
  fn cell<T: 'static>(&self) -> Result<&DataCell<T>> {
    let state = self.reference.state();
    unsafe {
      self.reference.push();
      let ptr = lua_touserdata(state, -1);
      lua_pop(state, 1);
      if ptr.is_null() {
        return Err(Error::UserDataTypeMismatch);
      }
      // The wrapper stores the CellKey first; check it before downcasting.
      let header = &*(ptr as *const DataHeader);
      if header.key != CellKey::Typed(TypeId::of::<T>()) {
        return Err(Error::UserDataTypeMismatch);
      }
      Ok(&*(ptr as *const DataCell<T>))
    }
  }

  /// Whether the stored value is of concrete type `T`. Mirrors
  /// `mlua::AnyUserData::is`. Returns `false` after the value has been taken.
  pub fn is<T: 'static>(&self) -> bool {
    match self.cell::<T>() {
      Ok(cell) => cell.cell.borrow().is_some(),
      Err(_) => false,
    }
  }

  /// The [`TypeId`] of the stored value, if it is an ordinary ulua-rt
  /// userdata. Mirrors `mlua::AnyUserData::type_id` (returns the concrete
  /// `TypeId` whenever the userdata carries a ulua-rt typed header; scope-
  /// created userdata carry only a unique marker, so this returns `None`).
  pub fn type_id(&self) -> Option<TypeId> {
    let state = self.reference.state();
    unsafe {
      self.reference.push();
      let ptr = lua_touserdata(state, -1);
      lua_pop(state, 1);
      if ptr.is_null() {
        return None;
      }
      // Only ulua-rt userdata carry a header; raw VM userdata do not, but
      // every userdata this crate creates does.
      match (&*(ptr as *const DataHeader)).key {
        CellKey::Typed(id) => Some(id),
        CellKey::Scoped(_) => None,
      }
    }
  }

  /// Immutably borrow the stored value as `T`. Mirrors
  /// `mlua::AnyUserData::borrow`. Errors with [`Error::UserDataTypeMismatch`]
  /// on a type mismatch, [`Error::UserDataDestructed`] if it was taken, or
  /// [`Error::UserDataBorrowError`] if already mutably borrowed.
  pub fn borrow<T: 'static>(&self) -> Result<UserDataRef<'_, T>> {
    let cell = self.cell::<T>()?;
    let guard = cell
      .cell
      .try_borrow()
      .map_err(|_| Error::UserDataBorrowError)?;
    if guard.is_none() {
      return Err(Error::UserDataDestructed);
    }
    Ok(UserDataRef {
      guard,
      _marker: PhantomData,
    })
  }

  /// Mutably borrow the stored value as `T`. Mirrors
  /// `mlua::AnyUserData::borrow_mut`.
  pub fn borrow_mut<T: 'static>(&self) -> Result<UserDataRefMut<'_, T>> {
    let cell = self.cell::<T>()?;
    let guard = cell
      .cell
      .try_borrow_mut()
      .map_err(|_| Error::UserDataBorrowMutError)?;
    if guard.is_none() {
      return Err(Error::UserDataDestructed);
    }
    Ok(UserDataRefMut {
      guard,
      _marker: PhantomData,
    })
  }

  /// Take the stored value out of the userdata, leaving it destructed.
  /// Mirrors `mlua::AnyUserData::take`. Errors with
  /// [`Error::UserDataBorrowMutError`] if currently borrowed, or
  /// [`Error::UserDataDestructed`] if already taken.
  pub fn take<T: 'static>(&self) -> Result<T> {
    let cell = self.cell::<T>()?;
    let mut guard = cell
      .cell
      .try_borrow_mut()
      .map_err(|_| Error::UserDataBorrowMutError)?;
    guard.take().ok_or(Error::UserDataDestructed)
  }
}

/// A RAII guard for an immutable userdata borrow ([`AnyUserData::borrow`]).
/// Mirrors `mlua::UserDataRef`.
pub struct UserDataRef<'a, T> {
  guard: Ref<'a, Option<T>>,
  _marker: PhantomData<T>,
}

impl<T> Deref for UserDataRef<'_, T> {
  type Target = T;
  fn deref(&self) -> &T {
    // Invariant: `borrow` returns only when the option is `Some`.
    self.guard.as_ref().expect("userdata destructed")
  }
}

impl<T: Debug> Debug for UserDataRef<'_, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Debug::fmt(&**self, f)
  }
}

impl<T: Display> Display for UserDataRef<'_, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Display::fmt(&**self, f)
  }
}

/// A RAII guard for a mutable userdata borrow ([`AnyUserData::borrow_mut`]).
/// Mirrors `mlua::UserDataRefMut`.
pub struct UserDataRefMut<'a, T> {
  guard: RefMut<'a, Option<T>>,
  _marker: PhantomData<T>,
}

impl<T> Deref for UserDataRefMut<'_, T> {
  type Target = T;
  fn deref(&self) -> &T {
    self.guard.as_ref().expect("userdata destructed")
  }
}

impl<T> DerefMut for UserDataRefMut<'_, T> {
  fn deref_mut(&mut self) -> &mut T {
    self.guard.as_mut().expect("userdata destructed")
  }
}

impl<T: Debug> Debug for UserDataRefMut<'_, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Debug::fmt(&**self, f)
  }
}

impl<T: Display> Display for UserDataRefMut<'_, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Display::fmt(&**self, f)
  }
}

impl Debug for AnyUserData {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "UserData")
  }
}

impl PartialEq for AnyUserData {
  fn eq(&self, other: &Self) -> bool {
    // Pointer identity (matches mlua): same underlying userdata object.
    self.to_pointer() == other.to_pointer()
  }
}

// Any `T: UserData` value converts into Lua by wrapping it in a fresh userdata.
// Mirrors mlua's `impl<T: UserData + MaybeSend + MaybeSync + 'static> IntoLua`.
// This is what lets `create_registry_value(MyUserdata(..))`,
// `table.set("k", MyUserdata(..))`, etc. accept a userdata value directly. It
// coexists with the concrete `IntoLua` impls below because none of those
// (local) types implement `UserData`.
impl<T: UserData + MaybeSend + MaybeSync + 'static> IntoLua for T {
  fn into_lua(self, lua: &Lua) -> Result<Value> {
    Ok(Value::UserData(lua.create_userdata(self)?))
  }
}

impl IntoLua for AnyUserData {
  fn into_lua(self, _lua: &Lua) -> Result<Value> {
    Ok(Value::UserData(self))
  }
}

impl IntoLua for &AnyUserData {
  fn into_lua(self, _lua: &Lua) -> Result<Value> {
    Ok(Value::UserData(self.clone()))
  }
}

impl FromLua for AnyUserData {
  fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
    match value {
      Value::UserData(ud) => Ok(ud),
      other => Err(Error::FromLuaConversionError {
        from: other.type_name(),
        to: "AnyUserData".to_string(),
        message: None,
      }),
    }
  }
}

// ---------------------------------------------------------------------------
// Typed userdata storage
// ---------------------------------------------------------------------------

/// The key tagging a ulua-rt userdata cell:
/// - `Typed(TypeId)` — ordinary userdata; supports sound typed read-back
///   ([`AnyUserData::borrow`] / [`borrow_mut`](AnyUserData::borrow_mut) /
///   [`take`](AnyUserData::take) / [`is`](AnyUserData::is)), requires
///   `T: 'static`.
/// - `Scoped(u64)` — scope-created userdata wrapping a non-`'static` `T`;
///   carries a process-unique marker instead (markers are never reused, so a
///   key match proves the userdata is a `DataCell<T>` for exactly one `T`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum CellKey {
  Typed(TypeId),
  Scoped(u64),
}

/// The fixed leading layout of every ulua-rt userdata wrapper. Reading the
/// `CellKey` through this header (a prefix of [`DataCell<T>`]) lets us verify
/// the concrete type before downcasting. Both share `#[repr(C)]` so the `key`
/// field is at offset 0 for any `T`.
#[repr(C)]
struct DataHeader {
  key: CellKey,
}

/// The userdata storage: a [`CellKey`] followed by the `RefCell<Option<T>>`.
#[repr(C)]
struct DataCell<T> {
  key: CellKey,
  cell: RefCell<Option<T>>,
}

/// Recover `&DataCell<T>` from a `self` userdata [`Value`] (Lua argument 1),
/// verifying the embedded `CellKey`.
///
/// For `CellKey::Typed` the stored `TypeId` must equal `TypeId::of::<T>()`; for
/// `CellKey::Scoped` the caller guarantees the marker was minted for exactly
/// this `T` (which holds because each marker is handed out to exactly one
/// `create_scoped_userdata::<T>` call). A mismatch is an
/// [`Error::UserDataTypeMismatch`].
fn recover_cell<'a, T>(lua: &Lua, value: &Value, key: CellKey) -> Result<&'a DataCell<T>> {
  let Value::UserData(ud) = value else {
    return Err(Error::UserDataTypeMismatch);
  };
  let state = lua.state();
  unsafe {
    ud.reference.push();
    let ptr = lua_touserdata(state, -1);
    lua_pop(state, 1);
    if ptr.is_null() {
      return Err(Error::UserDataTypeMismatch);
    }
    let header = &*(ptr as *const DataHeader);
    if header.key != key {
      return Err(Error::UserDataTypeMismatch);
    }
    Ok(&*(ptr as *const DataCell<T>))
  }
}

// ---------------------------------------------------------------------------
// Method collection
// ---------------------------------------------------------------------------

/// A registered method or meta-method, paired with its name and whether it is a
/// meta-method.
struct Registered {
  name: String,
  is_meta: bool,
  callback: BoxedCallback,
}

/// A registered field getter or setter.
struct FieldEntry {
  name: String,
  /// `true` if a getter, `false` if a setter.
  is_get: bool,
  callback: BoxedCallback,
}

/// Concrete [`UserDataMethods`] / [`UserDataFields`] implementation that
/// collects the type-erased callbacks; the metatable is built from these. The
/// [`CellKey`] decides typed (TypeId) vs scoped (marker) cell recovery, so one
/// impl set serves both ordinary and scope-created userdata.
struct Collector<T> {
  key: CellKey,
  methods: Vec<Registered>,
  fields: Vec<FieldEntry>,
  _phantom: PhantomData<T>,
}

impl<T> Collector<T> {
  fn new(key: CellKey) -> Self {
    Collector {
      key,
      methods: Vec::new(),
      fields: Vec::new(),
      _phantom: PhantomData,
    }
  }
}

impl<T> UserDataMethods<T> for Collector<T> {
  fn add_method<M, A, R>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &T, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
  {
    let key = self.key;
    let callback: BoxedCallback = Box::new(move |lua, mut args| {
      let this = args.pop_front().unwrap_or(Value::Nil);
      let cell = recover_cell::<T>(lua, &this, key)?;
      let a = A::from_lua_multi(args, lua)?;
      let borrowed = cell
        .cell
        .try_borrow()
        .map_err(|_| Error::UserDataBorrowError)?;
      let data = borrowed.as_ref().ok_or(Error::UserDataDestructed)?;
      let r = method(lua, data, a)?;
      r.into_lua_multi(lua)
    });
    self.methods.push(Registered {
      name: name.into(),
      is_meta: false,
      callback,
    });
  }

  fn add_method_mut<M, A, R>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &mut T, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
  {
    let key = self.key;
    let callback: BoxedCallback = Box::new(move |lua, mut args| {
      let this = args.pop_front().unwrap_or(Value::Nil);
      let cell = recover_cell::<T>(lua, &this, key)?;
      let a = A::from_lua_multi(args, lua)?;
      let mut borrowed = cell
        .cell
        .try_borrow_mut()
        .map_err(|_| Error::UserDataBorrowMutError)?;
      let data = borrowed.as_mut().ok_or(Error::UserDataDestructed)?;
      let r = method(lua, data, a)?;
      r.into_lua_multi(lua)
    });
    self.methods.push(Registered {
      name: name.into(),
      is_meta: false,
      callback,
    });
  }

  fn add_function<F, A, R>(&mut self, name: impl Into<String>, function: F)
  where
    F: Fn(&Lua, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
  {
    let callback: BoxedCallback = Box::new(move |lua, args| {
      let a = A::from_lua_multi(args, lua)?;
      let r = function(lua, a)?;
      r.into_lua_multi(lua)
    });
    self.methods.push(Registered {
      name: name.into(),
      is_meta: false,
      callback,
    });
  }

  fn add_meta_method<M, A, R>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &T, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
  {
    let key = self.key;
    let callback: BoxedCallback = Box::new(move |lua, mut args| {
      let this = args.pop_front().unwrap_or(Value::Nil);
      let cell = recover_cell::<T>(lua, &this, key)?;
      let a = A::from_lua_multi(args, lua)?;
      let borrowed = cell
        .cell
        .try_borrow()
        .map_err(|_| Error::UserDataBorrowError)?;
      let data = borrowed.as_ref().ok_or(Error::UserDataDestructed)?;
      let r = method(lua, data, a)?;
      r.into_lua_multi(lua)
    });
    self.methods.push(Registered {
      name: name.into(),
      is_meta: true,
      callback,
    });
  }

  fn add_meta_method_mut<M, A, R>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &mut T, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
  {
    let key = self.key;
    let callback: BoxedCallback = Box::new(move |lua, mut args| {
      let this = args.pop_front().unwrap_or(Value::Nil);
      let cell = recover_cell::<T>(lua, &this, key)?;
      let a = A::from_lua_multi(args, lua)?;
      let mut borrowed = cell
        .cell
        .try_borrow_mut()
        .map_err(|_| Error::UserDataBorrowMutError)?;
      let data = borrowed.as_mut().ok_or(Error::UserDataDestructed)?;
      let r = method(lua, data, a)?;
      r.into_lua_multi(lua)
    });
    self.methods.push(Registered {
      name: name.into(),
      is_meta: true,
      callback,
    });
  }
}

impl<T> UserDataFields<T> for Collector<T> {
  fn add_field<V>(&mut self, name: impl Into<String>, value: V)
  where
    V: IntoLua + Clone + MaybeSend + 'static,
  {
    let callback: BoxedCallback = Box::new(move |lua, _args| {
      let v = value.clone().into_lua(lua)?;
      v.into_lua_multi(lua)
    });
    self.fields.push(FieldEntry {
      name: name.into(),
      is_get: true,
      callback,
    });
  }

  fn add_field_method_get<M, R>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &T) -> Result<R> + MaybeSend + 'static,
    R: IntoLua,
  {
    let key = self.key;
    let callback: BoxedCallback = Box::new(move |lua, mut args| {
      let this = args.pop_front().unwrap_or(Value::Nil);
      let cell = recover_cell::<T>(lua, &this, key)?;
      let borrowed = cell
        .cell
        .try_borrow()
        .map_err(|_| Error::UserDataBorrowError)?;
      let data = borrowed.as_ref().ok_or(Error::UserDataDestructed)?;
      let r = method(lua, data)?;
      r.into_lua_multi(lua)
    });
    self.fields.push(FieldEntry {
      name: name.into(),
      is_get: true,
      callback,
    });
  }

  fn add_field_method_set<M, A>(&mut self, name: impl Into<String>, method: M)
  where
    M: Fn(&Lua, &mut T, A) -> Result<()> + MaybeSend + 'static,
    A: FromLua,
  {
    let key = self.key;
    let callback: BoxedCallback = Box::new(move |lua, mut args| {
      let this = args.pop_front().unwrap_or(Value::Nil);
      let cell = recover_cell::<T>(lua, &this, key)?;
      let val = A::from_lua(args.pop_front().unwrap_or(Value::Nil), lua)?;
      let mut borrowed = cell
        .cell
        .try_borrow_mut()
        .map_err(|_| Error::UserDataBorrowMutError)?;
      let data = borrowed.as_mut().ok_or(Error::UserDataDestructed)?;
      method(lua, data, val)?;
      ().into_lua_multi(lua)
    });
    self.fields.push(FieldEntry {
      name: name.into(),
      is_get: false,
      callback,
    });
  }

  fn add_field_function_get<F, R>(&mut self, name: impl Into<String>, function: F)
  where
    F: Fn(&Lua, AnyUserData) -> Result<R> + MaybeSend + 'static,
    R: IntoLua,
  {
    let callback: BoxedCallback = Box::new(move |lua, mut args| {
      let this = args.pop_front().unwrap_or(Value::Nil);
      let ud = AnyUserData::from_lua(this, lua)?;
      let r = function(lua, ud)?;
      r.into_lua_multi(lua)
    });
    self.fields.push(FieldEntry {
      name: name.into(),
      is_get: true,
      callback,
    });
  }

  fn add_field_function_set<F, A>(&mut self, name: impl Into<String>, function: F)
  where
    F: Fn(&Lua, AnyUserData, A) -> Result<()> + MaybeSend + 'static,
    A: FromLua,
  {
    let callback: BoxedCallback = Box::new(move |lua, mut args| {
      let this = args.pop_front().unwrap_or(Value::Nil);
      let ud = AnyUserData::from_lua(this, lua)?;
      let val = A::from_lua(args.pop_front().unwrap_or(Value::Nil), lua)?;
      function(lua, ud, val)?;
      ().into_lua_multi(lua)
    });
    self.fields.push(FieldEntry {
      name: name.into(),
      is_get: false,
      callback,
    });
  }
}

/// Destructor for the [`DataCell<T>`] stored inside the userdata (both ordinary
/// and scope-created userdata share the same layout).
unsafe extern "C-unwind" fn data_dtor<T>(ptr: *mut c_void) {
  if !ptr.is_null() {
    unsafe { drop_in_place(ptr as *mut DataCell<T>) };
  }
}

// ---------------------------------------------------------------------------
// Scoped (non-'static) userdata — used by `Lua::scope`
// ---------------------------------------------------------------------------
//
// Ordinary userdata stores a `TypeId` so values can be soundly read back out by
// concrete type (`borrow`/`take`/`is`). That requires `T: 'static`.
//
// A scope can create userdata wrapping a **non-`'static`** `T` (e.g. one that
// borrows from the enclosing stack frame). For these there is no `TypeId`, so
// instead each scoped userdata is tagged with a process-unique `u64` **marker**
// ([`CellKey::Scoped`]). Every method/field/meta closure for that userdata
// captures the *same* marker, so on dispatch it can confirm the `self` it
// received is exactly the userdata it belongs to (recovering `&DataCell<T>` is
// sound only after the marker matches — markers are never reused, so no other
// userdata can collide).
//
// Soundness over the scope lifetime: while the scope is active the wrapped `T`
// is `Some` and methods may form a transient `&T`/`&mut T`. On scope exit the
// scope's destructor `take()`s the value to `None` (dropping the borrowed `T`,
// ending its borrows) but leaves the cell memory valid; any later dispatch finds
// `None` and returns `Error::UserDataDestructed`. The cell itself is freed only
// when the GC collects the userdata, never while a `&DataCell<T>` could exist.

/// Source of process-unique scoped-userdata markers.
static SCOPED_MARKER: AtomicU64 = AtomicU64::new(1);

fn next_scoped_marker() -> u64 {
  SCOPED_MARKER.fetch_add(1, Ordering::Relaxed)
}

/// Install the userdata metatable's `__index`.
///
/// Mirrors mlua's generated `__index` (`mlua::userdata::util`): a lookup falls
/// through **field getters → the method table → a user-registered `__index`
/// meta-method**. A custom `__index` set by [`UserDataMethods::add_meta_method`]
/// therefore stays reachable instead of being overwritten by the method table
/// or the field dispatcher.
///
/// The cheap case — no fields and no custom `__index` — keeps `__index` as the
/// method table itself, so ordinary method lookup stays a plain table access.
///
/// DEVIATION from mlua: an unresolved key yields `nil` rather than raising
/// "attempt to get an unknown field", matching ulua-rt's existing behavior.
fn install_index_dispatcher(
  lua: &Lua,
  metatable: &Table,
  method_table: Table,
  getters: Table,
  has_fields: bool,
) -> Result<()> {
  // A custom `__index` was written onto the metatable by the meta-method loop
  // above; capture it here so the dispatcher can fall back to it. Only
  // functions can land there (`add_meta_method`/`add_meta_method_mut` are the
  // only ways in), so a non-function `__index` is not representable.
  let user_index: Option<Function> = metatable.raw_get("__index")?;

  if !has_fields && user_index.is_none() {
    metatable.set("__index", method_table)?;
    return Ok(());
  }

  let index_fn = lua.create_function(move |_, (ud, key): (Value, Value)| {
    if has_fields {
      let getter: Value = getters.get(key.clone())?;
      if let Value::Function(f) = getter {
        return f.call::<Value>(ud);
      }
    }
    let method: Value = method_table.get(key.clone())?;
    if method != Value::Nil {
      return Ok(method);
    }
    match &user_index {
      Some(f) => f.call::<Value>((ud, key)),
      None => Ok(Value::Nil),
    }
  })?;
  metatable.set("__index", index_fn)?;
  Ok(())
}

/// Build the userdata metatable from collected methods and fields: meta-methods
/// go onto the metatable, methods into the method table, field getters/setters
/// into their tables, and the `__index`/`__newindex` dispatchers are installed.
/// Shared by [`create_userdata`] and [`create_scoped_userdata`].
fn build_userdata_metatable(
  lua: &Lua,
  methods: Vec<Registered>,
  fields: Vec<FieldEntry>,
) -> Result<Table> {
  let method_table = lua.create_table();
  let metatable = lua.create_table();
  for item in methods {
    let func = create_callback_function(lua, item.callback)?;
    if item.is_meta {
      metatable.set(item.name, func)?;
    } else {
      method_table.set(item.name, func)?;
    }
  }

  let has_fields = !fields.is_empty();
  let getters = lua.create_table();
  let setters = lua.create_table();
  for field in fields {
    let func = create_callback_function(lua, field.callback)?;
    if field.is_get {
      getters.set(field.name, func)?;
    } else {
      setters.set(field.name, func)?;
    }
  }

  if has_fields {
    // __newindex dispatcher: try a field setter, else raise.
    let setters_c = setters.clone();
    let newindex_fn = lua.create_function(move |_, (ud, key, val): (Value, Value, Value)| {
      let setter: Value = setters_c.get(key.clone())?;
      if let Value::Function(f) = setter {
        f.call::<()>((ud, val))?;
        return Ok(());
      }
      let name = key.to_string().unwrap_or_default();
      Err(Error::RuntimeError(format!(
        "attempt to set unknown field '{name}' on userdata"
      )))
    })?;
    metatable.set("__newindex", newindex_fn)?;
  }
  // __index: field getters -> method table -> a custom `__index` meta-method.
  install_index_dispatcher(lua, &metatable, method_table, getters, has_fields)?;
  Ok(metatable)
}

/// Build a scoped (non-`'static`) userdata wrapping `data`, with a metatable
/// assembled from `T::add_fields` + `T::add_methods`. Returns the
/// [`AnyUserData`] handle plus a closure that, when called, neutralises the
/// userdata (drops `data`, leaving later access to error with
/// [`Error::UserDataDestructed`]). The neutraliser is what `Lua::scope`
/// registers as a destructor.
///
/// # Safety
/// The returned [`AnyUserData`] must not be used to read `data` back out by
/// type (there is no `TypeId`); only metatable-driven method/field/meta dispatch
/// is supported. The scope must invoke the returned neutraliser before `data`'s
/// borrowed lifetime ends.
pub(crate) fn create_scoped_userdata<T: UserData>(
  lua: &Lua,
  data: T,
) -> Result<(AnyUserData, Box<dyn FnOnce()>)> {
  let state = lua.state();
  let marker = next_scoped_marker();

  // 1. Collect fields, methods, and meta-methods (marker-keyed recovery).
  let mut collector = Collector::<T>::new(CellKey::Scoped(marker));
  T::add_fields(&mut collector);
  T::add_methods(&mut collector);
  let metatable = build_userdata_metatable(lua, collector.methods, collector.fields)?;

  // 2. Allocate the scoped userdata holding DataCell<T> and move `data` in.
  let ud = unsafe {
    let storage = lua_newuserdatadtor(state, size_of::<DataCell<T>>(), Some(data_dtor::<T>));
    if storage.is_null() {
      return Err(Error::runtime(
        "ulua-rt: failed to allocate scoped userdata",
      ));
    }
    write(
      storage as *mut DataCell<T>,
      DataCell {
        key: CellKey::Scoped(marker),
        cell: RefCell::new(Some(data)),
      },
    );
    metatable.push_to_stack();
    lua_setmetatable(state, -2);
    AnyUserData::from_ref(lua.pop_ref())
  };

  // 3. Build the neutraliser: on scope exit, take the data out of the cell,
  //    dropping the (possibly borrowing) `T` while the cell memory stays valid.
  let ud_for_dtor = ud.clone();
  let neutralise: Box<dyn FnOnce()> = Box::new(move || {
    let state = ud_for_dtor.reference.state();
    unsafe {
      ud_for_dtor.reference.push();
      let ptr = lua_touserdata(state, -1);
      lua_pop(state, 1);
      if ptr.is_null() {
        return;
      }
      let cell = &*(ptr as *const DataCell<T>);
      // Drop the data (ends borrows). If currently borrowed (a method is
      // somehow live), `try_borrow_mut` fails and we leave it — but scope
      // exit only happens after `f` returns, so no method is in flight.
      if let Ok(mut guard) = cell.cell.try_borrow_mut() {
        let _ = guard.take();
      }
    }
  });

  Ok((ud, neutralise))
}

/// Build a userdata value wrapping `data`, with a metatable assembled from the
/// type's [`UserData::add_fields`] + [`UserData::add_methods`].
pub(crate) fn create_userdata<T: UserData + MaybeSend + MaybeSync + 'static>(
  lua: &Lua,
  data: T,
) -> Result<AnyUserData> {
  let state = lua.state();

  // 1. Collect fields, methods, and meta-methods.
  let mut collector = Collector::<T>::new(CellKey::Typed(TypeId::of::<T>()));
  T::add_fields(&mut collector);
  T::add_methods(&mut collector);
  let metatable = build_userdata_metatable(lua, collector.methods, collector.fields)?;

  // 2. Allocate the userdata holding DataCell<T> and move `data` in.
  unsafe {
    let storage = lua_newuserdatadtor(state, size_of::<DataCell<T>>(), Some(data_dtor::<T>));
    if storage.is_null() {
      return Err(Error::runtime("ulua-rt: failed to allocate userdata"));
    }
    write(
      storage as *mut DataCell<T>,
      DataCell {
        key: CellKey::Typed(TypeId::of::<T>()),
        cell: RefCell::new(Some(data)),
      },
    );

    // 3. Set the metatable on the userdata (which is on top of stack), then
    //    take a ref to the userdata and return.
    metatable.push_to_stack();
    lua_setmetatable(state, -2);
    Ok(AnyUserData::from_ref(lua.pop_ref()))
  }
}
