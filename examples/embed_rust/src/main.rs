//! 经典的 mlua 风格高层 API 实战。
//!
//! `ulua-rt` 镜像 mlua 的公开接口，所以读起来与 mlua 程序完全一致 ——
//! 只有 crate 名不同。运行方式：
//!
//!     cargo run -p ulua-example-embed-rust
//!
//! 演示：注册 Rust 函数、捕获闭包、从 Rust 调用 Lua、table 往返、
//! `Vec` <-> table、错误/panic 桥接、带方法 + `__add` 元方法的 `UserData`。

use std::sync::{
  atomic::{AtomicI64, Ordering},
  Arc,
};

// 所需的一切都来自 prelude，mlua 风格。
use ulua::prelude::*;

/// 闭包计数器被 Lua 调用的次数。
const CALLS: usize = 3;

/// 在 Lua 里 `pcall(call)` 并把捕获到的错误值 `tostring` 成字符串返回。
fn pcall_as_string(lua: &Lua, call: &str) -> Result<String> {
  lua
    .load(format!(
      "local ok, err = pcall({call}); return tostring(err)"
    ))
    .eval()
}

/// 暴露给 Lua 作为 userdata 的小 2D 向量。
struct Vec2 {
  x: f64,
  y: f64,
}

impl UserData for Vec2 {
  fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
    // obj:magnitude()
    methods.add_method("magnitude", |_, this, ()| {
      Ok((this.x * this.x + this.y * this.y).sqrt())
    });
    // obj:scale(k) 原地修改
    methods.add_method_mut("scale", |_, this, k: f64| {
      this.x *= k;
      this.y *= k;
      Ok(())
    });
    // v + n  ->  __add(v, n)  ->  x + y + n
    methods.add_meta_method("__add", |_, this, n: f64| Ok(this.x + this.y + n));
  }
}

fn main() -> Result<()> {
  let lua = Lua::new();

  // 1. 注册 Rust 函数并从 Lua 调用。
  let add = lua.create_function(|_, (a, b): (i64, i64)| Ok(a + b))?;
  lua.globals().set("add", add)?;
  let sum: i64 = lua.load("return add(2, 3)").eval()?;
  println!("add(2, 3) = {sum}");
  assert_eq!(sum, 5);

  // 2. 捕获闭包（与 Rust 共享的计数器）。
  let counter = Arc::new(AtomicI64::new(0));
  let c = Arc::clone(&counter);
  let inc = lua.create_function(move |_, ()| Ok(c.fetch_add(1, Ordering::SeqCst) + 1))?;
  lua.globals().set("inc", inc)?;
  lua.load("inc(); ".repeat(CALLS)).exec()?;
  let seen = counter.load(Ordering::SeqCst);
  println!("counter after {CALLS} calls = {seen}");
  assert_eq!(seen, CALLS as i64);

  // 3. 从 Rust 调用 Lua 函数。
  let doubler: Function = lua.load("return function(x) return x * 2 end").eval()?;
  let doubled: i64 = doubler.call(21)?;
  println!("doubler(21) = {doubled}");
  assert_eq!(doubled, 42);

  // 4. Table：set/get、嵌套、Vec<T> <-> table。
  let t = lua.create_table();
  t.set("name", "ulua")?;
  t.set("answer", 42i64)?;
  let name: String = t.get("name")?;
  let answer: i64 = t.get("answer")?;
  println!("table: name={name:?}, answer={answer}");

  lua.globals().set("nums", vec![1i64, 2, 3, 4])?;
  let total: i64 = lua
    .load("local s = 0; for _, n in ipairs(nums) do s = s + n end; return s")
    .eval()?;
  println!("sum of nums = {total}");
  assert_eq!(total, 10);
  let back: Vec<i64> = lua.load("return {10, 20, 30}").eval()?;
  println!("Lua sequence -> Vec = {back:?}");

  // 5. 错误桥接：Rust 的 Err 可被 Lua 的 pcall 捕获。
  let boom = lua.create_function(|_, ()| -> Result<()> { Err(Error::runtime("kaboom")) })?;
  lua.globals().set("boom", boom)?;
  let caught = pcall_as_string(&lua, "boom")?;
  println!("pcall caught: {caught}");
  assert!(caught.contains("kaboom"));

  // ……真正的 Rust panic 同样可捕获（变成 Lua 错误，不是 abort）。
  let panicky = lua.create_function(|_, ()| -> Result<()> { panic!("oh no") })?;
  lua.globals().set("panicky", panicky)?;
  let panic_msg = pcall_as_string(&lua, "panicky")?;
  println!("pcall caught panic: {panic_msg}");
  assert!(panic_msg.contains("oh no"));

  // Lua 的 `error(...)` 在 Rust 侧以 Err 出现。
  let err = lua
    .load("error('from lua')")
    .exec()
    .expect_err("脚本应报错");
  println!("Lua error surfaced as Err: {err}");

  // 6. 带方法与元方法的 UserData。
  let v = lua.create_userdata(Vec2 { x: 3.0, y: 4.0 })?;
  lua.globals().set("v", v)?;
  let mag: f64 = lua.load("return v:magnitude()").eval()?;
  println!("v:magnitude() = {mag}");
  assert_eq!(mag, 5.0);
  let scaled: f64 = lua.load("v:scale(2); return v:magnitude()").eval()?;
  println!("after v:scale(2), magnitude = {scaled}");
  assert_eq!(scaled, 10.0);
  let added: f64 = lua.load("return v + 100").eval()?; // __add: 6 + 8 + 100
  println!("v + 100 (via __add) = {added}");
  assert_eq!(added, 114.0);

  println!("\nAll examples passed.");
  Ok(())
}
