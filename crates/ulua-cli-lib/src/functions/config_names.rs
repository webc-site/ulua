//! cpp 侧的配置文件名常量：`Config.h` 的 `kConfigName` 与 `Luau::kLuauConfigName`
//! （`LuauConfig.h`）。`VfsNavigator`（require 侧）与 `CliConfigResolver`（analyze 侧）
//! 两条消费路径共用，避免同名常量各自漂移。
//!
//! 上游只认这两个文件名（cpp `VfsNavigator::getConfigStatus`/`getConfig` 只读
//! `kConfigName`、`kLuauConfigName`；对 `cpp/` 全量 grep `uluarc` 零命中），
//! 故此处不保留任何 legacy 别名。

/// cpp `Luau::kConfigName`。
pub const K_CONFIG_NAME: &str = ".luaurc";
/// cpp `Luau::kLuauConfigName`。
pub const K_LUAU_CONFIG_NAME: &str = ".config.luau";
