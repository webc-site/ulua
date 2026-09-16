//! The canonical, mlua-style high-level API in action.
//!
//! `ulua-rt` mirrors mlua's public interface, so this reads exactly like an
//! mlua program — only the crate name differs. Run it with:
//!
//!     cargo run -p ulua-example-embed-rust
//!
//! It demonstrates: registering a Rust function, a capturing closure, calling
//! Lua from Rust, table round-trips, `Vec` <-> table, error/panic bridging,
//! and `UserData` with methods + a `__add` meta-method.

use std::sync::{
  atomic::{AtomicI64, Ordering},
  Arc,
};

// Everything you need comes from the prelude, mlua-style.
use ulua::prelude::*;

/// A small 2D vector exposed to Lua as userdata.
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
    // obj:scale(k) mutates in place
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

  // 1. Register a Rust function and call it from Lua.
  let add = lua.create_function(|_, (a, b): (i64, i64)| Ok(a + b))?;
  lua.globals().set("add", add)?;
  let sum: i64 = lua.load("return add(2, 3)").eval()?;
  println!("add(2, 3) = {sum}");
  assert_eq!(sum, 5);

  // 2. A capturing closure (a counter shared with Rust).
  let counter = Arc::new(AtomicI64::new(0));
  let c = counter.clone();
  let inc = lua.create_function(move |_, ()| Ok(c.fetch_add(1, Ordering::SeqCst) + 1))?;
  lua.globals().set("inc", inc)?;
  lua.load("inc(); inc(); inc()").exec()?;
  println!("counter after 3 calls = {}", counter.load(Ordering::SeqCst));
  assert_eq!(counter.load(Ordering::SeqCst), 3);

  // 3. Call a Lua function from Rust.
  let doubler: Function = lua.load("return function(x) return x * 2 end").eval()?;
  let doubled: i64 = doubler.call(21)?;
  println!("doubler(21) = {doubled}");
  assert_eq!(doubled, 42);

  // 4. Tables: set/get, nested, and Vec<T> <-> table.
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

  // 5. Error bridging: a Rust Err is catchable by Lua's pcall.
  let boom = lua.create_function(|_, ()| -> Result<()> { Err(Error::runtime("kaboom")) })?;
  lua.globals().set("boom", boom)?;
  let caught: String = lua
    .load("local ok, err = pcall(boom); return tostring(err)")
    .eval()?;
  println!("pcall caught: {caught}");
  assert!(caught.contains("kaboom"));

  // ...and so is a genuine Rust panic (it becomes a Lua error, NOT an abort).
  let panicky = lua.create_function(|_, ()| -> Result<()> { panic!("oh no") })?;
  lua.globals().set("panicky", panicky)?;
  let panic_msg: String = lua
    .load("local ok, err = pcall(panicky); return tostring(err)")
    .eval()?;
  println!("pcall caught panic: {panic_msg}");
  assert!(panic_msg.contains("oh no"));

  // A Lua `error(...)` surfaces as Err on the Rust side.
  match lua.load("error('from lua')").exec() {
    Ok(()) => unreachable!(),
    Err(e) => println!("Lua error surfaced as Err: {e}"),
  }

  // 6. UserData with methods and a meta-method.
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
