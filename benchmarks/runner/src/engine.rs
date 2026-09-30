//! 评测引擎注册表与统一派发。
//!
//! 设计：全部引擎（exec / compile / analysis 三组）收敛为一个 [`EngineImpl`]
//! 枚举——类型集编译期封闭（每个 variant 对应一个已知引擎），故用 enum match
//! 派发而非 `dyn Trait` 虚分派，单态调用点 + 零 `dyn`。[`EngineSpec`] 是引擎的
//! 静态元数据 + 可选实现，注册表按 `cfg` 在编译期组装，未编译进当前二进制的
//! 后端引擎保留元数据（供 JSON 引擎清单）但不带实现；测量路径统一走
//! [`LiveEngine`]（spec + 实现按值配对），「实测引擎」由类型承载而非运行期
//! unwrap。
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

// ---------------------------------------------------------------------------
// ulua 侧执行（与后端无关，任何 feature 组合下都可实测）。
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
  Ok(Some(format_fvals(values.into_iter().map(|value| match value {
    Value::Nil => FVal::Nil,
    Value::Boolean(b) => FVal::Bool(b),
    Value::Integer(i) => FVal::Int(i),
    Value::Number(f) => FVal::Num(f),
    Value::String(s) => FVal::Str(s.to_string_lossy()),
    other => FVal::Other(format!("{other:?}")),
  }))))
}

// ---------------------------------------------------------------------------
// C 侧执行：按后端 feature 编译，每后端一个模块（模块内保留该后端 Value →
// FVal 的转换）。feature 关闭时模块整体不存在，对应枚举 variant 一并 cfg 掉。
// ---------------------------------------------------------------------------

#[cfg(feature = "engine-luau")]
mod backend_luau {
  use super::{FVal, format_fvals};

  pub(super) fn eval(src: &str, jit: bool) -> Result<Option<String>, String> {
    let lua = mlua_luau::Lua::new();
    lua.enable_jit(jit);
    let values: mlua_luau::MultiValue = lua.load(src).eval().map_err(|e| format!("{e:?}"))?;
    Ok(Some(format_fvals(values.into_iter().map(|value| match value {
      mlua_luau::Value::Nil => FVal::Nil,
      mlua_luau::Value::Boolean(b) => FVal::Bool(b),
      mlua_luau::Value::Integer(i) => FVal::Int(i),
      mlua_luau::Value::Number(f) => FVal::Num(f),
      mlua_luau::Value::String(s) => {
        FVal::Str(String::from_utf8_lossy(&s.as_bytes()).into_owned())
      }
      other => FVal::Other(format!("{other:?}")),
    }))))
  }
}

#[cfg(feature = "engine-luajit")]
mod backend_luajit {
  use super::{FVal, format_fvals};

  pub(super) fn eval(src: &str, jit: bool) -> Result<Option<String>, String> {
    let lua = mlua_luajit::Lua::new();
    // mlua-luajit 无 enable_jit 高层 API，用 Lua 侧 jit.on/off 等价开关。
    let _ = lua.load(if jit { "jit.on()" } else { "jit.off()" }).exec();
    let values: mlua_luajit::MultiValue = lua.load(src).eval().map_err(|e| format!("{e:?}"))?;
    Ok(Some(format_fvals(values.into_iter().map(|value| match value {
      mlua_luajit::Value::Nil => FVal::Nil,
      mlua_luajit::Value::Boolean(b) => FVal::Bool(b),
      mlua_luajit::Value::Integer(i) => FVal::Int(i),
      mlua_luajit::Value::Number(f) => FVal::Num(f),
      mlua_luajit::Value::String(s) => {
        FVal::Str(String::from_utf8_lossy(&s.as_bytes()).into_owned())
      }
      other => FVal::Other(format!("{other:?}")),
    }))))
  }
}

#[cfg(feature = "engine-lua54")]
mod backend_lua54 {
  use super::{FVal, format_fvals};

  pub(super) fn eval(src: &str) -> Result<Option<String>, String> {
    let lua = mlua_lua54::Lua::new();
    let values: mlua_lua54::MultiValue = lua.load(src).eval().map_err(|e| format!("{e:?}"))?;
    Ok(Some(format_fvals(values.into_iter().map(|value| match value {
      mlua_lua54::Value::Nil => FVal::Nil,
      mlua_lua54::Value::Boolean(b) => FVal::Bool(b),
      mlua_lua54::Value::Integer(i) => FVal::Int(i),
      mlua_lua54::Value::Number(f) => FVal::Num(f),
      mlua_lua54::Value::String(s) => {
        FVal::Str(String::from_utf8_lossy(&s.as_bytes()).into_owned())
      }
      other => FVal::Other(format!("{other:?}")),
    }))))
  }
}

/// 全部引擎的封闭集合：variant 与「跑一次被测源码」一一对应，`run` 的 match
/// 即全部派发（无 dyn、无注册表查找）。
///
/// `Ok` 携带「返回值指纹」（见 [`format_fvals`]），无脚本返回值的引擎
/// （解析/编译/类型检查）返回 `Ok(None)`。
#[derive(Clone, Copy)]
pub(crate) enum EngineImpl {
  UluaInterp,
  UluaJit,
  #[cfg(feature = "engine-luau")]
  LuauCppInterp,
  #[cfg(feature = "engine-luau")]
  LuauCppJit,
  #[cfg(feature = "engine-luajit")]
  LuaJitInterp,
  #[cfg(feature = "engine-luajit")]
  LuaJitJit,
  #[cfg(feature = "engine-lua54")]
  Lua54Interp,
  UluaParse,
  UluaCompile,
  #[cfg(feature = "engine-luau")]
  LuauCppCompile,
  AnalysisGlobals,
  AnalysisCheck,
}

impl EngineImpl {
  pub(crate) fn run(self, src: &str) -> Result<Option<String>, String> {
    match self {
      Self::UluaInterp => ulua_eval(src, false),
      Self::UluaJit => ulua_eval(src, true),
      #[cfg(feature = "engine-luau")]
      Self::LuauCppInterp => backend_luau::eval(src, false),
      #[cfg(feature = "engine-luau")]
      Self::LuauCppJit => backend_luau::eval(src, true),
      #[cfg(feature = "engine-luajit")]
      Self::LuaJitInterp => backend_luajit::eval(src, false),
      #[cfg(feature = "engine-luajit")]
      Self::LuaJitJit => backend_luajit::eval(src, true),
      #[cfg(feature = "engine-lua54")]
      Self::Lua54Interp => backend_lua54::eval(src),
      Self::UluaParse => crate::compile::run_parse(src),
      Self::UluaCompile => crate::compile::run_compile(src),
      #[cfg(feature = "engine-luau")]
      Self::LuauCppCompile => crate::compile::run_luau_cpp_compile(src),
      Self::AnalysisGlobals => crate::analysis::run(false, src),
      Self::AnalysisCheck => crate::analysis::run(true, src),
    }
  }
}

/// 引擎静态元数据（JSON 引擎清单与表格列的全部展示字段）。
#[derive(Clone, Copy)]
pub(crate) struct EngineMeta {
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
}

/// 引擎注册条目：元数据 + 可选实现。`engine: None` 表示当前二进制未编译该
/// 引擎（仅保留 JSON 引擎清单与文档用途），测量循环会跳过它。
/// 全 `&'static`/bool 字段，`Copy` 让注册与配对零开销。
#[derive(Clone, Copy)]
pub(crate) struct EngineSpec {
  pub meta: EngineMeta,
  pub engine: Option<EngineImpl>,
}

/// 实测引擎（注册条目 + 实现按值配对）：测量路径只接受本类型，「是否可测」
/// 由构造保证而非运行期断言。
#[derive(Clone, Copy)]
pub(crate) struct LiveEngine {
  pub spec: EngineSpec,
  pub imp: EngineImpl,
}

/// 注册条目便捷构造。
pub(crate) fn spec(meta: EngineMeta, engine: Option<EngineImpl>) -> EngineSpec {
  EngineSpec { meta, engine }
}

/// C 侧引擎占位（当前后端未编译该引擎：保留元数据，`engine: None`）。
pub(crate) fn uncompiled(meta: EngineMeta) -> EngineSpec {
  EngineSpec { meta, engine: None }
}

/// 分组引擎便捷构造（分组引擎 id 即 key，展示字段取 ulua 默认值）。
pub(crate) fn group_spec(
  key: &'static str,
  label: &'static str,
  is_ulua: bool,
  engine: Option<EngineImpl>,
) -> EngineSpec {
  spec(
    EngineMeta {
      id: key,
      key,
      label,
      lang: "Luau (纯 Rust)",
      mode: "interp",
      color: "#0969da",
      is_ulua,
      is_reference: false,
    },
    engine,
  )
}

/// exec 组全部引擎（含当前后端未编译的占位条目，JSON 引擎清单需要全集）。
pub(crate) fn exec_engines() -> Vec<EngineSpec> {
  let mut engines = vec![
    spec(
      EngineMeta {
        id: "ulua",
        key: "ulua",
        label: "ulua",
        lang: "Luau (纯 Rust)",
        mode: "interp",
        color: "#0969da",
        is_ulua: true,
        is_reference: false,
      },
      Some(EngineImpl::UluaInterp),
    ),
    spec(
      EngineMeta {
        id: "ulua_jit",
        key: "ulua-jit",
        label: "ulua (JIT)",
        lang: "Luau (纯 Rust)",
        mode: "jit",
        color: "#0284c7",
        is_ulua: true,
        is_reference: false,
      },
      Some(EngineImpl::UluaJit),
    ),
  ];
  #[cfg(feature = "engine-luau")]
  {
    engines.push(spec(
      EngineMeta {
        id: "mlua_luau",
        key: "mlua/luau",
        label: "mlua/luau",
        lang: "Luau (C++)",
        mode: "interp",
        color: "#1f883d",
        is_ulua: false,
        is_reference: false,
      },
      Some(EngineImpl::LuauCppInterp),
    ));
    engines.push(spec(
      EngineMeta {
        id: "mlua_luau_jit",
        key: "mlua/luau-jit",
        label: "mlua/luau (JIT)",
        lang: "Luau (C++)",
        mode: "jit",
        color: "#d97706",
        is_ulua: false,
        is_reference: false,
      },
      Some(EngineImpl::LuauCppJit),
    ));
  }
  #[cfg(not(feature = "engine-luau"))]
  {
    engines.push(uncompiled(EngineMeta {
      id: "mlua_luau",
      key: "mlua/luau",
      label: "mlua/luau",
      lang: "Luau (C++)",
      mode: "interp",
      color: "#1f883d",
      is_ulua: false,
      is_reference: false,
    }));
    engines.push(uncompiled(EngineMeta {
      id: "mlua_luau_jit",
      key: "mlua/luau-jit",
      label: "mlua/luau (JIT)",
      lang: "Luau (C++)",
      mode: "jit",
      color: "#d97706",
      is_ulua: false,
      is_reference: false,
    }));
  }
  #[cfg(feature = "engine-luajit")]
  {
    engines.push(spec(
      EngineMeta {
        id: "mlua_luajit_interp",
        key: "mlua/luajit-interp",
        label: "LuaJIT (解释)",
        lang: "LuaJIT 2.1",
        mode: "interp",
        color: "#6366f1",
        is_ulua: false,
        is_reference: true,
      },
      Some(EngineImpl::LuaJitInterp),
    ));
    engines.push(spec(
      EngineMeta {
        id: "mlua_luajit",
        key: "mlua/luajit",
        label: "LuaJIT (JIT)",
        lang: "LuaJIT 2.1",
        mode: "jit",
        color: "#8250df",
        is_ulua: false,
        is_reference: true,
      },
      Some(EngineImpl::LuaJitJit),
    ));
  }
  #[cfg(not(feature = "engine-luajit"))]
  {
    engines.push(uncompiled(EngineMeta {
      id: "mlua_luajit_interp",
      key: "mlua/luajit-interp",
      label: "LuaJIT (解释)",
      lang: "LuaJIT 2.1",
      mode: "interp",
      color: "#6366f1",
      is_ulua: false,
      is_reference: true,
    }));
    engines.push(uncompiled(EngineMeta {
      id: "mlua_luajit",
      key: "mlua/luajit",
      label: "LuaJIT (JIT)",
      lang: "LuaJIT 2.1",
      mode: "jit",
      color: "#8250df",
      is_ulua: false,
      is_reference: true,
    }));
  }
  #[cfg(feature = "engine-lua54")]
  engines.push(spec(
    EngineMeta {
      id: "mlua_lua54",
      key: "mlua/lua5.4",
      label: "Lua 5.4",
      lang: "Lua 5.4",
      mode: "interp",
      color: "#64748b",
      is_ulua: false,
      is_reference: true,
    },
    Some(EngineImpl::Lua54Interp),
  ));
  #[cfg(not(feature = "engine-lua54"))]
  engines.push(uncompiled(EngineMeta {
    id: "mlua_lua54",
    key: "mlua/lua5.4",
    label: "Lua 5.4",
    lang: "Lua 5.4",
    mode: "interp",
    color: "#64748b",
    is_ulua: false,
    is_reference: true,
  }));
  engines
}

/// 当前编译选中的引擎后端名（三选一互斥，见 main.rs 守卫）。
pub(crate) const fn active_backend_name() -> &'static str {
  if cfg!(feature = "engine-luau") {
    "engine-luau"
  } else if cfg!(feature = "engine-luajit") {
    "engine-luajit"
  } else {
    "engine-lua54"
  }
}
