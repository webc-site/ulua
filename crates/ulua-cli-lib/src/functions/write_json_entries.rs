use std::io::{Result, Write};

/// JSON 条目循环的统一写法：条目之间写 `,\n`，最后一条以 `\n` 收尾。
/// 取代各 serialize_* 里 peekable 前瞻与下标比较两种手写变体；
/// 条目体经回调写回同一个 `out`，避免对 writer 的双重可变借用。
pub fn write_json_entries<T, I, W, F>(out: &mut W, entries: I, mut write_entry: F) -> Result<()>
where
  W: Write,
  I: IntoIterator<Item = T>,
  F: FnMut(&mut W, &T) -> Result<()>,
{
  let mut entries = entries.into_iter().peekable();
  while let Some(entry) = entries.next() {
    write_entry(out, &entry)?;
    if entries.peek().is_some() {
      writeln!(out, ",")?;
    } else {
      writeln!(out)?;
    }
  }
  Ok(())
}
