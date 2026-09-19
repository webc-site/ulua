//! cpp 侧的配置文件名常量：`Config.h` 的 `kConfigName`、`LuauConfig.h` 的
//! `kLuauConfigName`，以及 legacy 的 `.uluarc`。`VfsNavigator`（require 侧）与
//! `CliConfigResolver`（analyze 侧）两条消费路径共用，避免同名常量各自漂移。

/// cpp `Luau::kConfigName`。
pub const K_CONFIG_NAME: &str = ".luaurc";
/// legacy JSON 配置文件名（`VfsNavigator` 的向后兼容分支）。
pub const K_LEGACY_CONFIG_NAME: &str = ".uluarc";
/// cpp `Luau::kLuauConfigName`。
pub const K_LUAU_CONFIG_NAME: &str = ".config.luau";
