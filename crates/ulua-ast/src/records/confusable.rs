/// Source: `Ast/include/Luau/Confusables.h`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Confusable {
  pub(crate) codepoint: u32,
  /// ASCII 骨架文本：C++ `char text[5]`（NUL 填充定长数组）的 Rust 现代化——
  /// 直接持有静态字符串切片，省去运行期 NUL 扫描与 UTF-8 还原。
  pub(crate) text: &'static str,
}

impl Confusable {
  /// 表项构造（cpp `{codepoint, text}` 聚合初始化的直接对应）：
  /// kConfusables 表 1786 条共用，取代每条 4 行的结构体字面量样板。
  pub(crate) const fn new(codepoint: u32, text: &'static str) -> Self {
    Self { codepoint, text }
  }
}
