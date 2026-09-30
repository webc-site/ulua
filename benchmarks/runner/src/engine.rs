//! 评测引擎抽象与注册表。
//!
//! 设计：`BenchEngine` trait 把「跑一次被测源码」抽象成单元结构体实现，每引擎
//! 一个类型，构造逻辑（state 建立、JIT 开关）收敛在各自实现内；`EngineSpec`
//! 是引擎的静态元数据 + 实现引用，注册表按 `cfg` 在编译期组装，未编译进当前
//! 二进制的后端引擎保留元数据（供 JSON 引擎清单）但不带实现。
//!
//! 后端互斥三选一（`engine-luau` / `engine-luajit` / `engine-lua54`，见 Cargo.toml
//! 与 main.rs 顶部守卫）：mlua 0.12 的 ffi 扁平 re-export 使同一二进制只能共链
//! 一种 C Lua；ulua 自家引擎与后端无关，任何后端下都可测。
//!
//! 跨引擎一致性：脚本返回值先归一为 [`FVal`] 再经 [`format_fvals`] 统一格式化
//! （f64 用最短往返 `Display`，确定性），三个 C 后端与 ulua 侧共用同一格式定义。

use ulua::{Lua, Value};

/// 返回值指纹的引擎无关中间表示（各后端 Value 形状同构但类型不同构）。
pub(crate) enum FVal {
  Nil,
  Bool(bool),
  Int(i64),
  Num(f64),
  Str(String),
  Other(String),
}

/// 统一指纹格式化（Rust 侧完成而非走各引擎 tostring，避免格式差异污染比对）。
pub(crate) fn format_fvals(vals: impl IntoIterator<Item = FVal>) -> String {
  vals
    .into_iter()
    .map(|value| match value {
      FVal::Nil => "nil".to_owned(),
      FVal::Bool(b) => b.to_string(),
      FVal::Int(i) => i.to_string(),
      FVal::Num(f) => f.to_string(),
      FVal::Str(s) => s,
      FVal::Other(o) => o,
    })
    .collect::<Vec<_>>()
    .join(",")
}

/// 单引擎执行器：完整运行一次被测源码。
///
/// `Ok` 携带「返回值指纹」（见 [`format_fvals`]），无脚本返回值的引擎
/// （解析/编译/类型检查）返回 `Ok(None)`。
pub trait BenchEngine: Sync {
  fn run(&self, src: &str) -> Result<Option<String>, String>;
}

/// 引擎静态元数据 + 实现。`engine: None` 表示当前二进制未编译该引擎
/// （仅保留 JSON 引擎清单与文档用途），测量循环会跳过它。
pub struct EngineSpec {
  pub id: &'static str,
  pub key: &'static str,
  pub label: &'static str,
  pub lang: &'static str,
  /// "interp" | "jit"，纯展示字段。
  pub mode: &'static str,
  pub color: &'static str,
  /// 是否 ulua 侧引擎（`--alloc` 分配计数只覆盖 ulua 侧）。
  pub is_ulua: bool,
  /// 是否第三方参考基线（纯展示字段，网页侧区分阵营用）。
  pub is_reference: bool,
  pub engine: Option<&'static dyn BenchEngine>,
}

// ---------------------------------------------------------------------------
// ulua 侧引擎（与后端无关，任何 feature 组合下都可实测）。
// ---------------------------------------------------------------------------

/// `Lua::new()` 的内置库注册开销对所有被测引擎完全对称，比值不受影响，而
/// 「开一个 state 跑一段脚本」正是嵌入方的真实成本，故刻意留在计时窗口内。
/// 脚本源码在扫描阶段一次性读入内存，文件 I/O 与进程创建都在窗口之外。
fn ulua_eval(src: &str, jit: bool) -> Result<Option<String>, String> {
  let lua = Lua::new();
  if jit {
    lua.enable_jit(true).map_err(|e| format!("{e:?}"))?;
  }
  let values: ulua::MultiValue = lua.load(src).eval().map_err(|e| format!("{e:?}"))?;
  Ok(Some(format_fvals(values.into_iter().map(
    |value| match value {
      Value::Nil => FVal::Nil,
      Value::Boolean(b) => FVal::Bool(b),
      Value::Integer(i) => FVal::Int(i),
      Value::Number(f) => FVal::Num(f),
      Value::String(s) => FVal::Str(s.to_string_lossy()),
      other => FVal::Other(format!("{other:?}")),
    },
  ))))
}

struct UluaInterp;

impl BenchEngine for UluaInterp {
  fn run(&self, src: &str) -> Result<Option<String>, String> {
    ulua_eval(src, false)
  }
}

struct UluaJit;

impl BenchEngine for UluaJit {
  fn run(&self, src: &str) -> Result<Option<String>, String> {
    ulua_eval(src, true)
  }
}

static ULUA_INTERP: UluaInterp = UluaInterp;
static ULUA_JIT: UluaJit = UluaJit;

// ---------------------------------------------------------------------------
// C 侧引擎：按后端 feature 编译，每后端一个模块（模块内保留该后端 Value →
// FVal 的 6 行转换）。feature 关闭时模块整体不存在，注册表以 `engine: None`
// 占位保留元数据。
// ---------------------------------------------------------------------------

#[cfg(feature = "engine-luau")]
mod backend_luau {
  use super::{BenchEngine, FVal, format_fvals};

  fn eval(src: &str, jit: bool) -> Result<Option<String>, String> {
    let lua = mlua_luau::Lua::new();
    lua.enable_jit(jit);
    let values: mlua_luau::MultiValue = lua.load(src).eval().map_err(|e| format!("{e:?}"))?;
    Ok(Some(format_fvals(values.into_iter().map(
      |value| match value {
        mlua_luau::Value::Nil => FVal::Nil,
        mlua_luau::Value::Boolean(b) => FVal::Bool(b),
        mlua_luau::Value::Integer(i) => FVal::Int(i),
        mlua_luau::Value::Number(f) => FVal::Num(f),
        mlua_luau::Value::String(s) => {
          FVal::Str(String::from_utf8_lossy(&s.as_bytes()).into_owned())
        }
        other => FVal::Other(format!("{other:?}")),
      },
    ))))
  }

  pub(super) struct LuauCppInterp;

  impl BenchEngine for LuauCppInterp {
    fn run(&self, src: &str) -> Result<Option<String>, String> {
      eval(src, false)
    }
  }

  pub(super) struct LuauCppJit;

  impl BenchEngine for LuauCppJit {
    fn run(&self, src: &str) -> Result<Option<String>, String> {
      eval(src, true)
    }
  }

  pub(super) static LUAU_CPP_INTERP: LuauCppInterp = LuauCppInterp;
  pub(super) static LUAU_CPP_JIT: LuauCppJit = LuauCppJit;
}

#[cfg(feature = "engine-luajit")]
mod backend_luajit {
  use super::{BenchEngine, FVal, format_fvals};

  fn eval(src: &str, jit: bool) -> Result<Option<String>, String> {
    let lua = mlua_luajit::Lua::new();
    if jit {
      let _ = lua.load("jit.on()").exec();
    } else {
      let _ = lua.load("jit.off()").exec();
    }
    let values: mlua_luajit::MultiValue = lua.load(src).eval().map_err(|e| format!("{e:?}"))?;
    Ok(Some(format_fvals(values.into_iter().map(
      |value| match value {
        mlua_luajit::Value::Nil => FVal::Nil,
        mlua_luajit::Value::Boolean(b) => FVal::Bool(b),
        mlua_luajit::Value::Integer(i) => FVal::Int(i),
        mlua_luajit::Value::Number(f) => FVal::Num(f),
        mlua_luajit::Value::String(s) => {
          FVal::Str(String::from_utf8_lossy(&s.as_bytes()).into_owned())
        }
        other => FVal::Other(format!("{other:?}")),
      },
    ))))
  }

  pub(super) struct LuaJitInterp;

  impl BenchEngine for LuaJitInterp {
    fn run(&self, src: &str) -> Result<Option<String>, String> {
      eval(src, false)
    }
  }

  pub(super) struct LuaJitJit;

  impl BenchEngine for LuaJitJit {
    fn run(&self, src: &str) -> Result<Option<String>, String> {
      eval(src, true)
    }
  }

  pub(super) static LUAJIT_INTERP: LuaJitInterp = LuaJitInterp;
  pub(super) static LUAJIT_JIT: LuaJitJit = LuaJitJit;
}

#[cfg(feature = "engine-lua54")]
mod backend_lua54 {
  use super::{BenchEngine, FVal, format_fvals};

  fn eval(src: &str) -> Result<Option<String>, String> {
    let lua = mlua_lua54::Lua::new();
    let values: mlua_lua54::MultiValue = lua.load(src).eval().map_err(|e| format!("{e:?}"))?;
    Ok(Some(format_fvals(values.into_iter().map(
      |value| match value {
        mlua_lua54::Value::Nil => FVal::Nil,
        mlua_lua54::Value::Boolean(b) => FVal::Bool(b),
        mlua_lua54::Value::Integer(i) => FVal::Int(i),
        mlua_lua54::Value::Number(f) => FVal::Num(f),
        mlua_lua54::Value::String(s) => {
          FVal::Str(String::from_utf8_lossy(&s.as_bytes()).into_owned())
        }
        other => FVal::Other(format!("{other:?}")),
      },
    ))))
  }

  pub(super) struct Lua54Interp;

  impl BenchEngine for Lua54Interp {
    fn run(&self, src: &str) -> Result<Option<String>, String> {
      eval(src)
    }
  }

  pub(super) static LUA54_INTERP: Lua54Interp = Lua54Interp;
}

// ---------------------------------------------------------------------------
// 注册表：便捷构造与组装函数（编译期 cfg 决定哪些引擎带实现）。
// ---------------------------------------------------------------------------

/// 便捷构造（元数据全字段 + 实现）。
#[allow(clippy::too_many_arguments)]
pub fn spec(
  id: &'static str,
  key: &'static str,
  label: &'static str,
  lang: &'static str,
  mode: &'static str,
  color: &'static str,
  is_ulua: bool,
  is_reference: bool,
  engine: Option<&'static dyn BenchEngine>,
) -> EngineSpec {
  EngineSpec {
    id,
    key,
    label,
    lang,
    mode,
    color,
    is_ulua,
    is_reference,
    engine,
  }
}

/// C 侧引擎占位（当前后端未编译该引擎：保留元数据，`engine: None`）。
fn uncompiled(
  id: &'static str,
  key: &'static str,
  label: &'static str,
  lang: &'static str,
  mode: &'static str,
  color: &'static str,
) -> EngineSpec {
  spec(id, key, label, lang, mode, color, false, true, None)
}

/// exec 组全部引擎（含当前后端未编译的占位条目，JSON 引擎清单需要全集）。
pub fn exec_engines() -> Vec<EngineSpec> {
  let mut engines = vec![
    spec(
      "ulua",
      "ulua",
      "ulua",
      "Luau (纯 Rust)",
      "interp",
      "#0969da",
      true,
      false,
      Some(&ULUA_INTERP),
    ),
    spec(
      "ulua_jit",
      "ulua-jit",
      "ulua (JIT)",
      "Luau (纯 Rust)",
      "jit",
      "#0284c7",
      true,
      false,
      Some(&ULUA_JIT),
    ),
  ];
  #[cfg(feature = "engine-luau")]
  {
    engines.push(spec(
      "mlua_luau",
      "mlua/luau",
      "mlua/luau",
      "Luau (C++)",
      "interp",
      "#1f883d",
      false,
      false,
      Some(&backend_luau::LUAU_CPP_INTERP),
    ));
    engines.push(spec(
      "mlua_luau_jit",
      "mlua/luau-jit",
      "mlua/luau (JIT)",
      "Luau (C++)",
      "jit",
      "#d97706",
      false,
      false,
      Some(&backend_luau::LUAU_CPP_JIT),
    ));
  }
  #[cfg(not(feature = "engine-luau"))]
  {
    engines.push(uncompiled(
      "mlua_luau",
      "mlua/luau",
      "mlua/luau",
      "Luau (C++)",
      "interp",
      "#1f883d",
    ));
    engines.push(uncompiled(
      "mlua_luau_jit",
      "mlua/luau-jit",
      "mlua/luau (JIT)",
      "Luau (C++)",
      "jit",
      "#d97706",
    ));
  }
  #[cfg(feature = "engine-luajit")]
  {
    engines.push(spec(
      "mlua_luajit_interp",
      "mlua/luajit-interp",
      "LuaJIT (解释)",
      "LuaJIT 2.1",
      "interp",
      "#6366f1",
      false,
      true,
      Some(&backend_luajit::LUAJIT_INTERP),
    ));
    engines.push(spec(
      "mlua_luajit",
      "mlua/luajit",
      "LuaJIT (JIT)",
      "LuaJIT 2.1",
      "jit",
      "#8250df",
      false,
      true,
      Some(&backend_luajit::LUAJIT_JIT),
    ));
  }
  #[cfg(not(feature = "engine-luajit"))]
  {
    engines.push(uncompiled(
      "mlua_luajit_interp",
      "mlua/luajit-interp",
      "LuaJIT (解释)",
      "LuaJIT 2.1",
      "interp",
      "#6366f1",
    ));
    engines.push(uncompiled(
      "mlua_luajit",
      "mlua/luajit",
      "LuaJIT (JIT)",
      "LuaJIT 2.1",
      "jit",
      "#8250df",
    ));
  }
  #[cfg(feature = "engine-lua54")]
  engines.push(spec(
    "mlua_lua54",
    "mlua/lua5.4",
    "Lua 5.4",
    "Lua 5.4",
    "interp",
    "#64748b",
    false,
    true,
    Some(&backend_lua54::LUA54_INTERP),
  ));
  #[cfg(not(feature = "engine-lua54"))]
  engines.push(uncompiled(
    "mlua_lua54",
    "mlua/lua5.4",
    "Lua 5.4",
    "Lua 5.4",
    "interp",
    "#64748b",
  ));
  engines
}

/// 分组引擎便捷构造（分组引擎 id 即 key；`engine: None` 表示当前后端未编译）。
pub fn spec_public(
  key: &'static str,
  label: &'static str,
  is_ulua: bool,
  engine: Option<&'static dyn BenchEngine>,
) -> EngineSpec {
  EngineSpec {
    id: key,
    key,
    label,
    lang: "Luau (纯 Rust)",
    mode: "interp",
    color: "#0969da",
    is_ulua,
    is_reference: false,
    engine,
  }
}

/// 当前编译选中的引擎后端名（三选一互斥，见 main.rs 守卫）。
pub const fn active_backend_name() -> &'static str {
  if cfg!(feature = "engine-luau") {
    "engine-luau"
  } else if cfg!(feature = "engine-luajit") {
    "engine-luajit"
  } else {
    "engine-lua54"
  }
}
