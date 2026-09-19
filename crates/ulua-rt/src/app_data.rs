//! Per-VM **application data** — a typed, borrow-checked side store keyed by
//! Rust `TypeId`. Mirrors `mlua::Lua`'s app-data surface (`set_app_data`,
//! `app_data_ref`, `app_data_mut`, `remove_app_data`, and the `try_*` variants).
//!
//! Each `Lua` instance has its own store. Values are kept behind a per-entry
//! [`RefCell`], so a `&T` ([`AppDataRef`]) and a `&mut T` ([`AppDataRefMut`])
//! of **different** types can coexist (the usual aliasing rules apply only
//! within a single type). A VM-wide borrow counter (matching mlua's `AppData`)
//! additionally makes *any* outstanding borrow block `set_app_data` /
//! `remove_app_data` of *any* type, so the container is never mutated while a
//! guard is live.
//!
//! The store lives in a thread-local map keyed by the VM's global-state pointer
//! (the same pattern `luau_ext` uses for the per-VM compiler), since `LuaInner`
//! itself is shared immutably behind an `XRc`.

use core::{
  ffi::c_void,
  fmt::{self, Debug, Display, Formatter},
  marker::PhantomData,
  mem::transmute,
};
use std::{
  any::{Any, TypeId},
  cell::{Cell, Ref, RefCell, RefMut},
  collections::HashMap,
  ops::{Deref, DerefMut},
  rc::Rc,
};

use crate::{
  error::{Error, Result},
  state::{Lua, vm_global_key},
  sys::lua_State,
};

/// One entry: a value behind a `RefCell` so per-type borrows can be tracked,
/// shared via `Rc` so a returned guard can outlive a borrow of the outer map.
type Entry = Rc<RefCell<Box<dyn Any>>>;

/// Per-VM store: the entries plus a VM-wide outstanding-borrow counter (shared
/// via `Rc<Cell<usize>>` so a live guard can decrement it on drop).
#[derive(Default)]
struct Store {
  entries: HashMap<TypeId, Entry>,
  borrow: Rc<Cell<usize>>,
}

thread_local! {
    /// Per-VM application-data store, keyed by global-state pointer.
    static APP_DATA: RefCell<HashMap<*mut c_void, Store>> =
        RefCell::new(HashMap::new());
}

/// 任何未结清的 app-data 借用都会阻塞容器的 insert/remove（mlua 语义）；
/// 三处（set / try_set / remove）共用此文案。
const BORROW_CONFLICT_MSG: &str = "cannot mutably borrow app data container";

/// An immutable borrow of an application-data value of type `T`. Mirrors
/// `mlua::AppDataRef`.
pub struct AppDataRef<T: 'static> {
  // Hold the `Rc<RefCell>` alive and the `Ref` borrow open for as long as the
  // guard lives. The `'static` `Ref` is sound because the `_owner` `Rc` below
  // keeps the `RefCell` alive. 字段顺序是 soundness 的一部分：`guard` 必须先于
  // `_owner` drop——若 VM 在 guard 存活期间被 drop（`clear_app_data` 无条件移除
  // store，不检查 borrow 计数），`_owner` 会成为 `RefCell` 的最后一个 `Rc`，
  // 先 drop 它即释放 `RefCell`，随后 `Ref::drop` 触碰已释放内存（UB）。
  guard: Ref<'static, Box<dyn Any>>,
  _owner: Entry,
  borrow: Rc<Cell<usize>>,
  _marker: PhantomData<T>,
}

impl<T: 'static> Drop for AppDataRef<T> {
  fn drop(&mut self) {
    self.borrow.set(self.borrow.get().saturating_sub(1));
  }
}

impl<T: 'static> Deref for AppDataRef<T> {
  type Target = T;
  fn deref(&self) -> &T {
    self
      .guard
      .downcast_ref::<T>()
      .expect("app data type mismatch")
  }
}

impl<T: Debug + 'static> Debug for AppDataRef<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    (**self).fmt(f)
  }
}

impl<T: Display + 'static> Display for AppDataRef<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    (**self).fmt(f)
  }
}

impl<T: PartialEq + 'static> PartialEq<T> for AppDataRef<T> {
  fn eq(&self, other: &T) -> bool {
    (**self) == *other
  }
}

/// A mutable borrow of an application-data value of type `T`. Mirrors
/// `mlua::AppDataRefMut`.
pub struct AppDataRefMut<T: 'static> {
  // 同 [`AppDataRef`]：`guard` 必须先于 `_owner` drop（字段顺序即析构顺序），
  // 保证 `RefMut::drop` 时 `RefCell` 经 `_owner` 仍存活。
  guard: RefMut<'static, Box<dyn Any>>,
  _owner: Entry,
  borrow: Rc<Cell<usize>>,
  _marker: PhantomData<T>,
}

impl<T: 'static> Drop for AppDataRefMut<T> {
  fn drop(&mut self) {
    self.borrow.set(self.borrow.get().saturating_sub(1));
  }
}

impl<T: 'static> Deref for AppDataRefMut<T> {
  type Target = T;
  fn deref(&self) -> &T {
    self
      .guard
      .downcast_ref::<T>()
      .expect("app data type mismatch")
  }
}

impl<T: 'static> DerefMut for AppDataRefMut<T> {
  fn deref_mut(&mut self) -> &mut T {
    self
      .guard
      .downcast_mut::<T>()
      .expect("app data type mismatch")
  }
}

impl<T: Debug + 'static> Debug for AppDataRefMut<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    (**self).fmt(f)
  }
}

impl<T: PartialEq + 'static> PartialEq<T> for AppDataRefMut<T> {
  fn eq(&self, other: &T) -> bool {
    (**self) == *other
  }
}

/// Fetch the entry of type `T` from the per-VM store (shared guard included),
/// or `None` if absent. Shared by `try_app_data_ref` / `try_app_data_mut`.
fn lookup_entry<T: 'static>(state: *mut lua_State) -> Option<(Entry, Rc<Cell<usize>>)> {
  let key = unsafe { vm_global_key(state) };
  APP_DATA.with(|m| {
    let outer = m.borrow();
    outer.get(&key).and_then(|store| {
      store
        .entries
        .get(&TypeId::of::<T>())
        .map(|e| (e.clone(), store.borrow.clone()))
    })
  })
}

impl Lua {
  /// Insert (or replace) a value of type `T` in this VM's application-data
  /// store. Mirrors `mlua::Lua::set_app_data`.
  ///
  /// # Panics
  /// Panics if **any** app-data value is currently borrowed.
  pub fn set_app_data<T: 'static>(&self, data: T) {
    self.try_set_app_data(data).expect(BORROW_CONFLICT_MSG);
  }

  /// Try to insert (or replace) a value of type `T`. Returns the previous
  /// value, or an error if any app-data value is currently borrowed. Mirrors
  /// `mlua::Lua::try_set_app_data`.
  pub fn try_set_app_data<T: 'static>(&self, data: T) -> Result<Option<T>> {
    let key = unsafe { vm_global_key(self.state()) };
    APP_DATA.with(|m| {
      let mut outer = m.borrow_mut();
      let store = outer.entry(key).or_default();
      // Any outstanding borrow blocks mutation of the container (mlua).
      if store.borrow.get() != 0 {
        return Err(Error::runtime(BORROW_CONFLICT_MSG));
      }
      let old = store
        .entries
        .insert(TypeId::of::<T>(), Rc::new(RefCell::new(Box::new(data))));
      Ok(old.and_then(|e| {
        Rc::try_unwrap(e)
          .ok()
          .and_then(|cell| cell.into_inner().downcast::<T>().ok().map(|b| *b))
      }))
    })
  }

  /// Borrow the application-data value of type `T` immutably, if present.
  /// Mirrors `mlua::Lua::app_data_ref`.
  ///
  /// # Panics
  /// Panics if the value is currently mutably borrowed.
  pub fn app_data_ref<T: 'static>(&self) -> Option<AppDataRef<T>> {
    match self.try_app_data_ref::<T>() {
      Ok(opt) => opt,
      Err(_) => panic!("already mutably borrowed"),
    }
  }

  /// Try to borrow the application-data value of type `T` immutably. Returns
  /// `Ok(None)` if absent, `Err` if it is currently mutably borrowed. Mirrors
  /// `mlua::Lua::try_app_data_ref`.
  pub fn try_app_data_ref<T: 'static>(&self) -> Result<Option<AppDataRef<T>>> {
    let Some((entry, borrow)) = lookup_entry::<T>(self.state()) else {
      return Ok(None);
    };
    let guard = entry
      .try_borrow()
      .map_err(|_| Error::runtime("app data is currently mutably borrowed"))?;
    // Extend the borrow lifetime to `'static`; the `_owner` `Rc` we keep
    // alongside keeps the `RefCell` alive for as long as the guard lives, and
    // the guard struct's field order (guard 先于 `_owner` drop) 保证
    // `Ref::drop` 时 `RefCell` 仍然存活——即使 VM 已被 drop、`_owner` 已是
    // 最后一个 `Rc`（此时释放 `RefCell` 的是 `_owner` 的 drop，晚于本 drop）。
    let guard: Ref<'static, Box<dyn Any>> = unsafe { transmute(guard) };
    borrow.set(borrow.get() + 1);
    Ok(Some(AppDataRef {
      _owner: entry,
      guard,
      borrow,
      _marker: PhantomData,
    }))
  }

  /// Borrow the application-data value of type `T` mutably, if present.
  /// Mirrors `mlua::Lua::app_data_mut`.
  ///
  /// # Panics
  /// Panics if the value is currently borrowed (immutably or mutably).
  pub fn app_data_mut<T: 'static>(&self) -> Option<AppDataRefMut<T>> {
    match self.try_app_data_mut::<T>() {
      Ok(opt) => opt,
      Err(_) => panic!("already borrowed"),
    }
  }

  /// Try to borrow the application-data value of type `T` mutably. Returns
  /// `Ok(None)` if absent, `Err` if it is currently borrowed. Mirrors
  /// `mlua::Lua::try_app_data_mut`.
  pub fn try_app_data_mut<T: 'static>(&self) -> Result<Option<AppDataRefMut<T>>> {
    let Some((entry, borrow)) = lookup_entry::<T>(self.state()) else {
      return Ok(None);
    };
    let guard = entry
      .try_borrow_mut()
      .map_err(|_| Error::runtime("app data is currently borrowed"))?;
    // 同 [`Lua::try_app_data_ref`]：`'static` 生命周期提升 + guard 结构体字段
    // 顺序保证 `RefMut::drop` 时 `RefCell` 仍存活。
    let guard: RefMut<'static, Box<dyn Any>> = unsafe { transmute(guard) };
    borrow.set(borrow.get() + 1);
    Ok(Some(AppDataRefMut {
      _owner: entry,
      guard,
      borrow,
      _marker: PhantomData,
    }))
  }

  /// Remove and return the application-data value of type `T`, if present.
  /// Mirrors `mlua::Lua::remove_app_data`.
  ///
  /// # Panics
  /// Panics if **any** app-data value is currently borrowed.
  pub fn remove_app_data<T: 'static>(&self) -> Option<T> {
    let key = unsafe { vm_global_key(self.state()) };
    APP_DATA.with(|m| {
      let mut outer = m.borrow_mut();
      let store = outer.get_mut(&key)?;
      if store.borrow.get() != 0 {
        panic!("{}", BORROW_CONFLICT_MSG);
      }
      let entry = store.entries.remove(&TypeId::of::<T>())?;
      Rc::try_unwrap(entry)
        .ok()
        .and_then(|cell| cell.into_inner().downcast::<T>().ok().map(|b| *b))
    })
  }
}

/// Drop this VM's entire application-data store. Called from `LuaInner::drop`.
pub(crate) fn clear_app_data(state: *mut lua_State) {
  let key = unsafe { vm_global_key(state) };
  APP_DATA.with(|m| {
    m.borrow_mut().remove(&key);
  });
}
