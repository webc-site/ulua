/// Require 路径处理的字节级常量（Lua 路径是字节串，非 UTF-8）。
/// 别名路径前缀字节（对应 cpp `path[0] == '@'`）。
pub(crate) const ALIAS_PREFIX: u8 = b'@';
/// 路径分隔符字节（对应 cpp `splitPath` 的 `'/'`）。cpp `navigateThroughPath`
/// 循环复用 `splitPath`，Rust 侧由 `Navigator::navigate_through_path` 的
/// `path.split(…)` 迭代器承担同语义。
pub(crate) const PATH_SEPARATOR: u8 = b'/';
/// cpp `navigate` 里被 `std::replace` 归一成分隔符的反斜杠字节。
pub(crate) const PATH_SEPARATOR_ALT: u8 = b'\\';
