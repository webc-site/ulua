//! The [`Table`] handle. Mirrors `mlua::Table`.

use core::{
  ffi::{CStr, c_void},
  fmt::{self, Debug, Formatter},
  marker::PhantomData,
};

use crate::{
  error::{Error, Result},
  state::{Lua, LuaRef, ensure_stack},
  sync::{NOT_SYNC, NotSync, XRc},
  sys::*,
  traits::{FromLua, IntoLua},
  value::{Number, Value},
};

/// A handle to a Lua table.
///
/// Mirrors `mlua::Table`. Holds a registry reference keeping the table alive.
///
/// Under the `send` feature this handle is `Send` (the VM can be moved across
/// threads) but never `Sync` — see [`crate::sync::NotSync`].
#[derive(Clone)]
pub struct Table {
  pub(crate) reference: XRc<LuaRef>,
  pub(crate) _not_sync: NotSync,
}

/// 写操作命中只读表时统一返回的错误文案。
const READONLY_TABLE_MSG: &str = "attempt to modify a readonly table";

impl Table {
  pub(crate) fn from_ref(reference: LuaRef) -> Table {
    Table {
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

  /// Set `table[key] = value`, honoring metamethods (`__newindex`).
  ///
  /// Mirrors `mlua::Table::set`. Errors (`RuntimeError`) propagate from a
  /// `__newindex` metamethod that raises.
  pub fn set<K: IntoLua, V: IntoLua>(&self, key: K, value: V) -> Result<()> {
    let lua = self.lua();
    let state = lua.state();
    let k = key.into_lua(&lua)?;
    let v = value.into_lua(&lua)?;
    // table + key + value + the protected trampoline each need a slot.
    ensure_stack(state, 4)?;
    // Drive the (possibly metamethod-invoking) settable under pcall so a
    // raising `__newindex` (or readonly table) surfaces as `Err`.
    unsafe {
      self.reference.push(); // table
      lua.push_value(&k)?; // key
      lua.push_value(&v)?; // value
      let status = protected_settable(state);
      if status != 0 {
        return Err(lua.pop_error(status));
      }
    }
    Ok(())
  }

  /// Get `table[key]`, honoring metamethods (`__index`), converting the
  /// result to `V`.
  ///
  /// Mirrors `mlua::Table::get`. The value type is the sole explicit type
  /// parameter (key type is inferred), matching mlua's `get::<V>(key)`.
  pub fn get<V: FromLua>(&self, key: impl IntoLua) -> Result<V> {
    let lua = self.lua();
    let state = lua.state();
    let k = key.into_lua(&lua)?;
    // table + key + the protected trampoline each need a slot.
    ensure_stack(state, 3)?;
    let value = unsafe {
      self.reference.push(); // table
      lua.push_value(&k)?; // key
      let status = protected_gettable(state);
      if status != 0 {
        return Err(lua.pop_error(status));
      }
      let v = lua.value_from_stack(-1)?;
      lua_pop(state, 1); // pop the result value
      v
    };
    V::from_lua(value, &lua)
  }

  /// Whether `table[key]` is non-nil.
  ///
  /// Mirrors `mlua::Table::contains_key`.
  pub fn contains_key<K: IntoLua>(&self, key: K) -> Result<bool> {
    let v: Value = self.get(key)?;
    Ok(!v.is_nil())
  }

  /// The border length (`#table`).
  ///
  /// Mirrors `mlua::Table::raw_len` (returns `usize`). ulua's `lua_objlen`
  /// gives the same border-length semantics as `lua_rawlen`.
  pub fn raw_len(&self) -> usize {
    unsafe { self.with_pushed(|state, t| lua_objlen(state, t).max(0) as usize) }
  }

  /// The length (`#table`), honoring a `__len` metamethod.
  ///
  /// Mirrors `mlua::Table::len`. Returns `Err` if a `__len` metamethod
  /// raises. Without a `__len` metamethod this is the raw border length.
  pub fn len(&self) -> Result<usize> {
    let lua = self.lua();
    // Fast path: no metatable -> raw border length (no metamethod possible).
    if self.metatable().is_none() {
      return Ok(self.raw_len());
    }
    // Evaluate `#self` protected so a raising/returning `__len` is honored.
    let f = lua.load("local t = ...; return #t").into_function()?;
    let n: i64 = f.call(self.clone())?;
    Ok(n.max(0) as usize)
  }

  /// Whether the table is empty, without invoking metamethods.
  ///
  /// Mirrors `mlua::Table::is_empty`: checks **both** the array part and the
  /// hash part (a table with only string keys is *not* empty). Uses a single
  /// `lua_next` probe — present iff the table has at least one key.
  pub fn is_empty(&self) -> bool {
    unsafe {
      self.with_pushed(|state, t| {
        lua_pushnil(state); // first key
        if lua_next(state, t) == 0 {
          // No first key: table is empty (lua_next already popped the nil).
          true
        } else {
          // stack: [table, key, value] — pop key+value.
          lua_pop(state, 2);
          false
        }
      })
    }
  }

  /// Iterate over `(key, value)` pairs.
  ///
  /// Mirrors `mlua::Table::pairs`. Returns an iterator yielding `Result<(K,
  /// V)>` items. Uses `lua_next` under the hood.
  pub fn pairs<K: FromLua, V: FromLua>(&self) -> TablePairs<K, V> {
    TablePairs {
      table: self.clone(),
      next_key: Some(Value::Nil),
      _phantom: PhantomData,
    }
  }

  /// Collect all `(key, value)` pairs into a `Vec`. Convenience over
  /// [`Table::pairs`].
  pub fn pairs_vec<K: FromLua, V: FromLua>(&self) -> Result<Vec<(K, V)>> {
    self.pairs().collect()
  }

  /// Iterate over the sequence part `[1..]`, stopping at the first `nil`
  /// (raw access — ignores `__index`). Mirrors `mlua::Table::sequence_values`.
  pub fn sequence_values<V: FromLua>(&self) -> TableSequence<V> {
    TableSequence {
      table: self.clone(),
      index: 1,
      _phantom: PhantomData,
    }
  }

  /// Call `f` for each `(key, value)` pair (raw `lua_next` traversal).
  /// Stops early on the first `Err`. Mirrors `mlua::Table::for_each`.
  ///
  /// The key/value types are the only explicit type parameters (the closure
  /// type is inferred), matching mlua's `for_each::<K, V>(f)`.
  pub fn for_each<K: FromLua, V: FromLua>(
    &self,
    mut f: impl FnMut(K, V) -> Result<()>,
  ) -> Result<()> {
    for pair in self.pairs::<K, V>() {
      let (k, v) = pair?;
      f(k, v)?;
    }
    Ok(())
  }

  /// Call `f` for each value in the sequence part. Mirrors
  /// `mlua::Table::for_each_value`.
  pub fn for_each_value<V: FromLua>(&self, mut f: impl FnMut(V) -> Result<()>) -> Result<()> {
    for v in self.sequence_values::<V>() {
      f(v?)?;
    }
    Ok(())
  }

  // --- raw (metamethod-bypassing) access ---------------------------------

  /// Set `table[key] = value` without invoking `__newindex`.
  ///
  /// Mirrors `mlua::Table::raw_set`. Errors if the table is readonly.
  pub fn raw_set<K: IntoLua, V: IntoLua>(&self, key: K, value: V) -> Result<()> {
    let lua = self.lua();
    let state = lua.state();
    let k = key.into_lua(&lua)?;
    let v = value.into_lua(&lua)?;
    self.ensure_writable()?;
    ensure_stack(state, 3)?;
    unsafe {
      self.reference.push(); // table
      lua.push_value(&k)?; // key
      lua.push_value(&v)?; // value
      lua_rawset(state, -3);
      lua_pop(state, 1); // pop table
    }
    Ok(())
  }

  /// Get `table[key]` without invoking `__index`.
  ///
  /// Mirrors `mlua::Table::raw_get`.
  pub fn raw_get<V: FromLua>(&self, key: impl IntoLua) -> Result<V> {
    let lua = self.lua();
    let state = lua.state();
    let k = key.into_lua(&lua)?;
    ensure_stack(state, 3)?;
    let value = unsafe {
      self.reference.push(); // table
      lua.push_value(&k)?; // key
      lua_rawget(state, -2); // replaces key with value
      let v = lua.value_from_stack(-1)?;
      lua_pop(state, 2); // pop value + table
      v
    };
    V::from_lua(value, &lua)
  }

  /// Append `value` at position `#table + 1` using raw access.
  ///
  /// Mirrors `mlua::Table::raw_push`. Errors if readonly.
  pub fn raw_push<V: IntoLua>(&self, value: V) -> Result<()> {
    let n = self.raw_len();
    self.raw_set((n + 1) as i64, value)
  }

  /// Remove and return the last sequence element via raw access.
  ///
  /// Mirrors `mlua::Table::raw_pop`. Errors if readonly.
  pub fn raw_pop<V: FromLua>(&self) -> Result<V> {
    let lua = self.lua();
    let n = self.raw_len();
    if n == 0 {
      return V::from_lua(Value::Nil, &lua);
    }
    self.ensure_writable()?;
    let v: V = self.raw_get(n as i64)?;
    self.raw_set(n as i64, Value::Nil)?;
    Ok(v)
  }

  /// Insert `value` at 1-based `idx`, shifting later elements up (raw).
  ///
  /// Mirrors `mlua::Table::raw_insert`. Errors on bad index or readonly.
  pub fn raw_insert<V: IntoLua>(&self, idx: i64, value: V) -> Result<()> {
    let n = self.raw_len() as i64;
    if idx < 1 || idx > n + 1 {
      return Err(Error::RuntimeError(format!(
        "bad argument #2 to 'insert' (position out of bounds): {idx}"
      )));
    }
    self.ensure_writable()?;
    let lua = self.lua();
    let state = lua.state();
    ensure_stack(state, 4)?;
    let v = value.into_lua(&lua)?;
    unsafe {
      self.with_pushed(|state, t| -> Result<()> {
        // [idx..=n] 自后向前上移一格：读 i → 写 i+1（表常驻 `t`，全程无
        // 注册表槽位开销）。
        for i in (idx..=n).rev() {
          lua_pushnumber(state, i as Number);
          lua_rawget(state, t); // 栈顶：t[i]
          lua_pushnumber(state, (i + 1) as Number);
          lua_pushvalue(state, -2);
          lua_rawset(state, t); // t[i+1] = t[i]
          lua_pop(state, 1); // 弃读出的值
        }
        // 放置新值：先 key 后 value，rawset 弹出两者。
        lua_pushnumber(state, idx as Number);
        lua.push_value(&v)?;
        lua_rawset(state, t);
        Ok(())
      })?
    }
    Ok(())
  }

  /// Remove and return the element at 1-based `idx`, shifting later
  /// elements down (raw). Mirrors `mlua::Table::raw_remove`.
  pub fn raw_remove(&self, idx: i64) -> Result<Value> {
    let n = self.raw_len() as i64;
    if n == 0 {
      return Ok(Value::Nil);
    }
    if idx < 1 || idx > n {
      return Err(Error::RuntimeError(format!(
        "bad argument #1 to 'remove' (position out of bounds): {idx}"
      )));
    }
    self.ensure_writable()?;
    let lua = self.lua();
    let state = lua.state();
    ensure_stack(state, 4)?;
    let removed = unsafe {
      self.with_pushed(|state, t| -> Result<Value> {
        // 读出 t[idx]（value_from_stack 不留栈位，读毕弹掉）。
        lua_pushnumber(state, idx as Number);
        lua_rawget(state, t);
        let removed = lua.value_from_stack(-1)?;
        lua_pop(state, 1);
        // [idx+1..=n] 自前向后下移一格。
        for i in idx..n {
          lua_pushnumber(state, (i + 1) as Number);
          lua_rawget(state, t);
          lua_pushnumber(state, i as Number);
          lua_pushvalue(state, -2);
          lua_rawset(state, t); // t[i] = t[i+1]
          lua_pop(state, 1);
        }
        lua_pushnumber(state, n as Number);
        lua_pushnil(state);
        lua_rawset(state, t); // t[n] = nil
        Ok(removed)
      })?
    };
    Ok(removed)
  }

  /// Append `value` honoring `__len`/`__newindex` (uses `#self + 1`).
  /// Mirrors `mlua::Table::push`.
  pub fn push<V: IntoLua>(&self, value: V) -> Result<()> {
    let n = self.len()?;
    self.set((n + 1) as i64, value)
  }

  /// Pop the last element honoring `__len`/`__index`/`__newindex`.
  /// Mirrors `mlua::Table::pop`.
  pub fn pop<V: FromLua>(&self) -> Result<V> {
    let lua = self.lua();
    let n = self.len()?;
    if n == 0 {
      return V::from_lua(Value::Nil, &lua);
    }
    let v: V = self.get(n as i64)?;
    self.set(n as i64, Value::Nil)?;
    Ok(v)
  }

  /// Remove all keys from the table (raw). Errors if readonly.
  /// Mirrors `mlua::Table::clear`.
  pub fn clear(&self) -> Result<()> {
    self.ensure_writable()?;
    let state = self.reference.state();
    ensure_stack(state, 4)?;
    unsafe {
      // 单趟 lua_next 遍历，边走边清除**当前** key（Lua 语义允许）。
      // 相比收集-再清空：零 Vec 分配、零注册表槽位。
      self.with_pushed(|state, t| {
        lua_pushnil(state); // 起始 key
        while lua_next(state, t) != 0 {
          // 栈: [table, key, value]
          lua_pop(state, 1); // 弹 value，留 key
          lua_pushvalue(state, -1); // 复制 key（lua_next 会消费原 key）
          lua_pushnil(state);
          lua_rawset(state, t); // t[key] = nil，弹出副本与 nil
        }
      });
    }
    Ok(())
  }

  // --- identity / equality / metatables ----------------------------------

  /// A raw pointer identifying this table (for identity comparison).
  /// Mirrors `mlua::Table::to_pointer`.
  pub fn to_pointer(&self) -> *const c_void {
    self.reference.to_pointer()
  }

  /// Compare for equality honoring an `__eq` metamethod.
  /// Mirrors `mlua::Table::equals`.
  pub fn equals(&self, other: &Table) -> Result<bool> {
    unsafe {
      Ok(self.with_pushed(|state, t| {
        other.reference.push(); // stack: [.., t, other]
        let eq = lua_equal(state, t, -1);
        lua_pop(state, 1); // pop other
        eq != 0
      }))
    }
  }

  /// The table's metatable, if any. Mirrors `mlua::Table::metatable`.
  pub fn metatable(&self) -> Option<Table> {
    let lua = self.lua();
    unsafe {
      self.with_pushed(|state, t| {
        if lua_getmetatable(state, t) == 0 {
          return None;
        }
        // stack: [table, metatable]; pop_ref 弹出元表并登记。
        Some(Table::from_ref(lua.pop_ref()))
      })
    }
  }

  /// Set (or clear, with `None`) the table's metatable.
  /// Mirrors `mlua::Table::set_metatable`. Errors if the table is readonly.
  pub fn set_metatable(&self, metatable: Option<Table>) -> Result<()> {
    self.ensure_writable()?;
    unsafe {
      // `t` 是表的绝对索引，元表压在其上方不影响它。
      self.with_pushed(|state, t| match metatable {
        Some(mt) => {
          mt.push_to_stack();
          lua_setmetatable(state, t);
        }
        None => {
          lua_pushnil(state);
          lua_setmetatable(state, t);
        }
      });
    }
    Ok(())
  }

  // --- readonly (Luau extension) -----------------------------------------

  /// Whether the table is marked readonly (Luau). Mirrors
  /// `mlua::Table::is_readonly`.
  pub fn is_readonly(&self) -> bool {
    unsafe { self.with_pushed(|state, t| lua_getreadonly(state, t) != 0) }
  }

  /// 写入路径的只读检查：命中只读表时返回统一的 `RuntimeError`。
  fn ensure_writable(&self) -> Result<()> {
    if self.is_readonly() {
      return Err(Error::RuntimeError(READONLY_TABLE_MSG.to_string()));
    }
    Ok(())
  }

  /// 把表压栈执行 `f`，随后弹出（封装重复的 push/pop 栈序列）。
  ///
  /// `f` 收到表的**绝对**栈索引（压栈期间其上方的压栈不会使它失效）。
  /// `f` 内不得运行可 panic 的用户代码，否则栈深不恢复。
  unsafe fn with_pushed<T>(&self, f: impl FnOnce(*mut lua_State, c_int) -> T) -> T {
    let state = self.reference.state();
    unsafe {
      self.reference.push();
      let t = lua_gettop(state);
      let out = f(state, t);
      lua_pop(state, 1);
      out
    }
  }

  /// Mark the table readonly or writable (Luau). Mirrors
  /// `mlua::Table::set_readonly`.
  pub fn set_readonly(&self, enabled: bool) {
    unsafe {
      self.with_pushed(|state, t| lua_setreadonly(state, t, enabled as c_int));
    }
  }
}

/// Iterator over a table's sequence part (see [`Table::sequence_values`]).
pub struct TableSequence<V> {
  table: Table,
  index: i64,
  _phantom: PhantomData<V>,
}

impl<V: FromLua> Iterator for TableSequence<V> {
  type Item = Result<V>;

  fn next(&mut self) -> Option<Self::Item> {
    let lua = self.table.lua();
    // Raw get of the next sequence slot; stop at the first nil.
    let value: Value = match self.table.raw_get(self.index) {
      Ok(v) => v,
      Err(e) => return Some(Err(e)),
    };
    if value.is_nil() {
      return None;
    }
    self.index += 1;
    Some(V::from_lua(value, &lua))
  }
}

impl Debug for Table {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "Table(len={})", self.raw_len())
  }
}

impl PartialEq for Table {
  fn eq(&self, other: &Self) -> bool {
    // Reference (pointer) identity, matching mlua's `==` on handles: two
    // handles are equal iff they point at the *same* Lua table object.
    self.to_pointer() == other.to_pointer()
  }
}

/// Compare a [`Table`]'s sequence part to a Rust slice of values.
///
/// Mirrors mlua's `impl PartialEq<[T]> for Table`: equal when the table's
/// `1..=len` sequence (read raw) matches `other` element-wise and has the
/// same length.
impl<T> PartialEq<[T]> for Table
where
  T: FromLua + PartialEq + Clone,
{
  fn eq(&self, other: &[T]) -> bool {
    // Compare the *sequence* part (stopping at the first nil border),
    // matching mlua's `sequence_values`-based slice comparison. This is the
    // robust border semantics for tables with nil holes (e.g.
    // `{1, 2, nil, 4, 5}` compares equal to `[1, 2]`).
    let mut iter = self.sequence_values::<T>();
    for expected in other.iter() {
      match iter.next() {
        Some(Ok(got)) if &got == expected => {}
        _ => return false,
      }
    }
    // The sequence must be exactly the slice length (no extra elements).
    iter.next().is_none()
  }
}

impl<T, const N: usize> PartialEq<[T; N]> for Table
where
  T: FromLua + PartialEq + Clone,
{
  fn eq(&self, other: &[T; N]) -> bool {
    self == other.as_slice()
  }
}

impl<T> PartialEq<&[T]> for Table
where
  T: FromLua + PartialEq + Clone,
{
  fn eq(&self, other: &&[T]) -> bool {
    self == *other
  }
}

/// Iterator over a table's key/value pairs (see [`Table::pairs`]).
pub struct TablePairs<K, V> {
  table: Table,
  next_key: Option<Value>,
  _phantom: PhantomData<(K, V)>,
}

impl<K: FromLua, V: FromLua> Iterator for TablePairs<K, V> {
  type Item = Result<(K, V)>;

  fn next(&mut self) -> Option<Self::Item> {
    let key = self.next_key.take()?;
    let lua = self.table.lua();
    let state = lua.state();
    unsafe {
      self.table.reference.push(); // [.. table]
      if lua.push_value(&key).is_err() {
        lua_pop(state, 1);
        return None;
      }
      // stack: [table, key]
      let has = lua_next(state, -2);
      if has == 0 {
        // lua_next popped the key; pop the table.
        lua_pop(state, 1);
        self.next_key = None;
        return None;
      }
      // stack: [table, next_key, value]
      let k_val = match lua.value_from_stack(-2) {
        Ok(v) => v,
        Err(e) => {
          lua_pop(state, 3);
          return Some(Err(e));
        }
      };
      let v_val = match lua.value_from_stack(-1) {
        Ok(v) => v,
        Err(e) => {
          lua_pop(state, 3);
          return Some(Err(e));
        }
      };
      // Remember the key for the next iteration, then clean the stack.
      self.next_key = Some(k_val.clone());
      lua_pop(state, 3); // value, next_key, table

      let k = match K::from_lua(k_val, &lua) {
        Ok(k) => k,
        Err(e) => return Some(Err(e)),
      };
      let v = match V::from_lua(v_val, &lua) {
        Ok(v) => v,
        Err(e) => return Some(Err(e)),
      };
      Some(Ok((k, v)))
    }
  }
}

/// Create a fresh empty table on `lua` and return a handle.
pub(crate) fn create_table(lua: &Lua) -> Table {
  let state = lua.state();
  unsafe {
    lua_createtable(state, 0, 0);
    Table::from_ref(lua.pop_ref())
  }
}

// ---------------------------------------------------------------------------
// Protected indexing
//
// `lua_gettable`/`lua_settable` may invoke `__index`/`__newindex` metamethods
// that *raise* (longjmp). Calling them unprotected across the Rust/VM boundary
// would unwind past Rust frames. We therefore run them inside `lua_pcall` via a
// tiny C trampoline, so a raising metamethod (or a readonly-table write) is
// reported as an ordinary non-zero status with the error object on the stack.
// ---------------------------------------------------------------------------

/// C trampoline: stack is `[table, key]`; performs `lua_gettable` and leaves
/// the result on top.
unsafe extern "C-unwind" fn c_gettable(state: *mut lua_State) -> c_int {
  unsafe {
    lua_gettable(state, 1);
    1
  }
}

/// C trampoline: stack is `[table, key, value]`; performs `lua_settable`.
unsafe extern "C-unwind" fn c_settable(state: *mut lua_State) -> c_int {
  unsafe {
    lua_settable(state, 1);
    0
  }
}

/// Run a C trampoline protected: push `f` below the `nargs` arguments (the
/// stack is `[.., f-position]`), then `lua_pcall`. `insert` is the negative
/// offset the function is inserted to; returns the non-zero status on failure
/// with the error object on top.
unsafe fn protected_table_op(
  state: *mut lua_State,
  f: unsafe extern "C-unwind" fn(*mut lua_State) -> c_int,
  name: &CStr,
  nargs: c_int,
  nresults: c_int,
  insert: c_int,
) -> c_int {
  unsafe {
    lua_pushcclosurek(state, Some(f), name.as_ptr(), 0, None);
    lua_insert(state, insert);
    lua_pcall(state, nargs, nresults, 0)
  }
}

/// Run `lua_gettable` protected. Expects `[table, key]` on top; on success
/// leaves `[result]` where the two inputs were; on failure leaves the error
/// object on top and returns the non-zero status.
unsafe fn protected_gettable(state: *mut lua_State) -> c_int {
  unsafe { protected_table_op(state, c_gettable, c"ulua-rt-gettable", 2, 1, -3) }
}

/// Run `lua_settable` protected. Expects `[table, key, value]` on top; pops
/// them on success; on failure leaves the error object and returns the status.
unsafe fn protected_settable(state: *mut lua_State) -> c_int {
  unsafe { protected_table_op(state, c_settable, c"ulua-rt-settable", 3, 0, -4) }
}
