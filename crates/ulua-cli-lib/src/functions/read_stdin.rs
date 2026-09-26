use alloc::string::String;
use std::io::{Read, stdin};

pub fn read_stdin() -> Option<String> {
  // cpp FileUtils.cpp:211-224 按 std::string 字节读 stdin，非 UTF-8 也进入解析
  // （报语法错而非读入失败）；与 read_file 的字节读取策略一致。
  let mut buffer = Vec::new();
  // DELIBERATE DEVIATION：cpp readStdin 用 fgets 分块 + strlen 语义，每个
  // 4096 块内首个 NUL 后字节被静默丢弃；这里全量保留（含 NUL 的输入是
  // 宿主管道异常场景，复刻丢数据只会放大错误）。非 UTF-8 字节 lossy。
  stdin().read_to_end(&mut buffer).ok()?;
  Some(match String::from_utf8(buffer) {
    Ok(s) => s,
    Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
  })
}
