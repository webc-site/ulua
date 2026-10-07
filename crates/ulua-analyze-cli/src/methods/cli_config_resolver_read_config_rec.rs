use alloc::{
  boxed::Box,
  format,
  string::{String, ToString},
};
use core::{ffi::c_void, ptr::NonNull};
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
    alias_options::AliasOptions, config::Config, config_options::ConfigOptions,
    interrupt_callbacks::InterruptCallbacks,
  },
};
use ulua_vm::records::lua_state::LuaState;

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
/// 只可能挂着 `luau_config_thread_data` 交出的 `NonNull<LuauConfigInterruptInfo>`
/// 地址或 null，null 已收编为 `Option` 的缺席臂（cpp 原版直接解引用，行为收敛为
/// no-op）。
// 真边界：本指针经 `InterruptCallbacks::interrupt_callback` 写入 VM 的
// `LuaCallbacks::interrupt` 槽（`ulua-vm` 声明即 `Option<unsafe extern
// "C-unwind" fn(*mut LuaState, i32)>`，safepoint 处按 C ABI 调用），故保留
// `extern "C-unwind"` + 裸 `LuaState` 指针；形参直接采用槽位的原生类型
// `i32`（`c_int` 即其别名，无需再引 `core::ffi`，review.md §10/§7）。
// DELIBERATE DEVIATION（review.md §9.3）：C-ABI interrupt 槽回调，`throw` 以
// 展开型 panic 跨 `extern "C-unwind"` 传播（等价 cpp 异常）；线程数据槽裸指针仅
// 在本边界单次解码。
pub(crate) unsafe extern "C-unwind" fn luau_config_interrupt(l: *mut LuaState, _gc: i32) {
  // 边界解码：`l` 只在物化这一句里被解引用，取回线程数据槽的裸载荷。
  // Safety: 依函数 `# Safety` 契约，`l` 为 VM 交出的合法活跃状态，本行只读其线程
  // 数据槽（不移动栈），借用窗止于当句。
  let thread_data = unsafe { (*l).get_thread_data() };

  // 可空载荷收编为 Option（review.md §2）：null 即「本回调无配对上下文」，缺席态
  // 由类型表达，不再是裸指针 + is_null 两态；cpp 原版直接解引用，此处行为收敛为
  // no-op。
  let Some(info) = NonNull::new(thread_data.cast::<LuauConfigInterruptInfo>()) else {
    return;
  };
  // Safety: 非空时该地址由 `luau_config_thread_data` 交出、`extract_config` 在配置
  // 执行窗口前单点挂接，指向 `read_config_rec` 的本栈帧局部 `info`；
  // `extract_luau_config` 同步返回前的回调窗口内始终存活且无人并发改写。
  let info = unsafe { info.as_ref() };

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
  /// 缓存只读面：`UnsafeCell` 裸指针的解码单点（review.md §2「把 `unsafe` 关进有契约
  /// 的最小边界」），`read_config_rec` 因此全程安全 Rust。
  ///
  /// 契约（本 fn 内唯一 `unsafe` 的依据）：
  /// - 单线程：resolver 由 Analyze 主线程独占驱动（见 [`CliConfigResolver`] 字段文档），
  ///   解码窗口内不存在其它可变借用；
  /// - 地址稳定：值以 `Box` 定址，表 grow 只搬内联槽位、不搬堆上的 `Config`，故借出的
  ///   `&Config` 寿命可提升到 `&self`（等价 cpp `return it->second`）。
  pub(crate) fn cached_config(&self, path: &str) -> Option<&Config> {
    // Safety: 上方两条契约——单线程故无重叠可变借用，Box 定址使返回引用独立于表结构。
    unsafe { (&*self.config_cache.get()).get(path).map(Box::as_ref) }
  }

  /// 缓存写入面：同样把裸指针解码收在一处；返回新插入（或已存在）项的共享引用。
  ///
  /// 契约：同 [`Self::cached_config`]；写入前调用方持有的缓存共享借用（find 分支）早已
  /// 收敛，`or_insert_with` 命中已有项时原地址不变，故先前交出的 `&Config` 仍有效。
  pub(crate) fn cache_config(&self, path: &str, config: Config) -> &Config {
    // Safety: 见本 fn 与 [`Self::cached_config`] 的契约（单线程 + Box 堆上定址）。
    unsafe {
      let cache = &mut *self.config_cache.get();
      cache
        .entry(path.to_string())
        .or_insert_with(move || Box::new(config))
    }
  }

  /// C++ `const Config& readConfigRec(const std::string& path, const TypeCheckLimits& limits) const`
  /// (`CLI/src/Analyze.cpp:252-320`).
  ///
  /// 逻辑 const：缓存读写经上面两个 `UnsafeCell` 收口面（C++ `mutable`），单线程契约见
  /// [`CliConfigResolver`] 字段文档；本函数体本身零 `unsafe`、零裸指针。
  pub(crate) fn read_config_rec(&self, path: &str, limits: &TypeCheckLimits) -> &Config {
    // auto it = configCache.find(path); if (it != configCache.end()) return it->second;
    if let Some(cached) = self.cached_config(path) {
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
    let config_path: Option<String> =
      is_file(&config_path_candidate).then_some(config_path_candidate);

    // std::optional<std::string> luauConfigPath = joinPaths(path, kLuauConfigName);
    // if (!isFile(*luauConfigPath)) luauConfigPath = std::nullopt;
    let luau_config_path_candidate = join_paths(path, K_LUAU_CONFIG_NAME, false);
    let luau_config_path: Option<String> =
      is_file(&luau_config_path_candidate).then_some(luau_config_path_candidate);

    if let (Some(config_path), Some(_)) = (&config_path, &luau_config_path) {
      // configErrors.emplace_back(*configPath, "Both ... files exist");
      let ambiguous_error = format!(
        "Both {} and {} files exist",
        K_CONFIG_NAME, K_LUAU_CONFIG_NAME
      );
      self
        .config_errors
        .borrow_mut()
        .push((config_path.clone(), ambiguous_error));
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
          self
            .config_errors
            .borrow_mut()
            .push((config_path.clone(), error.to_string()));
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
        let info = LuauConfigInterruptInfo {
          limits: limits.clone(),
          module: luau_config_path.clone(),
        };

        let callbacks = InterruptCallbacks {
          // 真边界：`thread_data` 是 VM lightuserdata 线程数据槽（cpp 原版
          // `lua_setthreaddata(l, &info)`，`Analyze.cpp:194-197` 同形闭包）的
          // 转手地址，由 `extract_config` 在配置执行窗口前单点挂接、不解引用。
          // 契约：`info` 是本栈帧的局部（cpp 原版同样是栈局部 `&info`），
          // `extract_luau_config` 同步返回前地址始终指向存活的
          // `LuauConfigInterruptInfo`，窗口结束后随沙箱状态失效。
          thread_data: Some(NonNull::from(&info).cast::<c_void>()),
          interrupt_callback: Some(luau_config_interrupt),
        };

        // std::optional<std::string> error = extractLuauConfig(*contents, result, aliasOpts, callbacks);
        if let Err(error) = extract_luau_config(&contents, &mut result, Some(alias_opts), callbacks)
        {
          self
            .config_errors
            .borrow_mut()
            .push((luau_config_path.clone(), error.to_string()));
        }
      }
    }

    // return configCache[path] = result;
    self.cache_config(path, result)
  }
}
