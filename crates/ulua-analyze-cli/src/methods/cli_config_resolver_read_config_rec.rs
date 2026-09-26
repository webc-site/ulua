use alloc::{
  boxed::Box,
  format,
  string::{String, ToString},
};
use core::ffi::{c_int, c_void};
use std::panic::panic_any;

use ulua_analysis::records::{
  time_limit_error::TimeLimitError, type_check_limits::TypeCheckLimits,
  user_cancel_error::UserCancelError,
};
use ulua_cli_lib::functions::{
  config_names::{K_CONFIG_NAME, K_LUAU_CONFIG_NAME},
  get_parent_path::get_parent_path,
  is_file::is_file,
  join_paths_file_utils::join_paths,
  read_file::read_file,
};
use ulua_common::functions::get_clock::get_clock;
use ulua_config::{
  functions::{extract_luau_config::extract_luau_config, parse_config::parse_config},
  records::{
    alias_options::AliasOptions,
    config::Config,
    config_options::ConfigOptions,
    interrupt_callbacks::{ConfigInitCallback, InterruptCallbacks, attach_threaddata_init},
  },
};
use ulua_vm::{functions::lua_getthreaddata::lua_getthreaddata, records::lua_state::LuaState};

use crate::records::{
  cli_config_resolver::CliConfigResolver, luau_config_interrupt_info::LuauConfigInterruptInfo,
};

/// The Lua interrupt callback used while evaluating a `.config.luau` file.
///
/// Mirrors the C++ lambda:
/// ```cpp
/// callbacks.interruptCallback = [](LuaState* l, int gc) {
///     LuauConfigInterruptInfo* info = static_cast<LuauConfigInterruptInfo*>(lua_getthreaddata(l));
///     if (info->limits.finish_time && getClock() > *info->limits.finish_time)
///         throw TimeLimitError{info->module};
///     if (info->limits.cancellation_token && info->limits.cancellation_token->requested())
///         throw UserCancelError{info->module};
/// };
/// ```
/// The `throw` becomes an unwinding panic carrying the typed error, which propagates
/// across the `extern "C-unwind"` boundary just as the C++ exception does.
///
/// # Safety
/// 由 VM 以合法 `LuaState*` 调用（`InterruptCallbacks` 契约）；线程数据槽里
/// 只可能挂着 `luau_config_init` 布线的 `*mut LuauConfigInterruptInfo` 或 null，
/// null 已在函数首行守卫（cpp 原版直接解引用，行为收敛为 no-op）。
pub(crate) unsafe extern "C-unwind" fn luau_config_interrupt(l: *mut LuaState, _gc: c_int) {
  // Safety: 依函数 `# Safety` 契约，`l` 合法；线程数据槽只可能挂
  // `*mut LuauConfigInterruptInfo` 或 null，故下方判空成立。
  let info = unsafe { lua_getthreaddata(l) as *const LuauConfigInterruptInfo };
  if info.is_null() {
    // cpp 原版直接解引用，行为收敛为 no-op。
    return;
  }
  // Safety: 非空时该指针指向 `luau_config_init` 布线的栈帧局部 `info`，
  // `extract_luau_config` 同步返回前回调窗口内始终存活、且无人并发改写。
  let info = unsafe { &*info };

  // 以下为纯安全逻辑（时钟/令牌只读 + 类型化 panic），出圈。
  if let Some(finish_time) = info.limits.finish_time()
    && get_clock() > finish_time
  {
    panic_any(TimeLimitError::time_limit_error_time_limit_error(
      &info.module,
    ));
  }
  if let Some(token) = info.limits.cancellation_token()
    && token.requested()
  {
    panic_any(UserCancelError::new(info.module.clone()));
  }
}

impl CliConfigResolver {
  /// C++ `const Config& readConfigRec(const std::string& path, const TypeCheckLimits& limits) const`
  /// (`CLI/src/Analyze.cpp:252-320`).
  ///
  /// 逻辑 const：缓存读写走 `UnsafeCell`（C++ `mutable`），单线程契约见
  /// [`CliConfigResolver`] 字段文档。
  pub(crate) fn read_config_rec(&self, path: &str, limits: &TypeCheckLimits) -> &Config {
    // auto it = configCache.find(path); if (it != configCache.end()) return it->second;
    // 借用只在单语句内存在：本函数会递归调用自身，长活 `&mut` 会自重叠（UB）。
    // Safety: 单线程契约（见 `CliConfigResolver` 字段文档），缓存 cell 的读窗口
    // 内无任何并发可变借用。
    if let Some(cached) = unsafe { &*self.config_cache.get() }.get(path) {
      // Safety: 单线程契约（见 CliConfigResolver 字段文档），返回的 &Config
      // 指向 Box 堆上的值，地址随表 grow 保持稳定，与 C++ `return it->second` 同义。
      return cached;
    }

    // std::optional<std::string> parent = getParentPath(path);
    // Config result = parent ? readConfigRec(*parent, limits) : defaultConfig;
    let mut result: Config = match get_parent_path(path) {
      Some(parent) => self.read_config_rec(&parent, limits).clone(),
      None => self.default_config.clone(),
    };

    // std::optional<std::string> configPath = joinPaths(path, kConfigName);
    // if (!isFile(*configPath)) configPath = std::nullopt;
    let config_path_candidate = join_paths(path, K_CONFIG_NAME, false);
    let config_path: Option<String> = if is_file(&config_path_candidate) {
      Some(config_path_candidate)
    } else {
      None
    };

    // std::optional<std::string> luauConfigPath = joinPaths(path, kLuauConfigName);
    // if (!isFile(*luauConfigPath)) luauConfigPath = std::nullopt;
    let luau_config_path_candidate = join_paths(path, K_LUAU_CONFIG_NAME, false);
    let luau_config_path: Option<String> = if is_file(&luau_config_path_candidate) {
      Some(luau_config_path_candidate)
    } else {
      None
    };

    if let (Some(config_path), Some(_)) = (&config_path, &luau_config_path) {
      // configErrors.emplace_back(*configPath, "Both ... files exist");
      let ambiguous_error = format!(
        "Both {} and {} files exist",
        K_CONFIG_NAME, K_LUAU_CONFIG_NAME
      );
      // Safety: 单线程契约（`CliConfigResolver` 字段文档）；错误表仅在回调
      // 栈帧内追加，无并存借用。
      unsafe { &mut *self.config_errors.get() }.push((config_path.clone(), ambiguous_error));
    } else if let Some(config_path) = config_path.as_ref() {
      // if (std::optional<std::string> contents = readFile(*configPath))
      if let Some(contents) = read_file(config_path) {
        let alias_opts = AliasOptions {
          config_location: Some(config_path.clone()),
          overwrite_aliases: true,
        };

        let opts = ConfigOptions {
          compat: false,
          alias_options: Some(alias_opts),
        };

        // std::optional<std::string> error = parseConfig(*contents, result, opts);
        if let Err(error) = parse_config(&contents, &mut result, &opts) {
          // Safety: 单线程契约同上；写窗口瞬时收敛，无并存借用。
          unsafe { &mut *self.config_errors.get() }.push((config_path.clone(), error.to_string()));
        }
      }
    } else if let Some(luau_config_path) = luau_config_path.as_ref() {
      // if (std::optional<std::string> contents = readFile(*luauConfigPath))
      if let Some(contents) = read_file(luau_config_path) {
        // C++ sets `aliasOpts.configLocation = *configPath;` here, but in this
        // branch `configPath` is `std::nullopt` (dereferencing it is UB upstream);
        // faithfully carry the (absent) value through.
        let alias_opts = AliasOptions {
          config_location: config_path.clone(),
          overwrite_aliases: true,
        };

        // The interrupt info lives on the stack for the duration of the
        // synchronous extractLuauConfig call (mirroring the C++ stack local
        // whose address is stored via lua_setthreaddata).
        let mut info = LuauConfigInterruptInfo {
          limits: limits.clone(),
          module: luau_config_path.clone(),
        };
        let info_ptr: *mut LuauConfigInterruptInfo = &mut info;

        let callbacks = InterruptCallbacks {
          // 静态分派对：cpp 原版闭包 `[&info](LuaState* l)` 仅捕获一枚 info
          // 指针，改为具名 `attach_threaddata_init` + 该指针转手 userdata。
          // Safety 契约随函数文档：`info` 是本栈帧的局部（cpp 原版同样是栈
          // 局部 `&info`），`extract_luau_config` 同步返回前回调即失效，挂入
          // VM 线程的纯指针数据在窗口内始终指向存活的 `LuauConfigInterruptInfo`。
          init_callback: Some(ConfigInitCallback {
            callback: attach_threaddata_init,
            userdata: info_ptr.cast::<c_void>(),
          }),
          interrupt_callback: Some(luau_config_interrupt),
        };

        // std::optional<std::string> error = extractLuauConfig(*contents, result, aliasOpts, callbacks);
        if let Err(error) = extract_luau_config(&contents, &mut result, Some(alias_opts), callbacks)
        {
          // Safety: 单线程契约同上；写窗口瞬时收敛，无并存借用。
          unsafe { &mut *self.config_errors.get() }
            .push((luau_config_path.clone(), error.to_string()));
        }
      }
    }

    // return configCache[path] = result;
    // Safety: 单线程契约（`CliConfigResolver` 字段文档）；插入前当前函数持有的
    // 缓存共享借用（find 分支）早已收敛，Box 值定址保证已返回的 `&Config` 不悬垂。
    let cache = unsafe { &mut *self.config_cache.get() };

    // 显式 `&**` 解出 Box 堆值再转共享借用——不靠 `as _` 目标推断，读者可直接
    // 核对指向的是堆上 Config 而非表内槽位
    cache
      .entry(path.to_string())
      .or_insert_with(move || Box::new(result))
  }
}
