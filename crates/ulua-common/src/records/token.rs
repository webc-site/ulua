/// cpp `struct Token`（`Common/include/Luau/TimeTrace.h:29-33`）。
///
/// cpp 镜像工件，sync-cpp 维护；仅存于 `pub(crate)` 的注册表字段，降 `pub(crate)`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) struct Token {
  pub(crate) name: &'static str,
  pub(crate) category: &'static str,
}
