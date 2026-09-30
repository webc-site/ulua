/// C `isspace`（cpp `<cctype>`）等价空白集：空格、`\t`、`\n`、`\v`、`\f`、`\r`。
/// Rust `is_ascii_whitespace` 不含 `\v`（0x0B），VM 多处须与 C 语义逐位对齐，故单源于此。
pub const fn is_c_space(c: u8) -> bool {
  matches!(c, b' ' | b'\t' | b'\n' | 0x0B | 0x0C | b'\r')
}
