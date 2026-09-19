use alloc::string::String;
use std::io::{Read, stdin};

pub fn read_stdin() -> Option<String> {
  // cpp FileUtils.cpp:211-224 按 std::string 字节读 stdin，非 UTF-8 也进入解析
  // （报语法错而非读入失败）；与 read_file 的字节读取策略一致。
  let mut buffer = Vec::new();
  stdin().read_to_end(&mut buffer).ok()?;
  Some(match String::from_utf8(buffer) {
    Ok(s) => s,
    Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
  })
}
