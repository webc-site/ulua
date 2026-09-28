//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：对应 C++ `isRequireAllowed`，
//! 仅允许 `=stdin` 或以 `@` 开头的 chunkname 触发 require。两侧判定条件语义
//! 相同（`== "=stdin"` 或 首字节为 `@`）。chunkname 为原始字节串，非 UTF-8
//! 字节不做校验。

/// 判定是否允许从该 requirer chunkname 发起 require。
pub fn is_require_allowed(requirer_chunkname: &[u8]) -> bool {
  requirer_chunkname == b"=stdin" || requirer_chunkname.first() == Some(&b'@')
}
