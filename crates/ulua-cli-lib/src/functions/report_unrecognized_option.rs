/// cpp 各 CLI（Repl/Analyze/Compile/Bytecode）参数循环共用的一行错误文案：
/// 消息原文以 `\n\n` 结尾（cpp `printf` 语义），`eprintln!` 自带一个换行，
/// 故字面量只再补一个 `\n`。收口于此以免各 CLI 复制同一字符串。
pub fn report_unrecognized_option(arg: &str) {
  eprintln!("Error: Unrecognized option '{arg}'.\n");
}
