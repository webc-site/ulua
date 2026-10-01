//! `cpp/tests/require`（`RequireByString.test.cpp` 语料）的入门移植集：以内存
//! 文件系统为夹具跑通 require-by-string 全链路（导航 → 缓存 → 编译 → 沙箱线程
//! 执行），对照 cpp 语义的代表性子集。
//!
//! 语料在内存中逐字复刻 cpp 夹具文件内容（`without_config/dependency.luau`、
//! `module.luau`、`lua_dependency.lua` 与一个 `.luaurc` 别名目录），宿主面即
//! [`RequireHost`] trait 的最小实现——导航语义逐行镜像 `cpp/CLI/src/
//! VfsNavigator.cpp`（`getRealPath` 后缀扫描、`toParent` 的斜杠计数、
//! `getConfigPath` 的后缀剥离），避免在测试侧重复文件系统 I/O 依赖。
//!
//! 覆盖：相对路径 require、模块内嵌套 require、子组件缺失报错、非法前缀报错、
//! 非允许上下文（chunkname 判定）报错、`.luaurc` 别名解析、缓存同一性、
//! `.lua` 扩展名加载。
//!
//! 未覆盖（留 gap，多数已由 ulua-cli-test 的 repl 夹具语料套件覆盖）：
//! export 关键字循环依赖占位、`.config.luau`、配置歧义、proxyrequire、
//! 非 UTF-8 路径字节语义。

use std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  str::from_utf8,
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_require::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
  },
  functions::luaopen_require::luaopen_require,
  records::navigation_context::RequireHost,
};
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_isstring::lua_isstring, lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs,
    lua_l_sandboxthread::lua_l_sandboxthread, lua_mainthread::lua_mainthread,
    lua_newthread::lua_newthread, lua_pcall::lua_pcall, lua_xmove::lua_xmove, luau_load::luau_load,
  },
  macros::lua_l_error::luaL_error,
  records::{lua_state::LuaState, lua_state_guard::LuaStateGuard},
};

/// cpp `kSuffixes` / `kInitSuffixes`（VfsNavigator.cpp:14 与 getModulePath 上方）。
const K_SUFFIXES: [&str; 2] = [".luau", ".lua"];
const K_INIT_SUFFIXES: [&str; 2] = ["/init.luau", "/init.lua"];

/// 虚拟文件系统（内存语料）：模块文件 + 目录集 + 配置文件（不入模块集）。
#[derive(Default)]
struct Tree {
  files: HashMap<String, &'static str>,
  dirs: HashSet<String>,
  configs: HashMap<String, &'static str>,
}

impl Tree {
  /// 注册模块文件并补齐全部祖先目录（虚拟路径一律 '/' 分隔、绝对）。
  fn add_file(&mut self, path: &'static str, source: &'static str) {
    self.files.insert(path.to_string(), source);
    self.add_ancestors(path);
  }

  fn add_config(&mut self, path: &str, contents: &'static str) {
    self.configs.insert(path.to_string(), contents);
  }

  fn add_ancestors(&mut self, path: &str) {
    let mut cursor = path;
    while let Some(idx) = cursor.rfind('/') {
      cursor = &cursor[..idx];
      if cursor.is_empty() {
        self.dirs.insert("/".to_string());
        break;
      }
      self.dirs.insert(cursor.to_string());
    }
  }
}

/// 内存 require 宿主：vfs 游标（modulePath 含当前模块自身组件、无扩展名，
/// realPath 为解析出的实体文件或其所在目录），镜像 cpp `VfsNavigator` 状态对。
struct MemHost {
  tree: Tree,
  module: RefCell<String>,
  file: RefCell<Option<String>>,
}

impl MemHost {
  fn new() -> Self {
    let mut tree = Tree::default();
    // 逐字对照 cpp/tests/require/without_config/{dependency,module}.luau 与
    // lua_dependency.lua。
    tree.add_file(
      "/require/without_config/dependency.luau",
      "return {\"result from dependency\"}",
    );
    tree.add_file(
      "/require/without_config/module.luau",
      "local result = require(\"./dependency\")\nresult[#result+1] = \"required into module\"\nreturn result",
    );
    tree.add_file(
      "/require/without_config/lua_dependency.lua",
      "return {\"result from lua_dependency\"}",
    );
    // 别名语料：config 目录 + .luaurc（对照 config_tests/with_config 的形态）
    tree.add_file(
      "/require/config/alias_requirer.luau",
      "return require(\"@dep\")",
    );
    tree.add_file(
      "/require/config/dependency.luau",
      "return {\"result from config dependency\"}",
    );
    tree.add_config(
      "/require/config/.luaurc",
      "{ \"aliases\": { \"dep\": \"./dependency\" } }",
    );
    Self {
      tree,
      module: RefCell::new("/require".to_string()),
      file: RefCell::new(None),
    }
  }

  /// cpp `normalizePath` 的受限形态：本夹具一律为 '/' 分隔绝对路径，处理
  /// `.`/`..`/空段即可（与 cli-lib 版逐语义一致）。
  fn normalize(path: &str) -> String {
    let mut stack: Vec<&str> = Vec::new();
    for component in path.split('/') {
      match component {
        "" | "." => {}
        ".." => {
          stack.pop();
        }
        other => stack.push(other),
      }
    }
    let mut out = String::with_capacity(path.len());
    for component in stack {
      out.push('/');
      out.push_str(component);
    }
    if out.is_empty() {
      out.push('/');
    }
    out
  }

  /// 镜像 cpp `getRealPath`（VfsNavigator.cpp:22-69）：常规后缀 → 目录（含
  /// init 后缀）扫描，多义返回 `Ambiguous`，未命中 `NotFound`；成功返回
  /// `modulePath + suffix`（纯目录时 suffix 为空，real 即目录路径）。
  fn get_real_path(&self, module: &str) -> Result<String, NavigateResult> {
    let last_component = module.rsplit('/').next().unwrap_or(module);
    let mut found = false;
    let mut suffix = String::new();

    if last_component != "init" {
      for potential in K_SUFFIXES {
        if self
          .tree
          .files
          .contains_key(&format!("{module}{potential}"))
        {
          if found {
            return Err(NavigateResult::Ambiguous);
          }
          suffix = potential.to_string();
          found = true;
        }
      }
    }
    if self.tree.dirs.contains(module) {
      if found {
        return Err(NavigateResult::Ambiguous);
      }
      for potential in K_INIT_SUFFIXES {
        if self
          .tree
          .files
          .contains_key(&format!("{module}{potential}"))
        {
          if found {
            return Err(NavigateResult::Ambiguous);
          }
          suffix = potential.to_string();
          found = true;
        }
      }
      // cpp：目录存在本身即 found（init 缺失时 real = 目录路径）
      found = true;
    }

    if found {
      Ok(format!("{module}{suffix}"))
    } else {
      Err(NavigateResult::NotFound)
    }
  }

  /// 镜像 cpp `updateRealPaths` 的单路形态：刷新 file 游标并回报状态。
  fn update(&self, status: Result<String, NavigateResult>) -> NavigateResult {
    match status {
      Ok(real) => {
        *self.file.borrow_mut() = Some(real);
        NavigateResult::Success
      }
      Err(status) => status,
    }
  }

  /// 复位到绝对模块路径（cpp `resetToPath` 的绝对分支）：剥模块后缀得
  /// modulePath，再 `updateRealPaths`。
  fn reset_to_path(&self, path: &str) -> NavigateResult {
    let module = Self::normalize(Self::strip_module_suffix(path));
    *self.module.borrow_mut() = module.clone();
    let status = self.get_real_path(&module);
    self.update(status)
  }

  /// 镜像 cpp `getModulePath` 的后缀剥离（init 后缀先、常规后缀后，均只剥一次）。
  fn strip_module_suffix(path: &str) -> &str {
    for suffix in K_INIT_SUFFIXES {
      if let Some(base) = path.strip_suffix(suffix) {
        return base;
      }
    }
    for suffix in K_SUFFIXES {
      if let Some(base) = path.strip_suffix(suffix) {
        return base;
      }
    }
    path
  }

  /// 镜像 cpp `getConfigPath`：对 realPath 剥模块后缀后拼接配置文件名。
  fn config_path(&self, filename: &str) -> Option<String> {
    let file = self.file.borrow();
    let directory = Self::strip_module_suffix(file.as_deref()?);
    Some(format!("{directory}/{filename}"))
  }
}

impl RequireHost for MemHost {
  fn is_require_allowed(&self, requirer_chunkname: &[u8]) -> bool {
    // cpp ReplRequirer 同款判定（`=stdin` 或 `@chunkname`）
    requirer_chunkname == b"=stdin" || requirer_chunkname.first() == Some(&b'@')
  }

  fn reset_to_requirer(&self, requirer_chunkname: &[u8]) -> NavigateResult {
    // 镜像 cpp ReplRequirer.cpp `reset`：`=stdin` → resetToStdIn（modulePath
    // 置 "./stdin" 占位，不扫盘）；`@绝对路径` → resetToPath(去掉 '@')。
    if requirer_chunkname == b"=stdin" {
      *self.module.borrow_mut() = "/require/stdin".to_string();
      *self.file.borrow_mut() = None;
      return NavigateResult::Success;
    }
    let Some(chunkname) = requirer_chunkname.strip_prefix(b"@".as_slice()) else {
      return NavigateResult::NotFound;
    };
    let Ok(chunkname) = from_utf8(chunkname) else {
      return NavigateResult::NotFound;
    };
    if !chunkname.starts_with('/') {
      return NavigateResult::NotFound;
    }
    self.reset_to_path(chunkname)
  }

  fn jump_to_alias(&self, alias_path: &[u8]) -> NavigateResult {
    // cpp repl `jumpToAlias`：仅接受绝对路径
    let Ok(path) = from_utf8(alias_path) else {
      return NavigateResult::NotFound;
    };
    if !path.starts_with('/') {
      return NavigateResult::NotFound;
    }
    self.reset_to_path(path)
  }

  fn to_parent(&self) -> NavigateResult {
    // 镜像 cpp `VfsNavigator::toParent`：绝对路径根与「仅一枚斜杠」处不可再上；
    // 上移无歧义，Ambiguous 降级为 Success（realPath 保持原值）。
    let module = self.module.borrow().clone();
    if module == "/" || module.matches('/').count() == 1 {
      return NavigateResult::NotFound;
    }
    let popped = Self::normalize(&format!("{module}/.."));
    let status = self.get_real_path(&popped);
    *self.module.borrow_mut() = popped;
    match status {
      Ok(real) => {
        *self.file.borrow_mut() = Some(real);
        NavigateResult::Success
      }
      Err(NavigateResult::Ambiguous) => NavigateResult::Success,
      Err(status) => status,
    }
  }

  fn to_child(&self, component: &[u8]) -> NavigateResult {
    // 镜像 cpp `VfsNavigator::toChild`：`.config` 目录名保留段拒入
    let Ok(name) = from_utf8(component) else {
      return NavigateResult::NotFound;
    };
    if name == ".config" {
      return NavigateResult::NotFound;
    }
    let module = Self::normalize(&format!("{}/{}", self.module.borrow(), name));
    let status = self.get_real_path(&module);
    *self.module.borrow_mut() = module;
    self.update(status)
  }

  fn is_module_present(&self) -> bool {
    // cpp repl `isModulePresent`：isFile(absoluteRealPath)；纯目录游标为 false
    self
      .file
      .borrow()
      .as_deref()
      .is_some_and(|file| self.tree.files.contains_key(file))
  }

  fn get_chunkname(&self) -> Option<Vec<u8>> {
    // cpp repl `getChunkname`："@" + realPath
    Some(format!("@{}", self.file.borrow().as_deref()?).into_bytes())
  }

  fn get_loadname(&self) -> Option<Vec<u8>> {
    // cpp repl `getLoadName`：absoluteRealPath
    Some(self.file.borrow().as_deref()?.to_owned().into_bytes())
  }

  fn get_cache_key(&self) -> Option<Vec<u8>> {
    // cpp repl：getCacheKey 与 getLoadName 逐字一致（绝对实路径）
    self.get_loadname()
  }

  fn get_config_status(&self) -> ConfigStatus {
    // 镜像 cpp `getConfigStatus`：同名目录级 `.luaurc` 与 `.config.luau` 并存在
    // 歧义，单独存在各自 Present，否则 Absent
    let Some(json_path) = self.config_path(".luaurc") else {
      return ConfigStatus::Absent;
    };
    let Some(luau_path) = self.config_path(".config.luau") else {
      return ConfigStatus::Absent;
    };
    let json = self.tree.configs.contains_key(&json_path);
    let luau = self.tree.configs.contains_key(&luau_path);
    if json && luau {
      ConfigStatus::Ambiguous
    } else if luau {
      ConfigStatus::PresentLuau
    } else if json {
      ConfigStatus::PresentJson
    } else {
      ConfigStatus::Absent
    }
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    // repl 宿主同款：只提供 get_config（不提供 get_alias）
    ConfigBehavior::GetConfig
  }

  fn get_config(&self) -> Option<Vec<u8>> {
    // navigator 在 PresentJson 下取 `.luaurc` 内容
    Some(
      self
        .tree
        .configs
        .get(&self.config_path(".luaurc")?)?
        .as_bytes()
        .to_vec(),
    )
  }

  fn load(&self, l: *mut LuaState, _path: &[u8], chunkname: &[u8], loadname: &[u8]) -> i32 {
    // 镜像 cpp ReplRequirer.cpp `load` 与 ulua-repl-cli 移植：新线程隔离执行，
    // 编译 → luau_load → resume → 结果移回调用线程。
    let chunkname = String::from_utf8_lossy(chunkname);
    let loadname = String::from_utf8_lossy(loadname);
    let Some(source) = self.tree.files.get(loadname.as_ref()).copied() else {
      // Safety: `l` 为 require 同步执行窗口的活跃状态（RequireHost::load 契约）。
      unsafe { luaL_error!(l, "could not read file '{}'", loadname) }
    };

    let bytecode = compile(
      &source,
      &CompileOptions::default(),
      &ParseOptions::default(),
      NoopEncoder,
    );

    // Safety: `l` 为 require 调用帧的活跃状态；以下序列与 cpp `load` 逐句对应：
    // 主线程开新线程 → xmove 到 l → 沙箱化 → luau_load/resume → 结果移回。
    // 线程槽在本帧持有，`ml` 全程随 l 栈槽存活。
    unsafe {
      let gl = lua_mainthread(&*l);
      let ml = lua_newthread(gl);
      lua_xmove(gl, l, 1);
      // new thread needs to have the globals sandboxed
      lua_l_sandboxthread(ml);

      let load_status = luau_load(ml, &chunkname, &bytecode, 0);
      if load_status == LuaStatus::Ok as i32 {
        let run_status = (*ml).resume(l, 0);
        if run_status == LuaStatus::Ok as i32 {
          if (*ml).get_top() != 1 {
            luaL_error!(l, "module must return a single value");
          }
        } else if run_status == LuaStatus::Yield as i32 {
          luaL_error!(l, "module can not yield");
        } else if lua_isstring(&*ml, -1) == 0 {
          luaL_error!(l, "unknown error while running module");
        } else {
          let msg = (*ml).to_str(-1).unwrap_or_default().to_owned();
          luaL_error!(l, "error while running module: {}", msg);
        }
      }

      // add ML result to l stack, then remove the ML thread slot
      lua_xmove(ml, l, 1);
      (*l).remove(-2);
      1
    }
  }
}

/// 夹具：状态生命周期由 [`LuaStateGuard`] 持有（Drop 关闭）。
struct Fixture {
  state: LuaStateGuard,
}

impl Fixture {
  fn new() -> Self {
    // Safety: `lua_l_newstate` 返回的独占活跃状态仅由守卫在 Drop 中 `lua_close`；
    // openlibs/luaopen_require 栈操作各自配平。
    let l = unsafe {
      let l = lua_l_newstate();
      assert!(!l.is_null(), "luaL_newstate failed");
      lua_l_openlibs(l);
      luaopen_require(l, MemHost::new());
      l
    };
    Self {
      state: LuaStateGuard(l),
    }
  }

  fn l(&self) -> *mut LuaState {
    self.state.0
  }

  /// 以 `chunkname` 编译执行 `code`（顶层不包函数），返回 Err(错误消息) 于
  /// 任一环节失败时——等价 cpp 测试夹具的 `runCode`。
  fn run(&mut self, chunkname: &str, code: &str) -> Result<(), String> {
    let bytecode = compile(
      code,
      &CompileOptions::default(),
      &ParseOptions::default(),
      NoopEncoder,
    );
    // Safety: `self.l()` 为本夹具独占活跃状态；code 为本文件内联字面量。
    unsafe {
      if luau_load(self.l(), chunkname, &bytecode, 0) != 0 {
        let msg = (*self.l()).to_str(-1).unwrap_or_default().to_owned();
        return Err(msg);
      }
      if lua_pcall(&mut *self.l(), 0, 0, 0) != 0 {
        let msg = (*self.l()).to_str(-1).unwrap_or_default().to_owned();
        (*self.l()).set_top(0);
        return Err(msg);
      }
      (*self.l()).set_top(0);
    }
    Ok(())
  }

  /// `require(path)` 包 pcall 后把 `ok/err|res` 交回 Lua 断言（`expect` 以 Lua
  /// 表达意，避免在本侧再造表访问 API）。
  fn require_assert(&mut self, path: &str, check: &str) -> Result<(), String> {
    let code = format!("local ok, res = pcall(require, {path:?})\n{check}\n");
    self.run("=stdin", &code)
  }

  fn expect_error(&mut self, path: &str, needle: &str) {
    // needle 原文匹配（plain find，Lua 模式字符免转义）
    let code = format!(
      concat!(
        "local ok, res = pcall(require, {:?})\n",
        "if ok then error('expected failure, got success', 0) end\n",
        "if not string.find(tostring(res), {:?}, 1, true) then\n",
        "  error('message mismatch: '..tostring(res), 0)\n",
        "end\n"
      ),
      path, needle
    );
    let result = self.run("=stdin", &code);
    assert!(
      result.is_ok(),
      "用例 {path} 期望错误片段 {needle:?}，实际: {result:?}"
    );
  }
}

// ---------- 移植用例（对照 cpp/tests/RequireByString.test.cpp 同名 TEST_CASE）----------

/// cpp `requireSimpleRelativePathWithinPcall`。
#[test]
fn require_simple_relative_path() {
  let mut fixture = Fixture::new();
  fixture
    .require_assert(
      "./without_config/dependency",
      "assert(ok and type(res) == 'table' and res[1] == 'result from dependency')",
    )
    .unwrap();
}

/// cpp `requireWithinPcall`（module 内再 require ./dependency）：嵌套 require
/// 经由同一宿主重入（trait `&self` + 内部可变游标的存在理由）。
#[test]
fn require_nested_relative_path() {
  let mut fixture = Fixture::new();
  fixture
    .require_assert(
      "./without_config/module",
      concat!(
        "assert(ok and type(res) == 'table', tostring(res))\n",
        "assert(res[1] == 'result from dependency')\n",
        "assert(res[2] == 'required into module')\n"
      ),
    )
    .unwrap();
}

/// cpp `requireMissingDependency` 类：错误文案逐字对照 `RequireNavigator.cpp`。
#[test]
fn require_missing_child_reports_component() {
  let mut fixture = Fixture::new();
  fixture.expect_error(
    "./without_config/nonexistent",
    "could not resolve child component \"nonexistent\"",
  );
}

/// cpp `requireInvalidPath` 类：无合法前缀的入参在触盘前即报错。
#[test]
fn require_invalid_prefix_reports() {
  let mut fixture = Fixture::new();
  fixture.expect_error("no/prefix", "require path must start with a valid prefix");
}

/// cpp `requireWithinCompile` 类：chunkname 非 `=stdin`/`@` 时 require 被禁。
#[test]
fn require_from_disallowed_chunkname_is_rejected() {
  let mut fixture = Fixture::new();
  // Safety: run 的前置同其函数本体（本夹具独占活跃状态）。
  let result = fixture.run(
    "=coroutine",
    concat!(
      "local ok, res = pcall(require, './without_config/dependency')\n",
      "assert(not ok)\n",
      "assert(string.find(tostring(res), 'require is not supported in this context', 1, true))\n"
    ),
  );
  assert!(result.is_ok(), "非允许上下文用例失败: {result:?}");
}

/// cpp `config_tests/with_config` 形态的别名解析：`.luaurc` 由宿主读出、
/// ulua-require 内解析（PresentJson + GetConfig 行为）。
#[test]
fn require_alias_via_luaurc() {
  let mut fixture = Fixture::new();
  fixture
    .require_assert(
      "./config/alias_requirer",
      concat!(
        "assert(ok and type(res) == 'table', tostring(res))\n",
        "assert(res[1] == 'result from config dependency')\n"
      ),
    )
    .unwrap();
}

/// cpp `validateCache` 断言的内核等价：同路径二次 require 命中缓存同一表。
#[test]
fn require_results_are_cached_by_identity() {
  let mut fixture = Fixture::new();
  fixture
    .require_assert(
      "./without_config/dependency",
      concat!(
        "assert(ok)\n",
        "local again = require('./without_config/dependency')\n",
        "assert(again == res)\n"
      ),
    )
    .unwrap();
}

/// cpp `luaDependency`（`without_config/lua_dependency.lua`）：常规 `.lua`
/// 后缀扫描命中。
#[test]
fn require_loads_lua_extension() {
  let mut fixture = Fixture::new();
  fixture
    .require_assert(
      "./without_config/lua_dependency",
      "assert(ok and type(res) == 'table' and res[1] == 'result from lua_dependency')",
    )
    .unwrap();
}
