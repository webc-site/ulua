/// cpp 各 CLI 文件读取/解析失败路径共用的一行错误文案（`fprintf(stderr,
/// "Error opening %s\n", name)` 形态）。收口于此以免各 CLI 复制同一字符串；
/// repl-cli 的 dump 建文件门面 `create_dump_writer` 消息含主题词
/// （`Error opening <subject> <path>`），形态不同，不并入。
pub fn report_open_error(name: &str) {
  eprint!("{}", report_open_error_string(name));
}

/// [`report_open_error`] 的无副作用版本，供并行编译按文件缓冲输出。
pub fn report_open_error_string(name: &str) -> String {
  format!("Error opening {name}\n")
}
