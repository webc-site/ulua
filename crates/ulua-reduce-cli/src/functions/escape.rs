//! `CLI/src/Reduce.cpp` 的自由函数 `escape`：由旧的 `Reducer::escape` 方法迁移而来
//! （cpp 版本不依赖 `this`，收为自由函数与上游一致，也便于 `run` 复用）。
use alloc::string::String;

/// `std::string escape(const std::string& s)` (`CLI/src/Reduce.cpp:126-140`)：
/// 包上双引号并转义其中嵌入的 `"`，供 shell 内嵌。
pub fn escape(s: &str) -> String {
  // cpp 逐 `char`（字节）复制、只转义 `"`，其余字节原样保留。Rust 按 char 迭代：
  // 非 ASCII 字节若走 `b as char` 会被重编码成 Latin-1（如 "é" → "Ã©"），
  // 改写脚本路径；chars 迭代原字符原样透传，产生的字节与 cpp 逐字节拷贝一致。
  let mut result = String::with_capacity(s.len() + 20);
  result.push('"');

  for ch in s.chars() {
    if ch == '"' {
      result.push('\\');
    }
    result.push(ch);
  }

  result.push('"');
  result
}
