use alloc::{
  format,
  rc::Rc,
  string::{String, ToString},
};
use core::ffi::{c_int, c_void};
// Luau::kConfigName / Luau::kLuauConfigName (Config.h / LuauConfig.h).
use std::panic::panic_any;

use ulua_analysis::records::{
  time_limit_error::TimeLimitError, type_check_limits::TypeCheckLimits,
  user_cancel_error::UserCancelError,
};
use ulua_cli_lib::functions::{
  get_parent_path::get_parent_path, is_file::is_file,
  join_paths_file_utils_alt_b::join_paths_string_view_string_view, read_file::read_file,
};
use ulua_common::functions::get_clock::get_clock;
use ulua_config::{
  functions::{extract_luau_config::extract_luau_config, parse_config::parse_config},
  records::{
    alias_options::AliasOptions, config::Config, config_options::ConfigOptions,
    interrupt_callbacks::InterruptCallbacks,
  },
};
use ulua_vm::{
  functions::{lua_getthreaddata::lua_getthreaddata, lua_setthreaddata::lua_setthreaddata},
  type_aliases::lua_state::lua_State,
};

use crate::records::{
  cli_config_resolver::CliConfigResolver, luau_config_interrupt_info::LuauConfigInterruptInfo,
};
const K_CONFIG_NAME: &str = ".luaurc";
const K_LUAU_CONFIG_NAME: &str = ".config.luau";

/// The Lua interrupt callback used while evaluating a `.config.luau` file.
///
/// Mirrors the C++ lambda:
/// ```cpp
/// callbacks.interruptCallback = [](lua_State* l, int gc) {
///     LuauConfigInterruptInfo* info = static_cast<LuauConfigInterruptInfo*>(lua_getthreaddata(l));
///     if (info->limits.finish_time && getClock() > *info->limits.finish_time)
///         throw TimeLimitError{info->module};
///     if (info->limits.cancellation_token && info->limits.cancellation_token->requested())
///         throw UserCancelError{info->module};
/// };
/// ```
/// The `throw` becomes an unwinding panic carrying the typed error, which propagates
/// across the `extern "C-unwind"` boundary just as the C++ exception does.
pub(crate) unsafe extern "C-unwind" fn luau_config_interrupt(l: *mut lua_State, _gc: c_int) {
  unsafe {
    let info = lua_getthreaddata(l) as *const LuauConfigInterruptInfo;
    if info.is_null() {
      return;
    }
    let info = &*info;

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
}

impl CliConfigResolver {
  /// C++ `const Config& readConfigRec(const std::string& path, const TypeCheckLimits& limits) const`
  /// (`CLI/src/Analyze.cpp:252-320`).
  pub fn read_config_rec(&mut self, path: &str, limits: &TypeCheckLimits) -> &Config {
    // auto it = configCache.find(path); if (it != configCache.end()) return it->second;
    if self.config_cache.contains_key(path) {
      return &self.config_cache[path];
    }

    // std::optional<std::string> parent = getParentPath(path);
    // Config result = parent ? readConfigRec(*parent, limits) : defaultConfig;
    let mut result: Config = match get_parent_path(path) {
      Some(parent) => self.read_config_rec(&parent, limits).clone(),
      None => self.default_config.clone(),
    };

    // std::optional<std::string> configPath = joinPaths(path, kConfigName);
    // if (!isFile(*configPath)) configPath = std::nullopt;
    let config_path_candidate = join_paths_string_view_string_view(path, K_CONFIG_NAME);
    let config_path: Option<String> = if is_file(&config_path_candidate) {
      Some(config_path_candidate)
    } else {
      None
    };

    // std::optional<std::string> luauConfigPath = joinPaths(path, kLuauConfigName);
    // if (!isFile(*luauConfigPath)) luauConfigPath = std::nullopt;
    let luau_config_path_candidate = join_paths_string_view_string_view(path, K_LUAU_CONFIG_NAME);
    let luau_config_path: Option<String> = if is_file(&luau_config_path_candidate) {
      Some(luau_config_path_candidate)
    } else {
      None
    };

    if config_path.is_some() && luau_config_path.is_some() {
      // configErrors.emplace_back(*configPath, "Both ... files exist");
      let ambiguous_error = format!(
        "Both {} and {} files exist",
        K_CONFIG_NAME, K_LUAU_CONFIG_NAME
      );
      self
        .config_errors
        .push((config_path.clone().unwrap(), ambiguous_error));
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
        if let Some(error) = parse_config(&contents, &mut result, &opts) {
          self.config_errors.push((config_path.clone(), error));
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
          init_callback: Some(Rc::new(move |l: *mut lua_State| unsafe {
            lua_setthreaddata(l, info_ptr as *mut c_void);
          })),
          interrupt_callback: Some(luau_config_interrupt),
        };

        // std::optional<std::string> error = extractLuauConfig(*contents, result, aliasOpts, callbacks);
        if let Some(error) =
          extract_luau_config(&contents, &mut result, Some(alias_opts), callbacks)
        {
          self.config_errors.push((luau_config_path.clone(), error));
        }
      }
    }

    // return configCache[path] = result;
    self.config_cache.insert(path.to_string(), result);
    &self.config_cache[path]
  }
}
