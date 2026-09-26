//! The [`Table`] handle. Mirrors `mlua::Table`.

use core::{
  ffi::c_void,
  fmt::{self, Debug, Formatter},
  marker::PhantomData,
};

use crate::{
  error::{Error, Result},
  registry::RegHandle,
  state::{Lua, LuaRef, ensure_stack, ensure_stack_or_panic, pop_stack, with_reference_pushed},
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
/// threads) but never `Sync` — see `crate::sync::NotSync`.
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
    // 压栈三步都是带契约的 safe 门面：`reference.push` 由注册表引用锚定表值在世（VM 侧
    // `lua_rawgeti` 自保一层），`push_value` 各净压一层，总量在上一行 `ensure_stack` 的
    // 4 层余量内。
    self.reference.push(); // table
    lua.push_value(&k)?; // key
    lua.push_value(&v)?; // value
    // Safety: `state` 存活且由当前线程驱动（句柄 XRc<LuaInner> + NotSync 纪律）；上面的
    // 压栈序列让栈顶恰为 `[table, key, value]`，`ensure_stack(state, 4)` 另留了闭包槽 +
    // pcall 帧头寸——逐条满足 `protected_table_op(TableOp::Set)` 函数头契约；失败路径
    // `pop_error`（safe 门面）弹出错误对象并回收栈深。
    let status = unsafe { protected_table_op(state, TableOp::Set) };
    if status != 0 {
      return Err(lua.pop_error(status));
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
    // 压栈两步是带契约的 safe 门面（`reference.push` 由注册表引用锚定表值在世，VM 侧
    // `lua_rawgeti` 自保一层；`push_value` 净压一层），共 2 层落在上一行 `ensure_stack`
    // 的 3 层余量内。
    self.reference.push(); // table
    lua.push_value(&k)?; // key
    // Safety: `state` 存活且当前线程驱动；`ensure_stack(state, 3)` 预留 table/key + pcall
    // 头寸；栈顶 `[table, key]` 满足 `protected_table_op(TableOp::Get)` 函数头契约。
    let status = unsafe { protected_table_op(state, TableOp::Get) };
    if status != 0 {
      return Err(lua.pop_error(status));
    }
    // `take_top` 是带契约的 safe 门面：读 result 槽（引用型值登记为注册表引用）并弹回
    // 这一层，与上面的压栈序列配对。
    let value = take_top(&lua)?;
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
    // 与其余读路径一致的余量探测（本闭包不再额外压栈）。
    ensure_stack_or_panic(self.reference.state(), 1);
    self.with_pushed(|lua, t| {
      let state = lua.state();
      // Safety: `state` 存活（句柄的 `XRc<LuaInner>`）且 `t` 是 `with_pushed` 刚给出的
      // 表索引；`lua_objlen` 只读该表取边界长度、不引发 GC 也不抛错。负值先经 `max(0)`
      // 夹住再转 `usize`，不会因符号扩展变成大数。
      unsafe { lua_objlen(state, t) }.max(0) as usize
    })
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
    let state = lua.state();
    // table + 闭包槽 + 结果槽各占一位。
    ensure_stack(state, 3)?;
    // 把 `#self` 交给 VM 的 luaV_objlen（字节码 OP_LEN 同一实现）在 pcall 下执行：
    // 会抛错的 `__len`（或返回非数字）被呈为 `Err`，而不是 longjmp 跨过 Rust 帧。
    // `reference.push` 是带契约的 safe 门面（注册表引用锚定表值、VM 侧自保一层）。
    self.reference.push(); // [table]
    // Safety: `state` 存活且由当前线程驱动；`ensure_stack(state, 3)` 预留了表 + 闭包槽 +
    // pcall 帧头寸；栈顶 `[table]` 恰满足 `protected_table_op(TableOp::Len)` 函数头契约——
    // `__len` 抛错在 pcall 内呈为状态码而非 longjmp 穿栈。
    let status = unsafe { protected_table_op(state, TableOp::Len) };
    if status != 0 {
      return Err(lua.pop_error(status));
    }
    // 行为对齐注记（S4）：本 cpp fork 没有独立的 `lua_len` API，
    // `#t` 一律走 luaV_objlen；它对非数字 `__len` 返回值直接
    // runerror（"'__len' must return a number"）。旧实现
    // （现编 `local t = ...; return #t` chunk 再调用）触发的正是同
    // 一条检查，故换成受保护 trampoline 无行为分歧，仅省去每次调
    // 用的 chunk 编译开销。
    // Safety: 栈顶是 trampoline 留下的结果数值槽；`number_at` 只读该槽、不动栈深，
    // `isnum` 出参收口为 `Option`（rt 域统一门面）。读出后 `lua_pop(state, 1)` 弹回这
    // 一层，与压栈配对；错误路径由 safe 的 `pop_error` 同样平衡。
    let number = unsafe {
      let number = number_at(state, -1);
      lua_pop(state, 1);
      number
    };
    // luaV_objlen 保证结果必为数字，此分支仅为防御。
    let Some(n) = number else {
      return Err(Error::runtime("'__len' must return a number"));
    };
    // f64→i64 向零截断，与旧路径 `Value::Number` → i64 的转换语义
    // 一致；负长度钳到 0。
    Ok((n as i64).max(0) as usize)
  }

  /// Whether the table is empty, without invoking metamethods.
  ///
  /// Mirrors `mlua::Table::is_empty`: checks **both** the array part and the
  /// hash part (a table with only string keys is *not* empty). Uses a single
  /// `lua_next` probe — present iff the table has at least one key.
  pub fn is_empty(&self) -> bool {
    // 栈峰值：表 + nil 起始 key + lua_next 产出的 key/value 共 3 层。
    ensure_stack_or_panic(self.reference.state(), 3);
    self.with_pushed(|lua, t| {
      let state = lua.state();
      // 一次探测即结论：起始 nil key + lua_next 的增减栈是一条自平衡事务，合为一个
      // 边界块。
      // Safety: `state` 存活（`with_pushed` 门面契约：句柄的 XRc<LuaInner> 保活）；
      // pushnil 净压一层，`lua_next(state, t)` 消费栈顶 key——有下一键时压出 key+value
      // （净 +2）、无下一键时只弹掉 key（净 -1，此时表已被 with_pushed 要求的单层收回）；
      // 峰值 3 层在上一行 `ensure_stack_or_panic` 的余量内。非空分支再弹回 key+value 两层，
      // 两条分支都回到 with_pushed 要求的单层表。
      unsafe {
        lua_pushnil(state); // first key
        let advanced = lua_next(state, t) != 0;
        if advanced {
          // stack: [table, key, value] — pop key+value.
          lua_pop(state, 2);
        }
        // No first key: table is empty (lua_next already popped the nil).
        !advanced
      }
    })
  }

  /// Iterate over `(key, value)` pairs.
  ///
  /// Mirrors `mlua::Table::pairs`. Returns an iterator yielding `Result<(K,
  /// V)>` items. Uses `lua_next` under the hood.
  pub fn pairs<K: FromLua, V: FromLua>(&self) -> TablePairs<K, V> {
    TablePairs {
      table: self.clone(),
      next_key: Some(Value::Nil),
      next_key_integer_tag: false,
      _phantom: PhantomData,
    }
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
    // 压栈三步是带契约的 safe 门面（注册表引用锚定表值；VM 侧自保一层 + 上面的余量）。
    self.reference.push(); // table
    lua.push_value(&k)?; // key
    lua.push_value(&v)?; // value
    // Safety: `state` 存活且当前线程驱动，`ensure_stack(state, 3)` 预留 table/key/value
    // 三层；栈顶 `[table, key, value]` 布局下 `lua_rawset(state, -3)` 弹出 key+value 写回
    // 表，`lua_pop(state, 1)` 弹表，与压栈严格配对。
    unsafe {
      lua_rawset(state, -3);
      lua_pop(state, 1); // pop table
    }
    Ok(())
  }

  /// 把迭代器按 1 基下标写成 Lua 序列（数组表）。`Lua::create_sequence_from` 与
  /// `conversion.rs` 里 `Vec<T>` / `&[T]` / `[T; N]` 的 `IntoLua` 共用此单点：
  /// 四处只在迭代器形态上不同，下标基与写入方式一致（目标表新建、无元表，raw 写足够）。
  pub(crate) fn fill_sequence<V, I>(&self, iter: I) -> Result<()>
  where
    V: IntoLua,
    I: IntoIterator<Item = V>,
  {
    for (i, item) in iter.into_iter().enumerate() {
      self.raw_set((i + 1) as i64, item)?;
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
    // 压栈两步是带契约的 safe 门面（注册表引用锚定表值；VM 侧自保一层 + 上面 3 层余量）。
    self.reference.push(); // table
    lua.push_value(&k)?; // key
    // Safety: `state` 存活且当前线程驱动，`ensure_stack(state, 3)` 预留 table+key+结果
    // 头寸；`lua_rawget(state, -2)` 在 `[table, key]` 顶布局上以值替换 key（净压一层）。
    unsafe { lua_rawget(state, -2) }; // replaces key with value
    // `value_from_stack` 是带契约的 safe 门面：读有效栈顶并登记引用型值。
    let value = lua.value_from_stack(-1)?;
    // `pop_stack` 精确回收 rawget 产物 + 表两层，与压栈配对（`state` 存活）。
    pop_stack(state, 2);
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
      return Err(Error::runtime(format!(
        "bad argument #2 to 'insert' (position out of bounds): {idx}"
      )));
    }
    self.ensure_writable()?;
    let lua = self.lua();
    let state = lua.state();
    ensure_stack(state, 4)?;
    let v = value.into_lua(&lua)?;
    // [idx..=n] 自后向前上移一格：读 i → 写 i+1（表常驻 `with_pushed` 给的 `t`，
    // 全程无注册表槽位开销）；`raw_copy_slot` 每轮自平衡栈深。
    self.with_pushed(|lua, t| -> Result<()> {
      let state = lua.state();
      for i in (idx..=n).rev() {
        // Safety: `raw_copy_slot` 函数头契约由本调用凑齐——`state` 存活、`t` 是刚压入
        // 表的绝对索引、3 层峰值在 `ensure_stack(state, 4)` 余量内。
        unsafe { raw_copy_slot(state, t, i, i + 1) };
      }
      // 放置新值：先 key 后 value，rawset 弹出两者。
      // Safety: `state` 存活、`t` 有效，数字键净压一层（余量同上）。
      unsafe { lua_pushnumber(state, idx as Number) };
      // `push_value` 是带契约的 safe 门面。失败时保持旧路径的既有语义：`?` 直接传播，
      // `with_pushed` 的收尾 pop 弹到的是刚压的 key（表槽因此多留一层）——本次改写
      // 严格保序，不在此修正该栈深细节（见批次报告的移交项）。
      lua.push_value(&v)?;
      // Safety: 栈顶两槽恰为 [key, value]，rawset 消费两者写回 t[idx]。
      unsafe { lua_rawset(state, t) };
      Ok(())
    })?;
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
      return Err(Error::runtime(format!(
        "bad argument #1 to 'remove' (position out of bounds): {idx}"
      )));
    }
    self.ensure_writable()?;
    let lua = self.lua();
    let state = lua.state();
    ensure_stack(state, 4)?;
    let removed = self.with_pushed(|lua, t| -> Result<Value> {
      let state = lua.state();
      // 读出 t[idx]：数字键 + rawget 是一组连续的边界压栈，收在一个块里；随后
      // `take_top`（safe 门面）读值并弹回这一层（保持 with_pushed 的单层表）。
      // Safety: `state` 存活、`t` 是 with_pushed 给的表绝对索引；数字键净压一层，
      // rawget 消费该键并压出 `t[idx]`，峰值 2 层在 `ensure_stack(state, 4)` 余量内。
      unsafe {
        lua_pushnumber(state, idx as Number);
        lua_rawget(state, t);
      }
      let removed = take_top(lua)?;
      // [idx+1..=n] 自前向后下移一格（每轮 `raw_copy_slot` 自平衡栈深）。
      for i in idx..n {
        // Safety: `raw_copy_slot` 函数头契约由本调用凑齐（存活 state、有效表索引 `t`、
        // 3 层峰值在 ensure_stack(4) 余量内）。
        unsafe { raw_copy_slot(state, t, i + 1, i) };
      }
      // Safety: 末尾三句恰好消费「n 键 + nil 值」两槽，写 t[n] = nil。
      unsafe {
        lua_pushnumber(state, n as Number);
        lua_pushnil(state);
        lua_rawset(state, t); // t[n] = nil
      }
      Ok(removed)
    })?;
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
    // 单趟 lua_next 遍历，边走边清除**当前** key（Lua 语义允许）。
    // 相比收集-再清空：零 Vec 分配、零注册表槽位。
    self.with_pushed(|lua, t| {
      let state = lua.state();
      // 整趟遍历是一条自平衡的栈事务（每轮压栈/弹出严格配对，不留残栈），故合为一个
      // 边界块；块内只有 C 栈操作，不运行用户代码。
      // Safety: `state` 存活（with_pushed 门面的 XRc<LuaInner> 链）且由当前线程驱动，
      // 峰值 3 层（起始 key + lua_next 产出的 key/value）在上一行 `ensure_stack(state, 4)`
      // 的余量内。逐步：pushnil 作起始 key；`lua_next(state, t)` 在 `[table, key]` 上产出
      // key+value（净 +2），遍历结束时返回 0 且已弹掉 key；弹 value 留 key；pushvalue 复制
      // key（下一轮 lua_next 会消费原 key）+ pushnil 压新值，rawset 消费「key 副本 + nil」
      // 写 t[key] = nil，栈深回到 `[table, key]`。
      unsafe {
        lua_pushnil(state); // 起始 key
        while lua_next(state, t) != 0 {
          // 栈: [table, key, value]
          lua_pop(state, 1);
          lua_pushvalue(state, -1);
          lua_pushnil(state);
          lua_rawset(state, t);
        }
      }
    });
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
    let state = self.reference.state();
    // 栈峰值：本表 + other 两层。
    ensure_stack(state, 2)?;
    Ok(self.with_pushed(|lua, t| {
      let state = lua.state();
      // `other.reference.push()` 是 safe 门面：其注册表引用锚定另一张表在世，VM 侧
      // `lua_rawgeti` 自保一层（上面的 `ensure_stack(state, 2)` 另留两表槽余量）。
      other.reference.push(); // stack: [.., t, other]
      // Safety: `state` 存活且当前线程驱动；`t` 是 with_pushed 给的绝对索引（不受上方
      // push 影响），-1 是刚压入的 other；`lua_equal` 可触发 `__eq` 但只比较、不改栈深，
      // 故紧随的 `lua_pop(state, 1)` 弹的仍是 other，栈深回到 with_pushed 要求的单层表。
      let eq = unsafe {
        let eq = lua_equal(state, t, -1);
        lua_pop(state, 1);
        eq
      };
      eq != 0
    }))
  }

  /// The table's metatable, if any. Mirrors `mlua::Table::metatable`.
  pub fn metatable(&self) -> Option<Table> {
    let lua = self.lua();
    // 栈峰值：本表 + lua_getmetatable 结果两层。
    ensure_stack_or_panic(lua.state(), 2);
    self.with_pushed(|lua, t| {
      let state = lua.state();
      // Safety: `state` 存活且当前线程驱动；预留两层覆盖 表+元表结果；`lua_getmetatable`
      // 命中时恰压入元表一层，未命中分支不留栈增量。
      if unsafe { lua_getmetatable(state, t) } == 0 {
        return None;
      }
      // stack: [table, metatable]; `pop_ref`（safe 门面）弹出元表一层并登记为引用，
      // 栈深回到 with_pushed 要求的单层表。
      Some(Table::from_ref(lua.pop_ref()))
    })
  }

  /// Set (or clear, with `None`) the table's metatable.
  /// Mirrors `mlua::Table::set_metatable`. Errors if the table is readonly.
  pub fn set_metatable(&self, metatable: Option<Table>) -> Result<()> {
    self.ensure_writable()?;
    // 栈峰值：本表 + 元表/nil 两层。
    ensure_stack(self.reference.state(), 2)?;
    self.with_pushed(|lua, t| {
      let state = lua.state();
      // `t` 是表的绝对索引，元表压在其上方不影响它。
      // Safety: `state` 存活且当前线程驱动，`t` 是 with_pushed 给的表绝对索引（元表压在
      // 其上方不影响它）。Some 分支的 `mt.push_to_stack()` 是 safe 封装（`lua_rawgeti`
      // 自带栈预留、owning VM 由句柄的 XRc<LuaInner> + move-not-share 纪律保证），None
      // 分支的 pushnil 与之一一同形；收尾 `lua_setmetatable(state, t)` 恰好消费上一步压入
      // 的那一层，满足 with_pushed 的栈深配对。
      unsafe {
        match metatable {
          Some(mt) => mt.push_to_stack(),
          None => lua_pushnil(state),
        }
        lua_setmetatable(state, t);
      }
    });
    Ok(())
  }

  // --- readonly (Luau extension) -----------------------------------------

  /// Whether the table is marked readonly (Luau). Mirrors
  /// `mlua::Table::is_readonly`.
  pub fn is_readonly(&self) -> bool {
    // 与其余读路径一致的余量探测（`reference.push` 自带 VM 侧保底，闭包不再额外压栈）。
    ensure_stack_or_panic(self.reference.state(), 1);
    self.with_pushed(|lua, t| {
      let state = lua.state();
      // Safety: `state` 存活（本句柄的 XRc<LuaInner>）且由当前线程驱动，`t` 是
      // with_pushed 刚给出的表索引；`lua_getreadonly` 只读该表的标志位、不动栈深。
      unsafe { lua_getreadonly(state, t) != 0 }
    })
  }

  /// 写入路径的只读检查：命中只读表时返回统一的 `RuntimeError`。
  fn ensure_writable(&self) -> Result<()> {
    if self.is_readonly() {
      return Err(Error::runtime(READONLY_TABLE_MSG));
    }
    Ok(())
  }

  /// 把表压栈执行 `f`，随后弹出（封装重复的 push/pop 栈序列）。
  ///
  /// `f` 收到 `(带生命周期的 [`Lua`] 句柄, 表的**绝对**栈索引)`——压栈期间其上方的
  /// 压栈不会使索引失效；句柄背后是 `reference` 的 `XRc<LuaInner>`，VM 存活由类型
  /// 系统担保（§2：上下文裸指针 → 带生命周期引用）。`f` 只在即将调用 `lua_*` C ABI
  /// 入口的瞬间经 [`Lua::state`] 收口点读出裸指针，并在自己的每个 `unsafe` 块上
  /// 就近写明契约——因此本门面是**安全**函数：`reference.push` 亦是 safe 门面
  /// （VM 侧 `lua_rawgeti` 自保一层头寸）。
  ///
  /// `f` 的两条前提是**正确性**（而非内存安全）约定：不得改动栈深、不得运行可
  /// panic/longjmp 的用户代码，否则收尾的 `lua_pop` 弹错槽位或根本不执行。
  ///
  /// 栈配对样板本体不在此处重复：委托给 `state::with_reference_pushed`（见该函数），
  /// 两侧分工是——那边唯一持有 push → `lua_gettop` → pop 的栈配对契约，本方法只是
  /// 面向 `Table` 句柄的安全外壳，让各调用闭包就近书写自己的窄 `unsafe` 契约。
  fn with_pushed<T>(&self, f: impl FnOnce(&Lua, c_int) -> T) -> T {
    with_reference_pushed(&self.reference, f)
  }

  /// Mark the table readonly or writable (Luau). Mirrors
  /// `mlua::Table::set_readonly`.
  pub fn set_readonly(&self, enabled: bool) {
    // 与其余写路径一致的余量探测（`reference.push` 自带 VM 侧保底，闭包不再额外压栈）。
    ensure_stack_or_panic(self.reference.state(), 1);
    self.with_pushed(|lua, t| {
      let state = lua.state();
      // Safety: `state` 存活且当前线程驱动，`t` 是 with_pushed 给的表索引；
      // `lua_setreadonly` 只对该表设置标志位、不动栈深。
      unsafe { lua_setreadonly(state, t, enabled as c_int) };
    });
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

impl RegHandle for Table {
  fn reference(&self) -> &XRc<LuaRef> {
    &self.reference
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
    for expected in other {
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
  /// 下一轮回压键的原始槽位是否 `LUA_TINTEGER`。INTEGER 键若经
  /// `push_value` 的 roundtrips 分支压回会漂移成 NUMBER tag，`lua_next`
  /// 的 `findindex` 按 tag 严格匹配会找不到并 runerror（panic 无保护穿出
  /// 迭代器）——记录 tag，回压时走 `lua_pushinteger_64` 保真。
  next_key_integer_tag: bool,
  _phantom: PhantomData<(K, V)>,
}

impl<K: FromLua, V: FromLua> Iterator for TablePairs<K, V> {
  type Item = Result<(K, V)>;

  fn next(&mut self) -> Option<Self::Item> {
    let key = self.next_key.take()?;
    let lua = self.table.lua();
    let state = lua.state();
    // 栈峰值：表 + key + lua_next 产出的 key/value，再加 value_from_stack
    // 对引用型值的一次临时重压，共 4 层。头寸不足时经迭代器的 `Err` 通道上抛。
    if let Err(e) = ensure_stack(state, 4) {
      return Some(Err(e));
    }
    // 压栈两步是带契约的 safe 门面：`reference.push` 由注册表引用锚定表值在世（VM 侧
    // `lua_rawgeti` 自保一层），`next_key` 是上一轮对栈值的克隆（同 VM 内有效值）。
    self.table.reference.push(); // [.. table]
    if self.next_key_integer_tag {
      // 保 tag 回压（INTEGER 键不得经 push_value 漂移成 NUMBER）。
      // Safety: `state` 存活且当前线程驱动；`key` 由上一轮 `value_from_stack`
      // 从 INTEGER 槽读出，此处精确还原。`lua_pushinteger_64` 自带 ensure_stack。
      if let Value::Integer(i) = &key {
        unsafe { lua_pushinteger_64(state, *i) };
      } else {
        let _ = lua.push_value(&key);
      }
    } else if lua.push_value(&key).is_err() {
      // 栈顶即上一句门面压入的表，弹回一层保持配对（`state` 存活、top>base）。
      pop_stack(state, 1);
      return None;
    }
    // stack: [table, key]
    // Safety: `state` 存活且当前线程驱动；ensure_stack(4) 已覆盖 表+key+lua_next 产出的
    // key/value 以及 value_from_stack 的一次临时重压；`lua_next(state, -2)` 消费 key，
    // 有下一键时压出 key+value，否则只弹掉 key（两条分支各自的回收动作见下）。
    if unsafe { lua_next(state, -2) } == 0 {
      // lua_next popped the key; pop the table（此刻栈上只剩刚压入的表）。
      pop_stack(state, 1);
      self.next_key = None;
      return None;
    }
    // stack: [table, next_key, value]
    // 两个槽的读取走 safe 门面 `value_from_stack`（内部 pushvalue+pop_ref 净零）；先读
    // key 再读 value，前者失败即短路（与旧实现的 `?` 顺序一致）。
    let pair = lua
      .value_from_stack(-2)
      .and_then(|k| lua.value_from_stack(-1).map(|v| (k, v)));
    // 记录新键的原始 tag——必须在 pop 之前读取（pop 后槽已回收）：INTEGER 键若
    // 经 `push_value` 的 roundtrips 分支压回会漂移成 NUMBER tag，`lua_next` 的
    // `findindex` 按 tag 严格匹配会找不到并 runerror（panic 无保护穿出迭代器）。
    // Safety: `state` 存活且当前线程驱动；-2 槽此刻仍持有 `lua_next` 压出的键，
    // `lua_type` 只读类型标记；随后 `lua_pop(state, 3)` 精确回收 [table, next_key,
    // value] 三槽（读取成败皆然），迭代器不留栈位。
    unsafe {
      self.next_key_integer_tag =
        LuaType::from_c_int(lua_type(state, -2)) == Some(LuaType::Integer);
      lua_pop(state, 3);
    }
    let (k_val, v_val) = match pair {
      Ok(pair) => pair,
      Err(e) => return Some(Err(e)),
    };
    // Remember the key for the next iteration.
    self.next_key = Some(k_val.clone());
    // 纯 Rust：K/V 的用户转换在栈外进行（可 panic/抛错，此时三槽已回收）。
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

/// Create a fresh empty table on `lua` and return a handle.
pub(crate) fn create_table(lua: &Lua) -> Table {
  create_table_with_capacity(lua, 0, 0)
}

/// Create a fresh empty table on `lua` with preallocated array and record capacities.
pub(crate) fn create_table_with_capacity(lua: &Lua, narr: usize, nrec: usize) -> Table {
  let state = lua.state();
  // 预留 `lua_createtable` 压入新表的一层（`pop_ref` 随即弹掉）。
  ensure_stack_or_panic(state, 1);
  // cpp `lapi.cpp:899` 的 `api_check(L, narray >= 0 && nrec >= 0)` 要求传给
  // `lua_createtable` 的容量为非负 `int`。原 `narr as i32` 对超过 `i32::MAX` 的
  // `usize` 静默截断成负数，既违该契约又把脏容量喂进 VM。改为饱和到 `i32::MAX`：
  // C API 参数本身即 `int`，比它更大无法表达，饱和是唯一不越界的方向。
  let narr = i32::try_from(narr).unwrap_or(i32::MAX);
  let nrec = i32::try_from(nrec).unwrap_or(i32::MAX);
  // Safety: `state` 存活且由当前线程驱动（`&Lua` 句柄保证）；上一行预留了 lua_createtable
  // 压入新表所需的一层。
  unsafe { lua_createtable(state, narr, nrec) };
  // `pop_ref` 是带契约的 safe 门面：弹出刚压入的栈顶表值并登记为注册表引用。
  Table::from_ref(lua.pop_ref())
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

/// 读 `idx` 槽位为数字（VM 原生强转：Number 直通、String 经 `luaO_str2d`）。
/// （rt 域统一门面，`state.rs::coerce_number_value` 亦复用。）
///
/// # Safety
/// `state` 必须存活且由当前线程驱动，`idx` 为有效栈索引；`lua_tonumberx`
/// 只读该槽、不动栈深，`Option` 返回值即 cpp `isnum` 出参的收口。
pub(crate) unsafe fn number_at(state: *mut LuaState, idx: c_int) -> Option<Number> {
  // Safety: 落实本函数 `# Safety` 契约——`lua_tonumberx` 只读栈槽值。
  unsafe { lua_tonumberx(state, idx) }
}

/// 收口门面：读出栈顶的 [`Value`]（引用型值由 `value_from_stack` 登记注册表引用），
/// 随后弹回这一层（owning VM 由 `&Lua` 句柄的 `XRc<LuaInner>` 保证存活——§2：上下文
/// 裸指针 → 带生命周期引用）。`Table::get` 与 `Table::raw_remove` 的收尾同形，于是
/// 「压栈配对的一次 `lua_pop`」契约在本模块只书写一处，调用点是安全读值。
///
/// 前提：`lua` 存活且栈顶恰有一层待读值（由调用点的压栈序列给出）。读取失败时与
/// 旧实现保持一致：错误先行返回、该层仍留在栈上（不额外弹，免得改坏调用方的栈）。
fn take_top(lua: &Lua) -> Result<Value> {
  let value = lua.value_from_stack(-1)?;
  // 栈顶即上一步读出的值，`pop_stack` 恰好回收这一层，与调用点的压栈序列配对。
  pop_stack(lua.state(), 1);
  Ok(value)
}

/// `t[dst] = t[src]`（raw 一格复制，**不触发 metamethod**，进出栈深度相等）。
///
/// 序列：压 `src` 键 → rawget 读出 `t[src]` → 压 `dst` 键 → 复制读出的值 →
/// rawset 消费「dst 键 + 值副本」→ 弹掉读出的值。`Table::raw_insert` 的上移与
/// `Table::raw_remove` 的下移共用这一同构序列。
///
/// # Safety
/// `state` 必须存活且由当前线程驱动；`t` 是其上的**有效表索引**（`with_pushed` 给的
/// 绝对索引）；栈上另有 4 层空位（峰值：src 键 → 读出的值 → dst 键 → 值副本）。
unsafe fn raw_copy_slot(state: *mut LuaState, t: c_int, src: i64, dst: i64) {
  // Safety: 函数头契约给出存活 `state`、有效表索引 `t` 与 4 层空位。六步是一条自平衡
  // 的栈事务（峰值：src 键 → 读出的值 → dst 键 → 值副本），逐步为：数字 src 键净压
  // 一层；rawget 消费该键并压出 `t[src]`（净零）；数字 dst 键净压一层；-2 即其下方的
  // `t[src]`，pushvalue 复制它再净压一层（峰值用满）；rawset 消费「dst 键 + 值副本」
  // 写回 `t[dst]`（raw 路径，不触发 metamethod）；最后弹掉第 2 步读出的 `t[src]`，
  // 栈深回到进入时。
  unsafe {
    lua_pushnumber(state, src as Number);
    lua_rawget(state, t);
    lua_pushnumber(state, dst as Number);
    lua_pushvalue(state, -2);
    lua_rawset(state, t);
    lua_pop(state, 1);
  }
}

/// C trampoline: stack is `[table, key]`; performs `lua_gettable` and leaves
/// the result on top.
///
/// # Safety
/// 仅由 [`protected_table_op`] 在 `lua_pcall` 下调用：`state` 为该帧正在执行的
/// 协程状态，栈布局 `[table, key]` 由 [`Table::get`] 的压栈序列保证。
unsafe extern "C-unwind" fn c_gettable(state: *mut LuaState) -> c_int {
  // Safety: 本函数只经 protected_table_op 在 lua_pcall 下被 VM 调用——`state` 是 pcall 帧内正在
  // 执行的协程状态，栈布局 `[table, key]`（+ 其下闭包槽）由调用方 `Table::get` 的 ensure_stack(3)
  // 与压栈序列保证，故 `lua_gettable(state, 1)` 的槽 1 为有效表索引；返回 1 声明留下结果一槽，
  // 与 gettable 行为一致。
  unsafe {
    lua_gettable(state, 1);
    1
  }
}

/// C trampoline: stack is `[table, key, value]`; performs `lua_settable`.
///
/// # Safety
/// 仅由 [`protected_table_op`] 在 `lua_pcall` 下调用：`state` 为该帧正在执行的
/// 协程状态，栈布局 `[table, key, value]` 由 [`Table::set`] 的压栈序列保证。
unsafe extern "C-unwind" fn c_settable(state: *mut LuaState) -> c_int {
  // Safety: 与 c_gettable 同理——只在 protected_table_op 的 lua_pcall 下运行；调用方 `Table::set`
  // 预留头寸并压入 `[table, key, value]`，槽 1 为有效表；`lua_settable` 消费 key+value，返回 0
  // 声明无结果留下，与 pcall nresults=0 一致。
  unsafe {
    lua_settable(state, 1);
    0
  }
}

/// C trampoline: 栈是 `[table]`；先压一个 nil 作 luaV_objlen 的可写结果
/// 槽，再调 VM 的 `#t` 核心（luaV_objlen，字节码 OP_LEN 同一实现）：
/// 无 `__len` 时直接写回裸边界长，有 `__len` 时调用元方法；元方法返回
/// 非数字由 VM 抛错（经 pcall 呈为状态码）。
///
/// # Safety
/// 仅由 [`protected_table_op`] 在 `lua_pcall` 下调用：`state` 为该帧正在执行的
/// 协程状态，栈布局 `[table]` 由 [`Table::len`] 的压栈序列保证。
unsafe extern "C-unwind" fn c_len(state: *mut LuaState) -> c_int {
  // Safety: 仅在 protected_table_op 的 pcall 内被 VM 调用，`state` 为该帧正在执行的协程；栈为
  // `[table(槽1), nil 结果槽]`——pushnil 占一层且在 `Table::len` 的 ensure_stack(3) 头寸内；
  // `index_2_addr` 对刚验证有效的槽 1 与栈顶返回活 Table*，lua_v_dolen_export 按 VM 契约把结果写进
  // ra 所指槽；返回 1 声明留下 nil 槽改写成的数值结果。
  unsafe {
    lua_pushnil(state); // 结果槽
    let ra = index_2_addr(state, lua_gettop(state));
    let rb = index_2_addr(state, 1);
    lua_v_dolen_export(state, ra, rb);
    1
  }
}

/// 三个受保护表算子只在「trampoline + 栈布局 + 结果数」上不同（原先是三个只差实参的
/// 薄转发包装，§3 合并为单实现 + 算子选择）。
#[derive(Clone, Copy)]
enum TableOp {
  /// 栈顶 `[table, key]`：跑 `lua_gettable`，成功时原位留下 `[result]`。
  /// 头寸由 [`Table::get`] 的 `ensure_stack(state, 3)` 预留。
  Get,
  /// 栈顶 `[table, key, value]`：跑 `lua_settable`，成功时消费三槽、无结果留下。
  /// 头寸由 [`Table::set`] 的 `ensure_stack(state, 4)` 预留。
  Set,
  /// 栈顶 `[table]`：跑 `#t`（luaV_objlen），成功时原位留下 `[number]`。
  /// 头寸由 [`Table::len`] 的 `ensure_stack(state, 3)` 预留。
  Len,
}

impl TableOp {
  /// 该算子的 C-ABI trampoline（均为本模块符合 `lua_CFunction` 契约的实现）。
  fn trampoline(self) -> unsafe extern "C-unwind" fn(*mut LuaState) -> c_int {
    match self {
      TableOp::Get => c_gettable,
      TableOp::Set => c_settable,
      TableOp::Len => c_len,
    }
  }

  /// 该算子的栈帧描述：闭包名（NUL 结尾静态字节串）、实参数、结果数、闭包插入的
  /// 负偏移（落在「刚压入的闭包 + `nargs` 实参」窗口内）。
  fn frame(self) -> (&'static [u8], c_int, c_int, c_int) {
    match self {
      TableOp::Get => (b"ulua-rt-gettable\0", 2, 1, -3),
      TableOp::Set => (b"ulua-rt-settable\0", 3, 0, -4),
      TableOp::Len => (b"ulua-rt-len\0", 1, 1, -2),
    }
  }
}

/// 在 `lua_pcall` 下运行一个表算子：把闭包插到 `nargs` 个实参之下再 pcall，成功
/// 时按各算子摘要留下结果，失败时错误对象留在栈顶并返回非零状态码。
///
/// # Safety
/// `state` 必须存活且由当前线程驱动；栈顶恰有 `op` 所要求的实参槽布局（见
/// [`TableOp`] 各变体摘要，由 [`Table::get`] / [`Table::set`] / [`Table::len`] 的
/// 压栈序列保证），并已按其 `ensure_stack` 预留 closure 槽 + `lua_pcall` 帧头寸。
unsafe fn protected_table_op(state: *mut LuaState, op: TableOp) -> c_int {
  let (name, nargs, nresults, insert) = op.frame();
  // Safety: 调用点是上表三个 `Table` 方法之一，函数头契约给出 `state` 存活、栈顶实参
  // 布局与闭包槽+pcall 头寸；`op.trampoline()` 是本模块符合 lua_CFunction 契约的
  // C-ABI trampoline；`name` 是 `b"..\0"` 静态字节串，`as_ptr().cast()` 指向 NUL 结尾
  // 串（创建闭包时即读取）；`insert`(-3/-4/-2) 落在「刚压闭包 + nargs 实参」窗口内，
  // lua_insert 仅换位、不增减栈深。
  unsafe {
    lua_pushcclosurek(state, Some(op.trampoline()), name.as_ptr().cast(), 0, None);
    lua_insert(state, insert);
    lua_pcall(state, nargs, nresults, 0)
  }
}
