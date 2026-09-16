use alloc::{boxed::Box, string::String};

use ulua_cli_lib::records::vfs_navigator::VfsNavigator;

use crate::records::luau_config_interrupt_info::LuauConfigInterruptInfo;

/// Port of `struct FileNavigationContext : Luau::Require::NavigationContext`
/// (`CLI/include/Luau/AnalyzeRequirer.h`).
///
/// The C++ `: NavigationContext` base relationship is expressed in Rust by
/// implementing [`ulua_require::records::navigation_context::NavigationContextTrait`]
/// for this type (see `methods/file_navigation_context_navigation_context_trait.rs`),
/// rather than by embedding the (non-constructible-from-here) base struct.
pub struct FileNavigationContext {
  pub(crate) requirer_path: String,
  pub(crate) vfs: VfsNavigator,
  /// Backing storage for the `luauConfigInit` / `luauConfigInterrupt` callbacks
  /// installed by `CliFileResolver::resolveModule`. In the C++ this is a stack
  /// local (`LuauConfigInterruptInfo info`) whose address is captured by the
  /// callbacks; here it is boxed and owned by the context so the captured raw
  /// pointer stays valid for the duration of `navigate`.
  pub(crate) interrupt_info: Option<Box<LuauConfigInterruptInfo>>,
}
