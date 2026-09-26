use memchr::memmem;

/// cpp `VM/src/lstrlib.cpp:lmemfind`：在 `haystack` 中查找子串 `needle` 的
/// 首次出现位置。
///
/// 用 `memchr::memmem::find`（Two-Way + SIMD）统一全平台实现；空 needle 命中
/// 位置 0、needle 长于 haystack 无命中，均与 cpp 边界行为一致。
pub(crate) fn lmemfind(haystack: &[u8], needle: &[u8]) -> Option<usize> {
  memmem::find(haystack, needle)
}
