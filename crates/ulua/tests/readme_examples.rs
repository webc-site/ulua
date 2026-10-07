use std::sync::{
  Arc,
  atomic::{AtomicI64, Ordering},
};

use ulua::prelude::*;

#[test]
fn test_readme_direct_execution() -> Result<()> {
  // 直接执行源码
  eval("assert(1 + 1 == 2)")?;

  // 编译源码为二进制字节码
  let bytecode = compile("assert(10 * 20 == 200)")?;
  assert!(!bytecode.is_empty());

  // 直接执行预编译字节码
  eval_bytecode(&bytecode)?;

  Ok(())
}

#[test]
fn test_readme_rust_calls_lua() -> Result<()> {
  let lua = Lua::new();

  // 1. 定义 Lua 函数并由 Rust 获取句柄
  lua
    .load(
      r#"
        function div_rem(n, d)
          return math.floor(n / d), n % d
        end
      "#,
    )
    .exec()?;

  let div_rem: Function = lua.globals().get("div_rem")?;

  // 2. 多参数传递与多返回值接收 (Tuple <-> Lua Multi-Return)
  let (quotient, remainder): (i64, i64) = div_rem.call((17, 5))?;
  assert_eq!((quotient, remainder), (3, 2));

  // 3. 传递序列切片/向量给 Lua
  let sum: Function = lua
    .load(
      r#"
        function(nums)
          local total = 0
          for _, n in ipairs(nums) do total += n end
          return total
        end
      "#,
    )
    .eval()?;

  let total: i64 = sum.call(vec![10, 20, 30])?;
  assert_eq!(total, 60);

  Ok(())
}

#[test]
fn test_readme_lua_calls_rust() -> Result<()> {
  let lua = Lua::new();

  // 1. 注册 Rust 函数：接收多参数、返回多值 (Tuple <-> Lua Multi-Return)
  let split = lua.create_function(|_, (s, sep): (String, String)| {
    let (left, right) = s.split_once(&sep).unwrap_or((&s, ""));
    Ok((left.to_string(), right.to_string()))
  })?;
  lua.globals().set("split", split)?;

  let (a, b): (String, String) = lua.load(r#"split("hello:world", ":")"#).eval()?;
  assert_eq!((a.as_str(), b.as_str()), ("hello", "world"));

  // 2. 状态捕获闭包 (Stateful Closure)
  let counter = Arc::new(AtomicI64::new(0));
  let c = counter.clone();
  let next_id = lua.create_function(move |_, ()| Ok(c.fetch_add(1, Ordering::SeqCst) + 1))?;
  lua.globals().set("next_id", next_id)?;

  lua.load("next_id(); next_id()").exec()?;
  assert_eq!(counter.load(Ordering::SeqCst), 2);

  // 3. 跨语言错误传递 (Rust Err -> Lua pcall)
  let safe_div = lua.create_function(|_, (a, b): (f64, f64)| {
    if b == 0.0 {
      return Err(Error::runtime("division by zero"));
    }
    Ok(a / b)
  })?;
  lua.globals().set("safe_div", safe_div)?;

  let (ok, err_msg): (bool, String) = lua
    .load(
      r#"
        local ok, res = pcall(safe_div, 1, 0)
        return ok, tostring(res)
      "#,
    )
    .eval()?;
  assert!(!ok);
  assert!(err_msg.contains("division by zero"));

  Ok(())
}

struct Player {
  name: String,
  score: i64,
}

impl UserData for Player {
  fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("get_score", |_, this, ()| Ok(this.score));
    methods.add_method_mut("add_score", |_, this, points: i64| {
      this.score += points;
      Ok(())
    });
    methods.add_meta_method("__tostring", |_, this, ()| {
      Ok(format!("Player({}, score={})", this.name, this.score))
    });
  }
}

#[test]
fn test_readme_userdata() -> Result<()> {
  let lua = Lua::new();

  let player = lua.create_userdata(Player {
    name: "Player1".to_string(),
    score: 100,
  })?;
  lua.globals().set("player", player)?;

  lua.load("player:add_score(50)").exec()?;
  let final_score: i64 = lua.load("return player:get_score()").eval()?;
  assert_eq!(final_score, 150);

  let repr: String = lua.load("return tostring(player)").eval()?;
  assert_eq!(repr, "Player(Player1, score=150)");

  Ok(())
}

#[cfg(feature = "typecheck")]
#[test]
fn test_readme_static_type_checking() {
  // 1. 验证符合类型约定的脚本
  let valid_script = "local total: number = 42";
  assert!(check(valid_script).is_ok());

  // 2. 结合宿主环境外部声明 (Definition) 进行校验
  let host_script = "local res = add(10, 20)";
  let defs = "declare function add(a: number, b: number): number";
  assert!(check_with_definitions(host_script, defs).is_ok());

  // 3. 验证类型错误能够被检测出来
  let invalid_script = "local total: number = 'not a number'";
  assert!(check(invalid_script).is_err());
}
