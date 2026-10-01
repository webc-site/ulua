//! 字节链上的 UTF-8 边界集合：ulua-config 的接口以 `String`/`&str` 为参数，
//! 故别名键与配置文件文本必须在此转换。合法 UTF-8 时转换是恒等映射，
//! 因此任何 UTF-8 路径/别名的判定结果与 cpp 的 `std::string` 比较完全一致；
//! 含非法 UTF-8 的字节只可能来自嵌入方，这类值本就不可能出现在
//! 由配置文本产生的 `String` 键表里，两侧同样查不到。

use alloc::{borrow::Cow, string::String};

/// 字节 → 拥有的 UTF-8 文本（别名表键、别名值等 `String` 字段）。
pub(crate) fn utf8_owned(bytes: &[u8]) -> String {
  String::from_utf8_lossy(bytes).into_owned()
}

/// 字节 → 借用优先的 UTF-8 文本视图（配置文件内容：合法 UTF-8 时零拷贝）。
pub(crate) fn utf8_view(bytes: &[u8]) -> Cow<'_, str> {
  String::from_utf8_lossy(bytes)
}
